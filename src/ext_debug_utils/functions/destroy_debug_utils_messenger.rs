use crate::{VkAllocationCallbacks, VkInstance, ext_debug_utils::VkDebugUtilsMessengerExt};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, ext_debug_utils};
#[allow(unused_imports)]
use std::ptr::null;

/// Destroy a debug messenger object
///
/// # Parameters
///  - `instance` is the instance where the callback was created.
///  - messenger is the [`VkDebugUtilsMessengerExt`] object to destroy. messenger is an externally
///    synchronized object and must not be used on more than one thread at a time. This means that
///    [`VkDestroyDebugUtilsMessengerExt`] must not be called when a callback is active.
///  - `allocator` controls host memory allocation.
///
/// # Description
/// The application must ensure that [`VkDestroyDebugUtilsMessengerExt`] is not executed in
/// parallel with any Vulkan command that is also called with instance or child of instance as the
/// dispatchable argument.
///
/// # Valid Usage
///  - If [`VkAllocationCallbacks`] were provided when `messenger` was created, a compatible set of
///    callbacks must be provided here
///  - If no [`VkAllocationCallbacks`] were provided when messenger was created, `allocator` must
///    be [`null`]
///
/// # Valid Usage (Implicit)
///  - `instance` must be a valid [`VkInstance`] handle
///  - If `messenger` is not [`VK_NULL_HANDLE`], `messenger` must be a valid
///    [`VkDebugUtilsMessengerExt`] handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - If `messenger` is a valid handle, it must have been created, allocated, or retrieved from
///    `instance`
///
/// # Host Synchronization
///  - Host access to `messenger` must be externally synchronized
///
/// Provided by [`ext_debug_utils`]
pub type VkDestroyDebugUtilsMessengerExt = unsafe extern "system" fn(
    instance: VkInstance,
    messenger: VkDebugUtilsMessengerExt,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroyDebugUtilsMessengerExt`]
pub const VK_DESTROY_DEBUG_UTILS_MESSENGER_EXT: &CStr = c"vkDestroyDebugUtilsMessengerEXT";
