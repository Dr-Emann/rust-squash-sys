use libc::{c_char, c_uint};

pub const SQUASH_VERSION_MAJOR: c_uint    = 0;
pub const SQUASH_VERSION_MINOR: c_uint    = 7;
pub const SQUASH_VERSION_REVISION: c_uint = 0;

pub const SQUASH_VERSION_API: &'static [u8] = b"0.7";
#[cfg(not(feature="nightly"))]
pub const SQUASH_VERSION_CURRENT: c_uint = 0x000700; // hard coded until constfn is stable
#[cfg(feature="nightly")]
pub const SQUAHS_VERSION_CURRENT: c_uint = SQUASH_VERSION(SQUASH_VERSION_MAJOR, SQUAH_VERSION_MINOR, SQUASH_VERSION_REVISION);

#[inline]
#[allow(non_snake_case)]
#[cfg(not(feature="nightly"))]
pub fn SQUASH_VERSION(major: c_uint, minor: c_uint, revision: c_uint) -> c_uint {
    ((major & 0xFF) << 16) | ((minor & 0xFF) << 8) | (revision & 0xFF)
}

#[inline]
#[allow(non_snake_case)]
#[cfg(not(feature="nightly"))]
pub fn SQUASH_VERSION_EXTRACT_MAJOR(version: c_uint) -> c_uint {
    (version >> 16) & 0xff
}

#[inline]
#[allow(non_snake_case)]
#[cfg(not(feature="nightly"))]
pub fn SQUASH_VERSION_EXTRACT_MINOR(version: c_uint) -> c_uint {
    (version >> 8) & 0xff
}

#[inline]
#[allow(non_snake_case)]
#[cfg(not(feature="nightly"))]
pub fn SQUASH_VERSION_EXTRACT_REVISION(version: c_uint) -> c_uint {
    version & 0xff
}

#[allow(non_snake_case)]
#[cfg(feature="nightly")]
pub const fn SQUASH_VERSION(major: c_uint, minor: c_uint, revision: c_uint) -> c_uint {
    ((major & 0xFF) << 16) | ((minor & 0xFF) << 8) | (revision & 0xFF)
}

#[allow(non_snake_case)]
#[cfg(feature="nightly")]
pub const fn SQUASH_VERSION_EXTRACT_MAJOR(version: c_uint) -> c_uint {
    (version >> 16) & 0xff
}

#[allow(non_snake_case)]
#[cfg(feature="nightly")]
pub fn SQUASH_VERSION_EXTRACT_MINOR(version: c_uint) -> c_uint {
    (version >> 8) & 0xff
}

#[allow(non_snake_case)]
#[cfg(feature="nightly")]
pub fn SQUASH_VERSION_EXTRACT_REVISION(version: c_uint) -> c_uint {
    version & 0xff
}

extern {
    pub fn squash_version() -> c_uint;
    pub fn squash_version_api() -> *const c_char;
}
