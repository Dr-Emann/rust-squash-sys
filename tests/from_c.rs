extern crate libc;
extern crate squash_sys;
extern crate rand;
#[macro_use]
extern crate lazy_static;

mod stream;

use std::os::raw::c_void;
use std::{mem, ptr};
use std::borrow::Cow;
use std::ffi::CStr;
use std::cell::Cell;
use std::io::{self, Write};
use squash_sys::*;

#[test]
fn found_codecs() {
    set_up();
    assert!(ALL_CODECS.len() > 0);
    println!("found {} codecs", ALL_CODECS.len());
    
    assert!(!ERROR_OCCURED.with(|e| e.get()), "memory error while loading codecs");
}

pub const LOREM_IPSUM: &'static [u8] = include_bytes!("data/lorem.txt");
pub const SQUASH_PTR_TEST_INT: u64 = 0xBADC0FFEE0DDF00D;

thread_local!{
    pub static ERROR_OCCURED: Cell<bool> = Cell::new(false)
}

#[inline]
pub unsafe fn get_codec_name(codec: *mut SquashCodec) -> Cow<'static, str> {
    assert!(!codec.is_null());
    let result = squash_codec_get_name(codec);
    assert!(!result.is_null());
    let result = CStr::from_ptr(result);
    result.to_string_lossy()
}

pub fn set_up() {
    use std::sync::Once;

    static START: Once = Once::new();

    START.call_once(|| {
        unsafe {
            squash_set_memory_functions(SquashMemoryFuncs {
                malloc: Some(squash_test_malloc),
                realloc: Some(squash_test_realloc),
                calloc: Some(squash_test_calloc),
                free: Some(squash_test_free),
                
                aligned_alloc: None,
                aligned_free: None,
            });
        }
    });
    ERROR_OCCURED.with(|e| e.set(false));
}

lazy_static!{
    pub static ref ALL_CODECS: Vec<&'static SquashCodec> = {
        let mut vec = Vec::new();
        
        extern fn push_to_vec(codec: *mut SquashCodec, data: *mut c_void) {
            unsafe {
                let plugin = squash_codec_get_plugin(codec);
                if squash_plugin_init(plugin) != SquashStatus::SQUASH_OK {
                    return;
                }
                let data = data as *mut Vec<&'static SquashCodec>;
                (*data).push(&*codec);
            }
        }
        
        unsafe {
            squash_foreach_codec(Some(push_to_vec), &mut vec as *mut Vec<&'static SquashCodec> as *mut c_void);
        }
        vec
    };
}

// Can't panic, because we can't unwind into the c code that will be calling these functions

extern fn squash_test_malloc(size: usize) -> *mut c_void {
    unsafe {
        let ptr = libc::malloc(size + mem::size_of::<u64>()) as *mut u64;
        *ptr = SQUASH_PTR_TEST_INT;
        return ptr.offset(1) as *mut c_void
    }
}

extern fn squash_test_calloc(nmemb: usize, size: usize) -> *mut c_void {
    unsafe {
        let ptr = libc::calloc(1, (nmemb * size) + mem::size_of::<u64>()) as *mut u64;
        *ptr = SQUASH_PTR_TEST_INT;
        return ptr.offset(1) as *mut c_void;
    }
}

extern fn squash_test_realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    if ptr.is_null() {
        return squash_test_malloc(size);
    }
    unsafe {
        let real_ptr = (ptr as *mut u64).offset(-1);
        if *real_ptr != SQUASH_PTR_TEST_INT {
            let _ = writeln!(io::stderr(), "realloc: (*real_ptr != SQUASH_PTR_TEST_INT) ({:#x} != {:#x})", *real_ptr, SQUASH_PTR_TEST_INT);
            ERROR_OCCURED.with(|e| e.set(true));
            return ptr::null_mut();
        }
        let real_ptr = libc::realloc(real_ptr as *mut libc::c_void, size + mem::size_of::<u64>()) as *mut u64;
        return real_ptr.offset(1) as *mut c_void;
    }
}

extern fn squash_test_free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let real_ptr = (ptr as *mut u64).offset(-1);
        if *real_ptr != SQUASH_PTR_TEST_INT {
            let _ = writeln!(io::stderr(), "free: (*real_ptr != SQUASH_PTR_TEST_INT) ({:#x} != {:#x})", *real_ptr, SQUASH_PTR_TEST_INT);
            ERROR_OCCURED.with(|e| e.set(true));
            return;
        }
        libc::free(real_ptr as *mut libc::c_void);
    }
}
