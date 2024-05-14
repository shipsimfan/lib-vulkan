use crate::VkResult;

/// Query instance-level version before instance creation
///
///  * `api_version` is a pointer to a [`u32`], which is the version of Vulkan supported by
///    instance-level functionality. This must be a valid pointer to a [`u32`] value.
///
/// # Return Codes
/// On success, this command returns
///  * [`VkResult::VkSuccess`]
///
/// On failure, this command returns
///  * [`VkResult::VkErrorOutOfHostMemory`]
pub type VkEnumerateInstanceVersion = extern "C" fn(api_version: *mut u32) -> VkResult;
