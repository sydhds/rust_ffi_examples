use libc::{c_int, size_t};

/*
 * snappy_validate_compressed_buffer C definition is supposed to return an enum: snappy_status
 * however it is not recommended to use Field less Rust enum for external C functions
 * as described here:
 * https://doc.rust-lang.org/reference/type-layout.html?highlight=repr(C)#reprc-field-less-enums
 * https://users.rust-lang.org/t/ffi-with-structured-enums-in-rust/111027
 */

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq, Eq, Debug)]
#[non_exhaustive]
enum SnappyStatus {
    SNAPPY_OK = 0,
    SNAPPY_INVALID_INPUT = 1,
    SNAPPY_BUFFER_TOO_SMALL = 2,
    #[non_exhaustive]
    OTHER,
}

impl From<c_int> for SnappyStatus {
    fn from(x: c_int) -> Self {
        match x {
            0 => SnappyStatus::SNAPPY_OK,
            1 => SnappyStatus::SNAPPY_INVALID_INPUT,
            2 => SnappyStatus::SNAPPY_BUFFER_TOO_SMALL,
            _ => SnappyStatus::OTHER,
        }
    }
}

// Tell the linker to link to the snappy library
#[link(name = "snappy")]
unsafe extern "C" {

    // This function is defined in the snappy library
    // See: https://github.com/google/snappy/blob/main/snappy-c.h
    // C definition is:
    // size_t snappy_max_compressed_length(size_t source_length);
    fn snappy_max_compressed_length(source_length: size_t) -> size_t;

    // Other functions defined in the snappy library:

    fn snappy_compress(
        input: *const u8,
        input_length: size_t,
        compressed: *mut u8,
        compressed_length: *mut size_t,
    ) -> c_int;
    fn snappy_uncompress(
        compressed: *const u8,
        compressed_length: size_t,
        uncompressed: *mut u8,
        uncompressed_length: *mut size_t,
    ) -> c_int;
    fn snappy_uncompressed_length(
        compressed: *const u8,
        compressed_length: size_t,
        result: *mut size_t,
    ) -> c_int;

    // C definition:
    // snappy_status snappy_validate_compressed_buffer(const char* compressed, size_t compressed_length);
    // Note: return type here is c_int (see comments at the top of the file)
    fn snappy_validate_compressed_buffer(compressed: *const u8, compressed_length: size_t)
    -> c_int;
}

// Safe API (wrap unsafe calls)
pub fn validate_compressed_buffer(src: &[u8]) -> bool {
    unsafe {
        let res = snappy_validate_compressed_buffer(src.as_ptr(), src.len() as size_t);
        SnappyStatus::from(res) == SnappyStatus::SNAPPY_OK
    }
}

pub fn compress(src: &[u8]) -> Vec<u8> {
    unsafe {
        let srclen = src.len() as size_t;
        let psrc = src.as_ptr();

        let mut dstlen = snappy_max_compressed_length(srclen);
        let mut dst = Vec::with_capacity(dstlen as usize);
        let pdst = dst.as_mut_ptr();

        snappy_compress(psrc, srclen, pdst, &mut dstlen);
        dst.set_len(dstlen as usize);
        dst
    }
}

pub fn uncompress(src: &[u8]) -> Option<Vec<u8>> {
    unsafe {
        let srclen = src.len() as size_t;
        let psrc = src.as_ptr();

        let mut dstlen: size_t = 0;
        snappy_uncompressed_length(psrc, srclen, &mut dstlen);

        let mut dst = Vec::with_capacity(dstlen as usize);
        let pdst = dst.as_mut_ptr();

        if snappy_uncompress(psrc, srclen, pdst, &mut dstlen) == 0 {
            dst.set_len(dstlen as usize);
            Some(dst)
        } else {
            None // SNAPPY_INVALID_INPUT
        }
    }
}

fn main() {
    let x = unsafe { snappy_max_compressed_length(100) };
    println!("max compressed length of a 100 byte buffer: {x}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let d = vec![0xde, 0xad, 0xd0, 0x0d];
        let c: &[u8] = &compress(&d);
        assert!(validate_compressed_buffer(c));
        assert!(uncompress(c) == Some(d));
    }

    #[test]
    fn invalid() {
        let d = vec![0, 0, 0, 0];
        assert!(!validate_compressed_buffer(&d));
        assert!(uncompress(&d).is_none());
    }

    #[test]
    fn empty() {
        let d = vec![];
        assert!(!validate_compressed_buffer(&d));
        assert!(uncompress(&d).is_none());
        let c = compress(&d);
        assert!(validate_compressed_buffer(&c));
        assert!(uncompress(&c) == Some(d));
    }
}
