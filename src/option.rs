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

/// A set of compression/decompression options.
#[repr(C)]
pub struct SquashOptions {
    /// Base object.
    pub base_object: SquashObject,
    /// Codec.
    pub codec: *mut SquashCodec,
    /// NULL-terminated array of option values.
    pub values: *mut SquashOptionValue,
}

/// An item in a map of strings to integer values.
#[repr(C)]
pub struct SquashOptionInfoEnumStringMap {
    /// a string representing the option value 
    pub name: *const c_char,
    /// an integer representing the option value
    pub value: c_int,
}

/// A list of strings which are mapped to integer values.
#[repr(C)]
pub struct SquashOptionInfoEnumString {
    /// a NULL terminated list of string and integer pairs
    pub values: *const SquashOptionInfoEnumStringMap,
}

/// A list of potential integer values.
#[repr(C)]
pub struct SquashOptionInfoEnumInt {
    /// number of integer values understood for this option
    pub values_length: size_t,
    /// array of integer values understood for this option
    pub values: *const c_int,
}

/// A range of potential integer values.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SquashOptionInfoRangeInt {
    /// minimum value for this option 
    pub min: c_int,
    /// maximum value for this option
    pub max: c_int,
    /// modulus of acceptable values, or 0 to accept all
    pub modulus: c_int,
    /// whether to allow zero as a value
    ///
    /// *Note that this is in addition to the range, and independent of the
    /// modulus.*
    pub allow_zero: bool,
}

/// A range of potential size values.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SquashOptionInfoRangeSize {
    /// minimum value for this option
    pub min: size_t,
    /// maximum value for this option
    pub max: size_t,
    /// modulus of acceptable values, or 0 to accept all
    pub modulus: size_t,
    /// whether to allow zero as a value
    ///
    /// *Note that this is in addition to the range, and independent of the
    /// modulus.*
    pub allow_zero: bool,
}

/// A value.
#[repr(C)]
pub struct SquashOptionValue {
    _union_data_: [size_t; 1],
}

impl SquashOptionValue {
    /// the value as a string
    pub unsafe fn string_value(&self) -> *const *const c_char {
        let raw: *mut u8 = ::std::mem::transmute(&self._union_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    
    /// the value as an integer
    pub unsafe fn int_value(&self) -> *const c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._union_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    
    /// the value as a boolean
    pub unsafe fn bool_value(&self) -> *const bool {
        let raw: *mut u8 = ::std::mem::transmute(&self._union_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    
    /// the value as a size
    pub unsafe fn size_value(&self) -> *const size_t {
        let raw: *mut u8 = ::std::mem::transmute(&self._union_data_);
        ::std::mem::transmute(raw.offset(0))
    }

    /// the value as a string
    pub unsafe fn string_value_mut(&mut self) -> *mut *const c_char {
        let raw: *mut u8 = ::std::mem::transmute(&self._union_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    
    /// the value as an integer
    pub unsafe fn int_value_mut(&mut self) -> *mut c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._union_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    
    /// the value as a boolean
    pub unsafe fn bool_value_mut(&mut self) -> *mut bool {
        let raw: *mut u8 = ::std::mem::transmute(&self._union_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    
    /// the value as a size
    pub unsafe fn size_value_mut(&mut self) -> *mut size_t {
        let raw: *mut u8 = ::std::mem::transmute(&self._union_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}

/// Information about options which can be passed to a codec.
#[repr(C)]
pub struct SquashOptionInfo {
    /// name of the option
    pub name: *const c_char,
    /// type of the option
    pub _type: SquashOptionType,
    /// detailed information about the value
    pub info: SquashOptionInfoUnion,
    /// value to use if none is provided by the user
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
    /// Create a new group of options.
    ///
    /// # Parameters
    /// * `codec` The codec to create the options for.
    /// * `...` A variadic list of string key/value pairs followed by NULL
    ///
    /// # Returns
    /// A new option group, or NULL on failure.
    pub fn squash_options_new(codec: *mut SquashCodec, ...) -> *mut SquashOptions;
    
    /// Create a new group of options from key and value arrays.
    ///
    /// # Parameters
    /// * `codec` The codec to create the options for.
    /// * `keys` A NULL-terminated array of keys.
    /// * `values` A NULL-terminated array of values.
    ///
    /// # Returns
    /// A new option group, or NULL on failure.
    pub fn squash_options_newa(
        codec: *mut SquashCodec,
        keys: *const *const c_char,
        values: *const *const c_char) -> *mut SquashOptions;
    
    /// Retrieve the value of a string option
    ///
    /// # Note
    /// *If the option is not natively a string (e.g., if it is an integer, size,
    /// or boolean), it will not be serialized to one.*
    ///
    /// # Parameters
    /// * `options` the options to retrieve the value from
    /// * `codec` the codec to use
    /// * `key` name of the option to retrieve the value from
    ///
    /// # Returns
    /// the value, or NULL on failure
    pub fn squash_options_get_string(
        options: *mut SquashOptions,
        codec: *mut SquashCodec,
        key: *const c_char) -> *const c_char;
    
    /// Retrieve the value of a boolean option
    ///
    /// # Parameters
    /// * `options` the options to retrieve the value from
    /// * `key` name of the option to retrieve the value from
    ///
    /// # Returns
    /// the value, or false if the option named `key` is not a boolean value,
    /// or if there is no option named `key`
    pub fn squash_options_get_bool(
        options: *mut SquashOptions,
        codec: *mut SquashCodec,
        key: *const c_char) -> bool;
    
    /// Retrieve the value of an integer option
    ///
    /// # Parameters
    /// * `options` the options to retrieve the value from
    /// * `key` name of the option to retrieve the value from
    ///
    /// # Returns
    /// the value, or 0 if the option named `key` is not a integer value,
    /// or if there is no option named `key`
    pub fn squash_options_get_int(
        options: *mut SquashOptions,
        codec: *mut SquashCodec,
        key: *const c_char) -> c_int;

    /// Retrieve the value of a size option
    ///
    /// # Parameters
    /// * `options` the options to retrieve the value from
    /// * `key` name of the option to retrieve the value from
    ///
    /// # Returns
    /// the value
    pub fn squash_options_get_size(
        options: *mut SquashOptions,
        codec: *mut SquashCodec,
        key: *const c_char) -> size_t;

    /// Retrieve the value of a string option
    ///
    /// # Note
    /// *If the option is not natively a string (e.g., if it is an integer, size,
    /// or boolean), it will not be serialized to one.*
    ///
    /// *It is undefined behavior to specify an index greater than the number of
    /// options.*
    ///
    /// # Parameters
    /// * `options` the options to retrieve the value from
    /// * `index` the index of the desired option
    ///
    /// # Returns
    /// the value, or NULL on failure
    pub fn squash_options_get_string_at(
        options: *mut SquashOptions,
        codec: *mut SquashCodec,
        index: usize) -> *const c_char;
    
    /// Retrieve the value of a boolean option
    ///
    /// # Note
    /// *It is undefined behavior to specify an index greater than the number of
    /// options.*
    ///
    /// # Parameters
    /// * `options` the options to retrieve the value from
    /// * `index` the index of the desired option
    ///
    /// # Returns
    /// the value
    pub fn squash_options_get_bool_at(
        options: *mut SquashOptions,
        codec: *mut SquashCodec,
        index: usize) -> bool;

    /// Retrieve the value of an int option
    ///
    /// # Note
    /// *It is undefined behavior to specify an index greater than the number of
    /// options.*
    ///
    /// # Parameters
    /// * `options` the options to retrieve the value from
    /// * `index` the index of the desired option
    ///
    /// # Returns
    /// the value
    pub fn squash_options_get_int_at(
        options: *mut SquashOptions,
        codec: *mut SquashCodec,
        index: usize) -> c_int;

    /// Retrieve the value of a size option
    ///
    /// # Note
    /// *It is undefined behavior to specify an index greater than the number
    /// of options.*
    ///
    /// # Parameters
    /// * `options` the options to retrieve the value from
    /// * `index` the index of the desired option
    ///
    /// # Returns
    /// the value
    pub fn squash_options_get_size_at(
        options: *mut SquashOptions,
        codec: *mut SquashCodec,
        index: usize) -> usize;
    
    /// Set the value of a string option.
    ///
    /// # Parameters
    /// * `options` the options on which to set the value
    /// * `key` name of the option to set
    /// * `value`   new value to be set
    ///
    /// # Returns
    /// A status code.
    ///
    /// # Return values
    /// * `SQUASH_OK`   Option set successfully.
    /// * `SQUASH_BAD_PARAM`    Invalid key
    /// * `SQUASH_BAD_VALUE`    Invalid value
    pub fn squash_options_set_string(
        options: *mut SquashOptions,
        key: *const c_char,
        value: *const c_char) -> SquashStatus;

    /// Set the value of a bool option.
    ///
    /// # Parameters
    /// * `options` the options on which to set the value
    /// * `key` name of the option to set
    /// * `value` new value to be set
    ///
    /// # Returns
    /// A status code.
    ///
    /// # Return values
    /// * `SQUASH_OK` Option set successfully.
    /// * `SQUASH_BAD_PARAM` Invalid key
    pub fn squash_options_set_bool(
        options: *mut SquashOptions,
        key: *const c_char,
        value: bool) -> SquashStatus;
    
    /// Set the value of a int option.
    ///
    /// # Parameters
    /// * `options`	the options on which to set the value
    /// * `key` name of the option to set
    /// * `value` new value to be set
    ///
    /// # Returns
    /// A status code.
    ///
    /// # Return values
    /// * `SQUASH_OK` Option set successfully.
    /// * `SQUASH_BAD_PARAM` Invalid key
    /// * `SQUASH_BAD_VALUE` Invalid value
    pub fn squash_options_set_int(
        options: *mut SquashOptions,
        key: *const c_char,
        value: c_int) -> SquashStatus;
    
    /// Set the value of a size option.
    ///
    /// # Parameters
    /// * `options` the options on which to set the value
    /// * `key` name of the option to set
    /// * `value`   new value to be set
    /// # Returns
    /// A status code.
    ///
    /// # Return values
    /// * `SQUASH_OK`   Option set successfully.
    /// * `SQUASH_BAD_PARAM`    Invalid key
    /// * `SQUASH_BAD_VALUE`    Invalid value
    pub fn squash_options_set_size(
        options: *mut SquashOptions,
        key: *const c_char,
        value: usize) -> SquashStatus;

    /// Set the value of a string option at the given index.
    ///
    /// # Parameters
    /// * `options` the options on which to set the value
    /// * `index`   the index of the option to change
    /// * `value`   new value to be set
    ///
    /// # Returns
    /// A status code.
    ///
    /// # Return values
    /// * `SQUASH_OK`   Option set successfully.
    /// * `SQUASH_BAD_PARAM`    Invalid key
    /// * `SQUASH_BAD_VALUE`    Invalid value
    pub fn squash_options_set_string_at(
        options: *mut SquashOptions,
        index: usize,
        value: *const c_char) -> SquashStatus;

    /// Set the value of a bool option at the given `index`.
    ///
    /// # Parameters
    /// * `options` the options on which to set the value
    /// * `index` the index of the option to change
    /// * `value` new value to be set
    ///
    /// # Returns
    /// A status code.
    ///
    /// # Return values
    /// * `SQUASH_OK` Option set successfully.
    /// * `SQUASH_BAD_PARAM` Invalid key
    /// * `SQUASH_BAD_VALUE` Invalid value
    pub fn squash_options_set_bool_at(
        options: *mut SquashOptions,
        index: usize,
        value: bool) -> SquashStatus;

    /// Set the value of a int option at the given index.
    ///
    /// # Parameters
    /// * `options` the options on which to set the value
    /// * `index` the index of the option to change
    /// * `value` new value to be set
    ///
    /// # Returns
    /// A status code.
    ///
    /// # Return values
    /// * `SQUASH_OK` Option set successfully.
    /// * `SQUASH_BAD_PARAM` Invalid key
    /// * `SQUASH_BAD_VALUE` Invalid value
    pub fn squash_options_set_int_at(
        options: *mut SquashOptions,
        index: usize,
        value: c_int) -> SquashStatus;
    
    /// Set the value of a size option at the given index.
    ///
    /// # Parameters
    /// * `options` the options on which to set the value
    /// * `index`   the index of the option to change
    /// * `value`   new value to be set
    ///
    /// # Returns
    /// A status code.
    ///
    /// # Return values
    /// * `SQUASH_OK`   Option set successfully.
    /// * `SQUASH_BAD_PARAM`    Invalid key
    /// * `SQUASH_BAD_VALUE`    Invalid value
    pub fn squash_options_set_size_at(
        options: *mut SquashOptions,
        index: usize,
        value: usize) -> SquashStatus;

    /// Parse a variadic list of options.
    ///
    /// # Parameters
    /// * `options` The options context.
    /// * `...` The options to parse. These should be alternating key and value
    /// pairs of strings, one for each option, followed by NULL.
    ///
    /// # Returns
    /// A status code.
    pub fn squash_options_parse(options: *mut SquashOptions, ...) -> SquashStatus;

    /// Parse an array of options.
    ///
    /// # Parameters
    /// * `options` The options context.
    /// * `keys` The option keys to parse.
    /// * `values` The option values to parse.
    ///
    /// # Returns
    /// A status code.
    pub fn squash_options_parsea(
        options: *mut SquashOptions,
        keys: *const *const c_char,
        values: *const *const c_char) -> SquashStatus;
    
    /// Parse a single option.
    ///
    /// # Parameters
    /// * `options` The options context.
    /// * `key` The option key to parse.
    /// * `value` The option value to parse.
    ///
    /// # Returns
    /// A status code.
    ///
    /// # Return values
    /// * `SQUASH_OK` Option parsed successfully.
    /// * `SQUASH_BAD_PARAM` Invalid key
    /// * `SQUASH_BAD_VALUE` Invalid value
    /// * `SQUASH_RANGE` Value was well-formed, but outside of the allowable
    /// range
    pub fn squash_options_parse_option(
        options: *mut SquashOptions,
        key: *const c_char,
        value: *const c_char) -> SquashStatus;

    /// Initialize a new `SquashOptions` instance.
    ///
    /// This function should only be used for subclassing. See
    /// [`squash_object_init`] for more information.
    ///
    /// # Parameters
    /// * `options` The instance to initialize.
    /// * `codec` The codec to use.
    /// * `destroy_notify` The function to be called when the reference count
    /// reaches 0
    ///
    /// [`squash_object_init`]: ./fn.squash_object_init.html
    pub fn squash_options_init(
        options: *mut c_void,
        codec: *mut SquashCodec,
        destroy_notify: SquashDestroyNotify);
    
    /// Destroy a SquashOptions instance.
    ///
    /// This function should only be used for subclassing. See
    /// [`squash_object_destroy`] for more information.
    ///
    /// # Parameters
    /// * `options` The instance to destroy.
    ///
    /// [`squash_object_destroy`]: ./fn.squash_object_destroy.html
    pub fn squash_options_destroy(options: *mut c_void);
}

#[cfg(feature = "wide-char-api")]
use libc::wchar_t;
#[cfg(feature = "wide-char-api")]
extern {
    pub fn squash_options_neww(codec: *mut SquashCodec, ...) -> *mut SquashOptions;
    pub fn squash_options_newaw(
        codec: *mut SquashCodec,
        keys: *const *const wchar_t,
        values: *const *const wchar_t) -> *mut SquashOptions;
    pub fn squash_options_get_stringw(
        options: *mut SquashOptions,
        codec: *mut SquashCodec,
        key: *const wchar_t) -> *const c_char;
    pub fn squash_options_get_boolw(
        options: *mut SquashOptions,
        codec: *mut SquashCodec,
        key: *const wchar_t) -> u8;
    pub fn squash_options_get_intw(
        options: *mut SquashOptions,
        codec: *mut SquashCodec,
        key: *const wchar_t) -> c_int;
    pub fn squash_options_get_sizew(
        options: *mut SquashOptions,
        codec: *mut SquashCodec,
        key: *const wchar_t) -> size_t;
    pub fn squash_options_parsew(options: *mut SquashOptions, ...) -> SquashStatus;
    pub fn squash_options_parseaw(
        options: *mut SquashOptions,
        keys: *const *const wchar_t,
        values: *const *const wchar_t) -> SquashStatus;
    pub fn squash_options_parse_optionw(
        options: *mut SquashOptions,
        key: *const wchar_t,
        value: *const wchar_t) -> SquashStatus;
}
