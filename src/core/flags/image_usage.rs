use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_4};

flags! {
    /// Bitmask of [`VkImageUsageFlag`]
    ///
    /// # Description
    /// [`VkImageUsageFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkImageUsageFlag`].
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkImageUsageFlags;

    /// Bitmask specifying intended usage of an image
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkImageUsageFlag {
        /// [`VkImageUsageFlag::TransferSrcBit`] specifies that the image can be used as the source
        /// of a transfer command.
        TransferSrcBit = 0x00000001,

        /// [`VkImageUsageFlag::TransferDstBit`] specifies that the image can be used as the
        /// destination of a transfer command.
        TransferDstBit = 0x00000002,

        /// [`VkImageUsageFlag::SampledBit`] specifies that the image can be used to create a
        /// [`VkImageView`] suitable for occupying a [`VkDescriptorSet`] slot either of type
        /// [`VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`] or [`VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`],
        /// and be sampled by a shader.
        SampledBit = 0x00000004,

        /// [`VkImageUsageFlag::StorageBit`] specifies that the image can be used to create a
        /// [`VkImageView`] suitable for occupying a [`VkDescriptorSet`] slot of type
        /// [`VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`].
        StorageBit = 0x00000008,

        /// [`VkImageUsageFlag::ColorAttachmentBit`] specifies that the image can be used to create
        /// a [`VkImageView`] suitable for use as a color or resolve attachment in a
        /// [`VkFramebuffer`].
        ColorAttachmentBit = 0x00000010,

        /// [`VkImageUsageFlag::DepthStencilAttachmentBit`] specifies that the image can be used to
        /// create a [`VkImageView`] suitable for use as a depth/stencil or depth/stencil resolve
        /// attachment in a [`VkFramebuffer`].
        DepthStencilAttachmentBit = 0x00000020,

        /// [`VkImageUsageFlag::TransientAttachmentBit`] specifies that implementations may support
        /// using memory allocations with the [`VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT`] to back
        /// an image with this usage. This bit can be set for any image that can be used to create
        /// a [`VkImageView`] suitable for use as a color, resolve, depth/stencil, or input
        /// attachment.
        TransientAttachmentBit = 0x00000040,

        /// [`VkImageUsageFlag::InputAttachmentBit`] specifies that the image can be used to create
        /// a [`VkImageView`] suitable for occupying [`VkDescriptorSet`] slot of type
        /// [`VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`]; be read from a shader as an input attachment;
        /// and be used as an input attachment in a framebuffer.
        InputAttachmentBit = 0x00000080,

        /// [`VkImageUsageFlag::HostTransferBit`] specifies that the image can be used with host
        /// copy commands and host layout transitions.
        ///
        /// Provided by [`VK_VERSION_1_4`]
        HostTransferBit = 0x00400000,

        /// [`VkImageUsageFlag::VideoDecodeDstBitKhr`] specifies that the image can be used as a
        /// decode output picture in a video decode operation.
        ///
        /// Provided by [`khr_video_decode_queue`]
        VideoDecodeDstBitKhr = 0x00000400,

        /// [`VkImageUsageFlag::VideoDecodeSrcBitKhr`] is reserved for future use.
        ///
        /// Provided by [`khr_video_decode_queue`]
        VideoDecodeSrcBitKhr = 0x00000800,

        /// [`VkImageUsageFlag::VideoDecodeDpbBitKhr`] specifies that the image can be used as an
        /// output reconstructed picture or an input reference picture in a video decode operation.
        ///
        /// Provided by [`khr_video_decode_queue`]
        VideoDecodeDpbBitKhr = 0x00001000,

        /// [`VkImageUsageFlag::FragmentDensityMapBitExt`] specifies that the image can be used to
        /// create a [`VkImageView`] suitable for use as a fragment density map image.
        ///
        /// Provided by [`ext_fragment_density_map`]
        FragmentDensityMapBitExt = 0x00000200,

        /// [`VkImageUsageFlag::FragmentShadingRateAttachmentBitKhr`] specifies that the image can
        /// be used to create a [`VkImageView`] suitable for use as a fragment shading rate
        /// attachment or shading rate image
        ///
        /// Provided by [`khr_fragment_shading_rate`]
        FragmentShadingRateAttachmentBitKhr = 0x00000100,

        /// [`VkImageUsageFlag::VideoEncodeDstBitKhr`] is reserved for future use.
        ///
        /// Provided by [`khr_video_encode_queue`]
        VideoEncodeDstBitKhr = 0x00002000,

        /// [`VkImageUsageFlag::VideoEncodeSrcBitKhr`] specifies that the image can be used as an
        /// encode input picture in a video encode operation.
        ///
        /// Provided by [`khr_video_encode_queue`]
        VideoEncodeSrcBitKhr = 0x00004000,

        /// [`VkImageUsageFlag::VideoEncodeDpbBitKhr`] specifies that the image can be used as an
        /// output reconstructed picture or an input reference picture in a video encode operation.
        ///
        /// Provided by [`khr_video_encode_queue`]
        VideoEncodeDpbBitKhr = 0x00008000,

        /// [`VkImageUsageFlag::AttachmentFeedbackLoopBitExt`] specifies that the image can be
        /// transitioned to the [`VkImageLayout::AttachementFeedbackLoopOptimalExt`] layout to be
        /// used as a color or depth/stencil attachment in a VkFramebuffer and/or as a read-only
        /// input resource in a shader (sampled image, combined image sampler or input attachment)
        /// in the same render pass.
        ///
        /// Provided by [`ext_attachment_feedback_loop_layout`]
        AttachmentFeedbackLoopBitExt = 0x00080000,

        /// Provided by [`huawei_invocation_mask`]
        InvocationMaskBitHuawei = 0x00040000,

        /// Provided by [`qcom_image_processing`]
        SampleWeightBitQcom = 0x00100000,

        /// Provided by [`qcom_image_processing`]
        SampleBlockMatchBitQcom = 0x00200000,
    }
}
