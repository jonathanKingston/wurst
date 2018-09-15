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
    let mut div = create_element!("div", {
        // Element interface
        id: "Boop",

        // HTMLElement interface
        title: "hey I am a title",
        lang: "en-GB"
    });
    div.create();
    div.add_to_body();

    let mut input = create_element!("input", {
        // Element interface
        id: "Boop",

        // Input interface
        value: "hey!",

        // HTMLElement interface
        title: "2323",
        lang: "boom"
    });
    input.create();

    input.attrs.id = Some("boo".into());
    input.update();

    div.append(input);
}

fn main() {}
