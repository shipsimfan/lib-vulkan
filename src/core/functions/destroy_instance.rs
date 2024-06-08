use crate::{VkAllocationCallbacks, VkInstance};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Destroy an instance of Vulkan
///
/// # Parameters
///  - `instance` is the handle of the instance to destroy.
///  - `allocator` controls host memory allocation.
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkDestroyInstance =
    extern "system" fn(instance: VkInstance, allocator: *const VkAllocationCallbacks);

/// The name of [`VkDestroyInstance`]
pub const VK_DESTROY_INSTANCE: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkDestroyInstance\0") };
