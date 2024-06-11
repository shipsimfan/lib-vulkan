// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_1;

/// Bitmask specifying behavior of the queue
///
/// Provided by [`VK_VERSION_1_1`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkDeviceQueueCreateFlagBits {
    /// [`VkDeviceQueueCreateFlagBits::ProtectedBit`] specifies that the device queue is a
    /// protected-capable queue.
    ///
    /// Provided by [`VK_VERSION_1_1`]
    ProtectedBit = 0x00000001,
}
