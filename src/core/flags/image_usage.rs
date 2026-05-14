use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_4};

flags! {
    /// Bitmask of [`VkImageUsageFlag`]
    ///
    /// # Description
    /// [`VkImageUsageFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkImageUsageFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkImageUsageFlags;

    /// Bitmask specifying intended usage of an image
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkImageUsageFlag {
        /// [`VkImageUsageFlag::TransferSrc`] specifies that the image can be used as the source
        /// of a transfer command.
        TransferSrc = 0x00000001,

        /// [`VkImageUsageFlag::TransferDst`] specifies that the image can be used as the
        /// destination of a transfer command.
        TransferDst = 0x00000002,

        /// [`VkImageUsageFlag::Sampled`] specifies that the image can be used to create a
        /// [`VkImageView`] suitable for occupying a [`VkDescriptorSet`] slot either of type
        /// [`VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`] or [`VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`],
        /// and be sampled by a shader.
        Sampled = 0x00000004,

        /// [`VkImageUsageFlag::Storage`] specifies that the image can be used to create a
        /// [`VkImageView`] suitable for occupying a [`VkDescriptorSet`] slot of type
        /// [`VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`].
        Storage = 0x00000008,

        /// [`VkImageUsageFlag::ColorAttachment`] specifies that the image can be used to create
        /// a [`VkImageView`] suitable for use as a color or resolve attachment in a
        /// [`VkFramebuffer`].
        ColorAttachment = 0x00000010,

        /// [`VkImageUsageFlag::DepthStencilAttachment`] specifies that the image can be used to
        /// create a [`VkImageView`] suitable for use as a depth/stencil or depth/stencil resolve
        /// attachment in a [`VkFramebuffer`].
        DepthStencilAttachment = 0x00000020,

        /// [`VkImageUsageFlag::TransientAttachment`] specifies that implementations may support
        /// using memory allocations with the [`VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT`] to back
        /// an image with this usage. This bit can be set for any image that can be used to create
        /// a [`VkImageView`] suitable for use as a color, resolve, depth/stencil, or input
        /// attachment.
        TransientAttachment = 0x00000040,

        /// [`VkImageUsageFlag::InputAttachment`] specifies that the image can be used to create
        /// a [`VkImageView`] suitable for occupying [`VkDescriptorSet`] slot of type
        /// [`VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`]; be read from a shader as an input attachment;
        /// and be used as an input attachment in a framebuffer.
        InputAttachment = 0x00000080,

        /// [`VkImageUsageFlag::HostTransfer`] specifies that the image can be used with host
        /// copy commands and host layout transitions.
        ///
        /// Provided by [`VK_VERSION_1_4`]
        HostTransfer = 0x00400000,

        /// [`VkImageUsageFlag::VideoDecodeDstKhr`] specifies that the image can be used as a
        /// decode output picture in a video decode operation.
        ///
        /// Provided by [`khr_video_decode_queue`]
        VideoDecodeDstKhr = 0x00000400,

        /// [`VkImageUsageFlag::VideoDecodeSrcKhr`] is reserved for future use.
        ///
        /// Provided by [`khr_video_decode_queue`]
        VideoDecodeSrcKhr = 0x00000800,

        /// [`VkImageUsageFlag::VideoDecodeDpbKhr`] specifies that the image can be used as an
        /// output reconstructed picture or an input reference picture in a video decode operation.
        ///
        /// Provided by [`khr_video_decode_queue`]
        VideoDecodeDpbKhr = 0x00001000,

        /// [`VkImageUsageFlag::FragmentDensityMapExt`] specifies that the image can be used to
        /// create a [`VkImageView`] suitable for use as a fragment density map image.
        ///
        /// Provided by [`ext_fragment_density_map`]
        FragmentDensityMapExt = 0x00000200,

        /// [`VkImageUsageFlag::FragmentShadingRateAttachmentKhr`] specifies that the image can
        /// be used to create a [`VkImageView`] suitable for use as a fragment shading rate
        /// attachment or shading rate image
        ///
        /// Provided by [`khr_fragment_shading_rate`]
        FragmentShadingRateAttachmentKhr = 0x00000100,

        /// [`VkImageUsageFlag::VideoEncodeDstKhr`] is reserved for future use.
        ///
        /// Provided by [`khr_video_encode_queue`]
        VideoEncodeDstKhr = 0x00002000,

        /// [`VkImageUsageFlag::VideoEncodeSrcKhr`] specifies that the image can be used as an
        /// encode input picture in a video encode operation.
        ///
        /// Provided by [`khr_video_encode_queue`]
        VideoEncodeSrcKhr = 0x00004000,

        /// [`VkImageUsageFlag::VideoEncodeDpbKhr`] specifies that the image can be used as an
        /// output reconstructed picture or an input reference picture in a video encode operation.
        ///
        /// Provided by [`khr_video_encode_queue`]
        VideoEncodeDpbKhr = 0x00008000,

        /// [`VkImageUsageFlag::AttachmentFeedbackLoopExt`] specifies that the image can be
        /// transitioned to the [`VkImageLayout::AttachementFeedbackLoopOptimalExt`] layout to be
        /// used as a color or depth/stencil attachment in a VkFramebuffer and/or as a read-only
        /// input resource in a shader (sampled image, combined image sampler or input attachment)
        /// in the same render pass.
        ///
        /// Provided by [`ext_attachment_feedback_loop_layout`]
        AttachmentFeedbackLoopExt = 0x00080000,

        /// Provided by [`huawei_invocation_mask`]
        InvocationMaskHuawei = 0x00040000,

        /// Provided by [`qcom_image_processing`]
        SampleWeightQcom = 0x00100000,

        /// Provided by [`qcom_image_processing`]
        SampleBlockMatchQcom = 0x00200000,

        /// [`VkImageUsageFlag::TensorAliasingArm`] specifies that the image can be transitioned
        /// to the [`VkImageLayout::TensorAliasingArm`] layout.
        ///
        /// Provided by [`arm_tensors`]
        TensorAliasingArm = 0x00800000,

        /// Provided by [`qcom_tile_memory_heap`]
        TileMemoryQcom = 0x08000000,

        /// Provided by [`khr_video_encode_quantization_map`]
        VideoEncodeQuantizationDeltaMapKhr = 0x02000000,

        /// Provided by [`khr_video_encode_quantization_map`]
        VideoEncodeEmphasisMapKhr = 0x04000000,
    }
}
