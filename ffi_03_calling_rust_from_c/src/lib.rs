// turn off RUst's name mangling
#[unsafe(no_mangle)]
// `extern "C"` makes the function adhere to the C calling convention
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!");
}
