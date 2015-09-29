extern crate libc;

#[macro_use]
extern crate bitflags;

pub mod object;
pub mod codec;
pub mod status;
pub mod plugin;
pub mod license;
pub mod option;
pub mod stream;
pub mod version;
pub mod file;

pub use object::*;
pub use codec::*;
pub use status::*;
pub use plugin::*;
pub use license::*;
pub use option::*;
pub use stream::*;
pub use version::*;
pub use file::*;

#[cfg(test)]
mod test {
    use super::*;
    use libc::{c_char, size_t};
    use std::ptr;
    #[test]
    fn test_xz_round_trip() {
        let codec = b"xz\0";
        let uncompressed = include_bytes!("/bin/bash");
        let mut compressed_size = unsafe { squash_get_max_compressed_size(codec.as_ptr() as *const c_char, uncompressed.len() as size_t) };
        let mut compressed = Vec::with_capacity(compressed_size as usize);
        let mut decompressed_size = uncompressed.len() as size_t;
        let mut decompressed = Vec::with_capacity(decompressed_size as usize);

        assert_eq!(SQUASH_OK, unsafe {
            squash_compress(codec.as_ptr() as *const c_char,
                            &mut compressed_size,
                            compressed.as_mut_ptr(),
                            uncompressed.len() as size_t,
                            uncompressed.as_ptr(),
                            ptr::null_mut::<()>())
        });

        unsafe { compressed.set_len(compressed_size as usize); }
        assert_eq!(SQUASH_OK, unsafe {
            squash_decompress(codec.as_ptr() as *const c_char,
                              &mut decompressed_size,
                              decompressed.as_mut_ptr(),
                              compressed.len() as size_t,
                              compressed.as_ptr(),
                              ptr::null_mut::<()>())
        });

        unsafe { decompressed.set_len(decompressed_size as usize); }
        assert_eq!(&uncompressed[..], &decompressed[..]);
    }
}
