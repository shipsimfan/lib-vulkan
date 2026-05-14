use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

flags! {
    /// Bitmask of [`VkImageViewCreateFlag`]
    ///
    /// # Description
    /// [`VkImageViewCreateFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkImageViewCreateFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkImageViewCreateFlags;

    /// Bitmask specifying additional parameters of an image view
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkImageViewCreateFlag {
        /// [`VkImageViewCreateFlag::FragmentDensityMapDynamicExt`] specifies that the fragment
        /// density map will be read by device during
        /// [`VkPipelineStageFlag::FragmentDensityProcessExt`]
        ///
        /// Provided by [`ext_fragment_density_map`]
        FragmentDensityMapDynamicExt = 0x00000001,

        /// [`VkImageViewCreateFlag::FragmentDensityMapDeferredExt`] specifies that the fragment
        /// density map will be read by the host during [`VkEndCommandBuffer`] for the primary
        /// command buffer that the render pass is recorded into
        ///
        /// Provided by [`ext_fragment_density_map2`]
        FragmentDensityMapDeferredExt = 0x00000002,

        /// [`VkImageViewCreateFlag::DescriptorBufferCaptureReplayExt`] specifies that the image
        /// view can be used with descriptor buffers when capturing and replaying (e.g. for trace
        /// capture and replay), see [`VkOpaqueCaptureDescriptorDataCreateInfoExt`] for more
        /// detail.
        ///
        /// Provided by [`ext_descriptor_buffer`]
        DescriptorBufferCaptureReplayExt = 0x00000004,
    }
}
