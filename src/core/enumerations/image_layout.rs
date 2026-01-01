// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VK_VERSION_1_1, VK_VERSION_1_2, VK_VERSION_1_3, VK_VERSION_1_4, VkFramebuffer,
    VkImageUsageFlag, VkRenderPass, khr_swapchain,
};

/// Layout of image and image subresources
///
/// # Description
/// The layout of each image subresource is not a state of the image subresource itself, but is
/// rather a property of how the data in memory is organized, and thus for each mechanism of
/// accessing an image in the API the application must specify a parameter or structure member that
/// indicates which image layout the image subresource(s) are considered to be in when the image
/// will be accessed. For transfer commands, this is a parameter to the command. For use as a
/// framebuffer attachment, this is a member in the substructures of the
/// [`VkRenderPassCreateInfo`]. For use in a descriptor set, this is a member in the
/// [`VkDescriptorImageInfo`] structure.
///
/// If the [`unified_image_layouts`] feature is enabled, the [`VkImageLayout::General`] image
/// layout may be used in place of the other layouts where allowed with no loss of performance.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkImageLayout {
    /// [`VkImageLayout::Undefined`] specifies that the layout is unknown. Image memory cannot be
    /// transitioned into this layout. This layout can be used as the `initial_layout` member of
    /// [`VkImageCreateInfo`]. This layout can be used in place of the current image layout in a
    /// layout transition, but doing so will cause the contents of the image’s memory to be
    /// undefined.
    Undefined = 0,

    /// [`VkImageLayout::General`] supports all types of device access, unless specified otherwise.
    General = 1,

    /// [`VkImageLayout::ColorAttachmentOptimal`] must only be used as a color or resolve
    /// attachment in a [`VkFramebuffer`]. This layout is valid only for image subresources of
    /// images created with the [`VkImageUsageFlag::ColorAttachmentBit`] usage flag set.
    ColorAttachmentOptimal = 2,

    /// [`VkImageLayout::DepthStencilAttachmentOptimal`] specifies a layout for both the depth and
    /// stencil aspects of a depth/stencil format image allowing read and write access as a
    /// depth/stencil attachment. It is equivalent to [`VkImageLayout::DepthAttachmentOptimal`] and
    /// [`VkImageLayout::StencilAttachmentOptimal`].
    DepthStencilAttachmentOptimal = 3,

    /// [`VkImageLayout::DepthStencilReadOnlyOptimal`] specifies a layout for both the depth and
    /// stencil aspects of a depth/stencil format image allowing read only access as a
    /// depth/stencil attachment or in shaders as a sampled image, combined image/sampler, or input
    /// attachment. It is equivalent to [`VkImageLayout::DepthReadOnlyOptimal`] and
    /// [`VkImageLayout::StencilReadOnlyOptimal`].
    DepthStencilReadOnlyOptimal = 4,

    /// [`VkImageLayout::ShaderReadOnlyOptimal`] specifies a layout allowing read-only access in a
    /// shader as a sampled image, combined image/sampler, or input attachment. This layout is
    /// valid only for image subresources of images created with the
    /// [`VkImageUsageFlag::SampledBit`] or [`VkImageUsageFlag::InputAttachmentBit`] usage bits
    /// enabled.
    ShaderReadOnlyOptimal = 5,

    /// [`VkImageLayout::TransferSrcOptimal`] specifies a layout allowing read-only access in a
    /// shader as a sampled image, combined image/sampler, or input attachment. This layout is
    /// valid only for image subresources of images created with the
    /// [`VkImageUsageFlag::SampledBit`] or [`VkImageUsageFlag::InputAttachmentBit`] usage bits
    /// enabled.
    TransferSrcOptimal = 6,

    /// [`VkImageLayout::TransferDstOptimal`] must only be used as a destination image of a
    /// transfer command. This layout is valid only for image subresources of images created with
    /// the [`VkImageUsageFlag::TransferDstBit`] usage flag set.
    TransferDstOptimal = 7,

    /// [`VkImageLayout::Preinitialized`] specifies that an image’s memory is in a defined layout
    /// and can be populated by data, but that it has not yet been initialized by the driver. Image
    /// memory cannot be transitioned into this layout. This layout can be used as the
    /// `initial_layout` member of [`VkImageCreateInfo`]. This layout is intended to be used as the
    /// initial layout for an image whose contents are written by the host, and hence the data can
    /// be written to memory immediately, without first executing a layout transition. Currently,
    /// [`VkImageLayout::Preinitialized`] is only useful with linear images because there is not a
    /// standard layout defined for [`VkImageTiling::Optimal`] images.
    Preinitialized = 8,

    /// [`VkImageLayout::DepthReadOnlyStencilAttachmentOptimal`] specifies a layout for
    /// depth/stencil format images allowing read and write access to the stencil aspect as a
    /// stencil attachment, and read only access to the depth aspect as a depth attachment or in
    /// shaders as a sampled image, combined image/sampler, or input attachment. It is equivalent
    /// to [`VkImageLayout::DepthReadOnlyOptimal`] and [`VkImageLayout::StencilAttachmentOptimal`].
    ///
    /// Provided by [`VK_VERSION_1_1`]
    DepthReadOnlyStencilAttachmentOptimal = 1000117000,

    /// [`VkImageLayout::DepthAttachmentStencilReadOnlyOptimal`] specifies a layout for
    /// depth/stencil format images allowing read and write access to the depth aspect as a depth
    /// attachment, and read only access to the stencil aspect as a stencil attachment or in
    /// shaders as a sampled image, combined image/sampler, or input attachment. It is equivalent
    /// to [`VkImageLayout::DepthAttachmentOptimal`] and [`VkImageLayout::StencilReadOnlyOptimal`].
    ///
    /// Provided by [`VK_VERSION_1_1`]
    DepthAttachmentStencilReadOnlyOptimal = 1000117001,

    /// [`VkImageLayout::DepthAttachmentOptimal`] specifies a layout for the depth aspect of a
    /// depth/stencil format image allowing read and write access as a depth attachment.
    ///
    /// Provided by [`VK_VERSION_1_2`]
    DepthAttachmentOptimal = 1000241000,

    /// [`VkImageLayout::DepthReadOnlyOptimal`] specifies a layout for the depth aspect of a
    /// depth/stencil format image allowing read-only access as a depth attachment or in shaders as
    /// a sampled image, combined image/sampler, or input attachment.
    ///
    /// Provided by [`VK_VERSION_1_2`]
    DepthReadOnlyOptimal = 1000241001,

    /// [`VkImageLayout::StencilAttachmentOptimal`] specifies a layout for the stencil aspect of a
    /// depth/stencil format image allowing read and write access as a stencil attachment.
    ///
    /// Provided by [`VK_VERSION_1_2`]
    StencilAttachmentOptimal = 1000241002,

    /// [`VkImageLayout::StencilReadOnlyOptimal`] specifies a layout for the stencil aspect of a
    /// depth/stencil format image allowing read-only access as a stencil attachment or in shaders
    /// as a sampled image, combined image/sampler, or input attachment.
    ///
    /// Provided by [`VK_VERSION_1_2`]
    StencilReadOnlyOptimal = 1000241003,

    /// [`VkImageLayout::ReadOnlyOptimal`] specifies a layout allowing read only access as an
    /// attachment, or in shaders as a sampled image, combined image/sampler, or input attachment.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    ReadOnlyOptimal = 1000314000,

    /// [`VkImageLayout::AttachmentOptimal`] specifies a layout that must only be used with
    /// attachment accesses in the graphics pipeline.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    AttachmentOptimal = 1000314001,

    /// [`VkImageLayout::RenderingLocalRead`] must only be used as either a storage image, or a
    /// color or depth/stencil attachment and an input attachment. This layout is valid only for
    /// image subresources of images created with either the [`VkImageUsageFlag::StorageBit`] usage
    /// flag set, or both the [`VkImageUsageFlag::InputAttachmentBit`] and either of the
    /// [`VkImageUsageFlag::ColorAttachmentBit`] or [`VkImageUsageFlag::DepthStencilAttachmentBit`]
    /// usage flags set.
    ///
    /// Provided by [`VK_VERSION_1_4`]
    RenderingLocalRead = 1000232000,

    /// [`VkImageLayout::PresentSrcKhr`] must only be used for presenting a presentable image for
    /// display.
    ///
    /// Provided by [`khr_swapchain`]
    PresentSrcKhr = 1000001002,

    /// [`VkImageLayout::VideoDecodeDstKhr`] must only be used as a decode output picture in a
    /// video decode operation. This layout is valid only for image subresources of images created
    /// with the [`VkImageUsageFlag::VideoDecodeDstBitKhr`] usage flag set.
    ///
    /// Provided by [`khr_video_decode_queue`]
    VideoDecodeDstKhr = 1000024000,

    /// [`VkImageLayout::VideoDecodeSrcKhr`] is reserved for future use.
    ///
    /// Provided by [`khr_video_decode_queue`]
    VideoDecodeSrcKhr = 1000024001,

    /// [`VkImageLayout::VideoDecodeDpbKhr`] must only be used as an output reconstructed picture
    /// or an input reference picture in a video decode operation. This layout is valid only for
    /// image subresources of images created with the [`VkImageUsageFlag::VideoDecodeDpbBitKhr`]
    /// usage flag set.
    ///
    /// Provided by [`khr_video_decode_queue`]
    VideoDecodeDpbKhr = 1000024002,

    /// [`VkImageLayout::SharedPresentKhr`] is valid only for shared presentable images, and must
    /// be used for any usage the image supports.
    ///
    /// Provided by [`khr_shared_presentable_image`]
    SharedPresentKhr = 1000111000,

    /// [`VkImageLayout::FragmentDensityMapOptimalExt`] must only be used as a fragment density map
    /// attachment in a [`VkRenderPass`]. This layout is valid only for image subresources of
    /// images created with the [`VkImageUsageFlag::FragmentDensityMapBitExt`] usage flag set.
    ///
    /// Provided by [`ext_fragment_density_map`]
    FragmentDensityMapOptimalExt = 1000218000,

    /// [`VkImageLayout::FragmentShadingRateAttachmentOptimalKhr`] must only be used as a fragment
    /// shading rate attachment or shading rate image. This layout is valid only for image
    /// subresources of images created with the
    /// [`VkImageUsageFlag::FragmentShadingRateAttachmentBitKhr`] usage flag set.
    ///
    /// Provided by [`khr_fragment_shading_rate`]
    FragmentShadingRateAttachmentOptimalKhr = 1000164003,

    /// [`VkImageLayout::VideoEncodeDstKhr`] is reserved for future use.
    ///
    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeDstKhr = 1000299000,

    /// [`VkImageLayout::VideoEncodeSrcKhr`] must only be used as an encode input picture in a
    /// video encode operation. This layout is valid only for image subresources of images created
    /// with the [`VkImageUsageFlag::VideoEncodeSrcBitKhr`] usage flag set.
    ///
    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeSrcKhr = 1000299001,

    /// [`VkImageLayout::VideoEncodeDpbKhr`] must only be used as an output reconstructed picture
    /// or an input reference picture in a video encode operation. This layout is valid only for
    /// image subresources of images created with the [`VkImageUsageFlag::VideoEncodeDpbBitKhr`]
    /// usage flag set.
    ///
    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeDpbKhr = 1000299002,

    /// [`VkImageLayout::AttachmentFeedbackLoopOptimalExt`] must only be used as either a color
    /// attachment or depth/stencil attachment and/or read-only access in a shader as a sampled
    /// image, combined image/sampler, or input attachment. This layout is valid only for image
    /// subresources of images created with the [`VkImageUsageFlag::AttachmentFeedbackLoopBitExt`]
    /// usage flag set, and either the [`VkImageUsageFlag::ColorAttachmentBit`] or
    /// [`VkImageUsageFlag::DepthStencilAttachmentBit`] usage flags set, and either the
    /// [`VkImageUsageFlag::InputAttachmentBit`] or [`VkImageUsageFlag::SampledBit`] usage flags
    /// set
    ///
    /// Provided by [`ext_attachment_feedback_loop_layout`]
    AttachmentFeedbackLoopOptimalExt = 1000339000,

    /// [`VkImageLayout::TensorAliasingArm`] specifies the layout that an image created with
    /// [`VkImageTiling::Optimal`] must be in for it and a tensor bound to the same aliased range
    /// of memory to consistently interpret the data in memory. This layout is valid only for image
    /// subresources of images created with the [`VkImageUsageFlag::TensorAliasingBitArm`] usage
    /// flag set.
    ///
    /// Provided by [`arm_tensors`]
    TensorAliasingArm = 1000460000,

    /// [`VkImageLayout::VideoEncodeQuantizationMapKhr`] must only be used as a quantization map in
    /// a video encode operation. This layout is valid only for image subresources of images
    /// created with the [`VkImageUsageFlag::VideoEncodeQuantizationDeltaMapBitKhr`] or
    /// [`VkImageUsageFlag::VideoEncodeEmphasisMapBitKhr`] usage flags set.
    ///
    /// Provided by [`khr_video_encode_quantization_map`]
    VideoEncodeQuantizationMapKhr = 1000553000,

    /// [`VkImageLayout::ZeroInitializedExt`] specifies that an image’s memory is in a defined
    /// layout and is zeroed, but that it has not yet been initialized by the driver. Image memory
    /// cannot be transitioned into this layout. This layout can be used as the `initial_layout`
    /// member of [`VkImageCreateInfo`]. This layout is intended to be used as the initial layout
    /// for an image whose contents are already zeroed, either from being explicitly set to zero by
    /// an application or from being allocated with [`VkMemoryAllocateFlag::ZeroInitializeBitExt`].
    ///
    /// Provided by [`ext_zero_initialize_device_memory`]
    ZeroInitializedExt = 1000620000,
}
