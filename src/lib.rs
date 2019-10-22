#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

pub use libc::{FILE, wchar_t};

use libc::c_uint;

include!("bindings.rs");

/// Current version, encoded as a single number
pub const SQUASH_VERSION_CURRENT: c_uint =
    SQUASH_VERSION(SQUASH_VERSION_MAJOR as u8,
                   SQUASH_VERSION_MINOR as u8,
                   SQUASH_VERSION_REVISION as u8);

/// Encode the major, minor, and revisions into a single number
///
/// # Parameters
/// * `major` Major version number
/// * `minor` Minor version number
/// * `revision` Revision number
///
/// # Returns
/// Encoded version
pub const fn SQUASH_VERSION(major: u8, minor: u8, revision: u8) -> c_uint {
    ((major as c_uint) << 16) | ((minor as c_uint) << 8) | (revision as c_uint)
}

/// Extract the major version number from an encoded version
pub const fn SQUASH_VERSION_EXTRACT_MAJOR(version: c_uint) -> u8 {
    ((version >> 16) & 0xff) as u8
}

/// Extract the minor version number from an encoded version
pub const fn SQUASH_VERSION_EXTRACT_MINOR(version: c_uint) -> u8 {
    ((version >> 8) & 0xff) as u8
}

/// Extract the revsion number from an encoded version
pub const fn SQUASH_VERSION_EXTRACT_REVISION(version: c_uint) -> u8 {
    (version & 0xff) as u8
}
