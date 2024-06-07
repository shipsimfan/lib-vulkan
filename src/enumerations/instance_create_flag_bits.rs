// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Bitmask specifying behavior of the instance
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkInstanceCreateFlagBits {
    /// [`VkInstanceCreateFlagBits::EnumeratePortabilityBitKHR`] specifies that the instance will
    /// enumerate available Vulkan Portability-compliant physical devices and groups in addition to
    /// the Vulkan physical devices and groups that are enumerated by default.
    ///
    /// Provided by [`VK_KHR_PORTABILITY_ENUMERATION`]
    EnumeratePortabilityBitKHR = 0x00000001,
}
