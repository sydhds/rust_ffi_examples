unsafe extern "C" {
    fn foo(x: i32, ...);
}

fn main() {

    println!("Hello, world!");

    unsafe {
        foo(4, 22, 33, 42, 99);
    }
}
