use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

flags! {
    /// Bitmask of [`VkMemoryHeapFlag`]
    ///
    /// # Description
    /// [`VkMemoryHeapFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkMemoryHeapFlag`].
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkMemoryHeapFlags;

    /// Bitmask specifying attribute flags for a heap
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkMemoryHeapFlag {
        /// [`VkMemoryHeapFlag::DeviceLocalBit`] specifies that the heap corresponds to
        /// device-local memory. Device-local memory may have different performance characteristics
        /// than host-local memory, and may support different memory property flags.
        DeviceLocalBit = 0x00000001,

        /// [`VkMemoryHeapFlag::MultiInstanceBit`] specifies that in a logical device
        /// representing more than one physical device, there is a per-physical device instance of
        /// the heap memory. By default, an allocation from such a heap will be replicated to each
        /// physical device’s instance of the heap.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        MultiInstanceBit = 0x00000002,

        /// [`VkMemoryHeapFlag::TileMemoryBitQcom`] bit specifies that the heap corresponds to
        /// tile memory.
        ///
        /// Provided by [`qcom_tile_memory_heap`]
        TileMemoryBitQcom = 0x00000008,
    }
}
