// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_1};

/// Bitmask specifying properties for a memory type
///
/// # Description
/// For any memory allocated with both the [`VkMemoryPropertyFlagBits::HostCoherentBit`] and the
/// [`VkMemoryPropertyFlagBits::DeviceCoherentBitAmd`], host or device accesses also perform
/// automatic memory domain transfer operations, such that writes are always automatically
/// available and visible to both host and device memory domains.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkMemoryPropertyFlagBits {
    /// [`VkMemoryPropertyFlagBits::DeviceLocalBit`] bit specifies that memory allocated with this
    /// type is the most efficient for device access. This property will be set if and only if the
    /// memory type belongs to a heap with the [`VkMemoryHeapFlagBits::DeviceLocalBit`] set.
    DeviceLocalBit = 0x00000001,

    /// [`VkMemoryPropertyFlagBits::HostVisibleBit`] bit specifies that memory allocated with this
    /// type can be mapped for host access using [`VkMapMemory`].
    HostVisibleBit = 0x00000002,

    /// [`VkMemoryPropertyFlagBits::HostCoherentBit`] bit specifies that the host cache management
    /// commands [`VkFlushMappedMemoryRanges`] and [`VkInvalidateMappedMemoryRanges`] are not
    /// needed to manage availability and visibility on the host.
    HostCoherentBit = 0x00000004,

    /// [`VkMemoryPropertyFlagBits::HostCachedBit`] bit specifies that memory allocated with this
    /// type is cached on the host. Host memory accesses to uncached memory are slower than to
    /// cached memory, however uncached memory is always host coherent.
    HostCachedBit = 0x00000008,

    /// [`VkMemoryPropertyFlagBits::LazilyAllocatedBit`] bit specifies that the memory type only
    /// allows device access to the memory. Memory types must not have both
    /// [`VkMemoryPropertyFlagBits::LazilyAllocatedBit`] and
    /// [`VkMemoryPropertyFlagBits::HostVisibleBit`] set. Additionally, the object’s backing memory
    /// may be provided by the implementation lazily.
    LazilyAllocatedBit = 0x00000010,

    /// [`VkMemoryPropertyFlagBits::ProtectedBit`] bit specifies that the memory type only allows
    /// device access to the memory, and allows protected queue operations to access the memory.
    /// Memory types must not have [`VkMemoryPropertyFlagBits::ProtectedBit`] set and any of
    /// [`VkMemoryPropertyFlagBits::HostVisibleBit`] set, or
    /// [`VkMemoryPropertyFlagBits::HostCoherentBit`] set, or
    /// [`VkMemoryPropertyFlagBits::HostCachedBit`] set.
    ///
    /// Provided by [`VK_VERSION_1_1`]
    ProtectedBit = 0x00000020,

    /// [`VkMemoryPropertyFlagBits::DeviceCoherentBitAmd`] bit specifies that device accesses to
    /// allocations of this memory type are automatically made available and visible on the device.
    /// If paired with [`VkMemoryPropertyFlagBits::HostCoherentBit`], memory domain operations are
    /// also performed automatically between host and device.
    ///
    /// Provided by [`vk_amd_device_coherent_memory`]
    DeviceCoherentBitAmd = 0x00000040,

    /// [`VkMemoryPropertyFlagBits::DeviceUncachedBitAmd`] bit specifies that memory allocated with
    /// this type is not cached on the device. Uncached device memory is always device coherent.
    ///
    /// Provided by [`vk_amd_device_coherent_memory`]
    DeviceUncachedBitAmd = 0x00000080,

    /// [`VkMemoryPropertyFlagBits::RdmaCapableBitNv`] bit specifies that external devices can
    /// access this memory directly.
    ///
    /// Provided by [`vk_nv_external_memory_rdma`]
    RdmaCapableBitNv = 0x00000100,
}
