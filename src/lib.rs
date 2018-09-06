#[macro_use]
extern crate element_macro_derive;

extern crate web_sys;
use web_sys::Element;

#[macro_export]
macro_rules! create_element {
    ($name:tt, {$( $key:ident : $value:expr ),*}) => {
        {
            #[derive(Elementish)]
            struct MyEl {
                $( $key: String, )*
            };
            let el_container = MyEl {
                $( $key: $value.into(), )*
            };
            el_container
        }
    }
}

macro_rules! hashmap {
    ($( $key:tt : $value:expr ),*) => {
      {
        let mut hash = HashMap::new();
        $( hash.insert($key.to_string(), $value.to_string()); )*
        hash
      }
    };
}

pub trait Elementish {
    fn create(&mut self) -> web_sys::Element;
    fn append_dom(&mut self, el: web_sys::Element);
}
