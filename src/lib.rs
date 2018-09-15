#![feature(extern_prelude)]
extern crate wasm_bindgen;
extern crate web_sys;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct El<A> {
    pub el: Option<web_sys::Node>,
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
        let el: web_sys::Node = el.into();
        let el = self.attrs.flush(el);
        self.el = Some(el);
    }
    pub fn update(&mut self) {
        self.el = self.el.take().map(|e| self.attrs.flush(e));
    }

    pub fn append<T>(&mut self, mut child: El<T>)
    where
        attr::InterfaceType: From<El<T>>,
    {
        let maybe_el = self.el.take();
        if let Some(el) = maybe_el {
            // TODO fix unwrap
            let child_node = child.el.take().unwrap();
            if let Ok(child_el) = el.append_child(&child_node) {
                self.el = Some(el);
                child.el = Some(child_el);
                let child_interface: attr::InterfaceType = child.into();
                self.body.push(Box::new(child_interface));
            }
        }
    }

    // Helper just to append to dom
    pub fn add_to_body(&mut self) {
        let maybe_el = self.el.take();
        if let Some(el) = maybe_el {
            let node: web_sys::Node = web_sys::Window::document().unwrap().body().unwrap().into();
            if let Ok(el) = node.append_child(&el) {
                self.el = Some(el);
            }
        }
    }
}

pub trait Attributish {
    fn flush(&self, el: web_sys::Node) -> web_sys::Node;
}
