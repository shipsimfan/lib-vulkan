use crate::{VK_MAX_MEMORY_HEAPS, VK_MAX_MEMORY_TYPES, VkMemoryHeap, VkMemoryType};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkGetPhysicalDeviceMemoryProperties};

/// Structure specifying physical device memory properties
///
/// # Description
/// The [`VkPhysicalDeviceMemoryProperties`] structure describes a number of memory heaps as well
/// as a number of memory types that can be used to access memory allocated in those heaps. Each
/// heap describes a memory resource of a particular size, and each memory type describes a set of
/// memory properties (e.g. host cached vs. uncached) that can be used with a given memory heap.
/// Allocations using a particular memory type will consume resources from the heap indicated by
/// that memory type’s heap index. More than one memory type may share each heap, and the heaps and
/// memory types provide a mechanism to advertise an accurate size of the physical memory resources
/// while allowing the memory to be used with a variety of different properties.
///
/// The number of memory heaps is given by `memory_heap_count` and is less than or equal to
/// [`VK_MAX_MEMORY_HEAPS`]. Each heap is described by an element of the `memory_heaps` array as a
/// [`VkMemoryHeap`] structure. The number of memory types available across all memory heaps is
/// given by `memory_type_count` and is less than or equal to [`VK_MAX_MEMORY_TYPES`]. Each memory
/// type is described by an element of the `memory_types` array as a [`VkMemoryType`] structure.
///
/// At least one heap must include [`VkMemoryHeapFlagBits::DeviceLocalBit`] in
/// [`VkMemoryHeap::flags`]. If there are multiple heaps that all have similar performance
/// characteristics, they may all include [`VkMemoryHeapFlagBits::DeviceLocalBit`]. In a unified
/// memory architecture (UMA) system there is often only a single memory heap which is considered
/// to be equally “local” to the host and to the device, and such an implementation must advertise
/// the heap as device-local.
///
/// Memory contents within a tile memory heap, denoted by
/// [`VkMemoryHeapFlagBits::TileMemoryBitQcom`], are only visible across the command buffers
/// executed in a single command buffer submission batch within a [`VkQueueSubmit`] or
/// [`VkQueueSubmit2`] call. If the
/// [`VkPhysicalDeviceTileMemoryHeapPropertiesQCOM::queue_submit_boundary`] property is set, the
/// visibility is extended across all batches in the submit call. Memory contents are discarded and
/// made undefined after the respective submission batch or submit call. Tile memory may have
/// different performance characteristics than non tile memory. Tile memory can be used
/// simultaneously by command buffers in other queues without invalidating each others contents.
/// Collectively, these rules define the tile memory scope.
///
/// Each memory type returned by [`VkGetPhysicalDeviceMemoryProperties`] must have its
/// `property_flags` set to one of the following values:
///  - 0
///  - `VkMemoryPropertyFlagBits::HostVisibleBit | VkMemoryPropertyFlagBits::HostCoherentBit`
///  - `VkMemoryPropertyFlagBits::HostVisibleBit | VkMemoryPropertyFlagBits::HostCachedBit`
///  - `VkMemoryPropertyFlagBits::HostVisibleBit |
///     VkMemoryPropertyFlagBits::HostCachedBit |
///     VkMemoryPropertyFlagBits::HostCoherentBit`
///  - `VkMemoryPropertyFlagBits::DeviceLocalBit`
///  - `VkMemoryPropertyFlagBits::DeviceLocalBit |
///     VkMemoryPropertyFlagBits::HostVisibleBit |
///     VkMemoryPropertyFlagBits::HostCoherentBit`
///  - `VkMemoryPropertyFlagBits::DeviceLocalBit |
///     VkMemoryPropertyFlagBits::HostVisibleBit |
///     VkMemoryPropertyFlagBits::HostCachedBit`
///  - `VkMemoryPropertyFlagBits::DeviceLocalBit |
///     VkMemoryPropertyFlagBits::HostVisibleBit |
///     VkMemoryPropertyFlagBits::HostCachedBit |
///     VkMemoryPropertyFlagBits::HostCoherentBit`
///  - `VkMemoryPropertyFlagBits::DeviceLocalBit | VkMemoryPropertyFlagBits::LazilyAllocatedBit`
///  - `VkMemoryPropertyFlagBits::PROTECTED_BIT`
///  - `VkMemoryPropertyFlagBits::PROTECTED_BIT | VkMemoryPropertyFlagBits::DeviceLocalBit`
///  - `VkMemoryPropertyFlagBits::HostVisibleBit |
///     VkMemoryPropertyFlagBits::HostCoherentBit |
///     VkMemoryPropertyFlagBits::DeviceCoherentBitAmd`
///  - `VkMemoryPropertyFlagBits::HostVisibleBit |
///     VkMemoryPropertyFlagBits::HostCachedBit |
///     VkMemoryPropertyFlagBits::HostCoherentBit |
///     VkMemoryPropertyFlagBits::DeviceCoherentBitAmd`
///  - `VkMemoryPropertyFlagBits::DeviceLocalBit | VkMemoryPropertyFlagBits::DeviceCoherentBitAmd`
///  - `VkMemoryPropertyFlagBits::DeviceLocalBit |
///     VkMemoryPropertyFlagBits::HostVisibleBit |
///     VkMemoryPropertyFlagBits::HostCoherentBit |
///     VkMemoryPropertyFlagBits::DeviceCoherentBitAmd`
///  - `VkMemoryPropertyFlagBits::DeviceLocalBit |
///     VkMemoryPropertyFlagBits::HostVisibleBit |
///     VkMemoryPropertyFlagBits::HostCachedBit |
///     VkMemoryPropertyFlagBits::HostCoherentBit |
///     VkMemoryPropertyFlagBits::DeviceCoherentBitAmd`
///  - `VkMemoryPropertyFlagBits::HostVisibleBit |
///     VkMemoryPropertyFlagBits::HostCoherentBit |
///     VkMemoryPropertyFlagBits::DeviceCoherentBitAmd |
///     VkMemoryPropertyFlagBits::DeviceUncachedBitAmd`
///  - `VkMemoryPropertyFlagBits::HostVisibleBit |
///     VkMemoryPropertyFlagBits::HostCachedBit |
///     VkMemoryPropertyFlagBits::HostCoherentBit |
///     VkMemoryPropertyFlagBits::DeviceCoherentBitAmd |
///     VkMemoryPropertyFlagBits::DeviceUncachedBitAmd`
///  - `VkMemoryPropertyFlagBits::DeviceLocalBit |
///     VkMemoryPropertyFlagBits::DeviceCoherentBitAmd |
///     VkMemoryPropertyFlagBits::DeviceUncachedBitAmd`
///  - `VkMemoryPropertyFlagBits::DeviceLocalBit |
///     VkMemoryPropertyFlagBits::HostVisibleBit |
///     VkMemoryPropertyFlagBits::HostCoherentBit |
///     VkMemoryPropertyFlagBits::DeviceCoherentBitAmd |
///     VkMemoryPropertyFlagBits::DeviceUncachedBitAmd`
///  - `VkMemoryPropertyFlagBits::DeviceLocalBit |
///     VkMemoryPropertyFlagBits::HostVisibleBit |
///     VkMemoryPropertyFlagBits::HostCachedBit |
///     VkMemoryPropertyFlagBits::HostCoherentBit |
///     VkMemoryPropertyFlagBits::DeviceCoherentBitAmd |
///     VkMemoryPropertyFlagBits::DeviceUncachedBitAmd`
///  - `VkMemoryPropertyFlagBits::DeviceLocalBit | VkMemoryPropertyFlagBits::RdmaCapableBitNv`
///
/// There must be at least one memory type with both the
/// [`VkMemoryPropertyFlagBits::HostVisibleBit`] and [`VkMemoryPropertyFlagBits::HostCoherentBit`]
/// bits set in its `property_flags`. There must be at least one memory type with the
/// [`VkMemoryPropertyFlagBits::DeviceLocalBit`] bit set in its `property_flags`. If the
/// `device_coherent_memory` feature is enabled, there must be at least one memory type with the
/// [`VkMemoryPropertyFlagBits::DeviceCoherentBitAmd`] bit set in its `property_flags`.
///
/// For each pair of elements `X` and `Y` returned in `memory_types`, `X` must be placed at a lower
/// index position than `Y` if:
///  - the set of bit flags returned in the `property_flags` member of `X` is a strict subset of
///    the set of bit flags returned in the `property_flags` member of `Y`; or
///  - the `property_flags` members of `X` and `Y` are equal, and `X` belongs to a memory heap with
///    greater performance (as determined in an implementation-specific manner); or
///  - the `property_flags` members of `Y` includes
///    [`VkMemoryPropertyFlagBits::DeviceCoherentBitAmd`] or
///    [`VkMemoryPropertyFlagBits::DeviceUncachedBitAmd`] and `X` does not
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceMemoryProperties {
    /// `memory_type_count` is the number of valid elements in the `memory_types` array.
    pub memory_type_count: u32,

    /// `memory_types` is an array of [`VK_MAX_MEMORY_TYPES`] [`VkMemoryType`] structures
    /// describing the memory types that can be used to access memory allocated from the heaps
    /// specified by `memory_heaps`.
    pub memory_types: [VkMemoryType; VK_MAX_MEMORY_TYPES],

    /// `memory_heap_count` is the number of valid elements in the `memory_heaps` array.
    pub memory_heap_count: u32,

    /// `memory_heaps` is an array of [`VK_MAX_MEMORY_HEAPS`] [`VkMemoryHeap`] structures
    /// describing the memory heaps from which memory can be allocated.
    pub memory_heaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}

impl const Default for VkPhysicalDeviceMemoryProperties {
    fn default() -> Self {
        VkPhysicalDeviceMemoryProperties {
            memory_type_count: 0,
            memory_types: [VkMemoryType::default(); VK_MAX_MEMORY_TYPES],
            memory_heap_count: 0,
            memory_heaps: [VkMemoryHeap::default(); VK_MAX_MEMORY_HEAPS],
        }
    }
}
