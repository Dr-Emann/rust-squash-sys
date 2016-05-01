use libc::{c_char, c_uint};

/// Major version number
pub const SQUASH_VERSION_MAJOR: c_uint    = 0;
/// Minor version number
pub const SQUASH_VERSION_MINOR: c_uint    = 8;
/// Revision version number
pub const SQUASH_VERSION_REVISION: c_uint = 0;

/// API version
pub const SQUASH_VERSION_API: &'static [u8] = b"0.8";

pub use self::inner::{
    SQUASH_VERSION_CURRENT,
    SQUASH_VERSION,
    SQUASH_VERSION_EXTRACT_MAJOR,
    SQUASH_VERSION_EXTRACT_MINOR,
    SQUASH_VERSION_EXTRACT_REVISION,
};

#[cfg(not(feature="nightly"))]
mod inner {
    use libc::c_uint;

    /// Current version, encoded as a single number
    pub const SQUASH_VERSION_CURRENT: c_uint = 0x000800; // hard coded until constfn is stable

    /// Encode the major, minor, and revisions into a single number
    ///
    /// # Parameters
    /// * `major` Major version number
    /// * `minor` Minor version number
    /// * `revision` Revision number
    ///
    /// # Returns
    /// Encoded version
    #[inline]
    #[allow(non_snake_case)]
    pub fn SQUASH_VERSION(major: c_uint, minor: c_uint, revision: c_uint) -> c_uint {
        ((major & 0xFF) << 16) | ((minor & 0xFF) << 8) | (revision & 0xFF)
    }

    /// Extract the major version number from an encoded version
    #[inline]
    #[allow(non_snake_case)]
    pub fn SQUASH_VERSION_EXTRACT_MAJOR(version: c_uint) -> c_uint {
        (version >> 16) & 0xff
    }

    /// Extract the minor version number from an encoded version
    #[inline]
    #[allow(non_snake_case)]
    pub fn SQUASH_VERSION_EXTRACT_MINOR(version: c_uint) -> c_uint {
        (version >> 8) & 0xff
    }

    /// Extract the revision number from an encoded version
    #[inline]
    #[allow(non_snake_case)]
    pub fn SQUASH_VERSION_EXTRACT_REVISION(version: c_uint) -> c_uint {
        version & 0xff
    }
}

#[cfg(feature="nightly")]
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
    pub const fn SQUASH_VERSION(major: c_uint, minor: c_uint, revision: c_uint) -> c_uint {
        ((major & 0xFF) << 16) | ((minor & 0xFF) << 8) | (revision & 0xFF)
    }

    /// Extract the major version number from an encoded version
    #[allow(non_snake_case)]
    pub const fn SQUASH_VERSION_EXTRACT_MAJOR(version: c_uint) -> c_uint {
        (version >> 16) & 0xff
    }

    /// Extract the minor version number from an encoded version
    #[allow(non_snake_case)]
    pub const fn SQUASH_VERSION_EXTRACT_MINOR(version: c_uint) -> c_uint {
        (version >> 8) & 0xff
    }

    /// Extract the revsion number from an encoded version
    #[allow(non_snake_case)]
    pub const fn SQUASH_VERSION_EXTRACT_REVISION(version: c_uint) -> c_uint {
        version & 0xff
    }
}

extern {
    /// Get the library version.
    ///
    /// This function will return the version of the library currently in
    /// use. Note that this may be different than the version you compiled
    /// against; generally you should only use this function to make sure it
    /// is greater than or equal to [`SQUASH_VERSION_CURRENT`], or for
    /// reporting purposes.
    ///
    /// # Returns
    /// the library version
    ///
    /// [`SQUASH_VERSION_CURRENT`]: ./constant.SQUASH_VERSION_CURRENT.html
    pub fn squash_version() -> c_uint;

    /// Get the API version.
    ///
    /// Unlike the library version, the API version will only change when
    /// backwards-incompatible changes are made (i.e., it should be "1.0"
    /// for every 1.x release). Code linked against a library version not
    /// exactly equal to what it was compiled with will almost certainly fail.
    ///
    /// # Returns
    /// the API version
    pub fn squash_version_api() -> *const c_char;
}
