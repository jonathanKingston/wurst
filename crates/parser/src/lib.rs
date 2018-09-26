extern crate failure;
extern crate sourcefile;
extern crate weedle;

use std::collections::HashMap;

use failure::ResultExt;
use sourcefile::SourceFile;
use std::ffi::OsStr;
use std::fs;

extern crate wasm_bindgen;

extern crate heck;
use heck::SnakeCase;

#[derive(Debug)]
pub enum InterfaceFeature {
    Property(String),
    Function(String),
}

#[derive(Debug)]
pub struct Interfaces {
    data: HashMap<String, Vec<InterfaceFeature>>,
}

macro_rules! html_tag {
    ($interfaces:ident, $tag_name:tt, $class_name:tt, $interface_name:tt) => {
        let iface = format!("HTML{}Element", $interface_name);
        $interfaces.insert($tag_name.into(), iface);
    };
}

macro_rules! html_htmlelement_tag {
    ($interfaces:ident, $tag_name:tt) => {
        $interfaces.insert($tag_name.into(), String::from("HTMLElement"));
    };
}

impl Interfaces {
    pub fn tag_interfaces(&self) -> HashMap<String, String> {
        let mut interfaces: HashMap<String, String> = HashMap::new();

        // Defined from: https://searchfox.org/mozilla-central/rev/0640ea80fbc8d48f8b197cd363e2535c95a15eb3/parser/htmlparser/nsHTMLTagList.h#49

        html_tag!(interfaces, "a", "Anchor", "Anchor");
        html_htmlelement_tag!(interfaces, "abbr");
        html_htmlelement_tag!(interfaces, "acronym");
        html_htmlelement_tag!(interfaces, "address");
        html_tag!(interfaces, "applet", "Unknown", "Unknown");
        html_tag!(interfaces, "area", "Area", "Area");
        html_htmlelement_tag!(interfaces, "article");
        html_htmlelement_tag!(interfaces, "aside");
        html_tag!(interfaces, "audio", "Audio", "Audio");
        html_htmlelement_tag!(interfaces, "b");
        html_tag!(interfaces, "base", "Shared", "Base");
        html_htmlelement_tag!(interfaces, "basefont");
        html_htmlelement_tag!(interfaces, "bdi");
        html_htmlelement_tag!(interfaces, "bdo");
        html_tag!(interfaces, "bgsound", "Unknown", "Unknown");
        html_htmlelement_tag!(interfaces, "big");
        html_tag!(interfaces, "blockquote", "Shared", "Quote");
        html_tag!(interfaces, "body", "Body", "Body");
        html_tag!(interfaces, "br", "BR", "BR");
        html_tag!(interfaces, "button", "Button", "Button");
        html_tag!(interfaces, "canvas", "Canvas", "Canvas");
        html_tag!(interfaces, "caption", "TableCaption", "TableCaption");
        html_htmlelement_tag!(interfaces, "center");
        html_htmlelement_tag!(interfaces, "cite");
        html_htmlelement_tag!(interfaces, "code");
        html_tag!(interfaces, "col", "TableCol", "TableCol");
        html_tag!(interfaces, "colgroup", "TableCol", "TableCol");
        html_tag!(interfaces, "data", "Data", "Data");
        html_tag!(interfaces, "datalist", "DataList", "DataList");
        html_htmlelement_tag!(interfaces, "dd");
        html_tag!(interfaces, "del", "Mod", "Mod");
        html_tag!(interfaces, "details", "Details", "Details");
        html_htmlelement_tag!(interfaces, "dfn");
        html_tag!(interfaces, "dialog", "Dialog", "Dialog");
        html_tag!(interfaces, "dir", "Shared", "Directory");
        html_tag!(interfaces, "div", "Div", "Div");
        html_tag!(interfaces, "dl", "SharedList", "DList");
        html_htmlelement_tag!(interfaces, "dt");
        html_htmlelement_tag!(interfaces, "em");
        html_tag!(interfaces, "embed", "Embed", "Embed");
        html_tag!(interfaces, "fieldset", "FieldSet", "FieldSet");
        html_htmlelement_tag!(interfaces, "figcaption");
        html_htmlelement_tag!(interfaces, "figure");
        html_tag!(interfaces, "font", "Font", "Font");
        html_htmlelement_tag!(interfaces, "footer");
        html_tag!(interfaces, "form", "Form", "Form");
        html_tag!(interfaces, "frame", "Frame", "Frame");
        html_tag!(interfaces, "frameset", "FrameSet", "FrameSet");
        html_tag!(interfaces, "h1", "Heading", "Heading");
        html_tag!(interfaces, "h2", "Heading", "Heading");
        html_tag!(interfaces, "h3", "Heading", "Heading");
        html_tag!(interfaces, "h4", "Heading", "Heading");
        html_tag!(interfaces, "h5", "Heading", "Heading");
        html_tag!(interfaces, "h6", "Heading", "Heading");
        html_tag!(interfaces, "head", "Shared", "Head");
        html_htmlelement_tag!(interfaces, "header");
        html_htmlelement_tag!(interfaces, "hgroup");
        html_tag!(interfaces, "hr", "HR", "HR");
        html_tag!(interfaces, "html", "Shared", "Html");
        html_htmlelement_tag!(interfaces, "i");
        html_tag!(interfaces, "iframe", "IFrame", "IFrame");
        html_htmlelement_tag!(interfaces, "image");
        html_tag!(interfaces, "img", "Image", "Image");
        html_tag!(interfaces, "input", "Input", "Input");
        html_tag!(interfaces, "ins", "Mod", "Mod");
        html_htmlelement_tag!(interfaces, "kbd");
        html_tag!(interfaces, "keygen", "Span", "Span");
        html_tag!(interfaces, "label", "Label", "Label");
        html_tag!(interfaces, "legend", "Legend", "Legend");
        html_tag!(interfaces, "li", "LI", "LI");
        html_tag!(interfaces, "link", "Link", "Link");
        html_tag!(interfaces, "listing", "Pre", "Pre");
        html_htmlelement_tag!(interfaces, "main");
        html_tag!(interfaces, "map", "Map", "Map");
        html_htmlelement_tag!(interfaces, "mark");
        html_tag!(interfaces, "marquee", "Div", "Div");
        html_tag!(interfaces, "menu", "Menu", "Menu");
        html_tag!(interfaces, "menuitem", "MenuItem", "MenuItem");
        html_tag!(interfaces, "meta", "Meta", "Meta");
        html_tag!(interfaces, "meter", "Meter", "Meter");
        html_tag!(interfaces, "multicol", "Unknown", "Unknown");
        html_htmlelement_tag!(interfaces, "nav");
        html_htmlelement_tag!(interfaces, "nobr");
        html_htmlelement_tag!(interfaces, "noembed");
        html_htmlelement_tag!(interfaces, "noframes");
        html_htmlelement_tag!(interfaces, "noscript");
        html_tag!(interfaces, "object", "Object", "Object");
        html_tag!(interfaces, "ol", "SharedList", "OList");
        html_tag!(interfaces, "optgroup", "OptGroup", "OptGroup");
        html_tag!(interfaces, "option", "Option", "Option");
        html_tag!(interfaces, "output", "Output", "Output");
        html_tag!(interfaces, "p", "Paragraph", "Paragraph");
        html_tag!(interfaces, "param", "Shared", "Param");
        html_tag!(interfaces, "picture", "Picture", "Picture");
        html_htmlelement_tag!(interfaces, "plaintext");
        html_tag!(interfaces, "pre", "Pre", "Pre");
        html_tag!(interfaces, "progress", "Progress", "Progress");
        html_tag!(interfaces, "q", "Shared", "Quote");
        html_htmlelement_tag!(interfaces, "rb");
        html_htmlelement_tag!(interfaces, "rp");
        html_htmlelement_tag!(interfaces, "rt");
        html_htmlelement_tag!(interfaces, "rtc");
        html_htmlelement_tag!(interfaces, "ruby");
        html_htmlelement_tag!(interfaces, "s");
        html_htmlelement_tag!(interfaces, "samp");
        html_tag!(interfaces, "script", "Script", "Script");
        html_htmlelement_tag!(interfaces, "section");
        html_tag!(interfaces, "select", "Select", "Select");
        html_htmlelement_tag!(interfaces, "small");
        html_tag!(interfaces, "slot", "Slot", "Slot");
        html_tag!(interfaces, "source", "Source", "Source");
        html_tag!(interfaces, "span", "Span", "Span");
        html_htmlelement_tag!(interfaces, "strike");
        html_htmlelement_tag!(interfaces, "strong");
        html_tag!(interfaces, "style", "Style", "Style");
        html_htmlelement_tag!(interfaces, "sub");
        html_tag!(interfaces, "summary", "Summary", "");
        html_htmlelement_tag!(interfaces, "sup");
        html_tag!(interfaces, "table", "Table", "Table");
        html_tag!(interfaces, "tbody", "TableSection", "TableSection");
        html_tag!(interfaces, "td", "TableCell", "TableCell");
        html_tag!(interfaces, "textarea", "TextArea", "TextArea");
        html_tag!(interfaces, "tfoot", "TableSection", "TableSection");
        html_tag!(interfaces, "th", "TableCell", "TableCell");
        html_tag!(interfaces, "thead", "TableSection", "TableSection");
        html_tag!(interfaces, "template", "Template", "Template");
        html_tag!(interfaces, "time", "Time", "Time");
        html_tag!(interfaces, "title", "Title", "Title");
        html_tag!(interfaces, "tr", "TableRow", "TableRow");
        html_tag!(interfaces, "track", "Track", "Track");
        html_htmlelement_tag!(interfaces, "tt");
        html_htmlelement_tag!(interfaces, "u");
        html_tag!(interfaces, "ul", "SharedList", "UList");
        html_htmlelement_tag!(interfaces, "var");
        html_tag!(interfaces, "video", "Video", "Video");
        html_htmlelement_tag!(interfaces, "wbr");
        html_tag!(interfaces, "xmp", "Pre", "Pre");

        // TODO wrap unsafe tags in a feature flag

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
                           //         println!("a: {:#?}", a);
                                    if t.q_mark != None {
                                        // TODO handle optionals
                                        continue;
                                    }
                                    // TODO handle native naming
                                    if a.identifier.0 == "type" {
                                        continue;
                                    }
                                    let name = String::from(a.identifier.0).to_snake_case();
                                    setters.push(InterfaceFeature::Property(name));
                                }
                            }
                        } else if let weedle::interface::InterfaceMember::Operation(a) = attr {
                            // TODO we just support 0 argument functions with no return
                            if let weedle::types::ReturnType::Void(_) = a.return_type {
                                if a.args.body.list.is_empty() {
                                    if let Some(id) = a.identifier {
                                        // println!(">>>>>>>{:#?} {:?}", a.args, id.0);
                                        let name = String::from(id.0).to_snake_case();
                                        setters.push(InterfaceFeature::Function(name));
                                    }
                                }
                            }
                        }
                    }
                    interfaces.insert(n.identifier.0.into(), setters);
                }
            }
        });
        // TODO remove this
        // panic!("{:?}", interfaces);

        Ok(Interfaces { data: interfaces })
    }

    pub fn has_properties_in_interface(&self, interface_name: &str, method_name: &str) -> bool {
        if let Some(methods) = self.get_properties(interface_name) {
            methods.iter().filter(|a| &method_name == *a);
        }
        false
    }

    pub fn get_properties(&self, interface_name: &str) -> Option<Vec<&str>> {
        return self.data.get(interface_name).map(|methods| {
            return methods.iter().filter_map(|a| {
                if let InterfaceFeature::Property(method_name) = a {
                    Some(method_name.as_str())
                } else {
                    None
                }
            }).collect();
        });
    }

    pub fn get_methods(&self, interface_name: &str) -> Option<Vec<&str>> {
        return self.data.get(interface_name).map(|methods| {
            return methods.iter().filter_map(|a| {
                if let InterfaceFeature::Function(method_name) = a {
                    Some(method_name.as_str())
                } else {
                    None
                }
            }).collect();
        });
    }
}
