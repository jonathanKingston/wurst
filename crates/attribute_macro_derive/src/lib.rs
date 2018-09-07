#![feature(use_extern_macros, extern_prelude)]
#![recursion_limit = "128"]
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate failure;

use std::collections::HashMap;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};

extern crate parser;
use parser::Interfaces;

#[proc_macro_derive(AttributishTwo, attributes(TagName))]
pub fn element_macro_derive(input: TokenStream) -> TokenStream {
    let interfaces = Interfaces::parse().unwrap();
    println!("Parsed interfaces: {:#?}", interfaces);

    // Parse the string representation
    let input: syn::DeriveInput = syn::parse(input).unwrap();

    let name = input.ident;
    let mut attrs = Vec::new();
    if let syn::Data::Struct(v) = input.data {
        println!("Need to do something with fields like setting them on create");

        if let syn::Fields::Named(f) = v.fields {
            let attrs_named = f.named.iter().find(|e| {
                if let Some(ref i) = e.ident {
                    return i == "attrs";
                }
                return false;
            });

            for named in &f.named {
                if let Some(ref i) = named.ident {
                    // Potentially have a fallback to: println!("el.set_attribute({:?}, self);", i.to_string());
                    let attr_name = i.to_string();
                    let set_name = "set_".to_string() + &attr_name;
                    let setter = Ident::new(&set_name, Span::call_site());
                    attrs.push(quote!{
                        attrs.#setter(&self.#i);
                    });
                }
            }
        }
    }
    let interface = Ident::new("HtmlAttributes", Span::call_site());

    let expanded = quote! {
        impl <T: Attributish> Attributish for #name <T> {
            fn flush(&self, el: web_sys::Element) -> web_sys::Element {
                let attrs = self.attrs.unwrap();
                #(#attrs)*
                el
            }
        }
        impl <T: Attributish> #name <T> {
            fn create() -> #name {
                #name {
                    attrs: #interface::Default(),
                }
            }
        }
    };

    expanded.into()
}
