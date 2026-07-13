use crate::{VkAllocationCallbacks, VkBuffer, VkBufferCreateInfo, VkDevice, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Provided by [`VK_VERSION_1_0`]
pub type VkCreateBuffer = unsafe extern "system" fn(
    device: VkDevice,
    create_info: *const VkBufferCreateInfo,
    allocator: *const VkAllocationCallbacks,
    buffer: *mut VkBuffer,
) -> VkResult;

/// The name of [`VkCreateBuffer`]
pub const VK_CREATE_BUFFER: &CStr = c"vkCreateBuffer";
