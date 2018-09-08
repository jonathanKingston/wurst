#[macro_use]
extern crate wurst;
//use wurst;
#[macro_use]
extern crate element_macro_derive;
#[macro_use]
extern crate attribute_macro_derive;
// TODO generate a prelude
use wurst::{Attributish, El, HTMLElementAttributes, HTMLDivElementAttributes, HTMLImageElementAttributes, HTMLInputElementAttributes, HTMLLabelElementAttributes};

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

    let mut eli = create_element!("input", {
        // Element interface
        id: "Boop",
        // Input interface
        value: "hyyyyy",
        // HTMLElement interface
        title: "2323",
        lang: "boom"
    });
    let el = eli.create();
    eli.append_dom(el);
}

fn main() {}
