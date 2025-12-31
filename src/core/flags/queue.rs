use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_1};

flags! {
    /// Bitmask of [`VkQueueFlag`]
    ///
    /// # Description
    /// [`VkQueueFlags`] is a bitmask type for setting a mask of zero or more [`VkQueueFlag`].
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkQueueFlags;

    /// Bitmask specifying capabilities of queues in a queue family
    ///
    /// # Description
    /// At least one queue family of at least one physical device exposed by the implementation
    /// must support at least one of the following sets of operations:
    ///  - graphics operations
    ///  - compute operations
    ///  - video encode operations
    ///  - video decode operations
    ///
    /// If an implementation exposes any queue family that supports graphics operations, at least
    /// one queue family of at least one physical device exposed by the implementation must support
    /// both graphics and compute operations.
    ///
    /// Furthermore, if the `protected_memory` physical device feature is supported, then at least
    /// one queue family of at least one physical device exposed by the implementation must support
    /// graphics operations, compute operations, and protected memory operations.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkQueueFlag {
        /// [`VkQueueFlag::GraphicsBit`] specifies that queues in this queue family support
        /// graphics operations.
        GraphicsBit = 0x00000001,

        /// [`VkQueueFlag::ComputeBit`] specifies that queues in this queue family support compute
        /// operations.
        ComputeBit = 0x00000002,

        /// [`VkQueueFlag::TransferBit`] specifies that queues in this queue family support
        /// transfer operations.
        TransferBit = 0x00000004,

        /// [`VkQueueFlag::SparseBindingBit`] specifies that queues in this queue family support
        /// sparse memory management operations. If any of the sparse resource features are
        /// enabled, then at least one queue family must support this bit.
        SparseBindingBit = 0x00000008,

        /// [`VkQueueFlag::ProtectedBit`] specifies that queues in this queue family support the
        /// [`VkDeviceQueueCreateBits::ProtectedBit`] bit. If the physical device supports the
        /// `protected_memory` feature, at least one of its queue families must support this bit.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        ProtectedBit = 0x00000010,

        /// [`VkQueueFlag::VideoDecodeBitKhr`] specifies that queues in this queue family support
        /// video decode operations.
        ///
        /// Provided by [`khr_video_decode_queue`]
        VideoDecodeBitKhr = 0x00000020,

        /// [`VkQueueFlag::VideoEncodeBitKhr`] specifies that queues in this queue family support
        /// video encode operations.
        ///
        /// Provided by [`khr_video_encode_queue`]
        VideoEncodeBitKhr = 0x00000040,

        /// [`VkQueueFlag::OpticalFlowBitNv`] specifies that queues in this queue family support
        /// optical flow operations.
        ///
        /// Provided by [`nv_optical_flow`]
        OpticalFlowBitNv = 0x00000100,
    }
}
