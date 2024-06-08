use crate::{VkAllocationCallbacks, VkInstance, VkInstanceCreateInfo, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Create a new Vulkan instance
///
/// # Parameters
///  - `create_info` is a pointer to a [`VkInstanceCreateInfo`] structure controlling creation of
///    the instance.
///  - `allocator` controls host memory allocation.
///  - `instance` points a [`VkInstance`] handle in which the resulting instance is returned.
///
/// # Description
/// [`VkCreateInstance`] verifies that the requested layers exist. If not, [`VkCreateInstance`]
/// will return [`VkResult::VkErrorLayerNotPresent`]. Next [`VkCreateInstance`] verifies that the
/// requested extensions are supported (e.g. in the implementation or in any enabled instance
/// layer) and if any requested extension is not supported, [`VkCreateInstance`] must return
/// [`VkResult::VkErrorExtensionNotPresent`]. After verifying and enabling the instance layers and
/// extensions the [`VkInstance`] object is created and returned to the application. If a requested
/// extension is only supported by a layer, both the layer and the extension need to be specified
/// at [`VkCreateInstance`] time for the creation to succeed.
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorInitializationFailed`]
///  - [`VkResult::VkErrorLayerNotPresent`]
///  - [`VkResult::VkErrorExtensionNotPresent`]
///  - [`VkResult::VkErrorIncompatibleDriver`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCreateInstance = extern "system" fn(
    create_info: *const VkInstanceCreateInfo,
    allocator: *const VkAllocationCallbacks,
    instance: *mut VkInstance,
) -> VkResult;

/// The name of [`VkCreateInstance`]
pub const VK_CREATE_INSTANCE: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkCreateInstance\0") };
