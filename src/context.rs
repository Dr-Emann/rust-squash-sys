use libc::{c_char, c_void};

use codec::{SquashCodec, SquashCodecForeachFunc};
use plugin::{SquashPlugin, SquashPluginForeachFunc};

pub enum SquashContext { }

extern {
    pub fn squash_set_default_search_path(
        search_path: *const c_char);
    pub fn squash_context_get_default() -> *mut SquashContext;

    pub fn squash_context_get_codec(
        context: *mut SquashContext, codec: *const c_char) -> *mut SquashCodec;
    pub fn squash_context_get_codec_from_extension(
        context: *mut SquashContext, extension: *const c_char) -> *mut SquashCodec;
    pub fn squash_context_get_plugin(
        context: *mut SquashContext, plugin: *const c_char) -> *mut SquashPlugin;

    pub fn squash_context_foreach_plugin(
        context: *mut SquashContext,
        func: SquashPluginForeachFunc,
        data: *mut c_void);
    pub fn squash_context_foreach_codec(
        condext: *mut SquashContext,
        func: SquashCodecForeachFunc,
        data: *mut c_void);

    pub fn squash_get_codec(codec: *const c_char) -> *mut SquashCodec;
    pub fn squash_get_codec_from_extension(extension: *const c_char) -> *mut SquashCodec;
    pub fn squash_get_plugin(plugin: *const c_char) -> *mut SquashPlugin;

    pub fn squash_foreach_plugin(
        func: SquashPluginForeachFunc,
        data: *mut c_void);
    pub fn squash_foreach_codec(
        func: SquashCodecForeachFunc,
        data: *mut c_void);
}
