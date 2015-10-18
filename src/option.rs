use libc::{c_char, c_int, size_t, c_void};

use object::{SquashObject, SquashDestroyNotify};
use codec::SquashCodec;
use status::SquashStatus;

pub type SquashOptionType = c_int;

pub const SQUASH_OPTION_TYPE_NONE: SquashOptionType        = 0;
pub const SQUASH_OPTION_TYPE_BOOL: SquashOptionType        = 1;
pub const SQUASH_OPTION_TYPE_STRING: SquashOptionType      = 2;
pub const SQUASH_OPTION_TYPE_INT: SquashOptionType         = 3;
pub const SQUASH_OPTION_TYPE_SIZE: SquashOptionType        = 4;

pub const SQUASH_OPTION_TYPE_ENUM_STRING: SquashOptionType = (0x10 | SQUASH_OPTION_TYPE_STRING);
pub const SQUASH_OPTION_TYPE_ENUM_INT: SquashOptionType    = (0x10 | SQUASH_OPTION_TYPE_INT);

pub const SQUASH_OPTION_TYPE_RANGE_INT: SquashOptionType   = (0x20 | SQUASH_OPTION_TYPE_INT);
pub const SQUASH_OPTION_TYPE_RANGE_SIZE: SquashOptionType  = (0x20 | SQUASH_OPTION_TYPE_SIZE);

#[repr(C)]
pub struct SquashOptions {
    pub base_object: SquashObject,
    pub codec: *mut SquashCodec,
    pub values: *mut SquashOptionValue,
}

#[repr(C)]
pub struct SquashOptionInfoEnumStringMap {
    pub name: *const c_char,
    pub value: c_int,
}

#[repr(C)]
pub struct SquashOptionInfoEnumString {
    pub values: *const SquashOptionInfoEnumStringMap,
}

#[repr(C)]
pub struct SquashOptionInfoEnumInt {
    pub values_length: size_t,
    pub values: *const c_int,
}

#[repr(C)]
pub struct SquashOptionInfoRangeInt {
    pub min: c_int,
    pub max: c_int,
    pub modulus: c_int,
    pub allow_zero: bool,
}

#[repr(C)]
pub struct SquashOptionInfoRangeSize {
    pub min: size_t,
    pub max: size_t,
    pub modulus: size_t,
    pub allow_zero: bool,
}

#[repr(C)]
pub struct SquashOptionValue {
    _union_data_: [size_t; 1],
}

impl SquashOptionValue {
    pub unsafe fn string_value(&self) -> *mut *const c_char {
        let raw: *mut u8 = ::std::mem::transmute(&self._union_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn int_value(&self) -> *mut c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._union_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn bool_value(&self) -> *mut bool {
        let raw: *mut u8 = ::std::mem::transmute(&self._union_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn size_value(&self) -> *mut size_t {
        let raw: *mut u8 = ::std::mem::transmute(&self._union_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}

#[repr(C)]
pub struct SquashOptionInfo {
    pub name: *const c_char,
    pub _type: SquashOptionType,
    pub info: SquashOptionInfoUnion,
    pub default_value: SquashOptionValue,
}

#[repr(C)]
pub struct SquashOptionInfoUnion {
    _union_data_: [size_t; 4],
}

impl SquashOptionInfoUnion {
    pub unsafe fn enum_string(&self) -> *const SquashOptionInfoEnumString {
        let raw: *mut u8 = ::std::mem::transmute(&self._union_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn enum_int(&self) -> *const SquashOptionInfoEnumInt {
        let raw: *mut u8 = ::std::mem::transmute(&self._union_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn range_int(&self) -> *const SquashOptionInfoRangeInt {
        let raw: *mut u8 = ::std::mem::transmute(&self._union_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn range_size(&self) -> *const SquashOptionInfoRangeSize {
        let raw: *mut u8 = ::std::mem::transmute(&self._union_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
extern {
    pub fn squash_options_new(codec: *mut SquashCodec, ...) -> *mut SquashOptions;
    pub fn squash_options_newa(
        codec: *mut SquashCodec,
        keys: *const *const c_char,
        values: *const *const c_char) -> *mut SquashOptions;
    pub fn squash_options_get_string(
        options: *mut SquashOptions,
        key: *const c_char) -> *const c_char;
    pub fn squash_options_get_bool(
        options: *mut SquashOptions,
        key: *const c_char) -> u8;
    pub fn squash_options_get_int(
        options: *mut SquashOptions,
        key: *const c_char) -> c_int;
    pub fn squash_options_get_size(
        options: *mut SquashOptions,
        key: *const c_char) -> size_t;
    pub fn squash_options_parse(options: *mut SquashOptions, ...) -> SquashStatus;
    pub fn squash_options_parsea(
        options: *mut SquashOptions,
        keys: *const *const c_char,
        values: *const *const c_char) -> SquashStatus;
    pub fn squash_options_parse_option(
        options: *mut SquashOptions,
        key: *const c_char,
        value: *const c_char) -> SquashStatus;
    pub fn squash_options_init(
        options: *mut c_void,
        codec: *mut SquashCodec,
        destroy_notify: SquashDestroyNotify);
    pub fn squash_options_destroy(options: *mut c_void);
}
