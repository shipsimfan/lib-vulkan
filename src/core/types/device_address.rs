// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Vulkan device address type
///
/// # Valid Usage
///  - A valid [`VkDeviceAddress`] must be equal to the sum of an address retrieved from a
///    [`VkBuffer`] via [`VkGetBufferDeviceAddress`], and any offset in the range `[0, size)`,
///    where `size` is the value of [`VkBufferCreateInfo::size`] used to create that [`VkBuffer`]
///  - If a [`VkDeviceAddress`] was retrieved from a non-sparse buffer, that buffer must be bound
///    completely and contiguously to a single [`VkDeviceMemory`] object
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkDeviceAddress = u64;
