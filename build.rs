extern crate codegen;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use codegen::Codegen;
#[macro_use]
extern crate failure;
use failure::ResultExt;
use std::process::Command;

fn main() -> Result<(), failure::Error> {
    let out_dir = env::var("OUT_DIR").context("reading OUT_DIR environment variable")?;
    let codegen = Codegen::gen();
    let dest_path = Path::new(&out_dir).join("bindings.rs");
    let mut f = File::create(&dest_path).unwrap();
    write!(f, "{}", codegen.get_code());

    let status = Command::new("rustfmt")
        .arg(&dest_path)
        .status()
        .context("running rustfmt")?;
    if !status.success() {
        bail!("rustfmt failed: {}", status)
    }

    Ok(())
}
