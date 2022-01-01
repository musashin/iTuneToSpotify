mod utils;
extern crate xml;
extern crate stdweb;
extern crate minidom;
use xmltree::Element;

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
    let mut found_key = false;
    let mut found_artist = false;

    for event in parser {
        match event {
        Ok(xml::reader::XmlEvent::StartElement{name,attributes:_,namespace:_}) => {
            if name.local_name.trim() == "key"
            {
                found_key = true;
            }
            else {
                found_key = false;
            }

            if name.local_name.trim() == "string" && found_artist
            {
                log!("Artist {:?}",name.local_name.trim());
                found_artist = false;
            }

        }
        Ok(xml::reader::XmlEvent::Characters(text)) => {
            if(found_key && text.trim() == "Date"){log!("{:?}",text);}
            if(found_key && text.trim() == "Artist"){found_artist = true;}
        }
        Ok(_) => (),
        Err(why)=>log!("unrecognized even {:?}",&why)
        }
        
    }

     //let parser = xml::reader::EventReader::from_str(library_file_text);
    //for event in parser {
    //    log!("{:?}", event.unwrap());
    //}

    //let root: minidom::Element = 
    //log!("Ready to Parse");
    //let  itu/ne_lib  = Element::parse(library_file_text.as_bytes());
    //log!("Parsed!");
    //match itune_lib{
    //    Err(why) => log!("why1{:?}", why),
    //    Ok(root) =>   /    {
            //log!("OK");
            //let root_dict = root.get_child("dict").expect("Can't find dict element");
            //let key =root_dict.get_child("key").expect("Can't find key element");
            //log!("{:?}", key.get_text())
            //let mut key = root_dict.get_child("key");
            //while let Some(current_key) = key {
            //    let current_key_text = current_key.get_text().expect("Can't find text in key");
            //    log!("unknown text{:?}", current_key_text);
            //    if current_key_text == "Date"{
            //        log!("date{:?}", current_key_text)
            //    }
            //    key = key.get_child("key");
            //}

        //}

    //}
    

    //let v2: &[u8]   = library_file_text.as_bytes();



    //match library_file_text.parse::<xmltree::Element>() {
    //    Err(why) => log!("why2{:?}", why),
    //    Ok(ratio) => log!("OK2")
    //}

    //let mut reader = minidom::quick_xml::Reader::from_str(library_file_text);

    //  match minidom::Element:: from_reader(&mut reader){
    //    Err(why) => log!("why1{:?}", why),
    //    Ok(ratio) => log!("OK1")
    //}



    ////match library_file_text.parse::<minidom::Element>() {
    //    Err(why) => log!("why2{:?}", why),
    //    Ok(ratio) => log!("OK2")
    //}



    //for child in root.children() {
    //    //log!("{:?}", "toto"); //child.text());

    //    if child.is("dict",minidom::NSChoice::Any) {

    //        if child.has_child("key", minidom::NSChoice::Any) {

    //        }
            //log!("{:?}", child.parse().unwrap());
    //    }
    //}
    

    

    //let parser = xml::reader::EventReader::from_str(library_file_text);
    //for event in parser {
    //    log!("{:?}", event.unwrap());
    //}
}

