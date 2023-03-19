use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // override user `linkfile.ld` in the build directory
    File::create(out_dir.join("linkfile.ld"))?.write_all(include_bytes!("kernel-linkfile.ld"))?;

    Ok(())
}
