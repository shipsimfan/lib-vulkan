// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Bitmask specifying attribute flags for a heap
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkMemoryHeapFlagBits {
    /// [`VkMemoryHeapFlagBits::DeviceLocalBit`] specifies that the heap corresponds to
    /// device-local memory. Device-local memory may have different performance characteristics
    /// than host-local memory, and may support different memory property flags.
    DeviceLocalBit = 0x00000001,

    /// [`VkMemoryHeapFlagBits::MultiInstanceBit`] specifies that in a logical device representing
    /// more than one physical device, there is a per-physical device instance of the heap memory.
    /// By default, an allocation from such a heap will be replicated to each physical device’s
    /// instance of the heap.
    MultiInstanceBit = 0x00000002,

    /// [`VkMemoryHeapFlagBits::TileMemoryBitQcom`] bit specifies that the heap corresponds to tile
    /// memory.
    TileMemoryBitQcom = 0x00000008,
}
