#[macro_use]
extern crate wurst;
// TODO generate a prelude
use wurst::attr::*;
use wurst::{El, OutputConsole};

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate js_sys;
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
    input.map(|mut el| {
        let v = el.has_child_nodes();
        console_log!("Input has children:", v);

        let v = el.check_validity();
        console_log!("Input is valid:", v);

        el.id = Some("boo".into());
        el
    });
    input.update();

    div.append(input);

    div.map(|mut el| {
        let v = el.has_child_nodes();
        console_log!("Div has children:", v);
        el
    });
}

fn main() {}
