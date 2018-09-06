#![feature(use_extern_macros)]
//#![feature(wasm_custom_section, wasm_import_module, use_extern_macros)]
#[macro_use]
extern crate wurst;
//use wurst;
#[macro_use]
extern crate element_macro_derive;
use wurst::Elementish;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate web_sys;

#[wasm_bindgen]
pub fn make() {
    let mut eli = create_element!("div", {
        // Element interface
        id: "Boop",
        // HTMLElement interface
        title: "2323",
        lang: "boom"
    });
    let el = eli.create();
    eli.append_dom(el);
}

fn main() {}
