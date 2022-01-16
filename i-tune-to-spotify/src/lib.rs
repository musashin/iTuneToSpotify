mod utils;
extern crate xml;
extern crate stdweb;
extern crate web_sys;
use stdweb::web::document;

use std::collections::HashSet;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};

use oauth2::{
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    RedirectUrl,
    Scope
  
};
use oauth2::basic::BasicClient;

use reqwest;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

use regex::Regex;

async fn get_user_name(token: &str)
{
    log!("trying to get user name");
    let client = reqwest::Client::new();
    let response = client
    .get("https://api.spotify.com/v1/me")
    .header("Authorization:", format!("Bearer {}", token))
    .send()
    .await
    .unwrap();

    log!("response {:?}",response);
}

#[wasm_bindgen]
pub fn on_load() {
    log!("Loading Page");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let url = document.url().expect("should have a URL");
    log!("Loading Page {}",url);

    ///#access_token=([\w-]*)&token_type=([\w]*)&expires_in=([\d]*)&state=([\w]*)
    let re = Regex::new(r"#access_token=([\w-]*)&token_type=([\w]*)&expires_in=([\d]*)&state=([\w]*)").unwrap();
    let caps = re.captures(&url).unwrap();


    log!("caps[0] {}",&caps[0]);
    log!("caps[1] {}",&caps[1]);
    log!("caps[2] {}",&caps[2]);
    log!("caps[3] {}",&caps[3]);
    log!("caps[4] {}",&caps[4]);

    //TODO handle the case where we get errors... not token

    if let Ok(Some(local_storage)) = window.local_storage()
    {
        if let Ok(Some(value)) = local_storage.get_item(&"spotify_auth_state")
        {
            log!("Stored State {} URL State {}",value,&caps[4]);

            get_user_name(&caps[1]);
            if value == &caps[4]
            {
                get_user_name(&caps[1]);

            }

        }
    }



    

}


#[wasm_bindgen()]
pub fn loadlib(file_reader : web_sys::FileReader) {
    //alert(file_reader.result());


    match file_reader.result() {
        Ok(_t)=>  match _t.as_string(){
            Some(file_text) => parse_library(&file_text),
            None => alert("invalid file")

        }
        Err(_e) => alert("did not get it really")
    }

        
    
}

#[wasm_bindgen()]
pub fn spot_login() {

      log_into_spotify();
 
    
}

#[derive(PartialEq)]
enum ParseEvent {
    Parsing,
    KeyFound,
    KeyTextFound,
    KeyChildFound
}


#[derive(PartialEq)]
enum Field{
    NoField,
    Date,
    Artist,
    Album
}

#[derive(Debug, Eq, Clone)]
struct Artist {
    name: String,
    albums: HashSet::<String>
}


impl PartialEq for Artist {
    fn eq(&self, other: &Artist) -> bool {
        self.name == other.name
    }
}

impl Hash for Artist {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Borrow<String> for Artist {
    fn borrow(&self) -> &String {
        &self.name
    }
}

//todo externalize in module library
//todo make async
fn parse_library(library_file_text:&str)
{
  



    let parser = xml::reader::EventReader::from_str(library_file_text);
    let mut parse_state = ParseEvent::Parsing;
    let mut field = Field::NoField;
    let mut library = HashSet::<Artist>::new();
    let mut current_artist:Option<String> = None;
    

    //todo: only work if album...after artist
    for event in parser {
        match event {
        Ok(xml::reader::XmlEvent::StartElement{name,attributes:_,namespace:_}) => {
            if name.local_name.trim() == "key" {parse_state = ParseEvent::KeyFound; }
            if parse_state == ParseEvent::KeyTextFound{

                if name.local_name.trim() == "date" && field == Field::Date {parse_state = ParseEvent::KeyChildFound;}
                if name.local_name.trim() == "string" && field == Field::Artist {parse_state = ParseEvent::KeyChildFound;}
                if name.local_name.trim() == "string" && field == Field::Album {parse_state = ParseEvent::KeyChildFound;}
            }
            
           
        }
        Ok(xml::reader::XmlEvent::Characters(text)) => {
            if parse_state == ParseEvent::KeyFound {
                parse_state = ParseEvent::KeyTextFound;

                if text.trim() == "Date"{field = Field::Date;}
                if text.trim() == "Artist"{field = Field::Artist;}
                if text.trim() == "Album"{field = Field::Album;}
                

            }
            else if parse_state==ParseEvent::KeyChildFound {

                match field {
                    Field::Date=> log!("Date {}", text) ,
                    Field::Artist=> {
                        current_artist = Some(text.clone());
                        library.insert(Artist{name: text, albums: HashSet::<String>::new()});}
                    Field::Album=> {
                      //log!("adding album {:?} to arist {:?}",text,current_artist);
    
                      match &current_artist{
                        Some(album_artist) => { 
                            let lib_artist_entry = library.get(&album_artist.to_string()).unwrap();
                            let mut new_lib_artist_entry: Artist = lib_artist_entry.clone();
                            new_lib_artist_entry.albums.insert(text);
                            library.replace(new_lib_artist_entry);

                            
                        }
                        _ => ()
                      }
                   
                         //
                      
                    }
                    _=>()

                }

                parse_state = ParseEvent::Parsing;
                field = Field::NoField;

            }

            
        }
        Ok(_) => (),
        Err(why)=>log!("unrecognized event {:?}",&why)
        }
        
    }

    log!("Lib {:#?}", library) 


   
}





fn log_into_spotify()->Result<(), Box<dyn std::error::Error>> {

    log!("login started");
    let client =
    BasicClient::new(
        ClientId::new("6ccf804ccf54423283485223d1f93a66".to_string()),
        Some(ClientSecret::new("3483748c7f3f40c7a27615062e9df410".to_string())),
        AuthUrl::new("https://accounts.spotify.com/authorize".to_string())?,
        None
    ).set_redirect_uri(RedirectUrl::new("http://localhost:8080/".to_string())?);


// Generate the full authorization URL.
let (auth_url, csrf_token) = client
    .authorize_url(CsrfToken::new_random)
    .use_implicit_flow()
    .add_scope(Scope::new("user-library-modify".to_string()))
    .url();

 let window = web_sys::window().expect("no global `window` exists");


 if let Ok(Some(local_storage)) = window.local_storage()
 {
    local_storage.set_item("spotify_auth_state", csrf_token.secret()).unwrap();
 }



// This is the URL you should redirect the user to, in order to trigger the authorization
// process.
log!("Browse to: {}", auth_url);


    window.open_with_url(&auth_url.to_string()).expect("Could not open URL");
    //window.location = auth_url;

// Unwrapping token_result will either produce a Token or a RequestTokenError.

    Ok(())
}

//async fn log_into_spotify()
//{



//    let client_credential = SpotifyClientCredentials::default()
 //        .client_id("this-is-my-client-id")
 //       .client_secret("this-is-my-client-secret")
 //       .build();
 //   let spotify = Spotify::default()
 //   .client_credentials_manager(client_credential)
   // .build();
  //  let history = spotify.current_user_recently_played(10).await;
  //  println!("{:?}", history);
  //  
    
//}

