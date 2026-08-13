use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_TRUE, VK_VERSION_1_0, VK_VERSION_1_1, VkImageView, VkImageViewCreateInfo, VkImageViewType,
    VkPipelineStageFlag,
};

flags! {
    /// Bitmask of [`VkImageCreateFlag`]
    ///
    /// # Description
    /// [`VkImageCreateFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkImageCreateFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkImageCreateFlags;

    /// Bitmask specifying additional parameters of an image
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkImageCreateFlag {
        /// [`VkImageCreateFlag::SparseBinding`] specifies that the image will be backed using
        /// sparse memory binding.
        SparseBinding = 0x00000001,

        /// [`VkImageCreateFlag::SparseResidency`] specifies that the image can be partially backed
        /// using sparse memory binding. Images created with this flag must also be created with
        /// the [`VkImageCreateFlag::SparseBinding`] flag.
        SparseResidency = 0x00000002,

        /// [`VkImageCreateFlag::SparseAliased`] specifies that the image will be backed using
        /// sparse memory binding with memory ranges that might also simultaneously be backing
        /// another image (or another portion of the same image). Images created with this flag
        /// must also be created with the [`VkImageCreateFlag::SparseBinding`] flag.
        SparseAliased = 0x00000004,

        /// [`VkImageCreateFlag::MutableFormat`] specifies that the image can be used to create a
        /// [`VkImageView`] with a different format from the image. For multi-planar formats,
        /// [`VkImageCreateFlag::MutableFormat`] specifies that a [`VkImageView`] can be created
        /// of a plane of the image.
        MutableFormat = 0x00000008,

        /// [`VkImageCreateFlag::CubeCompatible`] specifies that the image can be used to create a
        /// [`VkImageView`] of type [`VkImageViewType::Cube`] or [`VkImageViewType::CubeArray`].
        CubeCompatible = 0x00000010,

        /// [`VkImageCreateFlag::Alias`] specifies that two images created with the same creation
        /// parameters and aliased to the same memory can interpret the contents of the memory
        /// consistently with each other, subject to the rules described in the Memory Aliasing
        /// section. This flag further specifies that each plane of a disjoint image can share an
        /// in-memory non-linear representation with single-plane images, and that a single-plane
        /// image can share an in-memory non-linear representation with a plane of a multi-planar
        /// disjoint image, according to the rules in Compatible Formats of Planes of Multi-Planar
        /// Formats. If the `next` chain includes a [`VkExternalMemoryImageCreateInfo`] or
        /// [`VkExternalMemoryImageCreateInfoNv`] structure whose `handle_types` member is not 0,
        /// it is as if [`VkImageCreateFlag::Alias`] is set.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        Alias = 0x00000400,

        /// [`VkImageCreateFlag::SplitInstanceBindRegions`] specifies that the image can be used
        /// with a non-zero value of the `split_instance_bind_region_count` member of a
        /// [`VkBindImageMemoryDeviceGroupInfo`] structure passed into [`VkBindImageMemory2`]. This
        /// flag also has the effect of making the image use the standard sparse image block
        /// dimensions.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        SplitInstanceBindRegions = 0x00000040,

        /// [`VkImageCreateFlag::2DArrayCompatible`] specifies that the image can be used to create
        /// a [`VkImageView`] of type [`VkImageViewType::_2d`] or [`VkImageViewType::_2dArray`].
        ///
        /// Provided by [`VK_VERSION_1_1`]
        _2dArrayCompatible = 0x00000020,

        /// [`VkImageCreateFlag::BlockTexelViewCompatible`] specifies that the image having a
        /// compressed format can be used to create a [`VkImageView`] with an uncompressed format
        /// where each texel in the image view corresponds to a compressed texel block of the
        /// image.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        BlockTexelViewCompatible = 0x00000080,

        /// [`VkImageCreateFlag::ExtendedUsage`] specifies that the image can be created with usage
        /// flags that are not supported for the format the image is created with but are supported
        /// for at least one format a [`VkImageView`] created from the image can have.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        ExtendedUsage = 0x00000100,

        /// [`VkImageCreateFlag::2DViewCompatibleExt`] specifies that the image can be used to
        /// create a [`VkImageView`] of type [`VkImageViewType::_2d`].
        ///
        /// Provided by [`VK_VERSION_1_1`]
        Protected = 0x00000800,

        /// [`VkImageCreateFlag::Disjoint`] specifies that an image with a multi-planar format must
        /// have each plane separately bound to memory, rather than having a single memory binding
        /// for the whole image; the presence of this bit distinguishes a disjoint image from an
        /// image without this bit set.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        Disjoint = 0x00000200,

        /// [`VkImageCreateFlag::CornerSampledNv`] specifies that the image is a corner-sampled
        /// image.
        ///
        /// Provided by [`nv_corner_sampled_image`]
        CornerSampledNv = 0x00002000,

        /// [`VkImageCreateFlag::DescriptorHeapCaptureReplayExt`] specifies that the image can be
        /// used with descriptor buffers when capturing and replaying (e.g. for trace capture and
        /// replay), see [`VkOpaqueCaptureDescriptorDataCreateInfoExt`] for more detail.
        ///
        /// Provided by [`ext_descriptor_heap`]
        DescriptorHeapCaptureReplayExt = 0x00010000,

        /// [`VkImageCreateFlag::SampleLocationsCompatibleDepthExt`] specifies that an image with a
        /// depth or depth/stencil format can be used with custom sample locations when used as a
        /// depth/stencil attachment.
        ///
        /// Provided by [`ext_sample_locations`]
        SampleLocationsCompatibleDepthExt = 0x00001000,

        /// [`VkImageCreateFlag::SubsampledExt`] specifies that an image can be in a subsampled
        /// format which may be more optimal when written as an attachment by a render pass that
        /// has a fragment density map attachment. Accessing a subsampled image has additional
        /// considerations:
        ///  - Image data read as an image sampler will have undefined values if the sampler was
        ///    not created with flags containing [`VkSamplerCreateFlag::SubsampledExt`] or was not
        ///    sampled through a combined embedded sampler and image mapping if using descriptor
        ///    heaps, or the use of a combined image sampler with an immutable sampler in
        ///    [`VkDescriptorSetLayoutBinding`].
        ///  - Image data read with an input attachment will have undefined values if the contents
        ///    were not written as an attachment in an earlier subpass of the same render pass.
        ///  - Image data read as an image sampler in the fragment shader will be additionally be
        ///    read by the device during [`VkPipelineStageFlag::VertexShader`] if
        ///    [`VkPhysicalDeviceFragmentDensityMap2PropertiesExt::subsampled_coarse_reconstruction_early_access`]
        ///    is [`VK_TRUE`] and the sampler was created with flags containing
        ///    [`VkSamplerCreateFlag::SubsampledCoarseReconstructionExt`].
        ///  - Image data read with load operations are resampled to the fragment density of the
        ///    render pass if
        ///    [`VkPhysicalDeviceFragmentDensityMap2PropertiesExt::subsampled_loads`] is
        ///    [`VK_TRUE`]. Otherwise, values of image data are undefined.
        ///  - Image contents outside of the render area take on undefined values if the image is
        ///    stored as a render pass attachment.
        ///
        /// Provided by [`ext_fragment_density_map`]
        SubsampledExt = 0x00004000,

        /// [`VkImageCreateFlag::MultisampledRenderToSingleSampledExt`] specifies that an image can
        /// be used with multisampled rendering as a single-sampled render pass attachment
        ///
        /// Provided by [`ext_multisampled_render_to_single_sampled`]
        MultisampledRenderToSingleSampledExt = 0x00040000,

        /// [`VkImageCreateFlag::2DViewCompatibleExt`] specifies that the image can be used to
        /// create a [`VkImageView`] of type [`VkImageViewType::_2d`].
        ///
        /// Provided by [`ext_image_2d_view_of_3d`]
        _2dViewCompatibleExt = 0x00020000,

        /// [`VkImageCreateFlag::VideoProfileIndependentKhr`] specifies that the image can be used
        /// in video coding operations without having to specify at image creation time the set of
        /// video profiles the image will be used with, except for images used only as DPB
        /// pictures, as long as the image is otherwise compatible with the video profile in
        /// question.
        ///
        /// Provided by [`khr_video_maintenance1`]
        VideoProfileIndependentKhr = 0x00100000,

        /// [`VkImageCreateFlag::FragmentDensityMapOffsetExt`] specifies that an image can be used
        /// in a render pass with non-zero fragment density map offsets. In a render pass with
        /// non-zero offsets, fragment density map attachments, input attachments, color
        /// attachments, depth/stencil attachment, resolve attachments, and preserve attachments
        /// must be created with [`VkImageCreateFlag::FragmentDensityMapOffsetExt`].
        ///
        /// Provided by [`ext_fragment_density_map_offset`]
        FragmentDensityMapOffsetExt = 0x00008000,

        /// [`VkImageCreateFlag::AliasSingleLayerDescriptorKhr`] specifies that a single layer
        /// image view created from this image can be accessed with both `arrayed` equal to 0 and
        /// `arrayed` equal to 1 in a shader. If accessed with `arrayed` equal to 1 in a shader,
        /// and [`VkImageViewCreateInfo::view_type`] is [`VkImageViewType::_1d`] or
        /// [`VkImageViewType::_2d`], the image is treated as having an array size of 1, and
        /// `subresource_range.base_array_layer` is accessed. If accessed with `arrayed` equal to 0
        /// in a shader, and [`VkImageViewCreateInfo::view_type`] is [`VkImageViewType::_1dArray`]
        /// or [`VkImageViewType::_2dArray`], the descriptor is accessed as-if the view was created
        /// with [`VkImageViewType::_1d`] or [`VkImageViewType::_2d`] respectively. Cube map view
        /// aliasing is not included by this flag bit. Also, image views created with a
        /// multi-planar format can not alias like this.
        ///
        /// Provided by [`khr_maintenance11`]
        AliasSingleLayerDescriptorKhr = 0x00400000,
    }
}
