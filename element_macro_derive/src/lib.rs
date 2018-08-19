#![feature(use_extern_macros, extern_prelude)]
//#![feature(proc_macro, wasm_custom_section, wasm_import_module, use_extern_macros)]
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;


//extern crate web_sys;

use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};

//extern crate wasm_bindgen;
//use wasm_bindgen::prelude::*;


#[proc_macro_derive(Elementish)]
pub fn element_macro_derive(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let input: syn::DeriveInput = syn::parse(input).unwrap();

    let name = input.ident;
println!("{:?}", input.data);
    let mut attrs = Vec::new();
/*
Struct(DataStruct { struct_token: Struct, fields: Named(FieldsNamed { brace_token: Brace, named: [Field { attrs: [], vis: Inherited, ident: Some(Ident { ident: "title", span: #0 bytes(0..0) }), colon_token: Some(Colon), ty: Path(TypePath { qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "String", span: #0 bytes(0..0) }, arguments: None }] } }) }, Comma] }), semi_token: None })
*/
  if let syn::Data::Struct(v) = input.data {
/*
DataStruct {
  struct_token: Struct,
  fields: Named(FieldsNamed { brace_token: Brace, named: [Field { attrs: [], vis: Inherited, ident: Some(Ident { ident: "title", span: #0 bytes(0..0) }), colon_token: Some(Colon), ty: Path(TypePath { qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "String", span: #0 bytes(0..0) }, arguments: None }] } }) }, Comma] }), semi_token: None }
*/
println!("Need to do something with fields like setting them on create");
    if let syn::Fields::Named(f) = v.fields {
    
        for named in f.named {
            if let Some(i) = named.ident {
              println!("el.set_attribute({:?}, self);", i.to_string());
               let attr_name = i.to_string();
               attrs.push(quote! {
                 el.set_attribute(#attr_name, &self.#i);
               });
            }
        }
    }
  }

    let expanded = quote! {
        #[wasm_bindgen]
        extern {
/*
   This would be better but it causes a JS error.
            #[wasm_bindgen(getter)]
            pub fn window() -> web_sys::Window;
*/
            pub static window: Window;
        }

        impl Elementish for #name {
            fn create(&self) -> web_sys::Element {
                let el = window.document().unwrap().create_element_using_local_name("div").unwrap();
                #(#attrs)*
                el
            }
            // Helper just to append to dom
            fn append_dom(&self, el: Element) {
                let node: Node = window.document().unwrap().body().unwrap().into();
                node.append_child(&el.into());
            }
        }
    };

    expanded.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
