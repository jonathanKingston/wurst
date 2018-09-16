extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
extern crate web_sys;

#[macro_use]
extern crate wurst;
use wurst::El;
use wurst::attr::*;

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() {
    let mut val = create_element!("p", {
      inner_text: "Hello from WuRst!"
    });
    val.create();
    /* to update attrs:
    val.el.inner_text = Some("Hello from WuRst!".into());
    val.update();
    */
    val.add_to_body();
}
