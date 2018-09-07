extern crate parser;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use parser::{Interfaces};
extern crate failure;
use failure::ResultExt;


fn main() -> Result<(),failure::Error> {
    let out_dir = env::var("OUT_DIR").context("reading OUT_DIR environment variable")?;
    let interfaces = Interfaces::parse().unwrap();
    let dest_path = Path::new(&out_dir).join("bindings.rs");
    let mut f = File::create(&dest_path).unwrap();

    // Code generating this macro because I can't find a way to make generics to line up for passing in an optional attribute holder whilst then calling methods on it in the containing structure
    write!(f, "{}", interfaces.get_macro());

    write!(f, "{}", interfaces.get_interfaces_code());
    write!(f, "{}", interfaces.get_interface_code("HTMLElement"));
    Ok(())
}
