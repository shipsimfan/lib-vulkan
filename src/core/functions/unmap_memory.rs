use crate::{VkDevice, VkDeviceMemory};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Unmap a previously mapped memory object
///
/// # Parameters
///  - `device` is the logical device that owns the memory.
///  - `memory` is the memory object to be unmapped.
///
/// # Description
/// Calling [`VkUnmapMemory`] is equivalent to calling [`VkUnmapMemory2`] with an empty `next`
/// chain and `flags` set to zero.
///
/// # Valid Usage
///  - `memory` must be currently host mapped
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `memory` must be a valid [`VkDeviceMemory`] handle
///  - `memory` must have been created, allocated, or retrieved from `device`
///
/// # Host Synchronization
///  - Host access to `memory` must be externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkUnmapMemory = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory);

/// The name of [`VkUnmapMemory`]
pub const VK_UNMAP_MEMORY: &CStr = c"vkUnmapMemory";
