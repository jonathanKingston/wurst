/*
#![feature(concat_idents)]
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro2;
use proc_macro2::{Ident, Span};
*/

#[macro_use]
extern crate element_macro_derive;

extern crate web_sys;
use web_sys::Element;

/*
pub struct HTMLDocument {

}
impl HTMLDocument {
  fn create_element(&self, name: &str) -> HtmlElement {
      //HtmlElement {title: "empty".to_string()}
      HtmlElement{}
  }
}
*/
/*
pub struct HTMLElement {
    title: String
}
impl HTMLElement {
  fn title(&self) -> String {
      self.title.to_string()
  }
  fn set_title(&mut self, title: &str) {
      self.title = title.to_string();
  }
  fn value(&self) {

  }
}*/

/*
I don't think this will work due to the inability to use $ident in param calling context.
Also concatting set_ to the ident name isn't possible either.

Notes:
proc macro hack. Reflect.
https://github.com/dtolnay/reflect

Use the full namespace path in proc macros.

Workspaces to keep thing upto date in sub crates
cargo outdated
proc macro exposes the generated code, so doesn't need to depend on code it generates.

macro_rules! create_element {
    ($name:tt, {$( $key:ident : $value:expr ),*}) => {
        {
            let document = HTMLDocument{};
            let mut el = document.create_element($name);
            el.set_title("boop");
            $( println!("{:?}", el.$key()); )*
            /*
            $( println!("{:?}", el.concat_idents!($value, $key)(set_, $key)()); )*
            */
            $( println!("el.set_{}({})", $key, $value); )*
$(
            let ident = syn::Ident::new(&format!("set_{}", $key), Span::call_site());
        let t = quote! {
            el.#ident($value);
        };
        //call!(t);
        t

)*

$( println!("{:?}", el.$key()); )*

            el
        }
    };
}
*/
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
