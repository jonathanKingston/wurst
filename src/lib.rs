#![feature(extern_prelude)]
extern crate web_sys;
extern crate wasm_bindgen;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct El<A> {
    pub el: Option<web_sys::Element>,
    pub name: String,
    pub attrs: A,
  // TODO figure out this:
  //  pub body: Vec<Box<El<A>>>,
}

impl <A: Attributish> El<A> {
    pub fn create(&mut self) {
        let el = web_sys::Window::document().unwrap().create_element(&self.name).unwrap();
        let el = self.attrs.flush(el);
        self.el = Some(el);
    }
    // Helper just to append to dom
    pub fn add_to_body(&mut self) {
        let maybe_el = self.el.take();
        if let Some(el) = maybe_el {
            let node: web_sys::Node = web_sys::Window::document().unwrap().body().unwrap().into();
            let el_node: web_sys::Node = el.into();
            if let Ok(el) = node.append_child(&el_node) {
                let dyn_el: Result<web_sys::Element, _> = wasm_bindgen::JsCast::dyn_into(el);
                dyn_el.map(|html_el| {
                     self.el = Some(html_el);
                });
            }
        }
    }
}

pub trait Attributish {
    fn flush(&self, el: web_sys::Element) -> web_sys::Element;
}
