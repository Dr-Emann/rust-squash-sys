extern crate squash_sys;

use std::{env, io, process, ptr};
use std::io::prelude::*;
use std::ffi::{CStr, CString};

use squash_sys::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stderr = io::stderr();
    if args.len() != 3 {
        let _ = writeln!(stderr, "USAGE: {} ALGORITHM STRING", args[0]);
        process::exit(1);
    }
    let raw_codec_name = CString::new(args[1].as_bytes()).unwrap();
    let codec = unsafe { squash_get_codec(raw_codec_name.as_ptr()) };
    if codec.is_null() {
        let _ = writeln!(stderr, "Unable to find algorithm '{}'.", args[1]);
        process::exit(1);
    }
    
    let uncompressed = args[2].as_bytes();
    let mut compressed_len = unsafe { squash_codec_get_max_compressed_size(codec, uncompressed.len()) };
    let mut compressed = vec![0u8; compressed_len];
    
    let mut decompressed_len = uncompressed.len();
    let mut decompressed = vec![0u8; decompressed_len];
    
    let res = unsafe { 
        squash_codec_compress(codec,
                              &mut compressed_len, compressed.as_mut_ptr(),
                              uncompressed.len(), uncompressed.as_ptr(),
                              ptr::null::<u8>())
    };
    
    if res != SQUASH_OK {
        let reason = unsafe { CStr::from_ptr(squash_status_to_string(res)) };
        let _ = writeln!(stderr, "Unable to compress data [{}]: {}", res, reason.to_string_lossy());
        process::exit(1);
    }
    
    println!("Compressed a {} byte buffer to {} bytes.", uncompressed.len(), compressed_len);
    
    let res = unsafe {
        squash_codec_decompress(codec,
                                &mut decompressed_len, decompressed.as_mut_ptr(),
                                compressed_len, compressed.as_ptr(),
                                ptr::null::<u8>())
    };
    
    
    if res != SQUASH_OK {
        let reason = unsafe { CStr::from_ptr(squash_status_to_string(res)) };
        let _ = writeln!(stderr, "Unable to decompress data [{}]: {}", res, reason.to_string_lossy());
        process::exit(1);
    }
    
    if &decompressed[..decompressed_len] != uncompressed {
        let _ = writeln!(stderr, "Bad decompressed data.");
        process::exit(1);
    }
    
    println!("Successfully decompressed.");
}