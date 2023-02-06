use std::env;

fn main() {
    let dir = env::current_dir().unwrap();
    println!("cargo:rustc-env=CARGO_WORKSPACE_ROOT={}", dir.display());
    println!("cargo:rerun-if-changed=locale");
}
