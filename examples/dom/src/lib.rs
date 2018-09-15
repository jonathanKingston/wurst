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
    let mut val = create_element!("p", {});
    val.create();
    // Hopefully we can solve the interface of below:
    val.el.take().map(|e| {
        let dyn_el: Result<web_sys::Element, _> = wasm_bindgen::JsCast::dyn_into(e);
        dyn_el.map(|html_el| {
            html_el.set_inner_html("Hello from WuRst!");
            val.el = Some(html_el.into());
        });
    });
    val.add_to_body();
}
