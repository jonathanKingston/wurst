#[macro_use]
extern crate wurst;
// TODO generate a prelude
use wurst::{Elementish, El};
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
    input.do_this(|mut el| {
        let v = el.has_child_nodes();
        let me  = wasm_bindgen::JsValue::from_str(&format!("Input has children: {:?}", v));
        web_sys::console::log_1(&me);

        let v = el.check_validity();
        let me  = wasm_bindgen::JsValue::from_str(&format!("Input is valid: {:?}", v));
        web_sys::console::log_1(&me);

        el.id = Some("boo".into());
        el
    });
    input.update();

    div.append(input);

    div.do_this(|mut el| {
        let v = el.has_child_nodes();
        let me  = wasm_bindgen::JsValue::from_str(&format!("Div has children: {:?}", v));
        web_sys::console::log_1(&me);

        el
    });
}

fn main() {}
