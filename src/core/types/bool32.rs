// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_FALSE, VK_TRUE, VK_VERSION_1_0};

/// Vulkan boolean type
///
/// [`VK_TRUE`] represents a boolean True (unsigned integer 1) value, and [`VK_FALSE`] a boolean
/// False (unsigned integer 0) value.
///
/// All values returned from a Vulkan implementation in a [`VkBool32`] will be either [`VK_TRUE`]
/// or [`VK_FALSE`].
///
/// Applications must not pass any other values than [`VK_TRUE`] or [`VK_FALSE`] into a Vulkan
/// implementation where a [`VkBool32`] is expected.
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkBool32 = u32;
