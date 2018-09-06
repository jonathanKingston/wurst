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
//use web_sys::{HtmlElement, Element, Window, Node};
/*
        #[wasm_bindgen]
        extern {
            #[wasm_bindgen(getter)]
            pub fn window() -> web_sys::Window;
        }
*/

/*
impl wasm_bindgen::cast::JsCast for web_sys::Element {
    // everything is a `JsValue`!
    fn instanceof(_val: &web_sys::Element) -> bool { true }
    fn unchecked_from_js(val: web_sys::Element) -> Self { val }
    fn unchecked_from_js_ref(val: &web_sys::Element) -> &Self { val }
    fn unchecked_from_js_mut(val: &mut web_sys::Element) -> &mut Self { val }
}
*/

/*
#[derive(Elementish)]
struct MyEl {
  title: String,
  test: String,
}
*/

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
