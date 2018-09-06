#![feature(use_extern_macros, extern_prelude)]
#![recursion_limit = "128"]
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

extern crate sourcefile;
#[macro_use]
extern crate failure;
use failure::{Fail, ResultExt};
use sourcefile::SourceFile;
use std::ffi::OsStr;

extern crate weedle;
use std::fs;

use std::collections::HashMap;

type Interfaces = HashMap<String, Vec<String>>;
fn me() -> Result<Interfaces, failure::Error> {
    let entries = fs::read_dir("webidls").context("reading webidls directory")?;
    let mut source = SourceFile::default();
    for entry in entries {
        let entry = entry.context("getting webidls/*.webidl entry")?;
        let path = entry.path();
        if path.extension() != Some(OsStr::new("webidl")) {
            continue;
        }
        source = source
            .add_file(&path)
            .with_context(|_| format!("reading contents of file \"{}\"", path.display()))?;
    }

    let mut interfaces = HashMap::new();
    weedle::parse(&source.contents).map(|r| {
        for i in r {
            match i {
                weedle::Definition::Interface(n) => {
                    let mut setters = vec![];
                    for attr in n.members.body {
                        match attr {
                            weedle::interface::InterfaceMember::Attribute(a) => {
                                setters.push(a.identifier.0.into());
                            }
                            _ => (),
                        }
                    }
                    interfaces.insert(n.identifier.0.into(), setters);
                }
                _ => (),
            }
        }
    });

    Ok(interfaces)
}

use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};

fn has_method_in_interface(interfaces: &Interfaces, interface_name: &str, method_name: &str) -> bool {
    if let Some(methods) = interfaces.get(interface_name) {
        for method in methods {
            if method == method_name {
                return true;
            }
        }
    }
    return false;
}

#[proc_macro_derive(Elementish)]
pub fn element_macro_derive(input: TokenStream) -> TokenStream {
    let interfaces = me().unwrap();
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

                    if has_method_in_interface(&interfaces, "HTMLElement", &attr_name) {
                        attrs.push(quote!{
                            {
                                let dyn_el: Option<&web_sys::HtmlElement> = wasm_bindgen::JsCast::dyn_ref(&el);
                                dyn_el.map(|html_el| {
                                    html_el.#setter(&self.#i);
                                });
                            }
                        });
                        continue;
                    }
                    if has_method_in_interface(&interfaces, "Element", &attr_name) {
                        attrs.push(quote!{
                            el.#setter(&self.#i);
                        });
                        continue;
                    }
                }
            }
        }
    }

    let expanded = quote! {
        impl Elementish for #name {
            fn create(&mut self) -> web_sys::Element {
                let el = web_sys::Window::document().unwrap().create_element("div").unwrap();
                #(#attrs)*
                el
            }
            // Helper just to append to dom
            fn append_dom(&mut self, el: web_sys::Element) {
                let node: web_sys::Node = web_sys::Window::document().unwrap().body().unwrap().into();
                let el_node: web_sys::Node = el.into();
                node.append_child(&el_node);
            }
        }
    };

    expanded.into()
}
