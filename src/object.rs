use libc::{c_int, c_uint, c_void};

/// Callback to be invoked when information data is no longer needed.
///
/// When you are not subclassing `SquashObject`, `SquashDestroyNotify` is used
/// almost exclusively for memory management, most simply by passing `free()`.
pub type SquashDestroyNotify = Option<extern fn(*mut c_void)>;

/// Reference-counting base class for other types.
#[repr(C)]
pub struct SquashObject {
    /// The reference count.
    pub ref_count: c_uint,
    
    /// Whether or not the object has a floating reference. 
    pub is_floating: c_int,
    
    /// Function to call when the reference count reaches 0. 
    pub destroy_notify: SquashDestroyNotify,
}

extern {
    /// Increment the reference count on an object.
    ///
    /// # Parameters
    /// * `obj` The object to increase the reference count of.
    ///
    /// # Returns
    /// The object which was passed in.
    pub fn squash_object_ref(obj: *mut c_void) -> *mut c_void;
    
    /// Decrement the reference count on an object.
    ///
    /// Once the reference count reaches 0 the object will be freed.
    ///
    /// # Parameters
    /// * `obj` The object to decrease the reference count of.
    ///
    /// # Returns
    /// NULL
    pub fn squash_object_unref(obj: *mut c_void) -> *mut c_void;
    
    /// Get the current reference count of an object.
    ///
    /// # Parameters
    /// * `obj` The object in question.
    ///
    /// # Returns
    /// The reference count of obj.
    pub fn squash_object_get_ref_count(obj: *mut c_void) -> c_uint;

    /// Sink a floating reference if one exists.
    /// 
    /// If a floating reference exists on the object, sink it.
    ///
    /// For a description of how floating references work, see GObject's
    /// documentation of the concept. The implementation here is different,
    /// but the concept remains the same.
    ///
    /// # Parameters
    /// * `obj` The object to sink the floating reference on.
    ///
    /// # Returns
    /// The object which was passed in.
    pub fn squash_object_ref_sink(obj: *mut c_void) -> *mut c_void;

    /// Initialize a new object.
    ///
    /// # Warning
    /// **This function must only be used to implement a subclass of
    /// `SquashObject`. Objects returned by *_new functions will already be
    /// initialized, and you must not call this function on them; doing so
    /// will likely trigger a memory leak.**
    ///
    /// # Parameters
    /// * `obj` The object to initialize.
    /// * `is_floating` Whether or not the object's reference is floating
    /// * `destroy_notify` Function to call when the reference count reaches 0
    pub fn squash_object_init(obj: *mut c_void, is_floating: bool, destroy_notify: SquashDestroyNotify);
    
    /// Destroy an object.
    ///
    /// # Warning
    /// **This function must only be used to implement a subclass of
    /// `SquashObject`. Each subclass should implement a *_destroy function
    /// which should perform any operations needed to destroy their own data
    /// and chain up to the *_destroy function of the base class, eventually
    /// invoking [`squash_object_destroy`]. Invoking this function in any
    /// other context is likely to cause a memory leak or crash.**
    ///
    /// # Parameters
    /// * `obj` The object to destroy.
    ///
    /// [`squash_object_destroy`]: ./fn.squash_object_destroy.html
    pub fn squash_object_destroy(obj: *mut c_void);
}
