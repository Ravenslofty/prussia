use std::{env, error::Error, fs::{self, File}, io::Write, path::PathBuf};

fn main() -> Result<(), Box<Error>> {
    // build directory for this crate
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // put `linkfile.ld` in the build directory
    File::create(out_dir.join("linkfile.ld"))?.write_all(include_bytes!("linkfile.ld"))?;

    // and put `libprussia-rt.a` in there too
    fs::copy("libprussia-rt.a", out_dir.join("libprussia-rt.a"))?;
    println!("cargo:rustc-link-lib=static=prussia-rt");

    Ok(())
}
