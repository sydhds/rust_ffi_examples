fn main() {
    let mut cfg = cc::Build::new();
    cfg.file("resources/extlib.c");
    // if let Some(include) = std::env::var_os("DEP_Z_INCLUDE") {
    //     cfg.include(include);
    // }
    cfg.compile("extlib");
    println!("cargo::rerun-if-changed=resources/extlib.c");
}