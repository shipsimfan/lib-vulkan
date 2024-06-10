// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_1};

/// Bitmask specifying capabilities of queues in a queue family
///
/// If an implementation exposes any queue family that supports graphics operations, at least one
/// queue family of at least one physical device exposed by the implementation must support both
/// graphics and compute operations.
///
/// Furthermore, if the `protected_memory` physical device feature is supported, then at least one
/// queue family of at least one physical device exposed by the implementation must support
/// graphics operations, compute operations, and protected memory operations.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkQueueFlagBits {
    /// [`VkQueueFlagBits::GraphicsBit`] specifies that queues in this queue family support
    /// graphics operations.
    GraphicsBit = 0x00000001,

    /// [`VkQueueFlagBits::ComputeBit`] specifies that queues in this queue family support compute
    /// operations.
    ComputeBit = 0x00000002,

    /// [`VkQueueFlagBits::TransferBit`] specifies that queues in this queue family support
    /// transfer operations.
    TransferBit = 0x00000004,

    /// [`VkQueueFlagBits::SparseBindingBit`] specifies that queues in this queue family support
    /// sparse memory management operations. If any of the sparse resource features are enabled,
    /// then at least one queue family must support this bit.
    SparseBindingBit = 0x00000008,

    /// [`VkQueueFlagBits::ProtectedBit`] specifies that queues in this queue family support the
    /// [`VkDeviceQueueCreateBits::ProtectedBit`] bit. If the physical device supports the
    /// `protected_memory` feature, at least one of its queue families must support this bit.
    ///
    /// Provided by [`VK_VERSION_1_1`]
    ProtectedBit = 0x00000010,

    /// [`VkQueueFlagBits::VideoDecodeBitKHR`] specifies that queues in this queue family support
    /// video decode operations.
    ///
    /// Provided by [`khr_video_decode_queue`]
    VideoDecodeBitKHR = 0x00000020,

    /// [`VkQueueFlagBits::VideoEncodeBitKHR`] specifies that queues in this queue family support
    /// video encode operations.
    ///
    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeBitKHR = 0x00000040,

    /// [`VkQueueFlagBits::OpticalFlowBitNV`] specifies that queues in this queue family support
    /// optical flow operations.
    ///
    /// Provided by [`nv_optical_flow`]
    OpticalFlowBitNV = 0x00000100,
}
