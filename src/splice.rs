use libc::{size_t, c_void, FILE};
use status::SquashStatus;
use codec::{SquashCodec, SquashReadFunc, SquashWriteFunc};
use stream::SquashStreamType;
use option::SquashOptions;

pub const SQUASH_SPLICE_BUF_SIZE: usize = 512;

extern {
    /// Compress or decompress the contents of one file to another
    ///
    /// This function will attempt to compress or decompress the contents of
    /// one file to another. It will attempt to use memory-mapped files in
    /// order to reduce memory usage and increase performance, and so should
    /// be preferred over writing similar code manually.
    ///
    /// # Parameters
    /// * `fp_in` the input FILE pointer
    /// * `fp_out` the output FILE pointer
    /// * `size` number of bytes (uncompressed) to transfer from fp_in to fp_out, or 0 to transfer the entire file
    /// * `stream_type` whether to compress or decompress the data
    /// * `codec` the name of the codec to use
    /// * `...` list of options (with a NULL sentinel)
    ///
    /// # Returns
    /// `SQUASH_OK` on success, or a negative error code on failure
    pub fn squash_splice(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType,
        fp_out: *mut FILE,
        fp_in: *mut FILE,
        size: size_t,
        ...) -> SquashStatus;
    
    /// compress or decompress the contents of one file to another
    ///
    /// This function will attempt to compress or decompress the contents of
    /// one file to another. It will attempt to use memory-mapped files in
    /// order to reduce memory usage and increase performance, and so should
    /// be preferred over writing similar code manually.
    /// # Parameters
    /// * `fp_in` the input FILE pointer
    /// * `fp_out` the output FILE pointer
    /// * `size` number of bytes (uncompressed) to transfer from fp_in to fp_out
    /// * `stream_type` whether to compress or decompress the data
    /// * `codec` codec to use
    /// * `options` options to pass to the codec
    ///
    /// # Returns
    /// `SQUASH_OK` on success, or a negative error code on failure
    pub fn squash_splice_with_options(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType,
        fp_out: *mut FILE,
        fp_in: *mut FILE,
        size: size_t,
        options: *mut SquashOptions) -> SquashStatus;
    pub fn squash_splice_custom(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType,
        write_cb: SquashWriteFunc,
        read_cb: SquashReadFunc,
        user_data: *mut c_void,
        size: size_t,
        ...) -> SquashStatus;
    pub fn squash_splice_custom_with_options(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType,
        write_cb: SquashWriteFunc,
        read_cb: SquashReadFunc,
        user_data: *mut c_void,
        size: size_t,
        options: *mut SquashOptions) -> SquashStatus;
}
