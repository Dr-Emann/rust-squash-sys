use std::{cmp, ptr};
use std::os::raw::c_void;
use rand::{Rng, thread_rng};
use squash_sys::*;

use {set_up, get_codec_name, ALL_CODECS, ERROR_OCCURED, LOREM_IPSUM};

#[test]
fn compress() {
    ERROR_OCCURED.with(|error_occurred| {
        set_up();
        assert!(!error_occurred.get());
        for &codec in ALL_CODECS.iter() {
            let codec = codec as *const SquashCodec as *mut SquashCodec;
            unsafe {
                let codec_name = get_codec_name(codec);
                
                let mut uncompressed = LOREM_IPSUM.to_vec();
                let mut compressed = vec![0u8; squash_codec_get_max_compressed_size(codec, uncompressed.len())];
                
                let mut compressed_len = compressed.len();
                let res = buffer_to_buffer_compress_with_stream(codec, &mut compressed_len, compressed.as_mut_ptr(), uncompressed.len(), uncompressed.as_ptr());
                assert!(res == SQUASH_OK, "{} failed to compress", codec_name);
                
                let mut uncompressed_len = uncompressed.len();
                let res = squash_codec_decompress(
                    codec, 
                    &mut uncompressed_len, uncompressed.as_mut_ptr(), 
                    compressed_len, compressed.as_ptr(),
                    ptr::null::<c_void>()
                );
                assert!(res == SQUASH_OK, "{} failed to decompress", codec_name);
                
                assert!(uncompressed_len == LOREM_IPSUM.len(),
                    "{} decompressed to the wrong size ({} should be {})", codec_name, uncompressed_len, LOREM_IPSUM.len());
                assert!(&uncompressed[..uncompressed_len] == LOREM_IPSUM, "{} decompressed to the wrong data", codec_name);
                
                assert!(!error_occurred.get(), "Memory error for Codec: {}", codec_name);
            }
        }
    });
}

#[test]
fn decompress() {
    ERROR_OCCURED.with(|error_occurred| {
        set_up();
        assert!(!error_occurred.get());
        for &codec in ALL_CODECS.iter() {
            let codec = codec as *const SquashCodec as *mut SquashCodec;
            unsafe {
                let codec_name = get_codec_name(codec);
                // FIXME: Unknown bug with lzham on travis only
                if codec_name == "lzham" {
                    continue;
                }
                
                let mut compressed_len = squash_codec_get_max_compressed_size(codec, LOREM_IPSUM.len());
                let mut compressed = vec![0u8; compressed_len];
                
                let res = squash_codec_compress(codec, &mut compressed_len, compressed.as_mut_ptr(), LOREM_IPSUM.len(), LOREM_IPSUM.as_ptr(), ptr::null::<c_void>());
                assert!(res == SQUASH_OK, "{} failed to compress", codec_name);
                
                let mut uncompressed_len =
                    if (squash_codec_get_info(codec) & SQUASH_CODEC_INFO_KNOWS_UNCOMPRESSED_SIZE) == SQUASH_CODEC_INFO_KNOWS_UNCOMPRESSED_SIZE {
                        squash_codec_get_uncompressed_size(codec, compressed_len, compressed.as_ptr())
                    } else {
                        LOREM_IPSUM.len()
                    };
                assert!(uncompressed_len == LOREM_IPSUM.len(),
                        "{} reported the wrong uncompressed size ({} should be {})", codec_name, uncompressed_len, LOREM_IPSUM.len());
                let mut uncompressed = vec![0u8; uncompressed_len];
                
                let res = buffer_to_buffer_decompress_with_stream(codec, &mut uncompressed_len, uncompressed.as_mut_ptr(), compressed_len, compressed.as_ptr());
                assert!(res == SQUASH_OK, "{} failed to decompress", codec_name);
                assert!(uncompressed_len == LOREM_IPSUM.len(),
                    "{} decompressed to the wrong size ({} should be {})", codec_name, uncompressed_len, LOREM_IPSUM.len());
                assert!(&uncompressed[..uncompressed_len] == LOREM_IPSUM, "{} decompressed to the wrong data", codec_name);
                
                assert!(!error_occurred.get(), "Memory error for Codec: {}", codec_name);
            }
        }
    });
}

#[test]
fn single_byte_input() {
    ERROR_OCCURED.with(|error_occurred| {
        set_up();
        assert!(!error_occurred.get());
        for &codec in ALL_CODECS.iter() {
            let codec = codec as *const SquashCodec as *mut SquashCodec;
            unsafe {
                let codec_name = get_codec_name(codec);
                
                let mut compressed = [0u8; 8192];
                let mut uncompressed = [0u8; 8192];
                let mut uncompressed_len = uncompressed.len();
                let mut res: SquashStatus;
                
                let stream = &mut *squash_codec_create_stream(codec, SQUASH_STREAM_COMPRESS, ptr::null::<c_void>());
                stream.next_out = compressed.as_mut_ptr();
                stream.avail_out = compressed.len();
                stream.next_in = LOREM_IPSUM.as_ptr();
                while stream.total_in < LOREM_IPSUM.len() {
                    stream.avail_in = 1;
                    res = SQUASH_PROCESSING;
                    while res == SQUASH_PROCESSING {
                        assert!(stream.avail_out != 0, "{} ran out of room compressing", codec_name);
                        res = squash_stream_process(stream);
                    }
                    
                    assert!(res == SQUASH_OK, "{} failed to compress", codec_name);
                }
                
                res = SQUASH_PROCESSING;
                while res == SQUASH_PROCESSING {
                    res = squash_stream_finish(stream);
                }
                assert!(res == SQUASH_OK, "{} failed to compress", codec_name);
                
                squash_object_unref(stream as *mut SquashStream as *mut c_void);
                
                res = squash_codec_decompress(codec, &mut uncompressed_len, uncompressed.as_mut_ptr(), stream.total_out, compressed.as_ptr(), ptr::null::<c_void>());
                assert!(res == SQUASH_OK, "{} failed to decompress", codec_name);
                
                assert!(uncompressed_len == LOREM_IPSUM.len(),
                    "{} decompressed to the wrong size ({} should be {})", codec_name, uncompressed_len, LOREM_IPSUM.len());
                assert!(&uncompressed[..uncompressed_len] == LOREM_IPSUM, "{} decompressed to the wrong data", codec_name);
                
                assert!(!error_occurred.get(), "Memory error for Codec: {}", codec_name);
                
                let stream = &mut *squash_codec_create_stream(codec, SQUASH_STREAM_COMPRESS, ptr::null::<c_void>());
                stream.next_out = compressed.as_mut_ptr();
                stream.avail_in = LOREM_IPSUM.len();
                stream.next_in = LOREM_IPSUM.as_ptr();
                while stream.total_in < LOREM_IPSUM.len() {
                    res = SQUASH_PROCESSING;
                    while res == SQUASH_PROCESSING {
                        assert!(stream.total_out < compressed.len());
                        stream.avail_out = 1;
                        res = squash_stream_process(stream);
                    }
                    assert!(res == SQUASH_OK, "{} failed to compress", codec_name);
                }
                
                res = SQUASH_PROCESSING;
                while res == SQUASH_PROCESSING {
                    assert!(stream.total_out < compressed.len());
                    stream.avail_out = 1;
                    res = squash_stream_finish(stream);
                }
                assert!(res == SQUASH_OK, "{} failed to compress", codec_name);
                uncompressed_len = LOREM_IPSUM.len();
                res = squash_codec_decompress(codec, &mut uncompressed_len, uncompressed.as_mut_ptr(), stream.total_out, compressed.as_ptr(), ptr::null::<c_void>());
                assert!(res == SQUASH_OK, "{} failed to decompress", codec_name);
                
                squash_object_unref(stream as *mut SquashStream as *mut c_void);
                
                assert!(uncompressed_len == LOREM_IPSUM.len(),
                    "{} decompressed to the wrong size ({} should be {})", codec_name, uncompressed_len, LOREM_IPSUM.len());
                assert!(&uncompressed[..uncompressed_len] == LOREM_IPSUM, "{} decompressed to the wrong data", codec_name);
                
                assert!(!error_occurred.get(), "Memory error for Codec: {}", codec_name);
            }
        }
    });
}

fn buffer_to_buffer_compress_with_stream<'a> (
        codec: *mut SquashCodec,
        compressed_len: *mut usize,
        compressed: *mut u8,
        uncompressed_len: usize,
        uncompressed: *const u8) -> SquashStatus {
    let mut rng = thread_rng();
    let step_size = rng.gen_range(64, 255);
    unsafe {
        let stream = &mut *squash_codec_create_stream(codec, SQUASH_STREAM_COMPRESS, ptr::null::<c_void>());
        stream.next_out = compressed;
        stream.avail_out = cmp::min(step_size, *compressed_len);
        stream.next_in = uncompressed;
        let mut res = SQUASH_PROCESSING;
        while stream.total_in < uncompressed_len {
            stream.avail_in = cmp::min(uncompressed_len - stream.total_in, step_size);
            res = SQUASH_PROCESSING;
            while res == SQUASH_PROCESSING {
                res = squash_stream_process(stream);
                
                if stream.avail_out < step_size {
                    stream.avail_out = cmp::min(*compressed_len - stream.total_out, step_size);
                }
            }
        }
        assert_eq!(res, SQUASH_OK);
        
        res = SQUASH_PROCESSING;
        while res == SQUASH_PROCESSING {
            stream.avail_out = cmp::min(*compressed_len - stream.total_out, step_size);
            
            res = squash_stream_finish(stream);
        }
        
        if res == SQUASH_OK {
            *compressed_len = stream.total_out;
        }
        
        squash_object_unref(stream as *mut SquashStream as *mut c_void);
        
        res
    }
}

fn buffer_to_buffer_decompress_with_stream (
        codec: *mut SquashCodec,
        decompressed_len: *mut usize,
        decompressed: *mut u8,
        compressed_len: usize,
        compressed: *const u8) -> SquashStatus {
    let mut rng = thread_rng();
    let step_size = rng.gen_range(64, 255);
    unsafe {
        let stream = &mut *squash_codec_create_stream(codec, SQUASH_STREAM_DECOMPRESS, ptr::null::<c_void>());
        stream.next_out = decompressed;
        stream.avail_out = cmp::min(step_size, *decompressed_len);
        stream.next_in = compressed;
        
        let mut res = SQUASH_OK;
        while stream.total_in < compressed_len && stream.avail_out < *decompressed_len {
            stream.avail_in = cmp::min(compressed_len - stream.total_in, step_size);
            stream.avail_out = cmp::min(*decompressed_len - stream.total_out, step_size);
            
            res = squash_stream_process(stream);
            if res == SQUASH_END_OF_STREAM || res < 0 {
                break;
            }
        }
        
        if res == SQUASH_END_OF_STREAM {
            res = SQUASH_OK;
        } else if res > 0 {
            res = SQUASH_PROCESSING;
            while res == SQUASH_PROCESSING {
                stream.avail_in = cmp::min(compressed_len - stream.total_in, step_size);
                stream.avail_out = cmp::min(*decompressed_len - stream.total_out, step_size);
                res = squash_stream_finish(stream);
            }
        }
        
        if res == SQUASH_OK {
            *decompressed_len = stream.total_out;
        }
        
        squash_object_unref(stream as *mut SquashStream as *mut c_void);
        
        res
    }
}
