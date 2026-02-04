use crate::{VkAllocationCallbacks, VkDevice};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkQueue};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

/// Destroy a logical device
///
/// # Parameters
///  - `device` is the logical device to destroy.
///  - `allocator` controls host memory allocation
///
/// # Description
/// To ensure that no work is active on the device, [`VkDeviceWaitIdle`] can be used to gate the
/// destruction of the device. Prior to destroying a device, an application is responsible for
/// destroying/freeing any Vulkan objects that were created using that device as the first
/// parameter of the corresponding `VkCreate*` or `VkAllocate*` command.
///
/// # Valid Usage
///  - All child objects created on `device` that can be destroyed or freed must have been
///    destroyed or freed prior to destroying `device`
///  - If [`VkAllocationCallbacks`] were provided when `device` was created, a compatible set of
///    callbacks must be provided here
///  - If no [`VkAllocationCallbacks`] were provided when `device` was created, `allocator` must be
///    [`null`]
///
/// # Valid Usage (Implicit)
///  - If `device` is not [`null_mut`], `device` must be a valid [`VkDevice`] handle
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///
/// # Host Synchronization
///  - Host access to `device` must be externally synchronized
///  - Host access to all [`VkQueue`] objects created from `device` must be externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkDestroyDevice =
    unsafe extern "system" fn(device: VkDevice, allocator: *const VkAllocationCallbacks);

/// The name of [`VkCreateInstance`]
pub const VK_DESTROY_DEVICE: &CStr = c"vkDestroyDevice";
