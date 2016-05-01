use libc::{c_void, c_char};

use status::SquashStatus;
use codec::{SquashCodec, SquashCodecForeachFunc};
use license::SquashLicense;

/// A plugin.
pub enum SquashPlugin { }

/// Callback to be invoked on each SquashPlugin in a set.
///
/// # Parameters
/// * `plugin` A plugin
/// * `data` User-supplied data
pub type SquashPluginForeachFunc = Option<extern fn(*mut SquashPlugin, *mut c_void)>;

extern {
    /// Load a SquashPlugin
    ///
    /// # Note
    /// *This function is generally only useful inside of a callback passed to
    /// squash_foreach_plugin. Every other way to get a plugin (such as
    /// [`squash_get_plugin`]) will initialize the plugin as well (and return
    /// NULL instead of the plugin if initialization fails). The foreach
    /// functions, however, do not initialize the plugin since doing so
    /// requires actually loading the plugin.*
    ///
    /// # Parameters
    /// * `plugin` The plugin to load.
    ///
    /// # Returns
    /// A status code.
    ///
    /// # Return values
    /// * `SQUASH_OK` The plugin has been loaded.
    /// * `SQUASH_UNABLE_TO_LOAD` Unable to load plugin.
    pub fn squash_plugin_init(plugin: *mut SquashPlugin) -> SquashStatus;
    
    /// Get the name of a plugin.
    /// # Parameters
    /// * `plugin` The plugin.
    ///
    /// # Returns
    /// The name.
    pub fn squash_plugin_get_name(plugin: *mut SquashPlugin) -> *const c_char;
    
    /// Get the licenses of the plugin.
    ///
    /// # Parameters
    /// * `plugin` The plugin.
    ///
    /// # Returns
    /// An array of the plugin's licenses terminated with
    /// `SQUASH_LICENSE_UNKNOWN`, or NULL if no licenses were specified.
    pub fn squash_plugin_get_licences(plugin: *mut SquashPlugin) -> *mut SquashLicense;
    
    /// Get a codec from a plugin by name.
    ///
    /// # Parameters
    /// * `plugin` The plugin.
    /// * `codec` The codec name.
    /// # Returns
    /// The codec, or NULL if it could not be found.
    pub fn squash_plugin_get_codec(plugin: *mut SquashPlugin, codec: *const c_char) -> *mut SquashCodec;

    /// Execute a callback for every codec in the plugin.
    /// # Note
    /// *`func` will be invoked for all codecs supplied by this plugin, even if
    /// a higher-priority implementation exists in another plugin. If you only
    /// want to list the codecs which supply the highest-priority
    /// implementations available, you can use [`squash_foreach_codec`]. If
    /// not jumping around the hierarchy is important, you can test to see if
    /// a codec provides the highest priority implementation by comparing the
    /// codec to the return value of [`squash_get_codec`].*
    ///
    /// # Parameters
    /// * `plugin`  The plugin
    /// * `func`    The callback to execute
    /// * `data`    Data to pass to the callback
    ///
    /// [`squash_foreach_codec`]: ./fn.squash_foreach_codec.html
    /// [`squash_foreach_codec`]: ./fn.squash_foreach_codec.html
    pub fn squash_plugin_foreach_codec(plugin: *mut SquashPlugin,
                                       func: SquashCodecForeachFunc,
                                       data: *mut c_void);
}
