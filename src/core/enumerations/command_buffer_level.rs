/// Enumerant specifying a command buffer level
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkCommandBufferLevel {
    /// [`VkCommandBufferLevel::Primary`] specifies a primary command buffer.
    Primary = 0,

    /// [`VkCommandBufferLevel::Secondary`] specifies a secondary command buffer.
    Secondary = 1,
}
