use crate::VkDeviceSize;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure specifying a buffer copy operation
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkBufferCopy {
    /// `src_offset` is the starting offset in bytes from the start of `src_buffer`.
    pub src_offset: VkDeviceSize,

    /// `dst_offset` is the starting offset in bytes from the start of `dst_buffer`.
    pub dst_offset: VkDeviceSize,

    /// `size` is the number of bytes to copy.
    ///
    /// # Valid Usage
    ///  -  The `size` must be greater than 0
    pub size: VkDeviceSize,
}

const impl Default for VkBufferCopy {
    fn default() -> Self {
        VkBufferCopy {
            src_offset: 0,
            dst_offset: 0,
            size: 0,
        }
    }
}
