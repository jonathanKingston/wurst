extern crate sourcefile;
extern crate weedle;
#[macro_use]
extern crate failure;

use std::collections::HashMap;

use failure::{Fail, ResultExt};
use sourcefile::SourceFile;
use std::ffi::OsStr;
use std::fs;

extern crate heck;
use heck::SnakeCase;


#[derive(Debug)]
pub struct Interfaces {
    data: HashMap<String, Vec<String>>,
}

impl Interfaces {
    pub fn tag_interfaces<'a>(&self) -> HashMap<&'a str, &'a str> {
        let mut interfaces = HashMap::new();
        interfaces.insert("div", "HTMLDivElement");
        interfaces.insert("label", "HTMLLabelElement");
        interfaces.insert("input", "HTMLInputElement");
        interfaces.insert("img", "HTMLImageElement");
        interfaces.insert("form", "HTMLFormElement");
        interfaces.insert("title", "HTMLTitleElement");
        interfaces.insert("body", "HTMLBodyElement");
        interfaces.insert("head", "HTMLHeadElement");
        interfaces.insert("meta", "HTMLMetaElement");
        interfaces.insert("button", "HTMLButtonElement");
        interfaces.insert("textarea", "HTMLTextAreaElement");
        interfaces.insert("select", "HTMLSelectElement");
        interfaces.insert("br", "HTMLBrElement");
        interfaces.insert("hr", "HTMLHrElement");
        interfaces.insert("ul", "HTMLUListElement");
        interfaces.insert("ol", "HTMLOListElement");
        interfaces.insert("li", "HTMLLiElement");
        interfaces.insert("dl", "HTMLDListElement");
        interfaces.insert("option", "HTMLOptionElement");
        interfaces.insert("canvas", "HTMLCanvasElement");
        interfaces.insert("pre", "HTMLPreElement");
        interfaces.insert("fieldset", "HTMLFieldSetElement");
        interfaces.insert("span", "HTMLSpanElement");
        interfaces.insert("p", "HTMLParagraphElement");
        interfaces.insert("html", "HTMLHtmlElement");
        interfaces.insert("font", "HTMLFontElement");
/*
TODO interfaces:
HTMLTableCellElement
HTMLAnchorElement
HTMLTableColElement
HTMLAreaElement
HTMLOutputElement
HTMLTableElement
HTMLAudioElement
HTMLEmbedElement
HTMLLegendElement
HTMLTableRowElement
HTMLBaseElement
HTMLParamElement
HTMLTableSectionElement
HTMLFontElement
HTMLLinkElement
HTMLPictureElement
HTMLTemplateElement
HTMLMapElement
HTMLMediaElement
HTMLProgressElement
HTMLTimeElement
HTMLFrameElement
HTMLMenuElement
HTMLQuoteElement
HTMLFrameSetElement
HTMLMenuItemElement
HTMLScriptElement
HTMLTrackElement
HTMLDataElement
HTMLDataListElement
HTMLHeadingElement
HTMLMeterElement
HTMLSlotElement
HTMLVideoElement
HTMLDetailsElement
HTMLModElement
HTMLSourceElement
HTMLDialogElement
HTMLObjectElement
HTMLDirectoryElement
HTMLStyleElement
HTMLIFrameElement
HTMLOptGroupElement
HTMLTableCaptionElement
*/
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

    pub fn get_methods(&self, interface_name: &str) -> Option<&Vec<String>> {
      self.data.get(interface_name)
    }

}
