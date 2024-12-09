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
    let include_flag = format!("-I{}", String::from_utf8_lossy(&include_dir_out.stdout).trim_end());

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-search={}", pkg_lib_dir);
    println!("cargo:rustc-link-search={}", lib_dir);
    println!("cargo::rustc-link-arg=-fPIC");

    if cfg!(target_os = "macos") {
        // Get gettext lib path from brew
        let gettext_lib = Command::new("brew")
            .args(["--prefix", "gettext"])
            .output()
            .expect("Failed to get gettext path from brew")
            .stdout;
        let gettext_lib = format!("{}/lib", String::from_utf8_lossy(&gettext_lib).trim_end());
        println!("cargo:rustc-link-search={}", gettext_lib);
        println!("cargo:rustc-link-lib=intl");

        // Add link to PostgreSQL libraries
        println!("cargo:rustc-link-lib=pq");
        println!("cargo:rustc-link-lib=pgcommon");
        println!("cargo:rustc-link-lib=pgport");
    }

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(include_flag)
        .clang_arg("-fPIC");

    // Add macOS-specific include path for gettext if on macOS
    if cfg!(target_os = "macos") {
        let gettext_include = Command::new("brew")
            .args(["--prefix", "gettext"])
            .output()
            .expect("Failed to get gettext path from brew")
            .stdout;
        let gettext_include = format!("-I{}/include", String::from_utf8_lossy(&gettext_include).trim_end());
        builder = builder.clang_arg(&gettext_include);
    }

    let bindings = builder
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
