use crate::{VkAllocationCallbacks, VkInstance};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkPhysicalDevice};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

/// Destroy an instance of Vulkan
///
/// # Parameters
///  - `instance` is the handle of the instance to destroy.
///  - `allocator` controls host memory allocation.
///
/// # Valid Usage
///  - All child objects that were created with `instance` or with a [`VkPhysicalDevice`] retrieved
///    from it, and that can be destroyed or freed, must have been destroyed or freed prior to
///    destroying `instance`
///  - If [`VkAllocationCallbacks`] were provided when `instance` was created, a compatible set of
///    callbacks must be provided here
///  - If no [`VkAllocationCallbacks`] were provided when `instance` was created, `allocator` must
///    be [`null`]
///
/// # Valid Usage (Implicit)
///  - If `instance` is not [`null_mut`], `instance` must be a valid [`VkInstance`] handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///
/// # Host Synchronization
///  - Host access to `instance` must be externally synchronized
///  - Host access to all [`VkPhysicalDevice`] objects enumerated from `instance` must be
///    externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkDestroyInstance =
    extern "system" fn(instance: VkInstance, allocator: *const VkAllocationCallbacks);

/// The name of [`VkDestroyInstance`]
pub const VK_DESTROY_INSTANCE: &CStr = c"vkDestroyInstance";
