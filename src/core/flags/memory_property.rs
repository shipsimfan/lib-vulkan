use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_1};

flags! {
    /// Bitmask of [`VkMemoryPropertyFlag`]
    ///
    /// # Description
    /// [`VkMemoryPropertyFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkMemoryPropertyFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkMemoryPropertyFlags;


    /// Bitmask specifying properties for a memory type
    ///
    /// # Description
    /// For any memory allocated with both the [`VkMemoryPropertyFlag::HostCoherentBit`] and the
    /// [`VkMemoryPropertyFlag::DeviceCoherentBitAmd`], host or device accesses also perform
    /// automatic memory domain transfer operations, such that writes are always automatically
    /// available and visible to both host and device memory domains.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkMemoryPropertyFlag {
        /// [`VkMemoryPropertyFlag::DeviceLocalBit`] bit specifies that memory allocated with this
        /// type is the most efficient for device access. This property will be set if and only if
        /// the memory type belongs to a heap with the [`VkMemoryHeapFlag::DeviceLocalBit`]
        /// set.
        DeviceLocalBit = 0x00000001,

        /// [`VkMemoryPropertyFlag::HostVisibleBit`] bit specifies that memory allocated with this
        /// type can be mapped for host access using [`VkMapMemory`].
        HostVisibleBit = 0x00000002,

        /// [`VkMemoryPropertyFlag::HostCoherentBit`] bit specifies that the host cache management
        /// commands [`VkFlushMappedMemoryRanges`] and [`VkInvalidateMappedMemoryRanges`] are not
        /// needed to manage availability and visibility on the host.
        HostCoherentBit = 0x00000004,

        /// [`VkMemoryPropertyFlag::HostCachedBit`] bit specifies that memory allocated with this
        /// type is cached on the host. Host memory accesses to uncached memory are slower than to
        /// cached memory, however uncached memory is always host coherent.
        HostCachedBit = 0x00000008,

        /// [`VkMemoryPropertyFlag::LazilyAllocatedBit`] bit specifies that the memory type only
        /// allows device access to the memory. Memory types must not have both
        /// [`VkMemoryPropertyFlag::LazilyAllocatedBit`] and
        /// [`VkMemoryPropertyFlag::HostVisibleBit`] set. Additionally, the object’s backing memory
        /// may be provided by the implementation lazily.
        LazilyAllocatedBit = 0x00000010,

        /// [`VkMemoryPropertyFlag::ProtectedBit`] bit specifies that the memory type only allows
        /// device access to the memory, and allows protected queue operations to access the
        /// memory. Memory types must not have [`VkMemoryPropertyFlag::ProtectedBit`] set and any
        /// of [`VkMemoryPropertyFlag::HostVisibleBit`] set, or
        /// [`VkMemoryPropertyFlag::HostCoherentBit`] set, or
        /// [`VkMemoryPropertyFlag::HostCachedBit`] set.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        ProtectedBit = 0x00000020,

        /// [`VkMemoryPropertyFlag::DeviceCoherentBitAmd`] bit specifies that device accesses to
        /// allocations of this memory type are automatically made available and visible on the
        /// device. If paired with [`VkMemoryPropertyFlag::HostCoherentBit`], memory domain
        /// operations are also performed automatically between host and device.
        ///
        /// Provided by [`amd_device_coherent_memory`]
        DeviceCoherentBitAmd = 0x00000040,

        /// [`VkMemoryPropertyFlag::DeviceUncachedBitAmd`] bit specifies that memory allocated with
        /// this type is not cached on the device. Uncached device memory is always device coherent.
        ///
        /// Provided by [`amd_device_coherent_memory`]
        DeviceUncachedBitAmd = 0x00000080,

        /// [`VkMemoryPropertyFlag::RdmaCapableBitNv`] bit specifies that external devices can
        /// access this memory directly.
        ///
        /// Provided by [`nv_external_memory_rdma`]
        RdmaCapableBitNv = 0x00000100,
    }
}
