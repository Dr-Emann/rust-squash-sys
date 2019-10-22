use rand::{thread_rng, Rng};
use squash_sys::*;
use std::os::raw::c_void;
use std::{cmp, ptr};

use super::{get_codec_name, set_up, ALL_CODECS, ERROR_OCCURED, LOREM_IPSUM};
use std::ffi::CString;

macro_rules! test_codec {
    ($codec_ident:ident) => {
        test_codec! {$codec_ident, stringify!($codec_ident)}
    };
    ($codec_ident:ident, $codec_name:expr) => {
        mod $codec_ident {
            #[test]
            fn compress() {
                super::compress(super::find($codec_name));
            }

            #[test]
            fn decompress() {
                super::decompress(super::find($codec_name));
            }

            #[test]
            fn single_byte_input() {
                super::single_byte_input(super::find($codec_name));
            }
        }
    };
}

test_codec! { brieflz }
test_codec! { brotli }
test_codec! { bsc }
test_codec! { bzip2 }
test_codec! { compress }
test_codec! { copy }
test_codec! { crush }
test_codec! { deflate }
test_codec! { fari }
test_codec! { fastlz }
test_codec! { gipfeli }
test_codec! { gzip }
test_codec! { heatshrink }
test_codec! { lz4 }
test_codec! { lz4_raw, "lz4-raw" }
test_codec! { lzf }
test_codec! { lzfse }
test_codec! { lzg }
test_codec! { lzham }
test_codec! { lzjb }
test_codec! { lzma }
test_codec! { lzma1 }
test_codec! { lzma2 }
test_codec! { lznt1 }
test_codec! { lzo1b }
test_codec! { lzo1c }
test_codec! { lzo1f }
test_codec! { lzo1x }
test_codec! { lzo1y }
test_codec! { lzo1z }
test_codec! { lzvn }
test_codec! { quicklz }
test_codec! { snappy }
test_codec! { wflz }
test_codec! { wflz_chunked, "wflz-chunked" }
test_codec! { xpress }
test_codec! { xpress_huffman, "xpress-huffman" }
test_codec! { xz }
test_codec! { yalz77 }
test_codec! { zlib }
test_codec! { zling }
test_codec! { zpaq }
test_codec! { zstd }

fn find(codec_name: &str) -> *mut SquashCodec {
    set_up();
    let codec_name_c = CString::new(codec_name).unwrap();
    ERROR_OCCURED.with(|error_occurred| {
        let found_codec: *mut SquashCodec = ALL_CODECS
            .iter()
            .cloned()
            .find(|codec| get_codec_name(codec) == codec_name)
            .unwrap() as *const SquashCodec
            as *mut SquashCodec;
        assert!(!error_occurred.get());
        let other_codec = unsafe { squash_get_codec(codec_name_c.as_ptr()) };
        assert!(!error_occurred.get(), "squash_get_codec");

        assert_eq!(found_codec, other_codec);
        found_codec
    })
}

fn compress(codec: *mut SquashCodec) {
    set_up();
    ERROR_OCCURED.with(|error_occurred| unsafe {
        let mut uncompressed = LOREM_IPSUM.to_vec();
        let mut compressed =
            vec![0u8; squash_codec_get_max_compressed_size(codec, uncompressed.len())];

        let mut compressed_len = compressed.len();
        let res = buffer_to_buffer_compress_with_stream(
            codec,
            &mut compressed_len,
            compressed.as_mut_ptr(),
            uncompressed.len(),
            uncompressed.as_ptr(),
        );
        assert_eq!(res, SquashStatus::SQUASH_OK, "failed to compress",);

        let mut uncompressed_len = uncompressed.len();
        let res = squash_codec_decompress(
            codec,
            &mut uncompressed_len,
            uncompressed.as_mut_ptr(),
            compressed_len,
            compressed.as_ptr(),
            ptr::null::<c_void>(),
        );
        assert_eq!(res, SquashStatus::SQUASH_OK, "failed to decompress",);

        assert_eq!(
            uncompressed_len,
            LOREM_IPSUM.len(),
            "decompressed to the wrong size ({} should be {})",
            uncompressed_len,
            LOREM_IPSUM.len()
        );
        assert_eq!(
            &uncompressed[..uncompressed_len],
            LOREM_IPSUM,
            "decompressed to the wrong data",
        );

        assert!(!error_occurred.get(), "Memory error",);
    });
}

fn decompress(codec: *mut SquashCodec) {
    set_up();
    ERROR_OCCURED.with(|error_occurred| {
        let codec = codec as *const SquashCodec as *mut SquashCodec;
        unsafe {
            let mut compressed_len = squash_codec_get_max_compressed_size(codec, LOREM_IPSUM.len());
            let mut compressed = vec![0u8; compressed_len];

            let res = squash_codec_compress(
                codec,
                &mut compressed_len,
                compressed.as_mut_ptr(),
                LOREM_IPSUM.len(),
                LOREM_IPSUM.as_ptr(),
                ptr::null::<c_void>(),
            );
            assert_eq!(res, SquashStatus::SQUASH_OK, "failed to compress",);

            let mut uncompressed_len = if (squash_codec_get_info(codec)
                & SquashCodecInfo::SQUASH_CODEC_INFO_KNOWS_UNCOMPRESSED_SIZE)
                == SquashCodecInfo::SQUASH_CODEC_INFO_KNOWS_UNCOMPRESSED_SIZE
            {
                squash_codec_get_uncompressed_size(codec, compressed_len, compressed.as_ptr())
            } else {
                LOREM_IPSUM.len()
            };
            assert_eq!(
                uncompressed_len,
                LOREM_IPSUM.len(),
                "reported the wrong uncompressed size ({} should be {})",
                uncompressed_len,
                LOREM_IPSUM.len()
            );
            let mut uncompressed = vec![0u8; uncompressed_len];

            let res = buffer_to_buffer_decompress_with_stream(
                codec,
                &mut uncompressed_len,
                uncompressed.as_mut_ptr(),
                compressed_len,
                compressed.as_ptr(),
            );
            assert_eq!(res, SquashStatus::SQUASH_OK, "failed to decompress",);
            assert_eq!(
                uncompressed_len,
                LOREM_IPSUM.len(),
                "decompressed to the wrong size ({} should be {})",
                uncompressed_len,
                LOREM_IPSUM.len()
            );
            assert_eq!(
                &uncompressed[..uncompressed_len],
                LOREM_IPSUM,
                "decompressed to the wrong data",
            );

            assert!(!error_occurred.get(), "Memory error",);
        }
    });
}

fn single_byte_input(codec: *mut SquashCodec) {
    set_up();
    ERROR_OCCURED.with(|error_occurred| unsafe {
        let mut compressed = [0u8; 8192];
        let mut uncompressed = [0u8; 8192];
        let mut uncompressed_len = uncompressed.len();
        let mut res: SquashStatus::Type;

        let stream = &mut *squash_codec_create_stream(
            codec,
            SquashStreamType::SQUASH_STREAM_COMPRESS,
            ptr::null::<c_void>(),
        );
        stream.next_out = compressed.as_mut_ptr();
        stream.avail_out = compressed.len();
        stream.next_in = LOREM_IPSUM.as_ptr();
        while stream.total_in < LOREM_IPSUM.len() {
            stream.avail_in = 1;
            res = SquashStatus::SQUASH_PROCESSING;
            while res == SquashStatus::SQUASH_PROCESSING {
                assert_ne!(stream.avail_out, 0, "ran out of room compressing");
                res = squash_stream_process(stream);
            }

            assert_eq!(res, SquashStatus::SQUASH_OK, "failed to compress");
        }

        res = SquashStatus::SQUASH_PROCESSING;
        while res == SquashStatus::SQUASH_PROCESSING {
            res = squash_stream_finish(stream);
        }
        assert_eq!(res, SquashStatus::SQUASH_OK, "failed to compress");

        squash_object_unref(stream as *mut SquashStream as *mut c_void);

        res = squash_codec_decompress(
            codec,
            &mut uncompressed_len,
            uncompressed.as_mut_ptr(),
            stream.total_out,
            compressed.as_ptr(),
            ptr::null::<c_void>(),
        );
        assert_eq!(res, SquashStatus::SQUASH_OK, "failed to decompress");

        assert_eq!(
            uncompressed_len,
            LOREM_IPSUM.len(),
            "decompressed to the wrong size ({} should be {})",
            uncompressed_len,
            LOREM_IPSUM.len()
        );
        assert_eq!(
            &uncompressed[..uncompressed_len],
            LOREM_IPSUM,
            "decompressed to the wrong data",
        );

        assert!(!error_occurred.get(), "Memory error",);

        let stream = &mut *squash_codec_create_stream(
            codec,
            SquashStreamType::SQUASH_STREAM_COMPRESS,
            ptr::null::<c_void>(),
        );
        stream.next_out = compressed.as_mut_ptr();
        stream.avail_in = LOREM_IPSUM.len();
        stream.next_in = LOREM_IPSUM.as_ptr();
        while stream.total_in < LOREM_IPSUM.len() {
            res = SquashStatus::SQUASH_PROCESSING;
            while res == SquashStatus::SQUASH_PROCESSING {
                assert!(stream.total_out < compressed.len());
                stream.avail_out = 1;
                res = squash_stream_process(stream);
            }
            assert_eq!(res, SquashStatus::SQUASH_OK, "failed to compress",);
        }

        res = SquashStatus::SQUASH_PROCESSING;
        while res == SquashStatus::SQUASH_PROCESSING {
            assert!(stream.total_out < compressed.len());
            stream.avail_out = 1;
            res = squash_stream_finish(stream);
        }
        assert_eq!(res, SquashStatus::SQUASH_OK, "failed to compress",);
        uncompressed_len = LOREM_IPSUM.len();
        res = squash_codec_decompress(
            codec,
            &mut uncompressed_len,
            uncompressed.as_mut_ptr(),
            stream.total_out,
            compressed.as_ptr(),
            ptr::null::<c_void>(),
        );
        assert_eq!(res, SquashStatus::SQUASH_OK, "failed to decompress",);

        squash_object_unref(stream as *mut SquashStream as *mut c_void);

        assert_eq!(
            uncompressed_len,
            LOREM_IPSUM.len(),
            "decompressed to the wrong size ({} should be {})",
            uncompressed_len,
            LOREM_IPSUM.len()
        );
        assert_eq!(
            &uncompressed[..uncompressed_len],
            LOREM_IPSUM,
            "decompressed to the wrong data",
        );

        assert!(!error_occurred.get(), "Memory error",);
    });
}

fn buffer_to_buffer_compress_with_stream<'a>(
    codec: *mut SquashCodec,
    compressed_len: *mut usize,
    compressed: *mut u8,
    uncompressed_len: usize,
    uncompressed: *const u8,
) -> SquashStatus::Type {
    let mut rng = thread_rng();
    let step_size = rng.gen_range(64, 255);
    unsafe {
        let stream = &mut *squash_codec_create_stream(
            codec,
            SquashStreamType::SQUASH_STREAM_COMPRESS,
            ptr::null::<c_void>(),
        );
        stream.next_out = compressed;
        stream.avail_out = cmp::min(step_size, *compressed_len);
        stream.next_in = uncompressed;
        let mut res = SquashStatus::SQUASH_PROCESSING;
        while stream.total_in < uncompressed_len {
            stream.avail_in = cmp::min(uncompressed_len - stream.total_in, step_size);
            res = SquashStatus::SQUASH_PROCESSING;
            while res == SquashStatus::SQUASH_PROCESSING {
                res = squash_stream_process(stream);

                if stream.avail_out < step_size {
                    stream.avail_out = cmp::min(*compressed_len - stream.total_out, step_size);
                }
            }
        }
        assert_eq!(res, SquashStatus::SQUASH_OK);

        res = SquashStatus::SQUASH_PROCESSING;
        while res == SquashStatus::SQUASH_PROCESSING {
            stream.avail_out = cmp::min(*compressed_len - stream.total_out, step_size);

            res = squash_stream_finish(stream);
        }

        if res == SquashStatus::SQUASH_OK {
            *compressed_len = stream.total_out;
        }

        squash_object_unref(stream as *mut SquashStream as *mut c_void);

        res
    }
}

fn buffer_to_buffer_decompress_with_stream(
    codec: *mut SquashCodec,
    decompressed_len: *mut usize,
    decompressed: *mut u8,
    compressed_len: usize,
    compressed: *const u8,
) -> SquashStatus::Type {
    let mut rng = thread_rng();
    let step_size = rng.gen_range(64, 255);
    unsafe {
        let stream = &mut *squash_codec_create_stream(
            codec,
            SquashStreamType::SQUASH_STREAM_DECOMPRESS,
            ptr::null::<c_void>(),
        );
        stream.next_out = decompressed;
        stream.avail_out = cmp::min(step_size, *decompressed_len);
        stream.next_in = compressed;

        let mut res = SquashStatus::SQUASH_OK;
        while stream.total_in < compressed_len && stream.avail_out < *decompressed_len {
            stream.avail_in = cmp::min(compressed_len - stream.total_in, step_size);
            stream.avail_out = cmp::min(*decompressed_len - stream.total_out, step_size);

            res = squash_stream_process(stream);
            if res == SquashStatus::SQUASH_END_OF_STREAM || res < 0 {
                break;
            }
        }

        if res == SquashStatus::SQUASH_END_OF_STREAM {
            res = SquashStatus::SQUASH_OK;
        } else if res > 0 {
            res = SquashStatus::SQUASH_PROCESSING;
            while res == SquashStatus::SQUASH_PROCESSING {
                stream.avail_in = cmp::min(compressed_len - stream.total_in, step_size);
                stream.avail_out = cmp::min(*decompressed_len - stream.total_out, step_size);
                res = squash_stream_finish(stream);
            }
        }

        if res == SquashStatus::SQUASH_OK {
            *decompressed_len = stream.total_out;
        }

        squash_object_unref(stream as *mut SquashStream as *mut c_void);

        res
    }
}
