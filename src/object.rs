use libc::{c_int, c_uint, c_void};

/// Callback to be invoked when information data is no longer needed.
///
/// When you are not subclassing `SquashObject`, `SquashDestroyNotify` is used
/// almost exclusively for memory management, most simply by passing `free()`.
pub type SquashDestroyNotify = Option<extern fn(*mut c_void)>;

/// Reference-counting base class for other types.
///
/// `SquashObject` is designed to provide a
/// lightweight reference-counted type which can
/// be used as a base class for other types in
/// Squash.
///
/// # Subclassing
/// Subclassing `SquashObject` is relatively
/// straightforward. The first step is to embed
/// `SquashObject` in your object. Assuming you're
/// inheriting directly from `SquashObject`, your
/// code would look something like this:
///
/// ```
/// use squash_sys::*;
/// #[repr(C)]
/// struct MyObject {
///     base_object: SquashObject,
///     value: *mut u32,
/// }
/// ```
///
/// If you are subclassing another type (which
/// inherits, possibly indirectly, from
/// `SquashObject`) then you should use that type
/// instead.
///
// Next, you should to create an *_init function
/// which takes an existing instance of your class,
/// chains up to the *_init function provided by
/// your base class, and initializes any fields
/// you want initialized:
///
/// ```
/// use std::mem;
/// use squash_sys::*;
/// # #[repr(C)]
/// # struct MyObject {
/// #     base_object: SquashObject,
/// #     value: *mut u32,
/// # }
/// extern fn my_object_init(obj: *mut MyObject,
///                          value: u32,
///                          destroy_notify: SquashDestroyNotify) {
///     unsafe {
///         squash_object_init(obj as *mut _, false, destroy_notify);
///         (*obj).value = squash_malloc(mem::size_of::<u32>()) as *mut u32;
///         (*(*obj).value) = value;
///     }
/// }
/// ```
///
/// Of course, whatever is created must be destroyed,so you'll also want to
/// create a *_destroy method to be called when the reference count reaches
/// 0. Destroy any of your fields first, then chain up to the base class'
/// *_destroy function:
///
/// ```
/// use squash_sys::*;
/// # #[repr(C)]
/// # struct MyObject {
/// #     base_object: SquashObject,
/// #     value: *mut u32,
/// # }
/// extern fn my_object_destroy(obj: *mut MyObject) {
///     unsafe {
///         if (*obj).value.is_null() {
///             squash_free((*obj).value as *mut _)
///         }
///         squash_object_destroy(obj as *mut _);
///     }
/// }
/// ```
///
/// If your class is not abstract (it is meant to be instantiated, not just
/// subclassed), you should create a constructor:
///
/// ```
/// # extern crate squash_sys;
/// extern crate libc;
/// use squash_sys::*;
/// use std::mem;
/// # fn main() {}
/// # #[repr(C)]
/// # struct MyObject {
/// #     base_object: SquashObject,
/// #     value: *mut u32,
/// # }
/// # extern fn my_object_init(obj: *mut MyObject,
/// #                          value: u32,
/// #                          destroy_notify: SquashDestroyNotify) {
/// #     unsafe {
/// #         squash_object_init(obj as *mut _, false, destroy_notify);
/// #         (*obj).value = squash_malloc(mem::size_of::<u32>()) as *mut u32;
/// #         (*(*obj).value) = value;
/// #     }
/// # }
/// # extern fn my_object_free(obj: *mut libc::c_void) { }
/// extern fn my_object_new(value: u32) -> *mut MyObject {
///      unsafe {
///          let obj = squash_malloc(mem::size_of::<MyObject>()) as *mut MyObject;
///          my_object_init(obj, value, Some(my_object_free));
///          obj
///      }
/// }
/// ```
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
