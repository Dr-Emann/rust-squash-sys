use libc::{c_char, c_int};

pub type SquashStatus = c_int;

pub const SQUASH_OK: SquashStatus                    =  1;
pub const SQUASH_PROCESSING: SquashStatus            =  2;
pub const SQUASH_END_OF_STREAM: SquashStatus         =  3;
pub const SQUASH_FAILED: SquashStatus                = -1;
pub const SQUASH_UNABLE_TO_LOAD: SquashStatus        = -2;
pub const SQUASH_BAD_PARAM: SquashStatus             = -3;
pub const SQUASH_BAD_VALUE: SquashStatus             = -4;
pub const SQUASH_MEMORY: SquashStatus                = -5;
pub const SQUASH_BUFFER_FULL: SquashStatus           = -6;
pub const SQUASH_BUFFER_EMPTY: SquashStatus          = -7;
pub const SQUASH_STATE: SquashStatus                 = -8;
pub const SQUASH_INVALID_OPERATION: SquashStatus     = -9;
pub const SQUASH_NOT_FOUND: SquashStatus             = -10;
pub const SQUASH_INVALID_BUFFER: SquashStatus        = -11;
pub const SQUASH_IO: SquashStatus                    = -12;
pub const SQUASH_RANGE: SquashStatus                 = -13;

extern {
    pub fn squash_status_to_string(status: SquashStatus) -> *const c_char;
    pub fn squash_error(status: SquashStatus) -> SquashStatus;
}
