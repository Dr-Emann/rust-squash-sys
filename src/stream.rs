use libc::{c_char, c_int, c_void, size_t, uint8_t};

use object::{SquashObject, SquashDestroyNotify};
use codec::SquashCodec;
use option::SquashOptions;
use status::SquashStatus;

pub use self::SquashStreamType::*;
pub use self::SquashOperation::*;

pub enum SquashStreamPrivate { }

/// Stream type.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SquashStreamType {
    /// A compression stream.
    SQUASH_STREAM_COMPRESS   = 1,
    /// A decompression stream.
    SQUASH_STREAM_DECOMPRESS = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SquashOperation {
    /// Continue processing the stream normally.
    ///
    /// See also: [`squash_stream_process`]
    ///
    /// [`squash_stream_process`]: ./fn.squash_stream_process.html
    SQUASH_OPERATION_PROCESS   = 1,

    /// Flush the stream.
    ///
    /// See also: [`squash_stream_flush`]
    ///
    /// [`squash_stream_flush`]: ./fn.squash_stream_flush.html
    SQUASH_OPERATION_FLUSH     = 2,

    /// Finishing processing the stream.
    ///
    /// See also: [`squash_stream_finish`]
    ///
    /// [`squash_stream_finish`]: ./fn.squash_stream_finish.html
    SQUASH_OPERATION_FINISH    = 3,

    /// Abort.
    ///
    /// This value is only passed to plugins with the
    /// SQUASH_CODEC_INFO_RUN_IN_THREAD flag set, and signals that the
    /// stream is being destroyed (likely before processing has completed).
    /// There will be no further input, and any output will be ignored.
    SQUASH_OPERATION_TERMINATE = 4,
}

/// State the stream is in.
///
/// This is managed internally by Squash and should not be modified by
/// consumers or plugins.
pub type SquashStreamState = c_int;

pub const SQUASH_STREAM_STATE_IDLE: SquashStreamState      = 0;
pub const SQUASH_STREAM_STATE_RUNNING: SquashStreamState   = 1;
pub const SQUASH_STREAM_STATE_FLUSHING: SquashStreamState  = 2;
pub const SQUASH_STREAM_STATE_FINISHING: SquashStreamState = 3;
pub const SQUASH_STREAM_STATE_FINISHED: SquashStreamState  = 4;

/// Compression/decompression streams.
#[repr(C)]
pub struct SquashStream {
    /// Base object.
    pub base_object: SquashObject,

    /// Private data.
    ///
    /// This is managed internally by Squash and should not be modified by
    /// consumers or plugins.
    pub _priv: *mut SquashStreamPrivate,

    /// The next input date to consume.
    pub next_in: *const uint8_t,

    /// Size (in bytes) of available input.
    pub avail_in: size_t,

    /// The total number of bytes input.
    ///
    /// This is managed internally by Squash and should not be modified by
    /// consumers or plugins.
    pub total_in: size_t,

    /// The buffer to write output to.
    pub next_out: *mut uint8_t,

    /// Number of bytes available in the output buffer.
    pub avail_out: size_t,

    /// Total number of bytes output.
    ///
    /// This is managed internally by Squash and should not be modified by
    /// consumers or plugins.
    pub total_out: size_t,

    /// Codec used for this stream.
    pub codec: *mut SquashCodec,

    /// Options used for this stream.
    pub options: *mut SquashOptions,

    /// Stream type.
    pub stream_type: SquashStreamType,

    /// State the stream is in.
    ///
    /// This is managed internally by Squash and should not be modified by
    /// consumers or plugins.
    pub state: SquashStreamState,

    /// User data.
    ///
    /// Note that this is for consumers of the library, not for plugins. It
    /// should be safe to use this from your application.
    pub user_data: *mut c_void,

    /// Callback to invoke on `user_data` when it is no longer necessary.
    pub destroy_user_data: SquashDestroyNotify,
}

extern {
    /// Create a new stream with options.
    ///
    /// # Parameters
    /// * `codec` The name of the codec.
    /// * `stream_type` Stream type.
    /// * `options` An option group.
    ///
    /// # Returns
    /// A new stream, or NULL on failure.
    pub fn squash_stream_new(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType, ...) -> *mut SquashStream;
    
    /// Create a new stream with key/value option arrays.
    ///
    /// # Parameters
    /// * `codec` The name of the codec.
    /// * `stream_type` Stream type.
    /// * `keys` NULL-terminated array of option keys.
    /// * `values` Array of option values.
    ///
    /// # Returns
    /// A new stream, or NULL on failure.
    pub fn squash_stream_newa(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType,
        keys: *const *const c_char,
        values: *const *const c_char) -> *mut SquashStream;

    /// Create a new stream with an options instance.
    ///
    /// # Parameters
    /// * `codec` Codec to use
    /// * `stream_type` Stream type
    /// * `options` Options
    ///
    /// # Returns
    /// A new stream, or NULL on failure
    pub fn squash_stream_new_with_options(
        codec: *mut SquashCodec,
        stream_type: SquashStreamType,
        options: *mut SquashOptions) -> *mut SquashStream;

    /// Process a stream.
    ///
    /// This method will attempt to process data in a stream. It should be
    /// called repeatedly, adding data to the avail_in field and removing
    /// data from the avail_out field as necessary.
    ///
    /// # Parameters
    /// * `stream` The stream.
    ///
    /// # Returns
    /// A status code.
    ///
    /// # Return values
    /// * `SQUASH_OK` All input successfully consumed.
    /// Check the output buffer for data then proceed with new input.
    /// * `SQUASH_PROCESSING` Progress was made, but not all input could be
    /// consumed. Remove some data from the output buffer and run
    /// [`squash_stream_process`] again.
    /// * `SQUASH_END_OF_STREAM` The end of stream was reached. You shouldn't
    /// call [`squash_stream_process`] again. Decompression only.
    ///
    /// [`squash_stream_process`]: ./fn.squash_stream_process.html
    pub fn squash_stream_process(stream: *mut SquashStream) -> SquashStatus;
    
    /// Flush a stream.
    ///
    /// This method will attempt to process data in a stream. It should be
    /// called repeatedly, adding data to the `avail_in` field and removing
    /// data from the `avail_out` field as necessary.
    /// # Parameters
    /// * `stream`  The stream.
    ///
    /// # Returns
    /// A status code.
    pub fn squash_stream_flush(stream: *mut SquashStream) -> SquashStatus;

    /// Finish writing to a stream.
    ///
    /// # Parameters
    /// * `stream`  The stream.
    ///
    /// # Returns
    /// A status code.
    pub fn squash_stream_finish(stream: *mut SquashStream) -> SquashStatus;
    
    /// Initialize a stream.
    ///
    /// # Warning
    /// **This function must only be used to implement a subclass of
    /// SquashStream. Streams returned by other functions will already be
    /// initialized, and you must not call this function on them; doing so
    /// will likely trigger a memory leak.**
    ///
    /// # Parameters
    /// * `stream` The stream to initialize.
    /// * `codec` The codec to use.
    /// * `stream_type` The stream type.
    /// * `options` The options.
    /// * `destroy_notify` Function to call to destroy the instance.
    ///
    /// # See also
    /// [`squash_object_init`]
    ///
    /// [`squash_object_init`]: ./fn.squash_object_init.html
    pub fn squash_stream_init(
        stream: *mut c_void,
        codec: *mut SquashCodec,
        stream_type: SquashStreamType,
        options: *mut SquashOptions,
        destroy_notify: SquashDestroyNotify) -> ();

    /// Destroy a stream.
    ///
    /// # Warning
    /// **This function must only be used to implement a subclass of
    /// `SquashObject`. Each subclass should implement a *_destroy function
    /// which should perform any operations needed to destroy their own data
    /// and chain up to the *_destroy function of the base class, eventually
    /// invoking squash_object_destroy. Invoking this function in any other
    /// context is likely to cause a memory leak or crash. If you are not
    /// creating a subclass, you should be calling
    /// [`squash_object_unref`] instead.**
    ///
    /// # Parameters
    /// * `stream` The stream.
    ///
    /// [`squash_object_unref`]: ./fn.squash_object_unref.html
    pub fn squash_stream_destroy(stream: *mut c_void) -> ();
}
