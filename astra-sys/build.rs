use std::env;
use std::path::PathBuf;

fn main() {
    let current_dir = env::current_dir().unwrap();

    let astra_include: PathBuf = env::var("ASTRA_SDK_INCLUDE")
        .expect("ASTRA_SDK_INCLUDE env not set")
        .parse()
        .unwrap();
    let astra_lib: PathBuf = env::var("ASTRA_SDK_LIB")
        .expect("ASTRA_SDK_LIB env not set")
        .parse()
        .unwrap();

    println!("cargo:rustc-link-lib=dylib=astra_core_api");
    println!("cargo:rustc-link-lib=dylib=astra_core");
    println!("cargo:rustc-link-lib=dylib=astra");
    println!(
        "cargo:rustc-link-search=native={}",
        astra_lib.to_str().unwrap()
    );
    println!("cargo:include=/usr/lib/jvm/java-8-openjdk/include");

    let path = current_dir.join("./android/jni/armeabi-v7a/");
    println!("cargo:rustc-link-search=native={}", path.to_str().unwrap());

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
