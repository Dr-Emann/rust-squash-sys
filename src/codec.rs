use libc::{c_void, c_int, c_uint, c_char, size_t, uint8_t};
use context::SquashContext;
use status::SquashStatus;
use plugin::SquashPlugin;
use option::{SquashOptions,SquashOptionInfo};
use stream::{SquashStreamType, SquashStream, SquashOperation};

/// A compression/decompression codec.
pub enum SquashCodec { }

/// Callback to be invoked on each SquashCodec in a set.
pub type SquashCodecForeachFunc = Option<extern fn(*mut SquashCodec, *mut c_void)>;
pub type SquashReadFunc = Option<extern fn (*mut size_t, *mut uint8_t, *mut c_void) -> SquashStatus>;
pub type SquashWriteFunc = Option<extern fn (*mut size_t, *const uint8_t, *mut c_void) -> SquashStatus>;

bitflags! {
    /// Information about the codec.
    ///
    /// This is a bitmask describing characteristics and features of the codec.
    #[repr(C)]
    pub flags SquashCodecInfo: c_int {
        /// Invalid codec.
        const SQUASH_CODEC_INFO_INVALID                 = 0,
        
        /// Flushing is supported.
        const SQUASH_CODEC_INFO_CAN_FLUSH               = 1 << 0,
        
        /// The codec is not safe to use when decompressing untrusted data.
        ///
        /// By default, codecs are assumed to be safe.
        ///
        /// Currently, in order for a plugin to be distributed with Squash it
        /// must be free from crashes (which, of course, are often exploitable).
        /// At first glance you might think this would prevent any unsafe
        /// plugin from being distributed with Squash, but that isn't quite true.
        ///
        /// ZPAQ is not considered safe. It allows a compressed stream to
        /// embed a program (written in ZPAQL) which is used to decompress
        /// the archive. Since it is impossible to determine whether the
        /// program will eventually terminate, it is possible to create a ZPAQL
        /// program with an infinite loop.
        ///
        /// Note that this restriction only applies to plugins distributed
        /// with Squash. It is possible (and encouraged) for people to
        /// distribute Squash plugins separately from Squash.
        const SQUASH_CODEC_INFO_DECOMPRESS_UNSAFE       = 1 << 1,
        
        const SQUASH_CODEC_INFO_WRAP_SIZE               = 1 << 2,

        /// Mask of flags which are automatically set based on
        /// which callbacks are provided.
        const SQUASH_CODEC_INFO_AUTO_MASK               = 0x00ff0000,
        
        /// The codec is valid.
        const SQUASH_CODEC_INFO_VALID                   = 1 << 16,
        
        /// The compressed data encodes the size of the uncompressed
        /// data without having to decompress it.
        const SQUASH_CODEC_INFO_KNOWS_UNCOMPRESSED_SIZE = 1 << 17,
        
        /// The codec natively supports a streaming interface.
        const SQUASH_CODEC_INFO_NATIVE_STREAMING        = 1 << 18,

        #[allow(overflowing_literals)]
        const SQUASH_CODEC_INFO_MASK                    = 0xffffffff
    }
}

/// Function table for plugins.
///
/// This struct should only be used from within a plugin.
///
/// This structure may grow over time to accomodate new features, so when
/// setting up the callbacks in a plugin you must set each field individually
/// instead of copying an entire instance of the struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SquashCodecImpl {
    /// Capability information about the codec. 
    pub info: SquashCodecInfo,
    
    /// options which may bo passed to the codec to modify its operation 
    pub options: *const SquashOptionInfo,
    
    /// Create a new SquashStream.
    ///
    /// # Parameters
    /// * `codec` The codec.
    /// * `stream_type` The type of stream to create.
    /// * `options` The stream options.
    ///
    /// # Returns
    /// A new SquashStream.
    pub create_stream: Option<extern "C" fn(
        codec: *mut SquashCodec,
        stream_type:SquashStreamType,
        options: *mut SquashOptions) -> *mut SquashStream>,
    
    /// Process a SquashStream.
    ///
    /// # Parameters
    /// * `stream` The stream.
    /// * `operation` The operation to perform.
    ///
    /// # Returns
    /// A status code.
    pub process_stream: Option<extern "C" fn(
        stream: *mut SquashStream,
        operation: SquashOperation) -> SquashStatus>,
    
    /// Splice.
    ///
    /// # Parameters
    /// * `options` Options to use
    /// * `stream_type` Whether to compress or decompress
    /// * `read_cb` Callback to use to read data
    /// * `write_cb` Callback to use to write data
    /// * `user_data` Date to pass to the callbacks
    ///
    /// # Returns
    /// A status code
    pub splice: Option<extern "C" fn(
        codec: *mut SquashCodec,
        options: *mut SquashOptions,
        stream_type: SquashStreamType,
        read_cb: SquashReadFunc,
        write_cb: SquashWriteFunc,
        user_data: *mut c_void) -> SquashStatus>,
    
    /// Decompress a buffer.
    ///
    /// # Parameters
    /// * `codec` The codec.
    /// * `compressed` The compressed data.
    /// * `compressed_size` Size of the compressed data.
    /// * `uncompressed` Buffer in which to store the uncompressed data.
    /// * `uncompressed_size` Location of the buffer size on input, used
    /// to store the size of the uncompressed data on output.
    /// * `options` Decompression options (or NULL)
    pub decompress_buffer: Option<extern "C" fn(
        codec: *mut SquashCodec,
        decompressed_size: *mut size_t,
        decompressed: *mut uint8_t,
        compressed_size: size_t,
        compressed: *const uint8_t,
        options: *mut SquashOptions) -> SquashStatus>,
    
    /// Compress a buffer.
    ///
    /// # Parameters
    /// * `codec` The codec.
    /// * `uncompressed` The uncompressed data.
    /// * `uncompressed_size` The size of the uncompressed data.
    /// * `compressed` Buffer in which to store the compressed data.
    /// * `compressed_size` Location of the buffer size on input, used to store
    /// the size of the compressed data on output.
    /// * `options` Compression options (or NULL)
    pub compress_buffer: Option<extern "C" fn(
        codec: *mut SquashCodec,
        compressed_size: *mut size_t,
        compressed: *mut uint8_t,
        uncompressed_size: size_t,
        uncompressed: *const uint8_t,
        options: *mut SquashOptions) -> SquashStatus>,
    
    /// Compress a buffer.
    ///
    /// Plugins implementing this function can be sure that compressed is at
    /// least as long as the maximum compressed size for a buffer of
    /// uncompressed_size bytes.
    ///
    /// # Parameters
    /// * `codec` The codec.
    /// * `uncompressed` The uncompressed data.
    /// * `uncompressed_size` The size of the uncompressed data.
    /// * `compressed` Buffer in which to store the compressed data.
    /// * `compressed_size` Location of the buffer size on input, used to store
    /// the size of the compressed data on output.
    /// * `options` Compression options (or NULL)
    pub compress_buffer_unsafe: Option<extern "C" fn(
        codec: *mut SquashCodec,
        compressed_size: *mut size_t,
        compressed: *mut uint8_t,
        uncompressed_size: size_t,
        uncompressed: *const uint8_t,
        options: *mut SquashOptions) -> SquashStatus>,
    
    /// Get the buffer's uncompressed size.
    ///
    /// # Parameters
    /// * `codec` The codec.
    /// * `compressed` Compressed data.
    /// * `compressed_size` Size of compressed data (in bytes).
    ///
    /// # Returns
    /// Size of the uncompressed data, or 0 if unknown.
    pub get_uncompressed_size: Option<extern "C" fn(
        codec: *mut SquashCodec,
        compressed_size: size_t,
        compressed: *const uint8_t) -> size_t>,
    
    /// Get the maximum compressed size.
    ///
    /// # Parameters
    /// * `codec` The codec.
    /// * `uncompressed_size` Size of the uncompressed data.
    ///
    /// # Returns
    /// The maximum buffer size necessary to contain the compressed data.
    pub get_max_compressed_size: Option<extern "C" fn(
        codec: *mut SquashCodec,
        uncompressed_size: size_t) -> size_t>,
    
    /// Reserved for future use.
    pub _reserved1: Option<extern "C" fn() -> ()>,
    
    /// Reserved for future use.
    pub _reserved2: Option<extern "C" fn() -> ()>,
    
    /// Reserved for future use.
    pub _reserved3: Option<extern "C" fn() -> ()>,
    
    /// Reserved for future use.
    pub _reserved4: Option<extern "C" fn() -> ()>,
    
    /// Reserved for future use.
    pub _reserved5: Option<extern "C" fn() -> ()>,
    
    /// Reserved for future use.
    pub _reserved6: Option<extern "C" fn() -> ()>,
    
    /// Reserved for future use.
    pub _reserved7: Option<extern "C" fn() -> ()>,
    
    /// Reserved for future use.
    pub _reserved8: Option<extern "C" fn() -> ()>,
}

extern {
    /// Initialize a codec.
    ///
    /// # Note
    /// *This function is generally only useful inside of a callback
    /// passed to squash_foreach_codec or squash_plugin_foreach_codec.
    /// Every other way to get a codec (such as squash_get_codec or
    /// squash_plugin_get_codec) will initialize the codec as well (and
    /// return NULL instead of the codec if initialization fails). The
    /// foreach functions, however, do not initialize the codec since doing
    /// so requires actually loading the plugin.*
    ///
    /// # Parameters
    /// * `codec` The codec.
    ///
    /// # Returns
    /// A status code.
    ///
    /// ## Return values
    /// * [`SQUASH_OK`] Codec successfully initialized.
    /// * [`SQUASH_UNABLE_TO_LOAD`] Failed to load the codec.
    ///
    /// [`SQUASH_OK`]: ./constant.SQUASH_OK.html
    /// [`SQUASH_UNABLE_TO_LOAD`]: ./constant.SQUASH_UNABLE_TO_LOAD.html
    pub fn squash_codec_init(codec: *mut SquashCodec) -> SquashStatus;
    
    /// Get the name of a SquashCodec.
    ///
    /// # Parameters
    /// * `codec` The codec
    ///
    /// # Returns
    /// The codec's name
    pub fn squash_codec_get_name(codec: *mut SquashCodec) -> *const c_char;
    
    /// Get the priority of a SquashCodec.
    ///
    /// # Parameters
    /// * `codec` The codec
    ///
    /// # Returns
    /// The codec's priority
    pub fn squash_codec_get_priority(codec: *mut SquashCodec) -> c_uint;
    
    /// Get the plugin associated with a codec.
    ///
    /// # Parameters
    /// * `codec` The codec
    ///
    /// # Returns
    /// The plugin to which the codec belongs
    pub fn squash_codec_get_plugin(codec: *mut SquashCodec) -> *mut SquashPlugin;
    
    /// Get the context associated with a codec.
    ///
    /// # Parameters
    /// * `codec` The codec
    ///
    /// # Returns
    /// The context to which the codec belongs
    pub fn squash_codec_get_context(codec: *mut SquashCodec) -> *mut SquashContext;
    
    /// Get the codec's extension.
    ///
    /// # Parameters
    /// * `codec` The codec
    ///
    /// # Returns
    /// The extension, or NULL if none is known
    pub fn squash_codec_get_extension(codec: *mut SquashCodec) -> *const c_char;
    
    /// Get the uncompressed size of the compressed buffer.
    ///
    /// This function is only useful for codecs with the
    /// `SQUASH_CODEC_INFO_KNOWS_UNCOMPRESSED_SIZE` flag set. For situations
    /// where the codec does not know the uncompressed size, 0 will be
    /// returned.
    ///
    /// # Parameters
    /// * `codec` The codec
    /// * `compressed` The compressed data
    /// * `compressed_size` The size of the compressed data
    ///
    /// # Returns
    /// The uncompressed size, or 0 if unknown
    pub fn squash_codec_get_uncompressed_size(
        codec: *mut SquashCodec,
        compressed_size: size_t,
        compressed: *const u8) -> size_t;
    
    /// Get the maximum buffer size necessary to store compressed data.
    ///
    /// Typically the return value will be some percentage larger than
    /// the uncompressed size, plus a few bytes. For example, for bzip2
    /// it is the uncompressed size plus 1%, plus an additional 600 bytes.
    ///
    /// # Warning
    /// *The result of this function is not guaranteed to be correct for
    /// use with the SquashStream API—it should only be used with the
    /// single-call buffer-to-buffer functions such as squash_codec_compress
    /// and squash_codec_compress_with_options.*
    ///
    /// # Parameters
    /// * `codec` The codec
    /// * `uncompressed_size` Size of the uncompressed data in bytes
    ///
    /// # Returns
    /// The maximum size required to store a compressed buffer representing
    /// uncompressed_size of uncompressed data.
    pub fn squash_codec_get_max_compressed_size(
        codec: *mut SquashCodec,
        uncompressed_size: size_t) -> size_t;
    
    /// Create a new stream.
    ///
    /// # Parameters
    /// * `codec` The codec
    /// * `stream_type` The direction of the stream
    /// * `...` A variadic list of key/value option pairs, followed by NULL
    ///
    /// # Returns
    /// A new stream, or NULL on failure
    pub fn squash_codec_create_stream(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType, ...) -> *mut SquashStream;
    
    /// Create a new stream with existing SquashOptions.
    ///
    /// # Parameters
    /// * `codec` The codec
    /// * `stream_type` The direction of the stream
    /// * `options` The options for the stream, or NULL to use the defaults
    ///
    /// # Returns
    /// A new stream, or NULL on failure
    pub fn squash_codec_create_stream_with_options(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType,
        options: *mut SquashOptions) -> *mut SquashStream;
    
    /// Compress a buffer.
    ///
    /// # Parameters
    /// * `codec` The codec to use
    /// * `compressed` Location to store the compressed data
    /// * `compressed_size` Location storing the size of the compressed
    /// buffer on input, replaced with the actual size of the
    /// compressed data
    /// * `uncompressed` The uncompressed data
    /// * `uncompressed_size` Size of the uncompressed data (in bytes)
    /// * `...` A variadic list of key/value option pairs, followed by NULL
    ///
    /// # Returns
    /// A status code
    pub fn squash_codec_compress(
        codec: *mut SquashCodec,
        compressed_size: *mut size_t,
        compressed: *mut u8,
        uncompressed_size: size_t,
        uncompressed: *const u8, ...) -> SquashStatus;
    
    /// Compress a buffer with an existing SquashOptions.
    ///
    /// # Parameters
    /// * `codec` The codec to use
    /// * `compressed` Location to store the compressed data
    /// * `compressed_size` Location storing the size of the compressed
    /// buffer on input, replaced with the actual size of the compressed data
    /// * `uncompressed` The uncompressed data
    /// * `uncompressed_size` Size of the uncompressed data (in bytes)
    /// * `options` Compression options
    ///
    /// # Returns
    /// A status code
    pub fn squash_codec_compress_with_options(
        codec: *mut SquashCodec,
        compressed_size: *mut size_t,
        compressed: *mut u8,
        uncompressed_size: size_t,
        uncompressed: *const u8,
        options: *mut SquashOptions) -> SquashStatus;
    
    /// Decompress a buffer.
    ///
    /// # Parameters
    /// * `codec` The codec to use
    /// * `decompressed` The decompressed data
    /// * `decompressed_size` Size of the decompressed data (in bytes)
    /// * `compressed` Location to store the compressed data
    /// * `compressed_size` Location storing the size of the compressed
    /// buffer on input, replaced with the actual size of the compressed data
    /// * `...` A variadic list of key/value option pairs, followed by NULL
    ///
    /// # Returns
    /// A status code
    pub fn squash_codec_decompress(
        codec: *mut SquashCodec,
        decompressed_size: *mut size_t,
        decompressed: *mut u8,
        compressed_size: size_t,
        compressed: *const u8, ...) -> SquashStatus;
    
    /// Decompress a buffer with an existing SquashOptions.
    ///
    /// # Parameters
    /// * `codec` The codec to use
    /// * `decompressed` Location to store the decompressed data
    /// * `decompressed_size` Location storing the size of the decompressed
    /// buffer on input, replaced with the actual size of the decompressed data
    /// * `compressed` The compressed data
    /// * `compressed_size` Size of the compressed data (in bytes)
    /// * `options` Compression options
    ///
    /// # Returns
    /// A status code
    pub fn squash_codec_decompress_with_options(
        codec: *mut SquashCodec,
        decompressed_size: *mut size_t,
        decompressed: *mut u8,
        compressed_size: size_t,
        compressed: *const u8,
        options: *mut SquashOptions) -> SquashStatus;
    /// Get a bitmask of information about the codec.
    ///
    /// # Parameters
    /// * `codec` The codec
    ///
    /// # Returns
    /// The codec info
    pub fn squash_codec_get_info(codec: *mut SquashCodec) -> SquashCodecInfo;
    
    /// Get a list of options applicable to the codec.
    ///
    /// # Parameters
    /// * `codec` The codec
    ///
    /// # Returns
    /// A list of options, terminated by an option with a NULL name
    pub fn squash_codec_get_option_info(codec: *mut SquashCodec) -> *const SquashOptionInfo;
    
    /// Get the codec's function table.
    ///
    /// # Parameters
    /// * `codec` The codec.
    ///
    /// # Returns
    /// The function table.
    pub fn squash_codec_get_impl(codec: *mut SquashCodec) -> *mut SquashCodecImpl;
}
