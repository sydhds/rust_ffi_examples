#[unsafe(no_mangle)]
extern "C" fn rusty_cb(a: i32) {
    println!("I'm a Rust function called from C with value {a}");
}

/*
#[link(name = "extlib")]
unsafe extern {
    fn register_callback(cb: extern fn(i32)) -> i32;
    fn trigger_callback();
}
*/

/*
fn main() {
    unsafe {
        register_callback(callback);
        trigger_callback(); // Triggers the callback.
    }
}
*/
