// use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use safer_ffi::{
    derive_ReprC,
    ffi_export,
    boxed::Box_,
    prelude::{
        // c_slice,
        repr_c,
        char_p
    }
};
use safer_ffi::prelude::ReprC;

/// An opaque struct over PathBuf
#[derive_ReprC]
#[repr(opaque)]
#[derive(Debug, Clone)]
struct RustPath(PathBuf);

#[ffi_export]
fn rust_path_debug(rp: Option<&RustPath>) {
    println!("[Rust printing value] {:?}", rp);
}

#[ffi_export]
fn rust_path_default<'a>() -> repr_c::Box<RustPath> {
    Box_::new(RustPath(PathBuf::from("/usr/bin/../bin/ls")))
}

#[ffi_export]
fn rust_path_default2<'a>() -> repr_c::Box<RustPath> {
    Box_::new(RustPath(PathBuf::from("/usr/bin/../bin/lsLSlsLS")))
}

#[ffi_export]
fn rust_path_free(rp: Option<repr_c::Box<RustPath>>) {
    drop(rp);
}

#[derive_ReprC]
#[repr(C)]
struct CResult<T : ReprC, Err: ReprC> {
    ok: Option<repr_c::Box<T>>,
    err: Option<Err>,
}

#[ffi_export]
fn cresult_free(cr: CResult<RustPath, char_p::Box>) {
    drop(cr);
}

#[ffi_export]
fn rust_path_canonicalize(x: &Option<repr_c::Box<RustPath>>) -> CResult<RustPath, char_p::Box> {
    if let Some(x) = x {
        let res = x.0.canonicalize();
        match res {
            Ok(x) => {
                CResult {
                    ok: Some(Box_::new(RustPath(x))),
                    err: None,
                }
            },
            Err(e) => {
                CResult {
                    ok: None,
                    err: Some(e.to_string().try_into().unwrap()),
                }
            }
        }
    } else {
        CResult {
            ok: None,
            err: Some("NULL pointer".to_string().try_into().unwrap()),
        }
    }
}

//

#[derive_ReprC]
#[repr(C)]
struct CResult2<T : ReprC, Err: ReprC> {
    ok: Option<repr_c::Box<T>>,
    err: Option<Err>,
}

#[ffi_export]
fn cresult2_free(cr: CResult2<RustPath, repr_c::Box<IoError>>) {
    drop(cr);
}

#[derive_ReprC]
#[repr(opaque)]
#[derive(Debug)]
struct IoError {
    inner: std::io::Error,
}

impl Default for IoError {
    fn default() -> Self {
        Self {
            inner: std::io::Error::new(std::io::ErrorKind::Other, "NULL pointer")
        }
    }
}

impl From<std::io::Error> for IoError {
    fn from(inner: std::io::Error) -> Self {
        Self { inner }
    }
}

#[ffi_export]
fn io_error_debug(io_e: &Option<repr_c::Box<IoError>>) {
    println!("[Rust printing value] {:?}", io_e);
}

#[ffi_export]
fn rust_path_canonicalize_2(x: &Option<repr_c::Box<RustPath>>) -> CResult2<RustPath, repr_c::Box<IoError>> {
    if let Some(x) = x {
        let res = x.0.canonicalize();
        match res {
            Ok(x) => {
                CResult2 {
                    ok: Some(Box_::new(RustPath(x))),
                    err: None,
                }
            },
            Err(e) => {
                CResult2 {
                    ok: None,
                    // err: Some(Box_::new(IoError { inner: e })),
                    err: Some(Box_::new(e.into()))
                }
            }
        }
    } else {
        CResult2 {
            ok: None,
            err: Some(
                // Box_::new(IoError { inner: std::io::Error::new(std::io::ErrorKind::Other, "NULL pointer") })
                Box_::new(IoError::default())
            ),
        }
    }
}


//

// The following function is only necessary for the header generation.
#[cfg(feature = "headers")] // c.f. the `Cargo.toml` section
pub fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("rust_path.h")?
        .generate()
}