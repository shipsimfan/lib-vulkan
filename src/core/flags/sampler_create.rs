use crate::macros::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkImageCreateFlag};

flags! {
    /// Reserved for future use
    ///
    /// # Description
    /// [`VkSamplerCreateFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkSamplerCreateFlag`].
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkSamplerCreateFlags;


    /// Bitmask specifying additional parameters of sampler
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkSamplerCreateFlag {
        /// [`VkSamplerCreateFlag::SubsampledExt`] specifies that the sampler will read from an
        /// image created with flags containing [`VkImageCreateFlag::SubsampledExt`].
        ///
        /// Provided by [`ext_fragment_density_map`]
        SubsampledExt = 0x00000001,

        /// [`VkSamplerCreateFlag::SubsampledCoarseReconstructionExt`] specifies that the
        /// implementation may use approximations when reconstructing a full color value for
        /// texture access from a subsampled image.
        ///
        /// Provided by [`ext_fragment_density_map`]
        SubsampledCoarseReconstructionExt = 0x00000002,

        /// [`VkSamplerCreateFlag::NonSeamlessCubeMapExt`] specifies that cube map edge handling is
        /// not performed.
        ///
        /// Provided by [`ext_non_seamless_cube_map`]
        NonSeamlessCubeMapExt = 0x00000004,

        /// [`VkSamplerCreateFlag::DescriptorBufferCaptureReplayExt`] specifies that the sampler
        /// can be used with descriptor buffers when capturing and replaying (e.g. for trace
        /// capture and replay), see [`VkOpaqueCaptureDescriptorDataCreateInfoExt`] for more
        /// detail.
        ///
        /// Provided by [`ext_descriptor_buffer`]
        DescriptorBufferCaptureReplayExt = 0x00000008,

        /// [`VkSamplerCreateFlag::ImageProcessingQcom`] specifies that the sampler will read from
        /// images using only `OpImageSampleWeightedQCOM`, `OpImageBoxFilterQCOM`,
        /// `OpImageBlockMatchGatherSSDQCOM`, `OpImageBlockMatchGatherSADQCOM`,
        /// `OpImageBlockMatchWindowSSDQCOM`, `OpImageBlockMatchWindowSADQCOM`,
        /// `OpImageBlockMatchSSDQCOM`, or `OpImageBlockMatchSADQCOM`.
        ///
        /// Provided by [`qcom_image_processing`]
        ImageProcessingQcom = 0x00000010,
    }
}
