use sourcefile::SourceFile;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use failure::{Fail, ResultExt};

#[derive(Debug)]
pub struct interfaces {
    data: HashMap<String, Vec<String>>
}
impl interfaces {
    pub fn parse() -> Result<interfaces, failure::Error> {
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
                            if let weedle::interface::InterfaceMember::Attribute(a) = attr {
                                setters.push(a.identifier.0.into());
                            }
                        }
                        interfaces.insert(n.identifier.0.into(), setters);
                    }
                    _ => (),
                }
            }
        });
    
        Ok(interfaces { data: interfaces })
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
}
