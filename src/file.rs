use libc::{c_char, size_t, uint8_t, FILE};

use codec::SquashCodec;
use option::SquashOptions;
use status::SquashStatus;

/// stdio-like API and utilities
///
/// These functions provide an API which should be familiar for those used
/// to dealing with the standard I/O functions.
pub enum SquashFile { }

extern {
    /// Open a file.
    ///
    /// The mode parameter will be passed through to fopen, so the value
    /// must valid. Note that Squash may attempt to use mmap regardless of
    /// whether the m flag is passed.
    ///
    /// The file is always assumed to be compressed—calling
    /// [`squash_file_write`] will always compress, and calling
    /// [`squash_file_read`] will always decompress. Note, however, that you
    /// cannot mix reading and writing to the same file as you can with a
    /// standard `FILE`.
    ///
    /// # Note
    /// *Error handling for this function is somewhat limited, and it may
    /// be difficult to determine the exact nature of problems such as an
    /// invalid codec, where `errno` is not set. If this is unacceptable you
    /// should call [`squash_options_parse`] yourself and pass the results to
    /// [`squash_file_open_with_options`] \(which will only fail due to the
    /// underlying fopen failing).*
    ///
    /// # Parameters
    /// * `filename` name of the file to open
    /// * `mode` file mode
    /// * `codec` codec to use
    /// * `...` options
    ///
    /// # Returns
    /// The opened file, or NULL on error
    ///
    /// [`squash_file_write`]: ./fn.squash_file_write.html
    /// [`squash_file_read`]: ./fn.squash_file_read.html
    /// [`squash_options_parse`]: ./fn.squash_options_parse.html
    /// [`squash_file_open_with_options`]: ./fn.squash_file_open_with_options.html
    pub fn squash_file_open(
        codec: *mut SquashCodec,
        filename: *const c_char,
        mode: *const c_char,
        ...) -> *mut SquashFile;
    
    /// Open a file using a with the specified options.
    ///
    /// `filename` is assumed to be UTF-8 encoded. On Windows, this function
    /// will call `squash_file_wopen_with_options` internally. On other
    /// platforms, filenames on disk are assumed to be in UTF-8 format,
    /// therefore the filename is passed through to fopen without any
    /// conversion.
    ///
    /// # Parameters
    /// * `filename` name of the file to open
    /// * `mode` file mode
    /// * `codec` codec to use
    /// * `options` options
    ///
    /// # Returns
    /// The opened file, or NULL on error
    pub fn squash_file_open_with_options(
        codec: *mut SquashCodec,
        filename: *const c_char,
        mode: *const c_char,
        options: *mut SquashOptions) -> *mut SquashFile;
    
    ///Open a file using a with the specified options.
    ///
    /// On non-Windows platforms this function will convert the filename
    /// to UTF-8 and call [`squash_file_open_with_options`]. On Windows,
    /// it will use `_wfopen`.
    ///
    /// # Parameters
    /// * `filename` name of the file to open
    /// * `mode` file mode
    /// * `codec` codec to use
    /// * `options` options
    ///
    /// # Returns
    /// The opened file, or NULL on error
    ///
    /// Note that Squash expects to have exclusive access to fp.
    /// When possible, Squash will acquire fp's lock (using `flockfile`)
    /// in this function and will not release it until the `SquashFile`
    /// instance is destroyed.
    ///
    /// # Warning
    /// *On Windows you should not use this function unless the code which
    /// opened the file descriptor is using the same runtime; see
    /// http://siomsystems.com/mixing-visual-studio-versions/ for more
    /// information.*
    ///
    /// # Parameters
    /// * `fp` the stdio file to use
    /// * `codec` codec to use
    /// * `...` options
    ///
    /// # Returns
    /// The opened file, or NULL on error
    pub fn squash_file_steal(
        codec: *mut SquashCodec,
        fp: *mut FILE,
        ...) -> *mut SquashFile;
    
    /// Open an existing stdio file with the specified options.
    /// 
    /// # Warning
    /// *On Windows you should not use this function unless the code which
    /// opened the file descriptor is using the same runtime; see
    /// http://siomsystems.com/mixing-visual-studio-versions/ for more
    /// information.*
    ///
    /// # Parameters
    /// * `fp` the stdio file to use
    /// * `codec` codec to use
    /// * `options` options
    ///
    /// # Returns
    /// The opened file, or NULL on error
    pub fn squash_file_steal_with_options(
        codec: *mut SquashCodec,
        fp: *mut FILE,
        options: *mut SquashOptions) -> *mut SquashFile;
    
    /// Read from a compressed file.
    ///
    /// Attempt to read `decompressed_size` bytes of decompressed data into
    /// the decompressed buffer. The number of bytes of compressed data read
    /// from the input file may be significantly more, or less, than
    /// `decompressed_size`.
    ///
    /// The number of decompressed bytes successfully read from the file
    /// will be stored in `decompressed_read` after this function is executed.
    /// This value will never be greater than `decompressed_size`, but it
    /// may be less if there was an error or the end of the input file was
    /// reached.
    ///
    /// # Note
    /// *Squash can, and frequently will, read more data from the input file
    /// than necessary to produce the requested amount of decompressed data.
    /// There is no way to know how much input will be required to produce
    /// the requested output, or even how much input was used.
    ///
    /// # Parameters
    /// * `file` the file to read from
    /// * `decompressed_size` number of bytes to attempt to write to
    /// `decompressed`
    /// * `decompressed` buffer to write the decompressed data to
    ///
    /// # Returns
    /// The result of the operation
    ///
    /// # Return values
    /// * `SQUASH_OK` successfully read some data
    /// * `SQUASH_END_OF_STREAM` the end of the file was reached
    pub fn squash_file_read(
        file: *mut SquashFile,
        decompressed_size: *mut size_t,
        decompressed: *mut uint8_t) -> SquashStatus;
    
    /// Write data to a compressed file.
    ///
    /// Attempt to write the compressed equivalent of uncompressed to a
    /// file. The number of bytes of compressed data written to the output
    /// file may be **significantly** more, or less, than the
    /// `uncompressed_size`.
    /// 
    /// # Note
    /// It is likely the compressed data will actually be buffered, not
    /// immediately written to the file. For codecs which support flushing
    /// you can use [`squash_file_flush`] to flush the data, otherwise the
    /// data may not be written until [`squash_file_close`] or
    /// [`squash_file_free`] is called.
    ///
    /// # Parameters
    /// * `file` file to write to
    /// * `uncompressed_size` number of bytes of uncompressed data in
    /// `uncompressed` to attempt to write
    /// * `uncompressed` data to write
    ///
    /// # Returns
    /// Result of the operation
    ///
    /// [`squash_file_flush`]: ./fn.squash_file_flush.html
    /// [`squash_file_close`]: ./fn.squash_file_close.html
    /// [`squash_file_free`]: ./fn.squash_file_free.html
    pub fn squash_file_write(
        file: *mut SquashFile,
        uncompressed_size: size_t,
        uncompressed: *const uint8_t) -> SquashStatus;

    pub fn squash_file_printf(
        file: *mut SquashFile,
        format: *const c_char,
        ...) -> SquashStatus;

    /// Immediately write any buffered data to a file.
    ///
    /// # Note
    /// *This function only works for codecs which support flushing (see the
    /// [`SQUASH_CODEC_INFO_CAN_FLUSH`] flag in the return value of 
    /// [`squash_codec_get_info`]).*
    ///
    /// # Parameters
    /// * `file` file to flush
    ///
    /// # Returns
    /// true if flushing succeeeded, false if flushing is not supported or
    /// there was another error.
    ///
    /// [`SQUASH_CODEC_INFO_CAN_FLUSH`]: ./constant.SQUASH_CODEC_INFO_CAN_FLUSH.html
    /// [`squash_codec_get_info`]: ./fn.squash_codec_get_info.html
    pub fn squash_file_flush(file: *mut SquashFile) -> SquashStatus;
    
    /// Close a file.
    ///
    /// If `file` is a compressor the stream will finish compressing,
    /// writing any buffered data. For codecs which do not provide a native
    /// streaming interface, all of the actual compression will take place
    /// during this call. In other words, it may block for a non-trivial
    /// period. If this is a problem please file a bug against Squash
    /// (including your use case), and we can discuss adding a function call
    /// which will simply abort compression.
    ///
    /// In addition to freeing the SquashFile instance, this function will
    /// close the underlying `FILE` pointer. If you wish to continue using
    /// the `FILE` for something else, use [`squash_file_free`] instead.
    ///
    /// # Parameters
    /// * `file` file to close
    ///
    /// # Returns
    /// `SQUASH_OK` on success or a negative error code on failure
    ///
    /// [`squash_file_free`]: ./fn.squash_file_free.html
    pub fn squash_file_close(file: *mut SquashFile) -> SquashStatus;
    
    /// Free a file.
    ///
    /// This function will free the SquashFile, but unlike
    /// [`squash_file_close`] it will not actually close the underlying
    /// `FILE` pointer. Instead, it will return the value in the fp argument,
    /// allowing you to further manipulate it.
    ///
    /// # Warning
    /// **On Windows you should not use this function unless Squash is
    /// linked against the same runtime as the code you want to continue
    /// using the file pointer from; see
    /// http://siomsystems.com/mixing-visual-studio-versions/ for more
    /// information.**
    ///
    /// # Parameters
    /// * `file` file to free
    /// * `fp` location to store the underlying `FILE` pointer
    ///
    /// # Returns
    /// `SQUASH_OK` on success or a negative error code on failure
    ///
    /// [`squash_file_close`]: ./fn.squash_file_close.html
    pub fn squash_file_free(file: *mut SquashFile, fp: *mut *mut FILE) -> SquashStatus;

    /// Determine whether the `file` has reached the end of file.
    ///
    /// # Parameters
    /// * `file` file to check
    ///
    /// # Returns
    /// true if EOF was reached, false otherwise
    pub fn squash_file_eof(file: *mut SquashFile) -> bool;

    /// Retrieve the last return value.
    ///
    /// This will return the result of the previous function called on this
    /// file.
    ///
    /// # Parameters
    /// * `file` file to examine
    ///
    /// # Returns
    /// last status code returned by a function on file
    pub fn squash_file_error(file: *mut SquashFile) -> SquashStatus;

    /// Acquire the lock on a file.
    ///
    /// [`squash_file_read`], [`squash_file_write`], and [`squash_file_flush`]
    /// are thread-safe. This is accomplished by acquiring a lock on while
    /// each function is operating in order to ensure exclusive access.
    ///
    /// If, however, the programmer wishes to call a series of functions and
    /// ensure that they are performed without interference, they can
    /// manually acquire the lock with this function and use the unlocked
    /// variants ([`squash_file_read_unlocked`],
    /// [`squash_file_write_unlocked`], and [`squash_file_flush_unlocked`]).
    ///
    /// # Note
    /// This function has nothing to do with the kind of lock acquired by
    /// the flock function.
    ///
    /// # Parameters
    /// * `file` the file to acquire the lock on
    ///
    /// [`squash_file_read`]: ./fn.squash_file_read.html
    /// [`squash_file_write`]: ./fn.squash_file_write.html
    /// [`squash_file_flush`]: ./fn.squash_file_flush.html
    /// [`squash_file_read_unlocked`]: ./fn.squash_file_read_unlocked.html
    /// [`squash_file_write_unlocked`]: ./fn.squash_file_write_unlocked.html
    /// [`squash_file_flush_unlocked`]: ./fn.squash_file_flush_unlocked.html
    pub fn squash_file_lock(file: *mut SquashFile);
    
    /// Release the lock on a file.
    ///
    /// This function releases the lock acquired by [`squash_file_lock`].
    /// If you have not called that function do not call this one.
    ///
    /// # Parameters
    /// * `file` the file to release the lock on
    ///
    /// [`squash_file_lock`]: ./fn.squash_file_lock.html
    pub fn squash_file_unlock(file: *mut SquashFile);

    /// Read from a compressed file.
    ///
    /// This function is the same as [`squash_file_read`], except it will
    /// not acquire a lock on the `SquashFile` instance. It should be used
    /// only when there is no possibility of other threads accessing the
    /// file, or if you have already acquired the lock with 
    /// [`squash_file_lock`].
    ///
    /// # Parameters
    /// * `file` the file to read from
    /// * `decompressed_size` number of bytes to attempt to write to
    /// `decompressed`
    /// * `decompressed` buffer to write the decompressed data to
    ///
    /// # Returns
    /// The result of the operation
    ///
    /// # Return values
    /// * `SQUASH_OK` successfully read some data
    /// * `SQUASH_END_OF_STREAM` the end of the file was reached
    ///
    /// [`squash_file_read`]: ./fn.squash_file_read.html
    /// [`squash_file_lock`]: ./fn.squash_file_lock.html
    pub fn squash_file_read_unlocked(file: *mut SquashFile, decompressed_size: *mut size_t, decompressed: *mut uint8_t) -> SquashStatus;
    
    /// Write data to a compressed file without acquiring the lock.
    ///
    /// This function is the same as [`squash_file_write`], except it will
    /// not acquire a lock on the `SquashFile` instance. It should be used
    /// only when there is no possibility of other threads accessing the
    /// file, or if you have already acquired the lock with
    /// [`squash_file_lock`].
    ///
    /// # Parameters
    /// * `file` file to write to
    /// * `uncompressed_size` number of bytes of uncompressed data in
    /// `uncompressed` to attempt to write
    /// * `uncompressed` data to write
    ///
    /// # Returns
    /// Result of the operation
    ///
    /// [`squash_file_write`]: ./fn.squash_file_write.html
    /// [`squash_file_lock`]: ./fn.squash_file_lock.html
    pub fn squash_file_write_unlocked(file: *mut SquashFile, uncompressed_size: size_t, uncompressed: *const uint8_t) -> SquashStatus;
    
    /// Immediately write any buffered data to a file without
    /// acquiring the lock
    ///
    /// Write data to a compressed file without acquiring the lock
    ///
    /// This function is the same as [`squash_file_write`], except it will
    /// not acquire a lock on the `SquashFile` instance. It should be used
    /// only when there is no possibility of other threads accessing the file,
    /// or if you have already acquired the lock with [`squash_file_lock`].
    ///
    /// # Parameters
    /// * `file` file to flush
    ///
    /// # Returns
    /// true if flushing succeeeded, false if flushing is not supported or
    /// there was another error.
    ///
    /// [`squash_file_write`]: ./fn.squash_file_write.html
    /// [`squash_file_lock`]: ./fn.squash_file_lock.html
    pub fn squash_file_flush_unlocked(file: *mut SquashFile) -> SquashStatus;
}

#[cfg(feature = "wide-char-api")]
use libc::wchar_t;
#[cfg(feature = "wide-char-api")]
extern {
    pub fn squash_file_wopen(
        codec: *mut SquashCodec,
        filename: *const wchar_t,
        mode: *const wchar_t,
        ...) -> *mut SquashFile;
    pub fn squash_file_wopen_with_options(
        codec: *mut SquashCodec,
        filename: *const wchar_t,
        mode: *const wchar_t,
        options: *mut SquashOptions) -> *mut SquashFile;
    pub fn squash_file_wprintf(
        file: *mut SquashFile,
        format: *const wchar_t,
        ...) -> SquashStatus;
}
