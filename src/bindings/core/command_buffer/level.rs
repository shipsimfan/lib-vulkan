#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkCommandBufferLevel {
    Primary = 0,
    Secondary = 1,
}
