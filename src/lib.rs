extern crate element_macro_derive;
extern crate attribute_macro_derive;

extern crate web_sys;
extern crate wasm_bindgen;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/*
#[macro_export]
macro_rules! create_element {
    ($name:tt, {$( $key:ident : $value:expr ),*}) => {
        {
/*
            #[derive(Attributish)]
            struct MyAttrs {
                $( $key: String, )*
            };

            let attrs = MyAttrs {
                $( $key: $value.into(), )*
            };
*/
            #[derive(AttributishTwo)]
            struct MyAttrs {
                attrs: Option<T>,
                $( $key: String, )*
            };
            let attrs2 = MyAttrs::create();

/*
            let attrs2 = MyAttrs {
                attrs: None,
                $( $key: $value.into(), )*
            };
*/

/*
let attrs2 = HtmlAttributes {
    $( $key: Some($value.into()), )*
    ..Default::default()
};
*/

            let el_container = El {
                name: $name.into(),
                //attrs
                attrs: attrs2
            };
            el_container
        }
    }
}
*/

pub struct El<A> {
    pub name: String,
    pub attrs: A
}

impl <A: Attributish> El<A> {
    pub fn create(&mut self) -> web_sys::Element {
        let el = web_sys::Window::document().unwrap().create_element(&self.name).unwrap();
        let el = self.attrs.flush(el);
        el
    }
    // Helper just to append to dom
    pub fn append_dom(&mut self, el: web_sys::Element) {
        let node: web_sys::Node = web_sys::Window::document().unwrap().body().unwrap().into();
        let el_node: web_sys::Node = el.into();
        node.append_child(&el_node);
    }
}

pub trait Attributish {
    fn flush(&self, el: web_sys::Element) -> web_sys::Element;
}
