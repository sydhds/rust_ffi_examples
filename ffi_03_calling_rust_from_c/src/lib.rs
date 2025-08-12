// turn off RUst's name mangling:
// Mangling is when a compiler changes the name we’ve given a function to a different name that
// contains more information for other parts of the compilation process to consume
// but is less human readable.
#[unsafe(no_mangle)]
// `extern "C"` makes the function adhere to the C calling convention
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!");
}
