use std::ffi::CString;
use std::ptr;

#[link(name = "readline")]
unsafe extern "C" {
    static rl_readline_version: libc::c_int;
    static mut rl_prompt: *const libc::c_char;
}

fn main() {

    println!("You have readline version {} installed.",
             unsafe { rl_readline_version as i32 });


    let prompt = CString::new("[my-awesome-shell] $").unwrap();

    #[allow(static_mut_refs)]
    unsafe {
        rl_prompt = prompt.as_ptr();
        println!("{:?}", rl_prompt);
        rl_prompt = ptr::null();
    }

}