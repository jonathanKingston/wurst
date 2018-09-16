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
        if let Some(methods) = self.interfaces.get_methods(interface_name) {
            for field in methods {
                let field_ident = Ident::new(field, Span::call_site());
                let attr_name = field.to_string();
                let set_name = "set_".to_string() + &attr_name;
                let setter = Ident::new(&set_name, Span::call_site());
                interface_calls.push(quote!{
                    if let Some(ref field) = self.#field_ident {
                       #el.#setter(&field.clone());
                    }
                });
            }
        }
        println!("iface {:?}", interface_calls);
        interface_calls
    }

    pub fn get_macro(&self) -> proc_macro2::TokenStream {
        let mut arms = vec![];
        let mut macro_interfaces: Vec<(&str, &str)> = self.interfaces.tag_interfaces().into_iter().collect();
        // Fallback for non named tags
        macro_interfaces.push(("$name".into(), "HTMLElementish".into()));
        for (tag, interface) in macro_interfaces {
            let mut tag_string = quote!{#tag};
            let mut match_name = tag_string.clone();
            if let "$name" = tag {
                tag_string = quote!{$name};
                match_name = quote!{$name:tt};
            };
            let interface_name = Ident::new(
                &Codegen::get_wurst_interface_name(interface),
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
                       let el_container = El {
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

    fn enum_variant_from_tag_name(s: &str) -> String {
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
        for (tag, interface) in enum_interfaces {
            let ident = Ident::new(
                &Codegen::get_wurst_interface_name(interface),
                Span::call_site(),
            );
            let tag_ident = Ident::new(
                &Codegen::enum_variant_from_tag_name(tag),
                Span::call_site()
            );
            interfaces.push(quote!{
                #tag_ident(El<#ident>),
            });
            froms.push(quote!{
                impl From<El<#ident>> for InterfaceType {
                    fn from(t: El<#ident>) -> Self {
                        InterfaceType::#tag_ident(t)
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
            interfaces.push(self.get_interface_code(interface));
        }
        interfaces
    }

    pub fn get_element_interface_name(interface_name: &str) -> String {
        String::from(interface_name).replace("HTML", "Html")
    }

    pub fn get_wurst_interface_name(interface_name: &str) -> String {
        format!("{}ish", interface_name)
    }

    pub fn fields(&self, interface_name: &str, fields: &mut Vec<proc_macro2::TokenStream>) {
        if let Some(methods) = self.interfaces.get_methods(interface_name) {
            for field in methods {
                let field_ident = Ident::new(field, Span::call_site());
                fields.push(quote!{pub #field_ident: Option<String>,});
            }
        }
    }

    // Given an `interface_name` like `HTMLDivElement` append all the attribute names to the structs fields
    // Also construct the dyn_ref interface which will call `set_<attr_name>` when flush is called.
    pub fn add_interface(&self, interface_name: &str, fields: &mut Vec<proc_macro2::TokenStream>, interfaces: &mut Vec<proc_macro2::TokenStream>) {
        let code_interface_name = Ident::new(&Codegen::get_element_interface_name(interface_name), Span::call_site());
        let interface_calls =
            self.method_calls(interface_name, Ident::new("iface_el", Span::call_site()));
        interfaces.push(quote!{
            {
                let dyn_el: Option<&web_sys::#code_interface_name> = wasm_bindgen::JsCast::dyn_ref(&el);
                dyn_el.map(|iface_el| {
                    #(#interface_calls)*
                });
            }
        });
        self.fields(interface_name, fields);
    }

    pub fn get_other_methods(&self, interface_name: &str) -> proc_macro2::TokenStream {
        let mut body = quote!{};
        let interface_ident = Ident::new(&interface_name, Span::call_site());

        if (interface_name == "HTMLInputElementish") {
            // TODO save me from this hardcoded lifestyle
            body = quote!{
                /* Example function interface
                pub fn boop(&self) {
                    let me  = wasm_bindgen::JsValue::from_str("boop");
                    web_sys::console::log_1(&me);
                }*/
            };
        }

        quote!{
            impl #interface_ident {
                #body
            }
        }
    }

    pub fn get_interface_code(&self, interface_name: &str) -> proc_macro2::TokenStream {
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

        let interface_name = Codegen::get_wurst_interface_name(interface_name);
        let interface_ident = Ident::new(&interface_name, Span::call_site());

        let other_methods = self.get_other_methods(&interface_name);

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
        let macro_code = self.get_macro();
        let fallback = self.get_interface_code("HTMLElement");
        let interfaces = self.get_interfaces_code();
        let enum_code = self.get_enum_code();
        (quote!{

            #macro_code

            pub mod attr {
                pub use Elementish;
                pub use El;
                #enum_code

                #(#interfaces)*

                // Fall back interface for all other tags
                #fallback
            }
        }).to_tokens(&mut tokens);

        tokens.to_string()
    }
}
