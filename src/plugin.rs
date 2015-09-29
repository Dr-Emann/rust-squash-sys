use libc::{c_void, c_char};

use status::SquashStatus;
use codec::{SquashCodec, SquashCodecForeachFunc};
use license::SquashLicense;

pub enum SquashPlugin { }

pub type SquashPluginForeachFunc = Option<extern fn(*mut SquashPlugin, *mut c_void)>;

extern {
    pub fn squash_plugin_init(plugin: *mut SquashPlugin) -> SquashStatus;
    pub fn squash_plugin_get_name(plugin: *mut SquashPlugin) -> *const c_char;
    pub fn squash_plugin_get_licences(plugin: *mut SquashPlugin) -> *mut SquashLicense;
    pub fn squash_plugin_get_codec(plugin: *mut SquashPlugin, codec: *const c_char) -> *mut SquashCodec;
    pub fn squash_plugin_foreach_codec(plugin: *mut SquashPlugin, func: SquashCodecForeachFunc, data: *mut c_void);
}
