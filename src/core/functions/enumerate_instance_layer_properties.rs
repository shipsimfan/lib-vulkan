use crate::{VkLayerProperties, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Returns up to requested number of global layer properties
///
/// # Parameters
///  - `property_count` is a pointer to an integer related to the number of layer properties
///    available or queried, as described below.
///  - `properties` is either [`null_mut`] or a pointer to an array of [`VkLayerProperties`]
///    structures.
///
/// # Description
/// If `properties` is [`null_mut`], then the number of layer properties available is returned in
/// `property_count`. Otherwise, `property_count` must point to a variable set by the user to the
/// number of elements in the `properties` array, and on return the variable is overwritten with
/// the number of structures actually written to `properties`. If `property_count` is less than the
/// number of layer properties available, at most `property_count` structures will be written, and
/// [`VkResult::VkIncomplete`] will be returned instead of [`VkResult::VkSuccess`], to indicate
/// that not all the available properties were returned.
///
/// The list of available layers may change at any time due to actions outside of the Vulkan
/// implementation, so two calls to [`VkEnumerateInstanceLayerProperties`] with the same parameters
/// may return different results, or retrieve different `property_count` values or `properties`
/// contents. Once an instance has been created, the layers enabled for that instance will continue
/// to be enabled and valid for the lifetime of that instance, even if some of them become
/// unavailable for future instances.
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///  - [`VkResult::VkIncomplete`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkEnumerateInstanceLayerProperties =
    extern "system" fn(property_count: *mut u32, properties: *mut VkLayerProperties) -> VkResult;

/// The name of [`VkEnumerateInstanceLayerProperties`]
pub const VK_ENUMERATE_INSTANCE_LAYER_PROPERTIES: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkEnumerateInstanceLayerProperties\0") };
