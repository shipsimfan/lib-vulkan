use crate::{VkAllocationCallbacks, VkDevice};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

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
/// Provided by [`VK_VERSION_1_0`]
pub type VkDestroyDevice =
    extern "system" fn(device: VkDevice, allocator: *const VkAllocationCallbacks);

/// The name of [`VkCreateInstance`]
pub const VK_DESTROY_DEVICE: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkDestroyDevice\0") };
