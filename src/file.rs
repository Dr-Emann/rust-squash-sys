use libc::{c_char, size_t, uint8_t, FILE};

use codec::SquashCodec;
use option::SquashOptions;
use status::SquashStatus;

pub enum SquashFile { }

extern {
    pub fn squash_file_open(codec: *const c_char,
                            filename: *const c_char,
                            mode: *const c_char, ...)
     -> *mut SquashFile;
    pub fn squash_file_open_codec(codec: *mut SquashCodec,
                                  filename: *const c_char,
                                  mode: *const c_char, ...)
     -> *mut SquashFile;
    pub fn squash_file_open_with_options(codec: *const c_char,
                                         filename: *const c_char,
                                         mode: *const c_char,
                                         options: *mut SquashOptions)
     -> *mut SquashFile;
    pub fn squash_file_open_codec_with_options(codec: *mut SquashCodec,
                                               filename:
                                                   *const c_char,
                                               mode: *const c_char,
                                               options: *mut SquashOptions)
     -> *mut SquashFile;
    pub fn squash_file_steal(codec: *const c_char, fp: *mut FILE, ...)
     -> *mut SquashFile;
    pub fn squash_file_steal_codec(codec: *mut SquashCodec,
                                   fp: *mut FILE, ...) -> *mut SquashFile;
    pub fn squash_file_steal_with_options(codec: *const c_char,
                                          fp: *mut FILE,
                                          options: *mut SquashOptions)
     -> *mut SquashFile;
    pub fn squash_file_steal_codec_with_options(codec: *mut SquashCodec,
                                                fp: *mut FILE,
                                                options: *mut SquashOptions)
     -> *mut SquashFile;
    pub fn squash_file_read(file: *mut SquashFile,
                            decompressed_size: *mut size_t,
                            decompressed: *mut uint8_t) -> SquashStatus;
    pub fn squash_file_write(file: *mut SquashFile,
                             uncompressed_size: size_t,
                             uncompressed: *const uint8_t) -> SquashStatus;
    pub fn squash_file_flush(file: *mut SquashFile) -> SquashStatus;
    pub fn squash_file_close(file: *mut SquashFile) -> SquashStatus;
    pub fn squash_file_free(file: *mut SquashFile, fp: *mut *mut FILE)
     -> SquashStatus;
    pub fn squash_file_eof(file: *mut SquashFile) -> bool;
    pub fn squash_file_error(file: *mut SquashFile) -> SquashStatus;
    pub fn squash_file_lock(file: *mut SquashFile);
    pub fn squash_file_unlock(file: *mut SquashFile);
    pub fn squash_file_read_unlocked(file: *mut SquashFile, decompressed_size: *mut size_t, decompressed: *mut uint8_t) -> SquashStatus;
    pub fn squash_file_write_unlocked(file: *mut SquashFile, uncompressed_size: size_t, uncompressed: *const uint8_t) -> SquashStatus;
    pub fn squash_file_flush_unlocked(file: *mut SquashFile) -> SquashStatus;
}
