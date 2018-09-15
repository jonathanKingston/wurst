#![feature(extern_prelude)]
extern crate wasm_bindgen;
extern crate web_sys;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct El<A> {
    pub el: Option<web_sys::Element>,
    pub name: String,
    pub attrs: A,
    pub body: Vec<Box<attr::InterfaceType>>,
}

impl<A: Attributish> El<A> {
    pub fn create(&mut self) {
        let el = web_sys::Window::document()
            .unwrap()
            .create_element(&self.name)
            .unwrap();
        let el = self.attrs.flush(el);
        self.el = Some(el);
    }
    pub fn update(&mut self) {
        self.el = self.el.take().map(|e| self.attrs.flush(e));
    }

    // TODO simplify this method by storing elements as nodes.
    pub fn append<T>(&mut self, mut child: El<T>)
    where
        attr::InterfaceType: From<El<T>>,
    {
        let maybe_el = self.el.take();
        if let Some(el) = maybe_el {
            let el_node: web_sys::Node = el.into();
            // TODO fix unwrap
            let child_node: web_sys::Node = child.el.take().unwrap().into();
            if let Ok(child_el) = el_node.append_child(&child_node) {
                let dyn_el: Result<web_sys::Element, _> = wasm_bindgen::JsCast::dyn_into(el_node);
                dyn_el.map(|html_el| {
                    self.el = Some(html_el);
                });
                let dyn_child_el: Result<web_sys::Element, _> =
                    wasm_bindgen::JsCast::dyn_into(child_el);
                dyn_child_el.map(|child_el| {
                    child.el = Some(child_el);
                    let child_interface: attr::InterfaceType = child.into();
                    self.body.push(Box::new(child_interface));
                });
            }
        }
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
