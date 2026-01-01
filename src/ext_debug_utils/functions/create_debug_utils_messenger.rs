use crate::{
    VkAllocationCallbacks, VkInstance, VkResult,
    ext_debug_utils::{VkDebugUtilsMessengerCreateInfoExt, VkDebugUtilsMessengerExt},
};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;
#[allow(unused_imports)]
use std::ptr::null;

/// Create a debug messenger object
///
/// # Parameters
///  - `instance` is the instance the messenger will be used with.
///  - `create_info` is a pointer to a [`VkDebugUtilsMessengerCreateInfoExt`] structure containing
///    the callback pointer, as well as defining conditions under which this messenger will trigger
///    the callback.
///  - `allocator` controls host memory allocation.
///  - `messenger` is a pointer to a [`VkDebugUtilsMessengerExt`] handle in which the created
///    object is returned.
///
/// # Description
/// The application must ensure that [`VkCreateDebugUtilsMessengerEXT`] is not executed in parallel
/// with any Vulkan command that is also called with instance or child of instance as the
/// dispatchable argument.
///
/// # Valid Usage (Implicit)
///  - `instance` must be a valid [`VkInstance`] handle
///  - `create_info` must be a valid pointer to a valid [`VkDebugUtilsMessengerCreateInfoExt`]
///    structure
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - `messenger` must be a valid pointer to a [`VkDebugUtilsMessengerExt`] handle
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
/// Provided by [`ext_debug_utils`]
pub type VkCreateDebugUtilsMessengerExt = extern "system" fn(
    instance: VkInstance,
    create_info: *const VkDebugUtilsMessengerCreateInfoExt,
    allocator: *const VkAllocationCallbacks,
    messenger: *mut VkDebugUtilsMessengerExt,
) -> VkResult;

/// The name of [`VkCreateDebugUtilsMessengerExt`]
pub const VK_CREATE_DEBUG_UTILS_MESSENGER_EXT: &CStr = c"vkCreateDebugUtilsMessengerEXT";
