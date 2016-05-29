use libc::{c_int, c_char};

bitflags! {
    #[repr(C)]
    pub flags SquashLicense: c_int {
        const SQUASH_LICENSE_UNKNOWN       = 0,

        const SQUASH_LICENSE_PERMISSIVE      = 0x01 << 24,
        const SQUASH_LICENSE_STRONG_COPYLEFT = 0x02 << 24,
        const SQUASH_LICENSE_WEAK_COPYLEFT   = 0x04 << 24,
        const SQUASH_LICENSE_PROPRIETARY     = 0x08 << 24,
        const SQUASH_LICENSE_TYPE_MASK       = 0xff << 24,

        const SQUASH_LICENSE_COPYLEFT_INCOMPATIBLE = 0x01 << 16,
        const SQUASH_LICENSE_OR_GREATER            = 0x02 << 16,
        const SQUASH_LICENSE_FLAGS_MASK            = 0xff << 16,

        const SQUASH_LICENSE_PUBLIC_DOMAIN   = SQUASH_LICENSE_PERMISSIVE.bits | 0x01,
        const SQUASH_LICENSE_BSD2            = SQUASH_LICENSE_PERMISSIVE.bits | 0x02,
        const SQUASH_LICENSE_BSD3            = SQUASH_LICENSE_PERMISSIVE.bits | 0x03,
        const SQUASH_LICENSE_BSD4            = SQUASH_LICENSE_PERMISSIVE.bits | 0x04 | SQUASH_LICENSE_COPYLEFT_INCOMPATIBLE.bits,
        const SQUASH_LICENSE_MIT             = SQUASH_LICENSE_PERMISSIVE.bits | 0x05,
        const SQUASH_LICENSE_ZLIB            = SQUASH_LICENSE_PERMISSIVE.bits | 0x06,
        const SQUASH_LICENSE_WTFPL           = SQUASH_LICENSE_PERMISSIVE.bits | 0x07,
        const SQUASH_LICENSE_X11             = SQUASH_LICENSE_PERMISSIVE.bits | 0x08,
        const SQUASH_LICENSE_APACHE          = SQUASH_LICENSE_PERMISSIVE.bits | 0x09,
        const SQUASH_LICENSE_APACHE2         = SQUASH_LICENSE_PERMISSIVE.bits | 0x0a,
        const SQUASH_LICENSE_CDDL            = SQUASH_LICENSE_PERMISSIVE.bits | 0x0b | SQUASH_LICENSE_COPYLEFT_INCOMPATIBLE.bits,
        const SQUASH_LICENSE_MSPL            = SQUASH_LICENSE_PERMISSIVE.bits | 0x0c | SQUASH_LICENSE_COPYLEFT_INCOMPATIBLE.bits,
        const SQUASH_LICENSE_ISC             = SQUASH_LICENSE_PERMISSIVE.bits | 0x0d,

        const SQUASH_LICENSE_MPL             = SQUASH_LICENSE_WEAK_COPYLEFT.bits | 0x01,
        const SQUASH_LICENSE_LGPL2P1         = SQUASH_LICENSE_WEAK_COPYLEFT.bits | 0x02,
        const SQUASH_LICENSE_LGPL2P1_PLUS    = SQUASH_LICENSE_LGPL2P1.bits | SQUASH_LICENSE_OR_GREATER.bits,
        const SQUASH_LICENSE_LGPL3           = SQUASH_LICENSE_WEAK_COPYLEFT.bits | 0x03,
        const SQUASH_LICENSE_LGPL3_PLUS      = SQUASH_LICENSE_LGPL3.bits | SQUASH_LICENSE_OR_GREATER.bits,

        const SQUASH_LICENSE_GPL1            = SQUASH_LICENSE_STRONG_COPYLEFT.bits | 0x01,
        const SQUASH_LICENSE_GPL1_PLUS       = SQUASH_LICENSE_GPL1.bits | SQUASH_LICENSE_OR_GREATER.bits,
        const SQUASH_LICENSE_GPL2            = SQUASH_LICENSE_STRONG_COPYLEFT.bits | 0x02,
        const SQUASH_LICENSE_GPL2_PLUS       = SQUASH_LICENSE_GPL2.bits | SQUASH_LICENSE_OR_GREATER.bits,
        const SQUASH_LICENSE_GPL3            = SQUASH_LICENSE_STRONG_COPYLEFT.bits | 0x03,
        const SQUASH_LICENSE_GPL3_PLUS       = SQUASH_LICENSE_GPL3.bits | SQUASH_LICENSE_OR_GREATER.bits,
    }
}

extern {
    pub fn squash_license_from_string(license: *const c_char) -> SquashLicense;
    pub fn squash_license_to_string(license: SquashLicense) -> *const c_char;
}
