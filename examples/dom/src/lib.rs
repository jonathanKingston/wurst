extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate wurst;
use wurst::{El};
use wurst::attr::*;

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() {
    let mut val = create_element!("p", {});
    val.create();
    val.el = match val.el {
        Some(e) => {
          e.set_inner_html("Hello from WuRst!");
          Some(e)
        },
        None => {
          None
        }
    };
    val.add_to_body();
}
