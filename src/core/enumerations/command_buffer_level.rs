// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Enumerant specifying a command buffer level
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkCommandBufferLevel {
    /// [`VkCommandBufferLevel::Primary`] specifies a primary command buffer.
    Primary = 0,

    /// [`VkCommandBufferLevel::Secondary`] specifies a secondary command buffer.
    Secondary = 1,
}
