use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_1};

flags! {
    /// Bitmask of [`VkQueueFlag`]
    ///
    /// # Description
    /// [`VkQueueFlags`] is a bitmask type for setting a mask of zero or more [`VkQueueFlag`]s.
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
        /// [`VkQueueFlag::Graphics`] specifies that queues in this queue family support
        /// graphics operations.
        Graphics = 0x00000001,

        /// [`VkQueueFlag::Compute`] specifies that queues in this queue family support compute
        /// operations.
        Compute = 0x00000002,

        /// [`VkQueueFlag::Transfer`] specifies that queues in this queue family support
        /// transfer operations.
        Transfer = 0x00000004,

        /// [`VkQueueFlag::SparseBinding`] specifies that queues in this queue family support
        /// sparse memory management operations. If any of the sparse resource features are
        /// enabled, then at least one queue family must support this bit.
        SparseBinding = 0x00000008,

        /// [`VkQueueFlag::Protected`] specifies that queues in this queue family support the
        /// [`VkDeviceQueueCreates::Protected`] bit. If the physical device supports the
        /// `protected_memory` feature, at least one of its queue families must support this bit.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        Protected = 0x00000010,

        /// [`VkQueueFlag::VideoDecodeKhr`] specifies that queues in this queue family support
        /// video decode operations.
        ///
        /// Provided by [`khr_video_decode_queue`]
        VideoDecodeKhr = 0x00000020,

        /// [`VkQueueFlag::VideoEncodeKhr`] specifies that queues in this queue family support
        /// video encode operations.
        ///
        /// Provided by [`khr_video_encode_queue`]
        VideoEncodeKhr = 0x00000040,

        /// [`VkQueueFlag::OpticalFlowNv`] specifies that queues in this queue family support
        /// optical flow operations.
        ///
        /// Provided by [`nv_optical_flow`]
        OpticalFlowNv = 0x00000100,

        /// [`VkQueueFlag::DataGraphArm`] specifies that queues in this queue family support
        /// data graph operations.
        ///
        /// Provided by [`arm_data_graph`]
        DataGraphArm = 0x00000400,
    }
}
