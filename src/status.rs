use libc::{c_char};

pub use self::SquashStatus::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SquashStatus {
    SQUASH_OK                    =  1,
    SQUASH_PROCESSING            =  2,
    SQUASH_END_OF_STREAM         =  3,
    
    SQUASH_FAILED                = -1,
    SQUASH_UNABLE_TO_LOAD        = -2,
    SQUASH_BAD_PARAM             = -3,
    SQUASH_BAD_VALUE             = -4,
    SQUASH_MEMORY                = -5,
    SQUASH_BUFFER_FULL           = -6,
    SQUASH_BUFFER_EMPTY          = -7,
    SQUASH_STATE                 = -8,
    SQUASH_INVALID_OPERATION     = -9,
    SQUASH_NOT_FOUND             = -10,
    SQUASH_INVALID_BUFFER        = -11,
    SQUASH_IO                    = -12,
    SQUASH_RANGE                 = -13,
}

extern {
    pub fn squash_status_to_string(status: SquashStatus) -> *const c_char;
    pub fn squash_error(status: SquashStatus) -> SquashStatus;
}
