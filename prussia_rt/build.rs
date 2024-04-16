use std::{
    env,
    error::Error,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::Command,
};

const LIBPRUSSIA_RT_ASM_SRC_NAME: &str = "src/routines.S";
const LIBPRUSSIA_RT_ASM_LIB_NAME: &str = "libprussia-rt.a";

fn main() -> Result<(), Box<dyn Error>> {
    // build directory for this crate
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    let libprussia_rt_asm_script_path =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join("rebuild-asm-lib.sh");

    // put `linkfile.ld` in the build directory
    File::create(out_dir.join("linkfile.ld"))?.write_all(include_bytes!("user-linkfile.ld"))?;

    // Rebuild if routines.S has changed.
    let libprussia_rt_asm_src_path =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join(LIBPRUSSIA_RT_ASM_SRC_NAME);
    println!(
        "cargo::rerun-if-changed={}",
        libprussia_rt_asm_src_path.display()
    );

    // Run the routines.S rebuild script.
    match Command::new(libprussia_rt_asm_script_path).status() {
        Ok(e) => {
            println!("rebuild-asm-lib.sh exit status: {e}");
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    }

    // and put `libprussia-rt.a` in there too
    fs::copy(
        LIBPRUSSIA_RT_ASM_LIB_NAME,
        out_dir.join(LIBPRUSSIA_RT_ASM_LIB_NAME),
    )?;
    println!("cargo:rustc-link-lib=static=prussia-rt");

    Ok(())
}
