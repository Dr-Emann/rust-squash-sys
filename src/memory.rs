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
    /// Set memory management functions
    ///
    /// The `aligned_alloc` and `aligned_free` functions may be NULL,
    /// as well as either `malloc` or `calloc`. Other callbacks require a
    /// value.
    ///
    /// # Note
    /// *If you choose to call this function then you must do so before any
    /// other function in the Squash, or your program will likely crash
    /// (due to attempting to free a buffer allocated with the standard
    /// allocator using your non-standard free function).*
    /// 
    /// While Squash itself does not call other memory management functions
    /// (such as malloc and free) directly, we can't make any promises about
    /// plugins or third-party libraries. We try to make sure as many as
    /// possible support custom memory management functions (often filing
    /// bugs and patches upstream), but it is unlikely we will ever reach
    /// 100% coverage.
    ///
    /// # Parameters
    /// * `memfn` Functions to use to manage memory
    pub fn squash_set_memory_functions(memfn: SquashMemoryFuncs);
    pub fn squash_malloc(size: usize) -> *mut c_void;
    pub fn squash_realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    pub fn squash_calloc(nmemb: usize, size: usize) -> *mut c_void;
    pub fn squash_free(ptr: *mut c_void);
    
    /// Allocate an aligned buffer
    ///
    /// Memory allocated with this function is assumed not to support
    /// reallocation. In reality, assuming nobody has installed thick
    /// wrappers it should be possible to squash_realloc the buffer, but
    /// the result is not constrained to the alignment requirements
    /// presented to the initial buffer.
    ///
    /// The value returned by this function must be freed with
    /// [`squash_aligned_free`]; While some implementations (such as C11's
    /// aligned_alloc and the POSIX posix_memalign function) allow values
    /// returned by `squash_aligned_alloc` to be passed directly to
    /// [`squash_free`], others (such as Windows' `_aligned_malloc`) do not.
    /// Passing the result of this function to [`squash_free`] is considered
    /// undefined behavior.
    ///
    /// # Note
    /// Values supported for the alignment parameter are implementation
    /// defined, but a fair assumption is that they must be a power of two
    /// and multiple of sizeof(void*).
    ///
    /// # Parameters
    /// * `ctx` The context
    /// * `alignment` Alignment of the buffer
    /// * `size` Number of bytes to allocate
    ///
    /// [`squash_aligned_free`]: ./fn.squash_aligned_free.html
    /// [`squash_free`]: ./fn.squash_free.html
    pub fn squash_aligned_alloc(alignment: usize, size: usize) -> *mut c_void;
    
    /// Deallocate an aligned buffer
    ///
    /// # Parameters
    /// * `ctx` The context
    /// * `ptr` Buffer to deallocate
    pub fn squash_aligned_free(ptr: *mut c_void);
}
