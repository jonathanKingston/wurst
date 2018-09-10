#[macro_use]
extern crate wurst;
// TODO generate a prelude
use wurst::{Attributish, El};
use wurst::attr::*;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate web_sys;

#[wasm_bindgen]
pub fn make() {
    let mut eli = create_element!("div", {
        // Element interface
        id: "Boop",
        // HTMLElement interface
        title: "hey I am a title",
        lang: "boom",
    });
    eli.create();
    eli.add_to_body();

    let mut eli = create_element!("input", {
        // Element interface
        id: "Boop",
        // Input interface
        value: "hyyyyy",
        // HTMLElement interface
        title: "2323",
        lang: "boom"
    });
    eli.create();
    eli.add_to_body();
}

fn main() {}
