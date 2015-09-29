use libc::{c_char, c_void, size_t, uint8_t};

use object::{SquashObject, SquashDestroyNotify};
use codec::SquashCodec;
use option::SquashOptions;
use status::SquashStatus;

pub enum SquashStreamPrivate { }

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SquashStreamType {
    SQUASH_STREAM_COMPRESS   = 1,
    SQUASH_STREAM_DECOMPRESS = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SquashOperation {
    SQUASH_OPERATION_PROCESS   = 1,
    SQUASH_OPERATION_FLUSH     = 2,
    SQUASH_OPERATION_FINISH    = 3,
    SQUASH_OPERATION_TERMINATE = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SquashStreamState {
    SQUASH_STREAM_STATE_IDLE      = 0,
    SQUASH_STREAM_STATE_RUNNING   = 1,
    SQUASH_STREAM_STATE_FLUSHING  = 2,
    SQUASH_STREAM_STATE_FINISHING = 3,
    SQUASH_STREAM_STATE_FINISHED  = 4,
}

#[repr(C)]
pub struct SquashStream {
    pub base_object: SquashObject,
    pub _priv: *mut SquashStreamPrivate,
    pub next_in: *const uint8_t,
    pub avail_in: size_t,
    pub total_in: size_t,
    pub next_out: *mut uint8_t,
    pub avail_out: size_t,
    pub total_out: size_t,
    pub codec: *mut SquashCodec,
    pub options: *mut SquashOptions,
    pub stream_type: SquashStreamType,
    pub state: SquashStreamState,
    pub user_data: *mut c_void,
    pub destroy_user_data: SquashDestroyNotify,
}

extern {
    pub fn squash_stream_new(
        codec: *const c_char,
        stream_type: SquashStreamType, ...) -> *mut SquashStream;
    pub fn squash_stream_newa(
        codec: *const c_char,
        stream_type: SquashStreamType,
        keys: *const *const c_char,
        values: *const *const c_char) -> *mut SquashStream;
    pub fn squash_stream_new_with_options(
        codec: *const c_char,
        stream_type: SquashStreamType,
        options: *mut SquashOptions) -> *mut SquashStream;
    pub fn squash_stream_new_codec(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType, ...) -> *mut SquashStream;
    pub fn squash_stream_new_codec_with_options(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType,
        options: *mut SquashOptions) -> *mut SquashStream;
    pub fn squash_stream_process(stream: *mut SquashStream) -> SquashStatus;
    pub fn squash_stream_flush(stream: *mut SquashStream) -> SquashStatus;
    pub fn squash_stream_finish(stream: *mut SquashStream) -> SquashStatus;
    pub fn squash_stream_init(
        stream: *mut c_void,
        codec: *mut SquashCodec,
        stream_type: SquashStreamType,
        options: *mut SquashOptions,
        destroy_notify: SquashDestroyNotify) -> ();
    pub fn squash_stream_destroy(stream: *mut c_void) -> ();
    pub fn squash_stream_yield(
        stream: *mut SquashStream,
        status: SquashStatus) -> SquashOperation;
}
