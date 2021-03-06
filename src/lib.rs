#![feature(tool_lints)]
extern crate wasm_bindgen;
extern crate web_sys;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct El<A: Elementish> {
    pub dom_node: Option<web_sys::Node>,
    pub name: String,
    pub el: Option<A>,
    pub body: Vec<Box<elements::InterfaceType>>,
}

// TODO rename
struct DOMHelper {
}
impl DOMHelper {
    fn document() -> Option<web_sys::Document> {
        if let Some(window) = web_sys::window() {
            return window.document();
        }
        None
    }

    fn body() -> Option<web_sys::HtmlElement> {
        let document = Self::document()?;
        return document.body();
    }
}

impl<A: Elementish> El<A> {
    pub fn create(&mut self) {
        if let Some(document) = DOMHelper::document() {
            if let Ok(el) = document.create_element(&self.name) {
                let dom_node: web_sys::Node = el.into();
                if let Some(interface) = self.el.take() {
                    let dom_node = interface.flush(dom_node);
                    self.dom_node = Some(dom_node);
                    self.el = Some(interface);
                }
            }
        }
    }

    pub fn update(&mut self) {
        if let Some(e) = self.dom_node.take() {
            if let Some(interface) = self.el.take() {
                let node = interface.flush(e);
                self.el = Some(interface);
                self.dom_node = Some(node);
            }
        }
    }

    pub fn append<T: Elementish>(&mut self, mut child: El<T>)
    where
        elements::InterfaceType: From<El<T>>,
    {
        let maybe_el = self.dom_node.take();
        if let Some(el) = maybe_el {
            if let Some(child_node) = child.dom_node.take() {
                if let Ok(child_el) = el.append_child(&child_node) {
                    self.dom_node = Some(el);
                    child.dom_node = Some(child_el);
                    let child_interface: elements::InterfaceType = child.into();
                    self.body.push(Box::new(child_interface));
                }
            }
        }
    }

    /// Public interface that exposes concrete `Elementish` impl
    pub fn map<T>(&mut self, callback: T)
    where
        T: Fn(A) -> A,
    {
        // TODO do something more graceful here for no el like creating one
        if let Some(mut interface) = self.el.take() {
            if let Some(dom_node) = self.dom_node.take() {
                interface.set_node(dom_node);
                interface = callback(interface);
                self.dom_node = interface.take_node();
                self.el = Some(interface);
            }
        }
    }

    // Helper just to append to dom this won't be sticking around
    // TODO remove in favour of something like `Wurst::body() -> El`
    pub fn add_to_body(&mut self) {
        let maybe_el = self.dom_node.take();
        if let Some(el) = maybe_el {
            if let Some(body) = DOMHelper::body() {
                let node: web_sys::Node = body.into();
                if let Ok(el) = node.append_child(&el) {
                    self.dom_node = Some(el);
                }
            }
        }
    }
}

pub trait Elementish {
    fn take_node(&mut self) -> Option<web_sys::Node>;
    fn set_node(&mut self, node: web_sys::Node);
    fn flush(&self, el: web_sys::Node) -> web_sys::Node;
}

pub trait OutputConsole {
    fn get_js_value(&self) -> wasm_bindgen::JsValue;
}

impl OutputConsole for bool {
    fn get_js_value(&self) -> wasm_bindgen::JsValue {
        wasm_bindgen::JsValue::from_bool(*self)
    }
}

impl<'t> OutputConsole for &'t str {
    fn get_js_value(&self) -> wasm_bindgen::JsValue {
        wasm_bindgen::JsValue::from_str(&self)
    }
}

impl OutputConsole for i32 {
    fn get_js_value(&self) -> wasm_bindgen::JsValue {
        wasm_bindgen::JsValue::from_str(&format!("{}", &self))
    }
}

impl OutputConsole for isize {
    fn get_js_value(&self) -> wasm_bindgen::JsValue {
        wasm_bindgen::JsValue::from_str(&format!("{}", &self))
    }
}
