use libc::{size_t, c_char, c_void, FILE};
use status::SquashStatus;
use codec::{SquashCodec, SquashReadFunc, SquashWriteFunc};
use stream::SquashStreamType;
use option::SquashOptions;

extern {
    pub fn squash_splice(
        codec: *const c_char,
        stream_type: SquashStreamType,
        fp_out: *mut FILE,
        fp_in: *mut FILE,
        size: size_t,
        ...) -> SquashStatus;
    pub fn squash_splice_codec(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType,
        fp_out: *mut FILE,
        fp_in: *mut FILE,
        size: size_t,
        ...) -> SquashStatus;
    pub fn squash_splice_with_options(
        codec: *const c_char,
        stream_type: SquashStreamType,
        fp_out: *mut FILE,
        fp_in: *mut FILE,
        size: size_t,
        options: *mut SquashOptions) -> SquashStatus;
    pub fn squash_splice_codec_with_options(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType,
        fp_out: *mut FILE,
        fp_in: *mut FILE,
        size: size_t,
        options: *mut SquashOptions) -> SquashStatus;
    pub fn squash_splice_custom_codec_with_options(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType,
        write_cb: SquashWriteFunc,
        read_cb: SquashReadFunc,
        user_data: *mut c_void,
        size: size_t,
        options: *mut SquashOptions) -> SquashStatus;
}
