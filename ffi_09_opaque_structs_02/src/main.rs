#[derive(Debug)]
#[repr(C)]
enum Status {
    Success,
    Failure,
}

#[repr(C)]
struct CMyLib {
    // Original code:
    // _f: [u8; 0],
    // Nomicon code:
    _f: (),
    // *mut u8: is not Send or Sync
    // PhantomPinned: https://doc.rust-lang.org/nightly/std/marker/struct.PhantomPinned.html
    // If a type contains a PhantomPinned, it will not implement Unpin
    _m: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

type MyLibHandle = *mut CMyLib;
impl CMyLib {
    fn handle() -> MyLibHandle {
        std::ptr::null_mut()
    }
}

#[link(name = "mylib")]
unsafe extern "C" {
    fn mylib_create(handle_ptr: *mut MyLibHandle) -> Status;
    fn mylib_destroy(handle: MyLibHandle) -> Status;
    fn mylib_set(handle: MyLibHandle, value: i32) -> Status;
    fn mylib_dump(handle: MyLibHandle) -> Status;
}

pub struct MyLib {
    handle: MyLibHandle,
}

impl MyLib {
    pub fn create() -> Result<Self, String> {

        let mut handle = CMyLib::handle(); // type: MyLibHandle
        let ptr = &raw mut handle; // type: *mut MyLibHandle

        match unsafe { mylib_create(ptr) } {
            Status::Success => Ok(Self { handle }),
            err_code => Err(format!("Failed to create MyLib. Error: {err_code:?}")),
        }

        // Original code (std::ptr::addr_of_mut! is deprecated in favor of &raw mut)
        /*
        match unsafe { mylib_create(std::ptr::addr_of_mut!(handle)) } {
            Status::Success => Ok(Self { handle }),
            err_code => Err(format!("Failed to create MyLib. Error: {err_code:?}")),
        }
        */
    }

    pub fn set(&mut self, val: i32) {
        let Status::Success = (unsafe {
            mylib_set(self.handle, val)
        }) else {
            panic!("Failed to set value");
        };
    }

    pub fn dump(&mut self) {
        let Status::Success = (unsafe {
            mylib_dump(self.handle)
        }) else {
            panic!("Failed to dump mylib");
        };
    }
}

impl Drop for MyLib {
    fn drop(&mut self) {
        let Status::Success = (unsafe { mylib_destroy(self.handle) })
        else {
            panic!("Something went wrong");
        };
    }
}

fn main() {
    let mut ml = MyLib::create().unwrap();
    println!("Calling MyLib::dump():");
    ml.dump();
    ml.set(123);
    println!("Calling MyLib::dump() again:");
    ml.dump();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ml = MyLib::create().unwrap();
        ml.dump();
        ml.set(123);
        ml.dump();
    }
}
