use std::env;
use std::path::PathBuf;
use std::process::Command;
fn main() {
    let pkg_lib_dir_out = Command::new("pg_config")
        .arg("--pkglibdir")
        .output()
        .unwrap();
    let lib_dir_out = Command::new("pg_config").arg("--libdir").output().unwrap();
    let include_dir_out = Command::new("pg_config")
        .arg("--includedir-server")
        .output()
        .unwrap();
    let pkg_lib_dir = String::from_utf8_lossy(&pkg_lib_dir_out.stdout);
    let pkg_lib_dir = pkg_lib_dir.trim_end();
    let lib_dir = String::from_utf8_lossy(&lib_dir_out.stdout);
    let lib_dir = lib_dir.trim_end();
    let include_flag = format!("-I{}", String::from_utf8_lossy(&include_dir_out.stdout));
    let include_flag = include_flag.trim_end();
    //    println!("cargo:rustc-link-lib=pgcommon");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-search={}", pkg_lib_dir);
    println!("cargo:rustc-link-search={}", lib_dir);
    println!("cargo::rustc-link-arg=-fPIC");
    //    println!("cargo::rustc-flags=-fPIC");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(include_flag)
        .clang_arg("-fPIC")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("FP_SUBNORMAL")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
