use libc::c_void;

#[repr(C)]
pub struct SquashMemoryFuncs {
    pub malloc: Option<extern fn(size: usize) -> *mut c_void>,
    pub realloc: Option<extern fn(ptr: *mut c_void, size: usize) -> *mut c_void>,
    pub calloc: Option<extern fn(nmemb: usize, size: usize) -> *mut c_void>,
    pub free: Option<extern fn(ptr: *mut c_void)>,

    pub aligned_alloc: Option<extern fn(alignment: usize, size: usize) -> *mut c_void>,
    pub aligned_free: Option<extern fn(ptr: *mut c_void)>,
}

extern {
    pub fn squash_set_memory_functions(memfn: SquashMemoryFuncs);
    pub fn squash_malloc(size: usize) -> *mut c_void;
    pub fn squash_realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    pub fn squash_calloc(nmemb: usize, size: usize) -> *mut c_void;
    pub fn squash_free(ptr: *mut c_void);
    pub fn squash_aligned_alloc(alignment: usize, size: usize) -> *mut c_void;
    pub fn squash_aligned_free(ptr: *mut c_void);
}
