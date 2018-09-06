use sourcefile::SourceFile;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use failure::{Fail, ResultExt};

pub type Interfaces = HashMap<String, Vec<String>>;
pub fn interface_parse() -> Result<Interfaces, failure::Error> {
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
