// build.rs

fn main() {
    pkg_config::Config::new().probe("snappy").unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}
