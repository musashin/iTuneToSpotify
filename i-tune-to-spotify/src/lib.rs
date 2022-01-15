mod utils;
extern crate xml;
extern crate stdweb;
use std::collections::HashSet;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};


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

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, Nicolas!");
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
    log_into_spotify();

    return;

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


use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use url::Url;

async fn log_into_spotify()->Result<(), Box<dyn std::error::Error>> {

// Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
// token URL.
let client =
    BasicClient::new(
        ClientId::new("client_id".to_string()),
        Some(ClientSecret::new("client_secret".to_string())),
        AuthUrl::new("http://authorize".to_string())?,
        Some(TokenUrl::new("http://token".to_string())?)
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new("http://redirect".to_string())?);

// Generate a PKCE challenge.
let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

// Generate the full authorization URL.
let (auth_url, csrf_token) = client
    .authorize_url(CsrfToken::new_random)
    // Set the desired scopes.
    .add_scope(Scope::new("read".to_string()))
    .add_scope(Scope::new("write".to_string()))
    // Set the PKCE code challenge.
    .set_pkce_challenge(pkce_challenge)
    .url();

// This is the URL you should redirect the user to, in order to trigger the authorization
// process.
println!("Browse to: {}", auth_url);

// Once the user has been redirected to the redirect URL, you'll have access to the
// authorization code. For security reasons, your code should verify that the `state`
// parameter returned by the server matches `csrf_state`.

// Now you can trade it for an access token.
let token_result = client
    .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
    // Set the PKCE code verifier.
    .set_pkce_verifier(pkce_verifier)
    .request_async(async_http_client)
    .await?;

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

