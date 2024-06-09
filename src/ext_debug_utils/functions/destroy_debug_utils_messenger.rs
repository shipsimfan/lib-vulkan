use crate::{VkAllocationCallbacks, VkDebugUtilsMessengerEXT, VkInstance};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

/// Destroy a debug messenger object
///
/// # Parameters
///  - `instance` is the instance where the callback was created.
///  - messenger is the [`VkDebugUtilsMessengerEXT`] object to destroy. messenger is an externally
///    synchronized object and must not be used on more than one thread at a time. This means that
///    [`VkDestroyDebugUtilsMessengerEXT`] must not be called when a callback is active.
///  - `allocator` controls host memory allocation.
///
/// # Description
/// The application must ensure that [`VkDestroyDebugUtilsMessengerEXT`] is not executed in
/// parallel with any Vulkan command that is also called with instance or child of instance as the
/// dispatchable argument.
///
/// Provided by [`ext_debug_utils`]
pub type VkDestroyDebugUtilsMessengerEXT = extern "system" fn(
    instance: VkInstance,
    messenger: VkDebugUtilsMessengerEXT,
    allocator: *const VkAllocationCallbacks,
);

/// The name of [`VkDestroyDebugUtilsMessengerEXT`]
pub const VK_DESTROY_DEBUG_UTILS_MESSENGER_EXT: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkDestroyDebugUtilsMessengerEXT\0") };
