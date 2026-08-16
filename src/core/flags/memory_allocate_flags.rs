use crate::macros::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_1, VkBufferUsageFlag, VkMemoryAllocateFlagsInfo};

flags! {
    /// Bitmask of [`VkMemoryAllocateFlag`]
    ///
    /// # Description
    /// [`VkMemoryAllocateFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkMemoryAllocateFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_1`]
    pub struct VkMemoryAllocateFlags;

    /// Bitmask specifying flags for a device memory allocation
    ///
    /// Provided by [`VK_VERSION_1_1`]
    pub enum VkMemoryAllocateFlag {
        /// [`VkMemoryAllocateFlag::DeviceMask`] specifies that memory will be allocated for the
        /// devices in [`VkMemoryAllocateFlagsInfo::device_mask`].
        DeviceMask = 0x00000001,

        /// [`VkMemoryAllocateFlag::DeviceAddress`] specifies that the memory can be attached to a
        /// buffer object created with the [`VkBufferUsageFlag::ShaderDeviceAddress`] usage flag
        /// set.
        DeviceAddress = 0x00000002,

        /// [`VkMemoryAllocateFlag::DeviceAddressCaptureReplay`] specifies that the memory’s
        /// address can be saved and reused on a subsequent run (e.g. for trace capture and
        /// replay), see [`VkBufferOpaqueCaptureAddressCreateInfo`] for more detail. If this bit is
        /// set, [`VkMemoryAllocateFlag::DeviceAddress`] must also be set.
        DeviceAddressCaptureReplay = 0x00000004,

        /// [`VkMemoryAllocateFlag::ZeroInitializeExt`] specifies that the memory will be zeroed
        /// automatically by the implementation before application is able to access it.
        ZeroInitializeExt = 0x00000008,
    }
}
