use libc::{c_uint, c_void};

pub type SquashDestroyNotify = Option<extern fn(*mut c_void)>;

#[repr(C)]
pub struct SquashObject {
    pub ref_count: c_uint,
    pub is_floating: bool,
    pub destroy_notify: SquashDestroyNotify,
}

extern {
    pub fn squash_object_ref(obj: *mut c_void) -> *mut c_void;
    pub fn squash_object_unref(obj: *mut c_void) -> *mut c_void;
    pub fn squash_object_get_ref_count(obj: *mut c_void) -> c_uint;
    pub fn squash_object_ref_sink(obj: *mut c_void) -> *mut c_void;

    pub fn squash_object_init(obj: *mut c_void, is_floating: bool, destroy_notify: SquashDestroyNotify);
    pub fn squash_object_destroy(obj: *mut c_void);
}
