extern crate squash_sys;

use std::ffi::{CStr, CString};
use std::io::prelude::*;
use std::{env, io, process, ptr};

use squash_sys::*;

fn main() {
    let return_code = real_main();
    process::exit(return_code);
}

fn real_main() -> i32 {
    let mut stderr = io::stderr();
    // fuse ensures it it safe to call .next() after None is returned
    let mut args = env::args().fuse();

    let prog_name = args.next().unwrap();

    let (codec_name, string) = match (args.next(), args.next()) {
        (Some(codec_name), Some(string)) => (codec_name, string),
        _ => {
            let _ = writeln!(stderr, "USAGE: {} ALGORITHM STRING", prog_name);
            return 1;
        }
    };

    let raw_codec_name = CString::new(codec_name.as_bytes()).unwrap();
    let codec = unsafe { squash_get_codec(raw_codec_name.as_ptr()) };
    if codec.is_null() {
        let _ = writeln!(stderr, "Unable to find algorithm '{}'.", codec_name);
        return 1;
    }

    let uncompressed = string.as_bytes();
    let mut compressed_len =
        unsafe { squash_codec_get_max_compressed_size(codec, uncompressed.len()) };
    let mut compressed = vec![0u8; compressed_len];

    let mut decompressed_len = uncompressed.len();
    let mut decompressed = vec![0u8; decompressed_len];

    let res = unsafe {
        squash_codec_compress(
            codec,
            &mut compressed_len,
            compressed.as_mut_ptr(),
            uncompressed.len(),
            uncompressed.as_ptr(),
            ptr::null::<u8>(),
        )
    };

    if res != SquashStatus::SQUASH_OK {
        let reason = unsafe { CStr::from_ptr(squash_status_to_string(res)) };
        let _ = writeln!(
            stderr,
            "Unable to compress data [{}]: {}",
            res,
            reason.to_string_lossy()
        );
        return 1;
    }

    println!(
        "Compressed a {} byte buffer to {} bytes.",
        uncompressed.len(),
        compressed_len
    );

    let res = unsafe {
        squash_codec_decompress(
            codec,
            &mut decompressed_len,
            decompressed.as_mut_ptr(),
            compressed_len,
            compressed.as_ptr(),
            ptr::null::<u8>(),
        )
    };

    if res != SquashStatus::SQUASH_OK {
        let reason = unsafe { CStr::from_ptr(squash_status_to_string(res)) };
        let _ = writeln!(
            stderr,
            "Unable to decompress data [{}]: {}",
            res,
            reason.to_string_lossy()
        );
        return 1;
    }

    if &decompressed[..decompressed_len] != uncompressed {
        let _ = writeln!(stderr, "Bad decompressed data.");
        return 1;
    }

    println!("Successfully decompressed.");
    return 0;
}
