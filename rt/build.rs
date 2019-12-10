use std::{env, error::Error, fs::{self, File}, io::Write, path::PathBuf};
use cc::Build;

fn main() -> Result<(), Box<dyn Error>> {
    // build directory for this crate
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // put `link.x` in the build directory
    File::create(out_dir.join("link.x"))?.write_all(include_bytes!("link.x"))?;

    /*
    // assemble the `asm.s` file
    Build::new().file("asm.s").compile("asm");

    // rebuild if `asm.s` changed
    println!("cargo:rerun-if-changed=asm.s");
    */

    // link to `librt.a`
    fs::copy("librt.a", out_dir.join("librt.a"))?; // <- NEW!
    println!("cargo:rustc-link-lib=static=rt"); // <- NEW!

    // rebuild if `librt.a` changed
    println!("cargo:rerun-if-changed=librt.a"); // <- NEW!

    Ok(())
}