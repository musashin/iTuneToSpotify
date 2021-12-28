mod utils;
extern crate xml;
extern crate stdweb;


use wasm_bindgen::prelude::*;
use web_sys::FileReader;



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
    alert("Got the file");
    
    
}