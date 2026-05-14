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
    /// For any memory allocated with both the [`VkMemoryPropertyFlag::HostCoherent`] and the
    /// [`VkMemoryPropertyFlag::DeviceCoherentAmd`], host or device accesses also perform
    /// automatic memory domain transfer operations, such that writes are always automatically
    /// available and visible to both host and device memory domains.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkMemoryPropertyFlag {
        /// [`VkMemoryPropertyFlag::DeviceLocal`] bit specifies that memory allocated with this
        /// type is the most efficient for device access. This property will be set if and only if
        /// the memory type belongs to a heap with the [`VkMemoryHeapFlag::DeviceLocal`]
        /// set.
        DeviceLocal = 0x00000001,

        /// [`VkMemoryPropertyFlag::HostVisible`] bit specifies that memory allocated with this
        /// type can be mapped for host access using [`VkMapMemory`].
        HostVisible = 0x00000002,

        /// [`VkMemoryPropertyFlag::HostCoherent`] bit specifies that the host cache management
        /// commands [`VkFlushMappedMemoryRanges`] and [`VkInvalidateMappedMemoryRanges`] are not
        /// needed to manage availability and visibility on the host.
        HostCoherent = 0x00000004,

        /// [`VkMemoryPropertyFlag::HostCached`] bit specifies that memory allocated with this
        /// type is cached on the host. Host memory accesses to uncached memory are slower than to
        /// cached memory, however uncached memory is always host coherent.
        HostCached = 0x00000008,

        /// [`VkMemoryPropertyFlag::LazilyAllocated`] bit specifies that the memory type only
        /// allows device access to the memory. Memory types must not have both
        /// [`VkMemoryPropertyFlag::LazilyAllocated`] and
        /// [`VkMemoryPropertyFlag::HostVisible`] set. Additionally, the object’s backing memory
        /// may be provided by the implementation lazily.
        LazilyAllocated = 0x00000010,

        /// [`VkMemoryPropertyFlag::Protected`] bit specifies that the memory type only allows
        /// device access to the memory, and allows protected queue operations to access the
        /// memory. Memory types must not have [`VkMemoryPropertyFlag::Protected`] set and any
        /// of [`VkMemoryPropertyFlag::HostVisible`] set, or
        /// [`VkMemoryPropertyFlag::HostCoherent`] set, or
        /// [`VkMemoryPropertyFlag::HostCached`] set.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        Protected = 0x00000020,

        /// [`VkMemoryPropertyFlag::DeviceCoherentAmd`] bit specifies that device accesses to
        /// allocations of this memory type are automatically made available and visible on the
        /// device. If paired with [`VkMemoryPropertyFlag::HostCoherent`], memory domain
        /// operations are also performed automatically between host and device.
        ///
        /// Provided by [`amd_device_coherent_memory`]
        DeviceCoherentAmd = 0x00000040,

        /// [`VkMemoryPropertyFlag::DeviceUncachedAmd`] bit specifies that memory allocated with
        /// this type is not cached on the device. Uncached device memory is always device coherent.
        ///
        /// Provided by [`amd_device_coherent_memory`]
        DeviceUncachedAmd = 0x00000080,

        /// [`VkMemoryPropertyFlag::RdmaCapableNv`] bit specifies that external devices can
        /// access this memory directly.
        ///
        /// Provided by [`nv_external_memory_rdma`]
        RdmaCapableNv = 0x00000100,
    }
}
