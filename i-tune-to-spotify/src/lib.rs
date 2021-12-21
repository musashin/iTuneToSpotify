mod utils;
extern crate xml;


use wasm_bindgen::prelude::*;
use std::fs::File;
use std::io::{self, Write};
use xml::writer::{EventWriter, EmitterConfig, XmlEvent, Result};

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
    let mut file = File::create("output.xml").unwrap();
    alert("Hello, Nicolas!");
}
