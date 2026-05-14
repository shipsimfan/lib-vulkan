use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_1};

flags! {
    /// Bitmask of [`VkCommandPoolCreateFlag`]s
    ///
    /// # Description
    /// [`VkCommandPoolCreateFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkCommandPoolCreateFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkCommandPoolCreateFlags;

    /// Bitmask specifying usage behavior for a command pool
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkCommandPoolCreateFlag {
        /// [`VkCommandPoolCreateFlag::Transient`] specifies that command buffers allocated from
        /// the pool will be short-lived, meaning that they will be reset or freed in a relatively
        /// short timeframe. This flag may be used by the implementation to control memory
        /// allocation behavior within the pool.
        Transient = 0x00000001,

        /// [`VkCommandPoolCreateFlag::ResetCommandBuffer`] allows any command buffer allocated
        /// from a pool to be individually reset to the initial state; either by calling
        /// [`VkResetCommandBuffer`], or via the implicit reset when calling
        /// [`VkBeginCommandBuffer`]. If this flag is not set on a pool, then
        /// [`VkResetCommandBuffer`] must not be called for any command buffer allocated from that
        /// pool.
        ResetCommandBuffer = 0x00000002,

        /// [`VkCommandPoolCreateFlag::Protected`] specifies that command buffers
        /// allocated from the pool are protected command buffers.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        Protected = 0x00000004,
    }
}
