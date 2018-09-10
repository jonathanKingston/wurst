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
        for (tag, interface) in &self.interfaces.tag_interfaces() {
            let tag_string = String::from(tag.clone());
            let tag_ident = Ident::new(&tag_string, Span::call_site());
            let interface_name = Ident::new(&format!("{}Attributes", interface), Span::call_site());
            println!("{}, {}", tag_string, interface_name);
            arms.push(quote!{
               (#tag_string, {$( $key:ident : $value:expr ),*}) => {
                   {
                       let attrs = #interface_name {
                           $( $key: Some($value.into()), )*
                           ..Default::default()
                       };
                       let el_container = El {
                           el: None,
                           name: #tag_string.to_string(),
                           attrs
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
                ($name:tt, {$( $key:ident : $value:expr ),*}) => {
                    {
                        let attrs = HTMLElementAttributes {
                            $( $key: Some($value.into()), )*
                            ..Default::default()
                        };
                        let el_container = El {
                            el: None,
                            name: $name.into(),
                            attrs
                        };
                        el_container
                    }
                }
            }
        })
    }

    pub fn get_interfaces_code(&self) -> Vec<proc_macro2::TokenStream> {
        let mut interfaces = vec![];
        for (tag, interface) in &self.interfaces.tag_interfaces() {
            interfaces.push(self.get_interface_code(interface));
        }
        interfaces
    }

    pub fn get_interface_name(&self, interface_name: &str) -> String {
        String::from(interface_name).replace("HTML", "Html")
    }

    //TODO neaten
    pub fn get_interface_code(&self, interface_name: &str) -> proc_macro2::TokenStream {
        let mut tokens = TokenStream::new();
        let mut fields = vec![];
        let mut other_interface = quote!{};
        let interface_calls =
            self.method_calls(interface_name, Ident::new("html_el", Span::call_site()));
        let main_interface_calls =
            self.method_calls("Element", Ident::new("el", Span::call_site()));

        if let Some(methods) = self.interfaces.get_methods(interface_name) {
            for field in methods {
                let field_ident = Ident::new(field, Span::call_site());
                fields.push(quote!{pub #field_ident: Option<String>,});
            }
        }
        if interface_name != "HTMLElement" {
            let other_interface_calls =
                self.method_calls("HTMLElement", Ident::new("html_el", Span::call_site()));
            other_interface = quote!{
              let dyn_el: Option<&web_sys::HtmlElement> = wasm_bindgen::JsCast::dyn_ref(&el);
              dyn_el.map(|html_el| {
                  #(#other_interface_calls)*
              });
            };
            if let Some(methods) = self.interfaces.get_methods("HTMLElement") {
                for field in methods {
                    let field_ident = Ident::new(field, Span::call_site());
                    fields.push(quote!{pub #field_ident: Option<String>,});
                }
            }
        }
        if let Some(methods) = self.interfaces.get_methods("Element") {
            for field in methods {
                let field_ident = Ident::new(field, Span::call_site());
                fields.push(quote!{pub #field_ident: Option<String>,});
            }
        }
        let web_sys_ident = Ident::new(&self.get_interface_name(interface_name), Span::call_site());

        let interface = quote!{
          let dyn_el: Option<&web_sys::#web_sys_ident> = wasm_bindgen::JsCast::dyn_ref(&el);
          dyn_el.map(|html_el| {
              #(#interface_calls)*
          });
        };
        let interface_name = format!("{}Attributes", interface_name);
        let interface_ident = Ident::new(&interface_name, Span::call_site());

        (quote!{
          #[derive(Default)]
          pub struct #interface_ident {
              #(#fields)*
          }
          impl Attributish for #interface_ident {
              fn flush(&self, el: web_sys::Element) -> web_sys::Element {
                // TODO this inheritance tree should be defined from the parsed webidl
                {
                  #interface
                }
                {
                  #other_interface
                }
                #(#main_interface_calls)*
                el
              }
          }
        })
    }

    pub fn get_code(&self) -> String {
        let mut tokens = TokenStream::new();
        let macro_code = self.get_macro();
        let fallback = self.get_interface_code("HTMLElement");
        let interfaces = self.get_interfaces_code();
        (quote!{

            #macro_code

            pub mod attr {
                pub use Attributish;

                #(#interfaces)*

                // Fall back interface for all other tags
                #fallback
            }
        }).to_tokens(&mut tokens);

        tokens.to_string()
    }
}
