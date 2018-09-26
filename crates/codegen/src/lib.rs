#![recursion_limit = "128"]
extern crate proc_macro2;
#[macro_use]
extern crate quote;
use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
extern crate parser;
use parser::Interfaces;

#[derive(Debug)]
pub struct Codegen {
    interfaces: Interfaces,
}

impl Codegen {
    pub fn gen() -> Codegen {
        let interfaces = Interfaces::parse().unwrap();
        Codegen { interfaces }
    }

    fn method_calls(&self, interface_name: &str, el: Ident) -> Vec<proc_macro2::TokenStream> {
        let mut interface_calls = vec![];
        if let Some(methods) = self.interfaces.get_properties(interface_name) {
            for field in methods {
                let field_ident = Ident::new(field, Span::call_site());
                let attr_name = field.to_string();
                let set_name = "set_".to_string() + &attr_name;
                let setter = Ident::new(&set_name, Span::call_site());
                let setter_value = Ident::new(
                    &format!("_{}_i_dont_care_about", set_name),
                    Span::call_site(),
                );
                interface_calls.push(quote!{
                    if let Some(ref field) = self.#field_ident {
                       let #setter_value = #el.#setter(&field.clone());
                    }
                });
            }
        }
        println!("iface {:?}", interface_calls);
        interface_calls
    }

    pub fn get_console_macro(&self, name: &str) -> proc_macro2::TokenStream {
        let mut arms = vec![];
        let macro_name = Ident::new(&format!("console_{}", name), Span::call_site());
        let mut fn_name;
        for i in 1..=7 {
            fn_name = Ident::new(&format!("{}_{}", name, i), Span::call_site());
            let mut args = vec![];
            for j in 1..=i {
                args.push(Ident::new(&format!("arg{}", j), Span::call_site()));
            }
            let args_u = args.clone();
            let sig = quote!{#($#args:expr),*};
            let val = quote!{
                (#sig) => {
                    {
                        web_sys::console::#fn_name(#(&$#args_u.get_js_value()),*);
                    }
                };
            };
            arms.push(val);
        }
        let intro = format!("Calls console.{} in the browser", name);
        let example = format!("console_{}!(\"hey\", 1);", name);
        fn_name = Ident::new(&format!("{}", name), Span::call_site());
        quote!{
            #[doc=#intro]
            /// ```
            #[doc=#example]
            /// ```
            #[macro_export]
            macro_rules! #macro_name {
                #(#arms)*
                ($($args:expr),*) => {
                    {
                        let arr = js_sys::Array::new();
                        $( arr.push(&$args.get_js_value()); )*
                        web_sys::console::#fn_name(&arr);
                    }
                }
            }
        }
    }

    pub fn get_create_element_macro(&self) -> proc_macro2::TokenStream {
        let mut arms = vec![];
        let mut macro_interfaces: Vec<(String, String)> =
            self.interfaces.tag_interfaces().into_iter().collect();
        // Fallback for non named tags
        macro_interfaces.push(("$name".into(), "HTMLElementish".into()));
        for (tag, _interface) in macro_interfaces {
            let mut tag_string = quote!{#tag};
            let mut match_name = tag_string.clone();
            let mut tag_name = Some(tag.clone());
            if let "$name" = tag.as_str() {
                tag_string = quote!{$name};
                match_name = quote!{$name:tt};
                tag_name = None;
            };
            let interface_name = Ident::new(
                &Codegen::get_wurst_interface_name(tag_name),
                Span::call_site(),
            );
            arms.push(quote!{
               (#match_name, {$( $key:ident : $value:expr ),*}) => {
                   {
                       let el = #interface_name {
                           _node: None,
                           $( $key: Some($value.into()), )*
                           ..Default::default()
                       };
                       let el_container = crate::El {
                           dom_node: None,
                           name: #tag_string.into(),
                           el: Some(el),
                           body: vec![],
                       };
                       el_container
                   }
               };
            });
        }

        (quote!{
            #[macro_export]
            macro_rules! create_element {
                #(#arms)*
            }
        })
    }

    fn enum_variant_from_tag_name(s: String) -> String {
        let mut c = s.chars();
        match c.next() {
            None => panic!("enum variant can't be blank"),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }

    pub fn get_enum_code(&self) -> proc_macro2::TokenStream {
        let mut interfaces = vec![];
        let mut froms = vec![];
        let enum_interfaces = self.interfaces.tag_interfaces().clone();
        // TODO why is generic here breaking the code?! conflicting implementation error
        // enum_interfaces.insert("GenericEl", "HTMLElementish");
        for (tag, _interface) in enum_interfaces {
            let ident = Ident::new(
                &Codegen::get_wurst_interface_name(Some(tag.clone())),
                Span::call_site(),
            );
            let tag_ident =
                Ident::new(&Codegen::enum_variant_from_tag_name(tag), Span::call_site());
            interfaces.push(quote!{
                #tag_ident(Box<El<#ident>>),
            });
            froms.push(quote!{
                impl From<El<#ident>> for InterfaceType {
                    fn from(t: El<#ident>) -> Self {
                        InterfaceType::#tag_ident(Box::new(t))
                    }
                }
            });
        }
        quote!{
            pub enum InterfaceType {
                #(#interfaces)*
            }
            #(#froms)*
        }
    }

    pub fn get_interfaces_code(&self) -> Vec<proc_macro2::TokenStream> {
        let mut interfaces = vec![];
        for (tag, interface) in &self.interfaces.tag_interfaces() {
            interfaces.push(self.get_interface_code(interface, Some(tag)));
        }
        interfaces
    }

    pub fn get_element_interface_name(interface_name: &str) -> String {
        String::from(interface_name).replace("HTML", "Html")
    }

    pub fn get_wurst_interface_name(tag_name: Option<String>) -> String {
        match tag_name {
            None => String::from("GenericElement"),
            Some(tag_name) => {
                let tag_prefix = Codegen::enum_variant_from_tag_name(tag_name);
                format!("{}Element", tag_prefix)
            }
        }
    }

    pub fn fields(&self, interface_name: &str, fields: &mut Vec<proc_macro2::TokenStream>) {
        if let Some(methods) = self.interfaces.get_properties(interface_name) {
            for field in methods {
                let field_ident = Ident::new(field, Span::call_site());
                fields.push(quote!{pub #field_ident: Option<String>,});
            }
        }
    }

    // Given an `interface_name` like `HTMLDivElement` append all the attribute names to the structs fields
    // Also construct the dyn_ref interface which will call `set_<attr_name>` when flush is called.
    pub fn add_interface(
        &self,
        interface_name: &str,
        fields: &mut Vec<proc_macro2::TokenStream>,
        interfaces: &mut Vec<proc_macro2::TokenStream>,
    ) {
        let code_interface_name = Ident::new(
            &Codegen::get_element_interface_name(interface_name),
            Span::call_site(),
        );
        let flush_name = String::from(interface_name).to_lowercase() + "_flush";
        let flush_interface_calls = Ident::new(&flush_name, Span::call_site());
        let interface_calls =
            self.method_calls(interface_name, Ident::new("iface_el", Span::call_site()));
        if !interface_calls.is_empty() {
            interfaces.push(quote!{
                #[allow(clippy::let_unit_value)]
                let #flush_interface_calls = |el: &web_sys::Node| {
                    let dyn_el: Option<&web_sys::#code_interface_name> = wasm_bindgen::JsCast::dyn_ref(&*el);
                    if let Some(iface_el) = dyn_el {
                        #(#interface_calls)*
                    }
                };
                #flush_interface_calls(&el);
            });
        }
        self.fields(interface_name, fields);
    }

    pub fn get_other_methods(
        &self,
        interface_name: &str,
        tag_name: Option<&str>,
    ) -> proc_macro2::TokenStream {
        // TODO save me from this hardcoded lifestyle
        let mut body = quote!{};
        let code_interface_name = Ident::new(
            &Codegen::get_element_interface_name(interface_name),
            Span::call_site(),
        );

        if interface_name == "HTMLInputElement" {
            body = quote!{
                pub fn check_validity(&mut self) -> bool {
                    let el = self._node.take().unwrap();
                    let r = {
                        let dyn_el: Option<&web_sys::#code_interface_name> = wasm_bindgen::JsCast::dyn_ref(&el);
                        dyn_el.map(|iface_el| {
                            iface_el.check_validity()
                        }).unwrap()
                    };
                    self._node = Some(el);
                    r
                }
            };
        }

        let interface_name = Codegen::get_wurst_interface_name(tag_name.map(|v| String::from(v)));
        let interface_ident = Ident::new(&interface_name, Span::call_site());

        quote!{
            impl #interface_ident {
                pub fn has_child_nodes(&mut self) -> bool {
                    let el = self._node.take().unwrap();
                    let r = el.has_child_nodes();
                    self._node = Some(el);
                    r
                }
                #body
            }
        }
    }

    pub fn get_interface_code(
        &self,
        interface_name: &str,
        tag_name: Option<&str>,
    ) -> proc_macro2::TokenStream {
        let mut fields = vec![];
        let mut interfaces = vec![];

        // Construct an artificial stack of interfaces that we call based on the attributes we have
        self.add_interface(interface_name, &mut fields, &mut interfaces);

        if interface_name != "HTMLElement" {
            self.add_interface("HTMLElement", &mut fields, &mut interfaces);
        }
        if interface_name != "Element" {
            self.add_interface("Element", &mut fields, &mut interfaces);
        }
        let other_methods = self.get_other_methods(&interface_name, tag_name);

        let interface_name = Codegen::get_wurst_interface_name(tag_name.map(|v| String::from(v)));
        let interface_ident = Ident::new(&interface_name, Span::call_site());

        quote!{
            #[derive(Default)]
            pub struct #interface_ident {
                pub _node: Option<web_sys::Node>,
                #(#fields)*
            }
            #other_methods
            impl Elementish for #interface_ident {
                fn take_node(&mut self) -> Option<web_sys::Node> {
                    self._node.take()
                }
                fn set_node(&mut self, node: web_sys::Node) {
                    self._node = Some(node)
                }
                fn flush(&self, el: web_sys::Node) -> web_sys::Node {
                    // TODO this inheritance tree should be defined from the parsed webidl
                    #(#interfaces)*
                    el
                }
            }
        }
    }

    pub fn get_code(&self) -> String {
        let mut tokens = TokenStream::new();
        let macro_code = self.get_create_element_macro();
        let console_macro_code = vec![
            self.get_console_macro("log"),
            self.get_console_macro("debug"),
            self.get_console_macro("error"),
            self.get_console_macro("warn"),
        ];
        let fallback = self.get_interface_code("HTMLElement", None);
        let interfaces = self.get_interfaces_code();
        let enum_code = self.get_enum_code();
        (quote!{

            #macro_code
            #(#console_macro_code)*

            pub mod elements {
                pub use crate::{El, Elementish};
                #enum_code

                #(#interfaces)*

                // Fall back interface for all other tags
                #fallback
            }
        }).to_tokens(&mut tokens);

        tokens.to_string()
    }
}
