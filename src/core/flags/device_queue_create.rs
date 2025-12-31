use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_1};

flags! {
    /// Bitmask of [`VkDeviceQueueCreateFlag`]
    ///
    /// # Description
    /// [`VkDeviceQueueCreateFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkDeviceQueueCreateFlag`].
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkDeviceQueueCreateFlags;

    /// Bitmask specifying behavior of the queue
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkDeviceQueueCreateFlag {
        /// [`VkDeviceQueueCreateFlag::ProtectedBit`] specifies that the device queue is a
        /// protected-capable queue.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        ProtectedBit = 0x00000001,
    }
}
