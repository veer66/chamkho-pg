use std::process::Command;

fn main() {
    let pg_share_dir = Command::new("pg_config")
        .arg("--sharedir")
        .output()
        .map(|o| String::from_utf8(o.stdout).unwrap().trim().to_string())
        .unwrap_or_else(|_| "/usr/local/share/postgresql".to_string());

    println!("cargo:rustc-env=PG_SHARE_DIR={}", pg_share_dir);
}
