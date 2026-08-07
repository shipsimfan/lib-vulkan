use crate::{VkImageLayout, VkImageView, VkSampler};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_FALSE, VK_NULL_HANDLE, VK_VERSION_1_0, VkDescriptorType, VkDevice, VkFormat};

/// Structure specifying descriptor image information
///
/// # Description
/// Members of [`VkDescriptorImageInfo`] that are not used in an update are ignored.
///
/// # Valid Usage (Implicit)
///  - Both of `image_view`, and `sampler` that are valid handles of non-ignored parameters must
///    have been created, allocated, or retrieved from the same [`VkDevice`]
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDescriptorImageInfo {
    /// `sampler` is a sampler handle, and is used in descriptor updates for types
    /// [`VkDescriptorType::Sampler`] and [`VkDescriptorType::CombinedImageSampler`] if the binding
    /// being updated does not use immutable samplers.
    ///
    /// # Valid Usage
    ///  - If `sampler` is used and the [`VkFormat`] of the image is a multi-planar format, the
    ///    image must have been created with [`VkImageCreateFlag::MutableFormat`], and the
    ///    `aspect_mask` of the `image_view` must be a valid multi-planar aspect mask bit
    ///  - If the [`khr_portability_subset`] extension is enabled, and
    ///    [`VkPhysicalDevicePortabilitySubsetFeaturesKhr::mutable_comparison_samplers`] is
    ///    [`VK_FALSE`], then `sampler` must have been created with
    ///    [`VkSamplerCreateInfo::compare_enable`] set to [`VK_FALSE`]
    pub sampler: VkSampler,

    /// `image_view` is [`VK_NULL_HANDLE`] or an image view handle, and is used in descriptor
    /// updates for types [`VkDescriptorType::SampledImage`], [`VkDescriptorType::StorageImage`],
    /// [`VkDescriptorType::CombinedImageSampler`], and [`VkDescriptorType::InputAttachment`].
    ///
    /// # Valid Usage
    ///  - `image_view` must not be a 2D array image view created from a 3D image
    ///  - If `image_view` is a 2D view created from a 3D image, then `descriptor_type` must be
    ///    [`VkDescriptorType::StorageImage`], [`VkDescriptorType::SampledImage`], or
    ///    [`VkDescriptorType::CombinedImageSampler`]
    ///  - If `image_view` is a 2D view created from a 3D image, then the image must have been
    ///    created with [`VkImageCreateFlag::2dViewCompatibleExt`] set
    ///  - If the `image_2d_view_of_3d` feature is not enabled or `descriptor_type` is not
    ///    [`VkDescriptorType::StorageImage`] then `image_view` must not be a 2D view created from
    ///    a 3D image
    ///  - If the `sampler_2d_view_of_3d` feature is not enabled or `descriptor_type` is not
    ///    [`VkDescriptorType::SampledImage`] or [`VkDescriptorType::CombinedImageSampler`] then
    ///    `image_view` must not be a 2D view created from a 3D image
    ///  - If `image_view` is created from a depth/stencil image, the `aspect_mask` used to create
    ///    the `image_view` must include either [`VkImageAspectFlag::Depth`] or
    ///    [`VkImageAspectFlag::Stencil`] but not both
    pub image_view: VkImageView,

    /// `image_layout` is the layout that the image subresources accessible from `image_view` will
    /// be in at the time this descriptor is accessed. `image_layout` is used in descriptor updates
    /// for types [`VkDescriptorType::SampledImage`], [`VkDescriptorType::StorageImage`],
    /// [`VkDescriptorType::CombinedImageSampler`], and [`VkDescriptorType::InputAttachment`].
    ///
    /// # Valid Usage
    ///  - If `image_layout` is [`VkImageLayout::ColorAttachmentOptimal`], then the `aspect_mask`
    ///    used to create `image_view` must not include either [`VkImageAspectFlag::Depth`] or
    ///    [`VkImageAspectFlag::Stencil`]
    ///  - If `image_layout` is [`VkImageLayout::DepthReadOnlyStencilAttachmentOptimal`],
    ///    [`VkImageLayout::DepthAttachmentStencilReadOnlyOptimal`],
    ///    [`VkImageLayout::DepthReadOnlyOptimal`], [`VkImageLayout::DepthAttachmentOptimal`],
    ///    [`VkImageLayout::StencilReadOnlyOptimal`], [`VkImageLayout::StencilAttachmentOptimal`],
    ///    [`VkImageLayout::DepthStencilAttachmentOptimal`] or
    ///    [`VkImageLayout::DepthStencilReadOnlyOptimal`], then the `aspect_mask` used to create
    ///    `image_view` must not include [`VkImageAspectFlag::Color`]
    pub image_layout: VkImageLayout,
}

const impl Default for VkDescriptorImageInfo {
    fn default() -> Self {
        VkDescriptorImageInfo {
            sampler: VkSampler::null(),
            image_view: VkImageView::null(),
            image_layout: VkImageLayout::Undefined,
        }
    }
}
