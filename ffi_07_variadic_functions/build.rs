fn main() {
    let mut cfg = cc::Build::new();
    cfg.file("resources/extlib.c");
    cfg.compile("extlib");
    println!("cargo::rerun-if-changed=resources/extlib.c");
}