mod utils;
extern crate xml;
extern crate stdweb;
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

pub fn parse_library(library_file_text:&str)
{
    let parser = xml::reader::EventReader::from_str(library_file_text);
    for event in parser {
        log!("{:?}", event.unwrap());
    }
}

