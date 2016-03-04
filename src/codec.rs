use libc::{c_void, c_int, c_uint, c_char, size_t, uint8_t};
use context::SquashContext;
use status::SquashStatus;
use plugin::SquashPlugin;
use option::{SquashOptions,SquashOptionInfo};
use stream::{SquashStreamType, SquashStream, SquashOperation};

pub enum SquashCodec { }

pub type SquashCodecForeachFunc = Option<extern fn(*mut SquashCodec, *mut c_void)>;
pub type SquashReadFunc = Option<extern fn (*mut size_t, *mut uint8_t, *mut c_void) -> SquashStatus>;
pub type SquashWriteFunc = Option<extern fn (*mut size_t, *const uint8_t, *mut c_void) -> SquashStatus>;

bitflags! {
    #[repr(C)]
    flags SquashCodecInfo: c_int {
        const SQUASH_CODEC_INFO_INVALID                 = 0,
        const SQUASH_CODEC_INFO_CAN_FLUSH               = 1 << 0,
        const SQUASH_CODEC_INFO_DECOMPRESS_UNSAFE       = 1 << 1,
        const SQUASH_CODEC_INFO_WRAP_SIZE               = 1 << 2,

        const SQUASH_CODEC_INFO_AUTO_MASK               = 0x00ff0000,
        const SQUASH_CODEC_INFO_VALID                   = 1 << 16,
        const SQUASH_CODEC_INFO_KNOWS_UNCOMPRESSED_SIZE = 1 << 17,
        const SQUASH_CODEC_INFO_NATIVE_STREAMING        = 1 << 18,

        #[allow(overflowing_literals)]
        const SQUASH_CODEC_INFO_MASK                    = 0xffffffff
    }
}

#[repr(C)]
pub struct SquashCodecImpl {
    pub info: SquashCodecInfo,
    pub options: *const SquashOptionInfo,
    pub create_stream: Option<extern "C" fn(
        codec: *mut SquashCodec,
        stream_type:SquashStreamType,
        options: *mut SquashOptions) -> *mut SquashStream>,
    pub process_stream: Option<extern "C" fn(
        stream: *mut SquashStream,
        operation: SquashOperation) -> SquashStatus>,
    pub splice: Option<extern "C" fn(
        codec: *mut SquashCodec,
        options: *mut SquashOptions,
        stream_type: SquashStreamType,
        read_cb: SquashReadFunc,
        write_cb: SquashWriteFunc,
        user_data: *mut c_void) -> SquashStatus>,
    pub decompress_buffer: Option<extern "C" fn(
        codec: *mut SquashCodec,
        decompressed_size: *mut size_t,
        decompressed: *mut uint8_t,
        compressed_size: size_t,
        compressed: *const uint8_t,
        options: *mut SquashOptions) -> SquashStatus>,
    pub compress_buffer: Option<extern "C" fn(
        codec: *mut SquashCodec,
        compressed_size: *mut size_t,
        compressed: *mut uint8_t,
        uncompressed_size: size_t,
        uncompressed: *const uint8_t,
        options: *mut SquashOptions) -> SquashStatus>,
    pub compress_buffer_unsafe: Option<extern "C" fn(
        codec: *mut SquashCodec,
        compressed_size: *mut size_t,
        compressed: *mut uint8_t,
        uncompressed_size: size_t,
        uncompressed: *const uint8_t,
        options: *mut SquashOptions) -> SquashStatus>,
    pub get_uncompressed_size: Option<extern "C" fn(
        codec: *mut SquashCodec,
        compressed_size: size_t,
        compressed: *const uint8_t) -> size_t>,
    pub get_max_compressed_size: Option<extern "C" fn(
        codec: *mut SquashCodec,
        uncompressed_size: size_t) -> size_t>,
    pub _reserved1: Option<extern "C" fn() -> ()>,
    pub _reserved2: Option<extern "C" fn() -> ()>,
    pub _reserved3: Option<extern "C" fn() -> ()>,
    pub _reserved4: Option<extern "C" fn() -> ()>,
    pub _reserved5: Option<extern "C" fn() -> ()>,
    pub _reserved6: Option<extern "C" fn() -> ()>,
    pub _reserved7: Option<extern "C" fn() -> ()>,
    pub _reserved8: Option<extern "C" fn() -> ()>,
}

extern {
    pub fn squash_codec_init(codec: *mut SquashCodec) -> SquashStatus;
    pub fn squash_codec_get_name(codec: *mut SquashCodec) -> *const c_char;
    pub fn squash_codec_get_priority(codec: *mut SquashCodec) -> c_uint;
    pub fn squash_codec_get_plugin(codec: *mut SquashCodec) -> *mut SquashPlugin;
    pub fn squash_codec_get_context(codec: *mut SquashCodec) -> *mut SquashContext;
    pub fn squash_codec_get_extension(codec: *mut SquashCodec) -> *const c_char;
    pub fn squash_codec_get_uncompressed_size(
        codec: *mut SquashCodec,
        compressed_size: size_t,
        compressed: *const u8) -> size_t;
    pub fn squash_codec_get_max_compressed_size(
        codec: *mut SquashCodec,
        uncompressed_size: size_t) -> size_t;
    pub fn squash_codec_create_stream(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType, ...) -> *mut SquashStream;
    pub fn squash_codec_create_stream_with_options(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType,
        options: *mut SquashOptions) -> *mut SquashStream;
    pub fn squash_codec_compress(
        codec: *mut SquashCodec,
        compressed_size: *mut size_t,
        compressed: *mut u8,
        uncompressed_size: size_t,
        uncompressed: *const u8, ...) -> SquashStatus;
    pub fn squash_codec_compress_with_options(
        codec: *mut SquashCodec,
        compressed_size: *mut size_t,
        compressed: *mut u8,
        uncompressed_size: size_t,
        uncompressed: *const u8,
        options: *mut SquashOptions) -> SquashStatus;
    pub fn squash_codec_decompress(
        codec: *mut SquashCodec,
        decompressed_size: *mut size_t,
        decompressed: *mut u8,
        compressed_size: size_t,
        compressed: *const u8, ...) -> SquashStatus;
    pub fn squash_codec_decompress_with_options(
        codec: *mut SquashCodec,
        decompressed_size: *mut size_t,
        decompressed: *mut u8,
        compressed_size: size_t,
        compressed: *const u8,
        options: *mut SquashOptions) -> SquashStatus;
    pub fn squash_codec_get_info(codec: *mut SquashCodec) -> SquashCodecInfo;
    pub fn squash_codec_get_option_info(codec: *mut SquashCodec) -> *const SquashOptionInfo;
}
