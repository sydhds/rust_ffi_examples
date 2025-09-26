extern crate core;

use ruint::aliases::U256;
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

/// An opaque struct over U256
#[derive_ReprC]
#[repr(opaque)]
#[derive(Debug, Clone)]
struct CUint256(U256);

impl Default for CUint256 {
    fn default() -> Self {
        Self(U256::ZERO)
    }
}

#[ffi_export]
fn cuint256_debug(cu: Option<&CUint256>) {
    println!("[Rust printing U256 value] {:?}", cu);
}

#[ffi_export]
fn cuint256_format(cu: Option<&CUint256>) -> char_p::Box {
    // From https://github.com/getditto/safer_ffi/blob/310a2c94c39e41335f5261724e733cd036a06629/ffi_tests/src/lib.rs
    let s = format!("{:?}", cu);
    s
        .try_into()
        .unwrap()
}

// Note: char_p::Box in C will translate to char * but it should not be freed with C free() function
// (unless you use an allocator like libc_alloc which use posix memalign, and thus allocated objects are free() compatible)
#[ffi_export]
fn free_char_p(_string: Option<char_p::Box>) {}

// Another solution is using an alias so this will draw more attention for the C developers
// (We expect he will not throw a free(XXX) with this type)

#[derive_ReprC(rename = "cuint256_format_string")]
#[repr(transparent)]
pub struct CUint256FormatStr(char_p::Box);

#[ffi_export]
fn cuint256_format_2(cu: Option<&CUint256>) -> CUint256FormatStr {
    let s = format!("{:?}", cu);
    CUint256FormatStr(s.try_into().unwrap())
}

#[ffi_export]
fn cuint256_format_string_free(_string: CUint256FormatStr) {}

#[ffi_export]
fn double_rand() -> repr_c::Vec<CUint256> {
    vec![
        CUint256(U256::random()),
        CUint256(U256::random()),
    ].into()
}

#[ffi_export]
fn cuint256_zero<'a>() -> repr_c::Box<CUint256> {
    Box_::new(CUint256::default())
}

#[ffi_export]
fn cuint256_one<'a>() -> repr_c::Box<CUint256> {
    Box_::new(CUint256(U256::ONE))
}

#[ffi_export]
fn cuint256_free(cfr: Option<repr_c::Box<CUint256>>) {
    drop(cfr);
}

// Vec<CUint256>

#[ffi_export]
fn vec_cuint256_get(v: Option<&repr_c::Vec<CUint256>>, i: usize) -> Option<&CUint256> {
    v.and_then(|v| v.get(i))
}

#[ffi_export]
fn vec_cuint256_free(v: repr_c::Vec<CUint256>) {
    drop(v);
}

// End Vec<CUint256>

// Using CUint256 in other non-opaque structure

#[derive_ReprC]
#[repr(C)]
#[derive(Debug)]
pub struct BigValues {
    x: repr_c::Box<CUint256>,
    y: repr_c::Box<CUint256>,
}

#[ffi_export]
fn big_values_debug(bv: Option<&BigValues>) {
    println!("[Rust printing BigValues] {:?}", bv);
}

#[ffi_export]
fn big_values_new(x: &Option<repr_c::Box<CUint256>>, y: &Option<repr_c::Box<CUint256>>) -> Option<repr_c::Box<BigValues>> {

    // BigValues constructor: copy x & y (so C user still need to free them)

    if let Some(x) = x && let Some(y) = y {
        Some(Box_::new(
            BigValues {
                x: x.clone(),
                y: Box_::new(CUint256(y.0)),
            }
        ))
    } else {
        None
    }
}

/*
#[ffi_export]
fn big_values_new_1(x: Option<&repr_c::Box<CUint256>>, y: Option<&repr_c::Box<CUint256>>) -> Option<repr_c::Box<BigValues>> {

    // BigValues constructor: copy x & y (so C user still need to free them)

    if let Some(x) = x && let Some(y) = y {
        Some(Box_::new(
            BigValues {
                x: x.clone(),
                y: Box_::new(CUint256(y.0)),
            }
        ))
    } else {
        None
    }
}
*/

#[ffi_export]
fn big_values_new_2(x: &mut Option<repr_c::Box<CUint256>>, y: &mut Option<repr_c::Box<CUint256>>) -> Option<repr_c::Box<BigValues>> {

    // BigValues constructor: move x & y (so C user DO NOT need to free them)

    let res = if let Some(x) = x && let Some(y) = y {
        let res = Some(Box_::new(BigValues {
            x: Box_::new(std::mem::take(x)),
            y: Box_::new(std::mem::take(y)),
        }));
        res
    } else { None };

    *x = None;
    *y = None;
    res
}

#[ffi_export]
fn big_values_free(v: Option<repr_c::Box<BigValues>>) {
    drop(v)
}

// The following function is only necessary for the header generation.
#[cfg(feature = "headers")] // c.f. the `Cargo.toml` section
pub fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("big_values.h")?
        .generate()
}