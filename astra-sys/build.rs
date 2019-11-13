use std::env;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let astra_sdk_dir = current_dir.join("astra-sdk");
    let astra_lib = astra_sdk_dir.join("lib");
    let astra_include = astra_sdk_dir.join("include");

    println!("cargo:rustc-link-lib=dylib=astra_core_api");
    println!("cargo:rustc-link-lib=dylib=astra_core");
    println!("cargo:rustc-link-lib=dylib=astra");
    println!(
        "cargo:rustc-link-search=native={}",
        astra_lib.to_str().unwrap()
    );

    let header_path = astra_include.join("astra/capi/astra.h");

    let bindings = bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path: PathBuf = env::var("OUT_DIR").unwrap().parse().unwrap();

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings.rs");
}
