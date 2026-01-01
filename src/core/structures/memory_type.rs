use crate::VkMemoryPropertyFlags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkMemoryPropertyFlag, VkPhysicalDeviceMemoryProperties};

/// Structure specifying memory type
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkMemoryType {
    /// `property_flags` is a bitmask of [`VkMemoryPropertyFlag`] of properties for this memory
    /// type.
    pub property_flags: VkMemoryPropertyFlags,

    /// `heap_index` describes which memory heap this memory type corresponds to, and must be less
    /// than `memory_heap_count` from the [`VkPhysicalDeviceMemoryProperties`] structure.
    pub heap_index: u32,
}

impl const Default for VkMemoryType {
    fn default() -> Self {
        VkMemoryType {
            property_flags: VkMemoryPropertyFlags::default(),
            heap_index: 0,
        }
    }
}
