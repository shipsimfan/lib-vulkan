use crate::{VkDeviceSize, VkMemoryHeapFlags};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkMemoryHeapFlag};

/// Structure specifying a memory heap
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkMemoryHeap {
    /// `size` is the total memory size in bytes in the heap.
    pub size: VkDeviceSize,

    /// `flags` is a bitmask of [`VkMemoryHeapFlag`] specifying attribute flags for the heap.
    pub flags: VkMemoryHeapFlags,
}

impl Default for VkMemoryHeap {
    fn default() -> Self {
        VkMemoryHeap {
            size: 0,
            flags: VkMemoryHeapFlags::default(),
        }
    }
}
