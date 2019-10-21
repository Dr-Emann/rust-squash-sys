#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

pub use libc::FILE;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use inner::{
    SQUASH_VERSION_CURRENT,
    SQUASH_VERSION,
    SQUASH_VERSION_EXTRACT_MAJOR,
    SQUASH_VERSION_EXTRACT_MINOR,
    SQUASH_VERSION_EXTRACT_REVISION
};

/// API version
pub const SQUASH_VERSION_API: &'static [u8] = b"0.8";

/// Major version number
pub const SQUASH_VERSION_MAJOR: u8 = 8;

/// Minor version number
pub const SQUASH_VERSION_MINOR: u8 = 0;

/// Revsion version number
pub const SQUASH_VERSION_REVISION: u8 = 0;

mod inner {
    use libc::c_uint;
    use super::{
        SQUASH_VERSION_MAJOR,
        SQUASH_VERSION_MINOR,
        SQUASH_VERSION_REVISION,
    };

    /// Current version, encoded as a single number
    pub const SQUASH_VERSION_CURRENT: c_uint =
        SQUASH_VERSION(SQUASH_VERSION_MAJOR,
                       SQUASH_VERSION_MINOR,
                       SQUASH_VERSION_REVISION);

    /// Encode the major, minor, and revisions into a single number
    ///
    /// # Parameters
    /// * `major` Major version number
    /// * `minor` Minor version number
    /// * `revision` Revision number
    ///
    /// # Returns
    /// Encoded version
    #[allow(non_snake_case)]
    pub const fn SQUASH_VERSION(major: u8, minor: u8, revision: u8) -> c_uint {
        ((major as c_uint) << 16) | ((minor as c_uint) << 8) | (revision as c_uint)
    }

    /// Extract the major version number from an encoded version
    #[allow(non_snake_case)]
    pub const fn SQUASH_VERSION_EXTRACT_MAJOR(version: c_uint) -> u8 {
        ((version >> 16) & 0xff) as u8
    }

    /// Extract the minor version number from an encoded version
    #[allow(non_snake_case)]
    pub const fn SQUASH_VERSION_EXTRACT_MINOR(version: c_uint) -> u8 {
        ((version >> 8) & 0xff) as u8
    }

    /// Extract the revsion number from an encoded version
    #[allow(non_snake_case)]
    pub const fn SQUASH_VERSION_EXTRACT_REVISION(version: c_uint) -> u8 {
        (version & 0xff) as u8
    }
}
