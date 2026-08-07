use crate::{VkBuffer, VkDeviceSize};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VK_VERSION_1_0, VK_WHOLE_SIZE, VkDescriptorType, VkUpdateDescriptorSets,
};

/// Structure specifying descriptor buffer information
///
/// # Description
/// For [`VkDescriptorType::UniformBufferDynamic`] and [`VkDescriptorType::StorageBufferDynamic`]
/// descriptor types, `offset` is the base offset from which the dynamic offset is applied and
/// `range` is the static size used for all dynamic offsets.
///
/// When range is [`VK_WHOLE_SIZE`] the effective range is calculated at [`VkUpdateDescriptorSets`]
/// is by taking the size of `buffer` minus the offset.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDescriptorBufferInfo {
    /// `buffer` is [`VK_NULL_HANDLE`] or the buffer resource.
    ///
    /// # Valid Usage
    ///  - If the `null_descriptor` feature is not enabled, `buffer` must not be [`VK_NULL_HANDLE`]
    ///  - If `buffer` is [`VK_NULL_HANDLE`], `offset` must be zero and `range` must be
    ///    [`VK_WHOLE_SIZE`]
    ///
    /// # Valid Usage (Implicit)
    ///  - If `buffer` is not [`VK_NULL_HANDLE`], `buffer` must be a valid [`VkBuffer`] handle
    pub buffer: VkBuffer,

    /// `offset` is the offset in bytes from the start of `buffer`. Access to buffer memory via
    /// this descriptor uses addressing that is relative to this starting offset.
    ///
    /// # Valid Usage
    ///  - `offset` must be less than the size of `buffer`
    pub offset: VkDeviceSize,

    /// `range` is the size in bytes that is used for this descriptor update, or [`VK_WHOLE_SIZE`]
    /// to use the range from `offset` to the end of the buffer.
    ///
    /// # Valid Usage
    ///  - If `range` is not equal to [`VK_WHOLE_SIZE`], `range` must be greater than 0
    ///  - If `range` is not equal to [`VK_WHOLE_SIZE`], `range` must be less than or equal to the
    ///    size of `buffer` minus `offset`
    pub range: VkDeviceSize,
}

const impl Default for VkDescriptorBufferInfo {
    fn default() -> Self {
        VkDescriptorBufferInfo {
            buffer: VkBuffer::null(),
            offset: 0,
            range: 0,
        }
    }
}
