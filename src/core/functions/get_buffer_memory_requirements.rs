use crate::{VkBuffer, VkDevice, VkMemoryRequirements};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Returns the memory requirements for specified Vulkan object
///
/// # Parameters
///  - `device` is the logical device that owns the buffer.
///  - `buffer` is the buffer to query.
///  - `memory_requirements` is a pointer to a [`VkMemoryRequirements`] structure in which the
///    memory requirements of the buffer object are returned.
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `buffer` must be a valid [`VkBuffer`] handle
///  - `memory_requirements` must be a valid pointer to a [`VkMemoryRequirements`] structure
///  - `buffer` must have been created, allocated, or retrieved from `device`
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkGetBufferMemoryRequirements = unsafe extern "system" fn(
    device: VkDevice,
    buffer: VkBuffer,
    memory_requirements: *mut VkMemoryRequirements,
);

/// The name of [`VkGetBufferMemoryRequirements`]
pub const VK_GET_BUFFER_MEMORY_REQUIREMENTS: &CStr = c"vkGetBufferMemoryRequirements";
