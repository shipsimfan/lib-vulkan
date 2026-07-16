use crate::VkDeviceSize;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkPhysicalDeviceMemoryProperties};

/// Structure specifying memory requirements
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMemoryRequirements {
    /// `size` is the size, in bytes, of the memory allocation required for the resource.
    pub size: VkDeviceSize,

    /// `alignment` is the alignment, in bytes, of the offset within the allocation required for
    /// the resource.
    pub alignment: VkDeviceSize,

    /// `memory_type_bits` is a bitmask and contains one bit set for every supported memory type
    /// for the resource. Bit `i` is set if and only if the memory type `i` in the
    /// [`VkPhysicalDeviceMemoryProperties`] structure for the physical device is supported for the
    /// resource.
    pub memory_type_bits: u32,
}

const impl Default for VkMemoryRequirements {
    fn default() -> Self {
        VkMemoryRequirements {
            size: 0,
            alignment: 0,
            memory_type_bits: 0,
        }
    }
}
