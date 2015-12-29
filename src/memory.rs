use context::SquashContext;
use libc::c_void;

#[repr(C)]
pub struct SquashMemoryFuncs {
    pub malloc: Option<extern fn(size: usize) -> *mut c_void>,
    pub realloc: Option<extern fn(ptr: *mut c_void, size: usize) -> *mut c_void>,
    pub free: Option<extern fn(ptr: *mut c_void)>,

    pub aligned_alloc: Option<extern fn(alignment: usize, size: usize) -> *mut c_void>,
    pub aligned_free: Option<extern fn(ptr: *mut c_void)>,
}

extern {
    pub fn squash_set_memory_functions(memfn: SquashMemoryFuncs);
    pub fn squash_malloc(ctx: *mut SquashContext, size: usize) -> *mut c_void;
    pub fn squash_realloc(ctx: *mut SquashContext, ptr: *mut c_void, size: usize) -> *mut c_void;
    pub fn squash_free(ctx: *mut SquashContext, ptr: *mut c_void);
    pub fn squash_aligned_alloc(
        ctx: *mut SquashContext,
        alignment: usize,
        size: usize) -> *mut c_void;
    pub fn squash_aligned_free(ctx: *mut SquashContext, ptr: *mut c_void);
}
