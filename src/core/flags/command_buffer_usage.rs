use crate::macros::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

flags! {
    /// Bitmask of [`VkCommandBufferUsageFlag`]s
    ///
    /// # Description
    /// [`VkCommandBufferUsageFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkCommandBufferUsageFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkCommandBufferUsageFlags;

    /// Bitmask specifying usage behavior for command buffer
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkCommandBufferUsageFlag {
        /// [`VkCommandBufferUsageFlag::OneTimeSubmitBit`] specifies that each recording of the
        /// command buffer will only be submitted once, and the command buffer will be reset and
        /// recorded again between each submission.
        OneTimeSubmitBit = 0x00000001,

        /// [`VkCommandBufferUsageFlag::RenderPassContinueBit`] specifies that a secondary command
        /// buffer is considered to be entirely inside a render pass. If this is a primary command
        /// buffer, then this bit is ignored.
        RenderPassContinueBit = 0x00000002,

        /// [`VkCommandBufferUsageFlag::SimultaneousUseBit`] specifies that a command buffer can be
        /// resubmitted to any queue of the same queue family while it is in the pending state, and
        /// recorded into multiple primary command buffers.
        SimultaneousUseBit = 0x00000004,
    }
}
