use crate::{VkDescriptorType, VkSampler, VkShaderStageFlags};
use std::ptr::null;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VkDescriptorSetLayoutCreateFlag, VkDescriptorSetLayoutCreateInfo,
    VkShaderStageFlag,
};

/// Structure specifying a descriptor set layout binding
///
/// # Description
/// The above layout definition allows the descriptor bindings to be specified sparsely such that
/// not all binding numbers between 0 and the maximum binding number need to be specified in the
/// `bindings` array. Bindings that are not specified have a `descriptor_count` and `stage_flags`
/// of zero, and the value of `descriptor_type` is undefined. However, all binding numbers between
/// 0 and the maximum binding number in the [`VkDescriptorSetLayoutCreateInfo::bindings`] array may
/// consume memory in the descriptor set layout even if not all descriptor bindings are used,
/// though it should not consume additional memory from the descriptor pool.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDescriptorSetLayoutBinding {
    /// `binding` is the binding number of this entry and corresponds to a resource of the same
    /// binding number in the shader stages.
    pub binding: u32,

    /// `descriptor_type` is a [`VkDescriptorType`] specifying which type of resource descriptors
    /// are used for this binding.
    ///
    /// # Valid Usage
    ///  - If the `inline_uniform_block` feature is not enabled, `descriptor_type` must not be
    ///    [`VkDescriptorType::InlineUniformBlock`]
    ///  - If [`VkDescriptorSetLayoutCreateInfo::flags`] contains
    ///    [`VkDescriptorSetLayoutCreateFlag::EmbeddedImmutableSamplersExt`], `descriptor_type`
    ///    must be [`VkDescriptorType::Sampler`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `descriptor_type` must be a valid [`VkDescriptorType`] value
    pub descriptor_type: VkDescriptorType,

    /// `descriptor_count` is the number of descriptors contained in the binding, accessed in a
    /// shader as an array, except if `descriptor_type` is [`VkDescriptorType::InlineUniformBlock`]
    /// in which case `descriptor_count` is the size in bytes of the inline uniform block. If
    /// `descriptor_count` is zero this binding entry is reserved and the resource must not be
    /// accessed from any stage via this binding within any pipeline using the set layout.
    ///
    /// # Valid Usage
    ///  - If `descriptor_type` is [`VkDescriptorType::InlineUniformBlock`] then `descriptor_count`
    ///    must be a multiple of 4
    ///  - If `descriptor_type` is [`VkDescriptorType::InlineUniformBlock`] and
    ///    [`VkDescriptorSetLayoutCreateInfo::flags`] does not contain
    ///    [`VkDescriptorSetLayoutCreateFlag::DescriptorBufferExt`] then `descriptor_count` must be
    ///    less than or equal to
    ///    [`VkPhysicalDeviceInlineUniformBlockProperties::max_inline_uniform_block_size`]
    ///  - If [`VkDescriptorSetLayoutCreateInfo::flags`] contains
    ///    [`VkDescriptorSetLayoutCreateFlag::EmbeddedImmutableSamplersExt`], `descriptor_count`
    ///    must be less than or equal to 1
    pub descriptor_count: u32,

    /// `stage_flags` member is a bitmask of [`VkShaderStageFlag`]s specifying which pipeline
    /// shader stages can access a resource for this binding. [`VkShaderStageFlag::All`] is a
    /// shorthand specifying that all defined shader stages, including any additional stages
    /// defined by extensions, can access the resource.
    ///
    /// If a shader stage is not included in `stage_flags`, then a resource must not be accessed
    /// from that stage via this binding within any pipeline using the set layout. Other than input
    /// attachments which are limited to the fragment shader, there are no limitations on what
    /// combinations of stages can use a descriptor binding, and in particular a binding can be
    /// used by both graphics stages and the compute stage.
    ///
    /// # Valid Usage
    ///  - If `descriptor_count` is not 0, `stage_flags` must be [`VkShaderStageFlag::All`] or a
    ///    valid combination of other [`VkShaderStageFlag`] values
    ///  - If `descriptor_type` is [`VkDescriptorType::InputAttachment`] and `descriptor_count` is
    ///    not 0, then `stage_flags` must be 0 or [`VkShaderStageFlag::Fragment`]
    ///  - If [`VkDescriptorSetLayoutCreateInfo::flags`] contains
    ///    [`VkDescriptorSetLayoutCreateFlag::PerStageNv`], and `descriptor_count` is not 0, then
    ///    `stage_flags` must be a valid combination of [`VkShaderStageFlag::Vertex`],
    ///    [`VkShaderStageFlag::TessellationControl`],
    ///    [`VkShaderStageFlag::TessellationEvaluation`], [`VkShaderStageFlag::Geometry`],
    ///    [`VkShaderStageFlag::Fragment`] and [`VkShaderStageFlag::Compute`] values
    pub stage_flags: VkShaderStageFlags,

    /// `immutable_samplers` affects initialization of samplers. If `descriptor_type` specifies a
    /// [`VkDescriptorType::Sampler`] or [`VkDescriptorType::CombinedImageSampler`] type
    /// descriptor, then `immutable_samplers` can be used to initialize a set of immutable
    /// samplers. Immutable samplers are permanently bound into the set layout and must not be
    /// changed; updating a [`VkDescriptorType::Sampler`] descriptor with immutable samplers is not
    /// allowed and updates to a [`VkDescriptorType::CombinedImageSampler`] descriptor with
    /// immutable samplers does not modify the samplers (the image views are updated, but the
    /// sampler updates are ignored). If `immutable_samplers` is not [`null`], then it is a pointer
    /// to an array of sampler handles that will be copied into the set layout and used for the
    /// corresponding binding. Only the sampler handles are copied; the sampler objects must not be
    /// destroyed before the final use of the set layout and any descriptor pools and sets created
    /// using it. If `immutable_samplers` is [`null`], then the sampler slots are dynamic and
    /// sampler handles must be bound into descriptor sets using this layout. If `descriptor_type`
    /// is not one of these descriptor types, then `immutable_samplers` is ignored.
    ///
    /// # Valid Usage
    ///  - If `descriptor_type` is [`VkDescriptorType::Sampler`] or
    ///    [`VkDescriptorType::CombinedImageSampler`], and `descriptor_count` is not 0 and
    ///    `immutable_samplers` is not [`null`], `immutable_samplers` must be a valid pointer to an
    ///    array of `descriptor_count` valid [`VkSampler`] handles
    ///  - If `descriptor_type` is [`VkDescriptorType::CombinedImageSampler`], and
    ///    `descriptor_count` is not 0 and `immutable_samplers` is not [`null`], either each
    ///    element of `immutable_samplers` must be a [`VkSampler`] that enables sampler Y′CBCR
    ///    conversion or none of them enable sampler Y′CBCR conversion
    ///  - If `descriptor_type` is [`VkDescriptorType::Sampler`], each element of
    ///    `immutable_samplers` must not be a [`VkSampler`] object that enables sampler Y′CBCR
    ///    conversion
    ///  - If [`VkDescriptorSetLayoutCreateInfo::flags`] contains
    ///    [`VkDescriptorSetLayoutCreateFlag::EmbeddedImmutableSamplersExt`], and
    ///    `descriptor_count` is equal to 1, `immutable_samplers` must not be [`null`]
    ///  - The sampler objects indicated by `immutable_samplers` must not have a `border_color`
    ///    with one of the values [`VkBorderColor::FloatCustomExt`] or
    ///    [`VkBorderColor::IntCustomExt`]
    ///  - If `descriptor_type` is [`VkDescriptorType::MutableExt`], then `immutable_samplers` must
    ///    be [`null`]
    pub immutable_samplers: *const VkSampler,
}

const impl Default for VkDescriptorSetLayoutBinding {
    fn default() -> Self {
        VkDescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: VkDescriptorType::Sampler,
            descriptor_count: 0,
            stage_flags: VkShaderStageFlags::default(),
            immutable_samplers: null(),
        }
    }
}
