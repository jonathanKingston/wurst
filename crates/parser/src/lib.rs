extern crate sourcefile;
extern crate weedle;
#[macro_use]
extern crate failure;

use std::collections::HashMap;

use failure::{Fail, ResultExt};
use sourcefile::SourceFile;
use std::ffi::OsStr;
use std::fs;

extern crate proc_macro2;
#[macro_use]
extern crate quote;
use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;

extern crate heck;
use heck::SnakeCase;

/*
HTMLDListElement
HTMLImageElement
HTMLOptionElement
HTMLTableCellElement
HTMLAnchorElement
HTMLInputElement
HTMLTableColElement
HTMLAreaElement
HTMLLabelElement
HTMLOutputElement
HTMLTableElement
HTMLAudioElement
HTMLEmbedElement
HTMLLegendElement
HTMLParagraphElement
HTMLTableRowElement
HTMLBaseElement
HTMLFieldSetElement
HTMLLIElement
HTMLParamElement
HTMLTableSectionElement
HTMLBodyElement
             HTMLFontElement
             HTMLLinkElement
             HTMLPictureElement
          HTMLTemplateElement
HTMLBRElement
  HTMLMapElement
              HTMLPreElement
              HTMLTextAreaElement
HTMLButtonElement
           HTMLFormElement
             HTMLMediaElement
            HTMLProgressElement
         HTMLTimeElement
HTMLCanvasElement
           HTMLFrameElement
            HTMLMenuElement
             HTMLQuoteElement
            HTMLTitleElement
HTMLCollection
              HTMLFrameSetElement
         HTMLMenuItemElement
         HTMLScriptElement
           HTMLTrackElement
HTMLDataElement
             HTMLHeadElement
             HTMLMetaElement
             HTMLSelectElement
           HTMLUListElement
HTMLDataListElement
         HTMLHeadingElement
          HTMLMeterElement
            HTMLSlotElement
             HTMLVideoElement
HTMLDetailsElement
          HTMLHRElement
               HTMLModElement
              HTMLSourceElement
HTMLDialogElement
           HTMLHtmlElement
             HTMLObjectElement
           HTMLSpanElement
HTMLDirectoryElement
        HTMLHyperlinkElementUtils
   HTMLOListElement
            HTMLStyleElement
HTMLDivElement
              HTMLIFrameElement
           HTMLOptGroupElement
         HTMLTableCaptionElement
}
*/

#[derive(Debug)]
pub struct Interfaces {
    data: HashMap<String, Vec<String>>,
}

impl Interfaces {
    fn tag_interfaces<'a>(&self) -> HashMap<&'a str, &'a str> {
        let mut interfaces = HashMap::new();
        interfaces.insert("div", "HTMLDivElement");
        interfaces.insert("label", "HTMLLabelElement");
        interfaces.insert("input", "HTMLInputElement");
        interfaces.insert("img", "HTMLImageElement");
        interfaces
    }

    pub fn parse() -> Result<Interfaces, failure::Error> {
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

        use weedle::types::NonAnyType::DOMString;
        use weedle::types::SingleType::NonAny;
        use weedle::types::Type::Single;
        let mut interfaces = HashMap::new();
        weedle::parse(&source.contents).map(|r| {
            for i in r {
                if let weedle::Definition::Interface(n) = i {
                    let mut setters = vec![];
                    for attr in n.members.body {
                        // TODO set more types here, booleans etc
                        if let weedle::interface::InterfaceMember::Attribute(a) = attr {
                            if a.readonly.is_none() {
                                if let Single(NonAny(DOMString(t))) = a.type_.type_ {
                                    println!("a: {:#?}", a);
                                    if t.q_mark != None {
                                        // TODO handle optionals
                                        continue;
                                    }
                                    // TODO handle native naming
                                    if (a.identifier.0 == "type") {
                                        continue;
                                    }
                                    let name = String::from(a.identifier.0).to_snake_case();
                                    setters.push(name);
                                }
                            }
                        }
                    }
                    interfaces.insert(n.identifier.0.into(), setters);
                }
            }
        });
        // TODO remove this
        //panic!("{:?}", interfaces);

        Ok(Interfaces { data: interfaces })
    }

    pub fn has_method_in_interface(&self, interface_name: &str, method_name: &str) -> bool {
        if let Some(methods) = self.data.get(interface_name) {
            for method in methods {
                if method == method_name {
                    return true;
                }
            }
        }
        return false;
    }

    fn method_calls(&self, interface_name: &str, el: Ident) -> Vec<proc_macro2::TokenStream> {
        let mut interface_calls = vec![];
        if let Some(methods) = self.data.get(interface_name) {
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

    pub fn get_macro(&self) -> String {
        let mut tokens = TokenStream::new();

        let mut arms = vec![];
        for (tag, interface) in &self.tag_interfaces() {
            let tag_string = String::from(tag.clone());
            let tag_ident = Ident::new(&tag_string, Span::call_site());
            let interface_name = Ident::new(&format!("{}Attributes", interface), Span::call_site());
            println!("{}, {}", tag_string, interface_name);
            arms.push(quote!{
               (#tag_string, {$( $key:ident : $value:expr ),*}) => {
                   {
                       let attrs = #interface_name {
                       //let attrs = HTMLElementAttributes {
                           $( $key: Some($value.into()), )*
                           ..Default::default()
                       };
                       let el_container = El {
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
                            name: $name.into(),
                            attrs
                        };
                        el_container
                    }
                }
            }
        }).to_tokens(&mut tokens);

        tokens.to_string()
    }

    pub fn get_interfaces_code(&self) -> String {
        let mut interfaces = String::new();
        for (tag, interface) in &self.tag_interfaces() {
            interfaces += &self.get_interface_code(interface);
        }
        interfaces
    }

    pub fn get_interface_name(&self, interface_name: &str) -> String {
        String::from(interface_name).replace("HTML", "Html")
    }

    //TODO neaten
    pub fn get_interface_code(&self, interface_name: &str) -> String {
        let mut tokens = TokenStream::new();
        let mut fields = vec![];
        let interface_calls =
            self.method_calls(interface_name, Ident::new("html_el", Span::call_site()));
        let main_interface_calls =
            self.method_calls("Element", Ident::new("el", Span::call_site()));

        if let Some(methods) = self.data.get(interface_name) {
            for field in methods {
                let field_ident = Ident::new(field, Span::call_site());
                fields.push(quote!{pub #field_ident: Option<String>,});
            }
        }
        if interface_name != "HTMLElement" {
            if let Some(methods) = self.data.get("HTMLElement") {
                for field in methods {
                    let field_ident = Ident::new(field, Span::call_site());
                    fields.push(quote!{pub #field_ident: Option<String>,});
                }
            }
        }
        if let Some(methods) = self.data.get("Element") {
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
                {
                  #interface
                }
                #(#main_interface_calls)*
                el
              }
          }
        }).to_tokens(&mut tokens);

        tokens.to_string()
    }
}
