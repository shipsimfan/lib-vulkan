use crate::{
    VkAllocationCallbacks, VkDebugUtilsMessengerCreateInfoEXT, VkDebugUtilsMessengerEXT,
    VkInstance, VkResult,
};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

/// Create a debug messenger object
///
/// # Parameters
///  - `instance` is the instance the messenger will be used with.
///  - `create_info` is a pointer to a [`VkDebugUtilsMessengerCreateInfoEXT`] structure containing
///    the callback pointer, as well as defining conditions under which this messenger will trigger
///    the callback.
///  - `allocator` controls host memory allocation.
///  - `messenger` is a pointer to a [`VkDebugUtilsMessengerEXT`] handle in which the created
///    object is returned.
///
/// # Description
/// The application must ensure that [`VkCreateDebugUtilsMessengerEXT`] is not executed in parallel
/// with any Vulkan command that is also called with instance or child of instance as the
/// dispatchable argument.
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///
/// Provided by [`ext_debug_utils`]
pub type VkCreateDebugUtilsMessengerEXT = extern "system" fn(
    instance: VkInstance,
    create_info: *const VkDebugUtilsMessengerCreateInfoEXT,
    allocator: *const VkAllocationCallbacks,
    messenger: *mut VkDebugUtilsMessengerEXT,
) -> VkResult;

/// The name of [`VkCreateDebugUtilsMessengerEXT`]
pub const VK_CREATE_DEBUG_UTILS_MESSENGER_EXT: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkCreateDebugUtilsMessengerEXT\0") };
