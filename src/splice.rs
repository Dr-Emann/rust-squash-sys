use libc::{size_t, c_char, FILE};
use status::SquashStatus;
use codec::SquashCodec;
use stream::SquashStreamType;
use option::SquashOptions;

extern {
    pub fn squash_splice(
        codec: *const c_char,
        stream_type: SquashStreamType,
        fp_out: *mut FILE,
        fp_in: *mut FILE,
        length: size_t,
        ...) -> SquashStatus;
    pub fn squash_splice_codec(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType,
        fp_out: *mut FILE,
        fp_in: *mut FILE,
        length: size_t,
        ...) -> SquashStatus;
    pub fn squash_splice_with_options(
        codec: *const c_char,
        stream_type: SquashStreamType,
        fp_out: *mut FILE,
        fp_in: *mut FILE,
        length: size_t,
        options: *mut SquashOptions) -> SquashStatus;
    pub fn squash_splice_codec_with_options(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType,
        fp_out: *mut FILE,
        fp_in: *mut FILE,
        length: size_t,
        options: *mut SquashOptions) -> SquashStatus;
}
