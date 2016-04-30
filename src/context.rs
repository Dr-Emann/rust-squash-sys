use libc::{c_char, c_void};

use codec::{SquashCodec, SquashCodecForeachFunc};
use plugin::{SquashPlugin, SquashPluginForeachFunc};

/// Context for all Squash operations.
pub enum SquashContext { }

#[cfg(windows)]
pub const SQUASH_SEARCH_PATH_SEPARATOR: char = ';';
#[cfg(not(windows))]
pub const SQUASH_SEARCH_PATH_SEPARATOR: char = ':';

extern {
    /// Set the default search path for plugins.
    ///
    /// This will change the default plugin search path to the value
    /// provided, overriding the value set at compile-time. Keep in mind
    /// that the `SQUASH_PLUGINS` environment variable has priority over
    /// the value set by this function.
    ///
    /// If called, this function must be called before any other function
    // in Squash (except for `squash_set_memory_functions`).
    ///
    /// # Parameters
    /// * `default_search_path` the new default search path
    pub fn squash_set_default_search_path(
        search_path: *const c_char);
    
    /// Retrieve the default `SquashContext`.
    ///
    /// If this is the first time calling this function, a new
    /// `SquashContext` will be created and Squash will attempt to scan the
    /// plugin directories for information.
    ///
    /// # Returns
    /// The SquashContext. Note that this is owned by Squash and must never
    /// be freed or unreffed.
    pub fn squash_context_get_default() -> *mut SquashContext;

    /// Retrieve a `SquashCodec` from a `SquashContext`.
    ///
    /// # Parameters
    /// * `context` The context to use.
    /// * `codec` Name of the codec to retrieve.
    ///
    /// # Returns
    /// The `SquashCodec`, or NULL on failure. This is owned by Squash and
    /// must never be freed or unreffed.
    pub fn squash_context_get_codec(
        context: *mut SquashContext, codec: *const c_char) -> *mut SquashCodec;
    
    ///Retrieve a codec from a context based on an extension.
    ///
    /// # Parameters
    /// * `context` The context
    /// * `extension` The extension
    ///
    /// # Returns
    /// A ref `SquashCodec` or NULL on failure
    pub fn squash_context_get_codec_from_extension(
        context: *mut SquashContext, extension: *const c_char) -> *mut SquashCodec;
    
    /// Retrieve a SquashPlugin from a `SquashContext`.
    ///
    /// # Parameters
    /// * `context` The context to use.
    /// * `plugin` Name of the plugin to retrieve.
    ///
    /// # Returns
    /// The `SquashPlugin`. This is owned by Squash and must never be freed
    /// or unreffed.
    pub fn squash_context_get_plugin(
        context: *mut SquashContext, plugin: *const c_char) -> *mut SquashPlugin;

    /// Execute a callback for every loaded plugin.
    ///
    /// # Parameters
    /// * `context` The context to use
    /// * `func` The callback to execute
    /// * `data` Data to pass to the callback
    pub fn squash_context_foreach_plugin(
        context: *mut SquashContext,
        func: SquashPluginForeachFunc,
        data: *mut c_void);
    
    /// Execute a callback for every loaded codec.
    ///
    /// # Note
    /// *If you have multiple plugins which supply a single codec,
    /// the callback will only be invoked for the highest-priority codec.
    /// If you would like to invoke a callback even when a higher priority
    /// codec exists, you can use `squash_context_foreach_plugin` to iterate
    /// through all the plugins and call `squash_plugin_foreach_codec` on each
    /// `SquashPlugin`.*
    ///
    /// # Parameters
    /// * `context`	The context to use
    /// * `func` The callback to execute
    /// * `data` Data to pass to the callback
    pub fn squash_context_foreach_codec(
        condext: *mut SquashContext,
        func: SquashCodecForeachFunc,
        data: *mut c_void);

    /// Retrieve a SquashCodec.
    ///
    /// # Parameters
    /// * `codec` Name of the codec to retrieve.
    ///
    /// # Returns
    /// The `SquashCodec`. This is owned by Squash and must never be freed
    /// or unreffed.
    pub fn squash_get_codec(codec: *const c_char) -> *mut SquashCodec;

    /// Retrieve a codec based on an extension.
    ///
    /// # Parameters
    /// * `extension` The extension
    ///
    /// # Returns
    /// A ref `SquashCodec` or NULL on failure
    pub fn squash_get_codec_from_extension(extension: *const c_char) -> *mut SquashCodec;
    
    /// Retrieve a SquashPlugin.
    ///
    /// # Parameters
    /// * `plugin` Name of the plugin to retrieve.
    ///
    /// # Returns
    /// The SquashPlugin. This is owned by Squash and must never be freed
    /// or unreffed.
    pub fn squash_get_plugin(plugin: *const c_char) -> *mut SquashPlugin;

    /// Execute a callback for every loaded plugin in the default context.
    ///
    /// # Parameters
    /// * `func` The callback to execute
    /// * `data` Data to pass to the callback
    pub fn squash_foreach_plugin(
        func: SquashPluginForeachFunc,
        data: *mut c_void);
    
    
    /// Execute a callback for every loaded codec in the default context.
    ///
    /// # Note
    /// *If you have multiple plugins which supply a single codec, the
    /// callback will only be invoked for the highest-priority codec.
    /// If you would like to invoke a callback even when a higher priority
    /// codec exists, you can use `squash_foreach_plugin` to iterate through
    /// all the plugins and call `squash_plugin_foreach_codec` on each
    /// `SquashPlugin`.*
    ///
    /// # Parameters
    /// * `func` The callback to execute
    /// * `data` Data to pass to the callback
    pub fn squash_foreach_codec(
        func: SquashCodecForeachFunc,
        data: *mut c_void);
}
