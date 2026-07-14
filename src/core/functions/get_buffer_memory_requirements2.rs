use crate::{VkBufferMemoryRequirementsInfo2, VkDevice, VkMemoryRequirements2};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_1;

/// Returns the memory requirements for specified Vulkan object
///
/// # Parameters
///  - `device` is the logical device that owns the buffer.
///  - `info` is a pointer to a [`VkBufferMemoryRequirementsInfo2`] structure containing parameters
///    required for the memory requirements query.
///  - `memory_requirements` is a pointer to a [`VkMemoryRequirements2`] structure in which the
///    memory requirements of the buffer object are returned.
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `info` must be a valid pointer to a valid [`VkBufferMemoryRequirementsInfo2`] structure
///  - `memory_requirements` must be a valid pointer to a [`VkMemoryRequirements2`] structure
///
/// Provided by [`VK_VERSION_1_1`]
pub type VkGetBufferMemoryRequirements2 = unsafe extern "system" fn(
    device: VkDevice,
    info: *const VkBufferMemoryRequirementsInfo2,
    memory_requirements: *mut VkMemoryRequirements2,
);

/// The name of [`VkGetBufferMemoryRequirements2`]
pub const VK_GET_BUFFER_MEMORY_REQUIREMENTS2: &CStr = c"vkGetBufferMemoryRequirements2";
