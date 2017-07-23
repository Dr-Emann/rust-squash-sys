extern crate squash_sys;

#[macro_use]
mod common;

use std::{env, io, ptr, process};
use std::io::prelude::*;
use std::ffi::{CStr, CString};

use squash_sys::*;

const BUFFER_SIZE: usize = 1024 * 1024;

fn main() {
    let return_code = real_main();
    process::exit(return_code);
}

fn real_main() -> i32 {
    let mut stderr = io::stderr();
    // fuse ensures it it safe to call .next() after None is returned
    let mut args = env::args().fuse();
    
    let prog_name = args.next().unwrap();
    
    let (stream_type, codec_name) = match (args.next(), args.next()) {
        (Some(stream_type), Some(codec_name)) => {
            let stream_type = match &stream_type[..] {
                "c" => SQUASH_STREAM_COMPRESS,
                "d" => SQUASH_STREAM_DECOMPRESS,
                unknown_mode => {
                    let _ = writeln!(stderr, "Invalid mode '{}': must be 'c' or 'd'", unknown_mode);
                    return 1;
                }
            };
            (stream_type, codec_name)
        }
        _ => {
            let _ = writeln!(stderr, "USAGE: {} (c|d) CODEC", prog_name);
            let _ = writeln!(stderr, "Input is read from stdin, output is written to stdout");
            return 1
        }
    };
    
    
    let raw_codec_name = CString::new(codec_name.as_bytes()).unwrap();
    
    let codec = unsafe { squash_get_codec(raw_codec_name.as_ptr()) };
    if codec.is_null() {
        let _ = writeln!(stderr, "Unable to find algorithm '{}'.", codec_name);
        return 1;
    }
    
    let mut input = vec![0; BUFFER_SIZE];
    let mut output = vec![0; BUFFER_SIZE];
    
    let stream = unsafe { squash_stream_new(codec, stream_type, ptr::null::<u8>()) };
    
    if stream.is_null() {
        let _ = writeln!(stderr, "Failed to create stream.");
        return 1;
    }
    
    defer!(unsafe { squash_object_unref(stream as *mut std::os::raw::c_void) });
    
    let stream = unsafe { &mut *stream };
    
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut res: SquashStatus;
    loop {
        let input_len = stdin.read(&mut input).unwrap();
        if input_len == 0 { break; }
        
        stream.next_in = input.as_ptr();
        stream.avail_in = input_len;
        
        res = SQUASH_PROCESSING;
        while res == SQUASH_PROCESSING {
            stream.next_out = output.as_mut_ptr();
            stream.avail_out = BUFFER_SIZE;
            
            res = unsafe { squash_stream_process(stream) };
            if res < 0 {
                let reason = unsafe { CStr::from_ptr(squash_status_to_string(res)) };
                let _ = writeln!(stderr, "Processing failed: {} ({})", reason.to_string_lossy(), res);
                return 1;
            }
            
            let output_size = (stream.next_out as usize) - (output.as_ptr() as usize);
            stdout.write_all(&output[..output_size]).unwrap();
        }
    }
    
    res = SQUASH_PROCESSING;
    while res == SQUASH_PROCESSING {
        stream.next_out = output.as_mut_ptr();
        stream.avail_out = BUFFER_SIZE;
        
        res = unsafe { squash_stream_finish(stream) };
        
        if res < 0 {
            let reason = unsafe { CStr::from_ptr(squash_status_to_string(res)) };
            let _ = writeln!(stderr, "Finishing failed {} ({})", reason.to_string_lossy(), res);
            return 1;
        }
        
        let output_size = (stream.next_out as usize) - (output.as_ptr() as usize);
        stdout.write_all(&output[..output_size]).unwrap();
    }
    return 0;
}
