// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Bitmask specifying intended usage of an image
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkImageUsageFlagBits {
    /// [`VkImageUsageFlagBits::TransferSrcBit`] specifies that the image can be used as the source of a transfer command.
    TransferSrcBit = 0x00000001,

    /// [`VkImageUsageFlagBits::TransferDstBit`] specifies that the image can be used as the destination of a transfer command.
    TransferDstBit = 0x00000002,

    /// [`VkImageUsageFlagBits::SampledBit`] specifies that the image can be used to create a VkImageView suitable for occupying a VkDescriptorSet slot either of type VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE or VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, and be sampled by a shader.
    SampledBit = 0x00000004,

    /// [`VkImageUsageFlagBits::StorageBit`] specifies that the image can be used to create a VkImageView suitable for occupying a VkDescriptorSet slot of type VK_DESCRIPTOR_TYPE_STORAGE_IMAGE.
    StorageBit = 0x00000008,

    /// [`VkImageUsageFlagBits::ColorAttachmentBit`] specifies that the image can be used to create a VkImageView suitable for use as a color or resolve attachment in a VkFramebuffer.
    ColorAttachmentBit = 0x00000010,

    /// [`VkImageUsageFlagBits::DepthStencilAttachmentBit`] specifies that the image can be used to create a VkImageView suitable for use as a depth/stencil or depth/stencil resolve attachment in a VkFramebuffer.
    DepthStencilAttachmentBit = 0x00000020,

    /// [`VkImageUsageFlagBits::TransientAttachmentBit`] specifies that implementations may support using memory allocations with the VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT to back an image with this usage. This bit can be set for any image that can be used to create a VkImageView suitable for use as a color, resolve, depth/stencil, or input attachment.
    TransientAttachmentBit = 0x00000040,

    /// [`VkImageUsageFlagBits::InputAttachmentBit`] specifies that the image can be used to create a VkImageView suitable for occupying VkDescriptorSet slot of type VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT; be read from a shader as an input attachment; and be used as an input attachment in a framebuffer.
    InputAttachmentBit = 0x00000080,

    /// [`VkImageUsageFlagBits::VideoDecodeDstBitKHR`] specifies that the image can be used as a decode output picture in a video decode operation.
    ///
    /// Provided by [`khr_video_decode_queue`]
    VideoDecodeDstBitKHR = 0x00000400,

    /// [`VkImageUsageFlagBits::VideoDecodeSrcBitKHR`] is reserved for future use.
    ///
    /// Provided by [`khr_video_decode_queue`]
    VideoDecodeSrcBitKHR = 0x00000800,

    /// [`VkImageUsageFlagBits::VideoDecodeDpbBitKHR`] specifies that the image can be used as an
    /// output reconstructed picture or an input reference picture in a video decode operation.
    ///
    /// Provided by [`khr_video_decode_queue`]
    VideoDecodeDpbBitKHR = 0x00001000,

    /// [`VkImageUsageFlagBits::FragmentDensityMapBitEXT`] specifies that the image can be used to
    /// create a [`VkImageView`] suitable for use as a fragment density map image.
    ///
    /// Provided by [`ext_fragment_density_map`]
    FragmentDensityMapBitEXT = 0x00000200,

    /// [`VkImageUsageFlagBits::FragmentShadingRateAttachmentBitKHR`] specifies that the image can
    /// be used to create a [`VkImageView`] suitable for use as a fragment shading rate attachment
    /// or shading rate image
    ///
    /// Provided by [`khr_fragment_shading_rate`]
    FragmentShadingRateAttachmentBitKHR = 0x00000100,

    /// [`VkImageUsageFlagBits::HostTransferBitEXT`] specifies that the image can be used with host
    /// copy commands and host layout transitions.
    ///
    /// Provided by [`ext_host_image_copy`]
    HostTransferBitEXT = 0x00400000,

    /// [`VkImageUsageFlagBits::VideoEncodeDstBitKHR`] is reserved for future use.
    ///
    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeDstBitKHR = 0x00002000,

    /// [`VkImageUsageFlagBits::VideoEncodeSrcBitKHR`] specifies that the image can be used as an
    /// encode input picture in a video encode operation.
    ///
    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeSrcBitKHR = 0x00004000,

    /// [`VkImageUsageFlagBits::VideoEncodeDpbBitKHR`] specifies that the image can be used as an
    /// output reconstructed picture or an input reference picture in a video encode operation.
    ///
    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeDpbBitKHR = 0x00008000,

    /// [`VkImageUsageFlagBits::AttachmentFeedbackLoopBitEXT`] specifies that the image can be
    /// transitioned to the [`VkImageLayout::AttachementFeedbackLoopOptimalEXT`] layout to be used
    /// as a color or depth/stencil attachment in a VkFramebuffer and/or as a read-only input
    /// resource in a shader (sampled image, combined image sampler or input attachment) in the
    /// same render pass.
    ///
    /// Provided by [`ext_attachment_feedback_loop_layout`]
    AttachmentFeedbackLoopBitEXT = 0x00080000,

    /// Provided by [`huawei_invocation_mask`]
    InvocationMaskBitHuawei = 0x00040000,

    /// Provided by [`qcom_image_processing`]
    SampleWeightBitQCOM = 0x00100000,

    /// Provided by [`qcom_image_processing`]
    SampleBlockMatchBitQCOM = 0x00200000,
}
