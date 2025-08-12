#[repr(C)]
struct RustObject {
    a: i32,
    // Other members...
}

unsafe extern "C" fn callback(target: *mut RustObject, a: i32) {
    println!("I'm called from C with value {a}");

    // Note: Removing the unsafe block makes the compiler prints some explanations:
    // raw pointers may be null, dangling or unaligned;
    // they can violate aliasing rules and cause data races: all of these are undefined behavior
    unsafe {
        // Update the value in RustObject with the value received from the callback:
        (*target).a = a;
    }
}

#[link(name = "extlib")]
unsafe extern "C" {
    fn register_callback(
        target: *mut RustObject,
        cb: unsafe extern "C" fn(*mut RustObject, i32),
    ) -> i32;
    fn trigger_callback();
}

const BOX_RUST_OBJECT: bool = true;

fn main() {

    match BOX_RUST_OBJECT {
        true => {
            let mut rust_object = Box::new(RustObject { a: 5 });
            unsafe {
                // Calling functions from C library "extlib"
                register_callback(&mut *rust_object, callback);
                trigger_callback();
            }

        }
        false => {

            let mut rust_object = RustObject { a: 5 };

            unsafe {
                // Calling functions from C library "extlib"
                register_callback(&mut rust_object, callback);
                trigger_callback();
            }

        }
    }
}
