extern crate codegen;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use codegen::Codegen;
extern crate failure;
use failure::ResultExt;

fn main() -> Result<(), failure::Error> {
    let out_dir = env::var("OUT_DIR").context("reading OUT_DIR environment variable")?;
    let codegen = Codegen::gen();
    let dest_path = Path::new(&out_dir).join("bindings.rs");
    let mut f = File::create(&dest_path).unwrap();
    write!(f, "{}", codegen.get_code());
    Ok(())
}
