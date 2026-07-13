use crate::macros::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_1, VK_VERSION_1_2};

flags! {
    /// Bitmask of [`VkBufferCreateFlag`]
    ///
    /// # Description
    /// [`VkBufferCreateFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkBufferCreateFlag`].
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkBufferCreateFlags;

    /// Bitmask specifying additional parameters of a buffer
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkBufferCreateFlag {
        /// [`VkBufferCreateFlag::SparseBinding`] specifies that the buffer will be backed using
        /// sparse memory binding.
        SparseBinding = 0x00000001,

        /// [`VkBufferCreateFlag::SparseResidency`] specifies that the buffer can be partially
        /// backed using sparse memory binding. Buffers created with this flag must also be created
        /// with the [`VkBufferCreateFlag::SparseBinding`] flag.
        SparseResidency = 0x00000002,

        /// [`VkBufferCreateFlag::SparseAliased`] specifies that the buffer will be backed using
        /// sparse memory binding with memory ranges that might also simultaneously be backing
        /// another buffer (or another portion of the same buffer). Buffers created with this flag
        /// must also be created with the [`VkBufferCreateFlag::SparseBinding`] flag.
        SparseAliased = 0x00000004,

        /// [`VkBufferCreateFlag::Protected`] specifies that the buffer is a protected buffer.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        Protected = 0x00000008,

        /// [`VkBufferCreateFlag::DeviceAddressCaptureReplay`] specifies that the buffer’s address
        /// can be saved and reused on a subsequent run (e.g. for trace capture and replay), see
        /// VkBufferOpaqueCaptureAddressCreateInfo for more detail.
        ///
        /// Provided by [`VK_VERSION_1_2`]
        DeviceAddressCaptureReplay = 0x00000010,

        /// [`VkBufferCreateFlag::DescriptorBufferCaptureReplayExt`] specifies that the buffer can
        /// be used with descriptor buffers when capturing and replaying (e.g. for trace capture
        /// and replay), see [`VkOpaqueCaptureDescriptorDataCreateInfoExt`] for more detail.
        ///
        /// Provided by [`ext_descriptor_buffer`]
        DescriptorBufferCaptureReplayExt = 0x00000020,

        /// [`VkBufferCreateFlag::VideoProfileIndependentKhr`] specifies that the buffer can be
        /// used in video coding operations without having to specify at buffer creation time the
        /// set of video profiles the buffer will be used with.
        ///
        /// Provided by [`khr_video_maintenance1`]
        VideoProfileIndependentKhr = 0x00000040,
    }
}
