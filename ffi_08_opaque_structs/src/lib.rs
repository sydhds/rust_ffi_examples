use std::ffi::CStr;
use std::path::{Path, PathBuf};
use std::io::{Error, Read};

// #[repr(C)]
#[derive(Debug)]
pub struct FileData {
    field1: u32,
    field2: u32,
    field3: u32,
    field4: u8,
}

impl FileData {

    fn new() -> Self {
        Self {
            field1: 11,
            field2: 22,
            field3: 42,
            field4: 255,
        }
    }

    fn set_field4(&mut self, value: u8) {
        self.field4 = value;
    }

    fn read(path: &Path) -> Result<FileData, Error> {
        println!("[Rust] reading path: {}", path.display());
        Ok(FileData::new())
    }

    fn write(path: &Path) -> Result<(), Error> {
        println!("[Rust] writing path: {}", path.display());
        Ok(())
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_data_new() -> *mut FileData {
    // Heap allocated new `FileData` with default values
    Box::into_raw(Box::new(FileData::new()))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_data_set_field4(f: *mut FileData, value: u8) {
    if !f.is_null() {
        let f = unsafe { &mut *f };
        f.set_field4(value);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_data_set_field4_v2(f: Option<&mut FileData>, value: u8) {
    f.map(|f| f.set_field4(value));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_data_debug(f: *mut FileData) {

    if !f.is_null() {
        let f = unsafe { &*f };
        println!("[Rust][file_data_debug] {:?}", f);
    } else {
        println!("[Rust][file_data_debug] f is null");
    }


}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_data_free(f: *mut FileData) {
    // Automatically drop `FileData`
    let _ = unsafe { Box::from_raw(f) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_data_read(f: *mut FileData, path: *const libc::c_char) -> bool {

    // Get pointer to heap allocated `FileData`

    let mut res = false;
    if !f.is_null() && !path.is_null() {
        let file_data = unsafe { Box::from_raw(f) };

        // Build path from c-string path argument
        let c_str = unsafe { CStr::from_ptr(path) };
        let p = PathBuf::from(c_str.to_str().unwrap_or_default());

        // Read the data and provide some minimal error handling
        res = FileData::read(&p).is_ok();

        // Forget the memory so Rust doesn't deallocate when `file_data` is dropped
        std::mem::forget(file_data);
    }

    res
}
