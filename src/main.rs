#![feature(use_extern_macros)]
//#![feature(wasm_custom_section, wasm_import_module, use_extern_macros)]
extern crate wurst;
//use wurst;
#[macro_use]
extern crate element_macro_derive;
use wurst::{Elementish};

    extern crate wasm_bindgen;
    use wasm_bindgen::prelude::*;

extern crate web_sys;
use web_sys::{Element, Window, Node};
/*
        #[wasm_bindgen]
        extern {
            #[wasm_bindgen(getter)]
            pub fn window() -> web_sys::Window;
        }
*/


#[derive(Elementish)]
struct MyEl {
  title: String,
  test: String,
}

#[wasm_bindgen]
pub fn make()  {//-> Element {
    let boop = MyEl{
      title: "my title".into(),
      test: "my test".into()
    };
    let el = boop.create();
    boop.append_dom(el);

    /*
    let _el = create_element!("div", {
        title: "2323",
        value: "boom"
    });*/
   //el


}

fn main() {
}
