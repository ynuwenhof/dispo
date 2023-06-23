use std::{env, fs};
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dispo.rs");

    fs::write(dest_path, "").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
