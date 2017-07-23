#![cfg_attr(feature="nightly", feature(const_fn))]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

pub use libc::FILE;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
