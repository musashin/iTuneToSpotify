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
    Artist
}

#[derive(Debug, Eq)]
struct Artist {
    name: String
,
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

pub fn parse_library(library_file_text:&str)
{
    let parser = xml::reader::EventReader::from_str(library_file_text);
    let mut parse_state = ParseEvent::Parsing;
    let mut field = Field::NoField;
    let mut library = HashSet::<Artist>::new();
    

    for event in parser {
        match event {
        Ok(xml::reader::XmlEvent::StartElement{name,attributes:_,namespace:_}) => {
            if name.local_name.trim() == "key" {parse_state = ParseEvent::KeyFound; }
            if parse_state == ParseEvent::KeyTextFound{

                if name.local_name.trim() == "date" && field == Field::Date {parse_state = ParseEvent::KeyChildFound;}
                if name.local_name.trim() == "string" && field == Field::Artist {parse_state = ParseEvent::KeyChildFound;}
            }
            
           
        }
        Ok(xml::reader::XmlEvent::Characters(text)) => {
            if parse_state == ParseEvent::KeyFound {
                parse_state = ParseEvent::KeyTextFound;

                if text.trim() == "Date"{field = Field::Date;}
                if text.trim() == "Artist"{field = Field::Artist;}
                

            }
            else if parse_state==ParseEvent::KeyChildFound {

                match field {
                    Field::Date=> log!("Date {}", text) ,
                    Field::Artist=> {library.insert(Artist{name: text});}
                    _=>()

                }

                parse_state = ParseEvent::Parsing;
                field = Field::NoField;

            }

            
        }
        Ok(_) => (),
        Err(why)=>log!("unrecognized even {:?}",&why)
        }
        
    }

    log!("Lib {:#?}", library) 

   
}

