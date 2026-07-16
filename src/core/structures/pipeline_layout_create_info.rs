use crate::{
    VkDescriptorSetLayout, VkPipelineLayoutCreateFlags, VkPushConstantRange, VkStructureType,
    util::NextChain,
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_0, VkPhysicalDeviceLimits, VkPipelineLayoutCreateFlag};

/// Structure specifying the parameters of a newly created pipeline layout object
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkPipelineLayoutCreateInfo {
    /// `r#type` is a [`VkStructureType`]value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PipelineLayoutCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkPipelineLayoutCreateFlag`]s specifying options for pipeline
    /// layout creation.
    ///
    /// # Valid Usage
    ///  - If `flags` has [`VkPipelineLayoutCreateFlag::NoTaskShaderKhr`] set, then
    ///    [`VkPipelineLayoutCreateFlag::IndependentSetsExt`] must also be set
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of [`VkPipelineLayoutCreateFlag`]s values
    pub flags: VkPipelineLayoutCreateFlags,

    /// `set_layout_count` is the number of descriptor sets included in the pipeline layout.
    ///
    /// # Valid Usage
    ///  - `set_layout_count` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_bound_descriptor_sets`]
    pub set_layout_count: u32,

    /// `set_layouts` is a pointer to an array of [`VkDescriptorSetLayout`] objects. The
    /// implementation must not access these objects outside of the duration of the command this
    /// structure is passed to.
    ///
    /// # Valid Usage
    ///  - The total number of descriptors in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::Sampler`] and
    ///    [`VkDescriptorType::CombinedImageSampler`] accessible to any given shader stage across
    ///    all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_per_stage_descriptor_samplers`]
    ///  - The total number of descriptors in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::UniformBuffer`] and
    ///    [`VkDescriptorType::UniformBufferDynamic`] accessible to any given shader stage across
    ///    all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_per_stage_descriptor_uniform_buffers`]
    ///  - The total number of descriptors in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::StorageBuffer`] and
    ///    [`VkDescriptorType::StorageBufferDynamic`] accessible to any given shader stage across
    ///    all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_per_stage_descriptor_storage_buffers`]
    ///  - The total number of descriptors in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::CombinedImageSampler`],
    ///    [`VkDescriptorType::SampledImage`], [`VkDescriptorType::SampleWeightImageQcom`],
    ///    [`VkDescriptorType::BlockMatchImageQcom`], and [`VkDescriptorType::UniformTexelBuffer`],
    ///    accessible to any given shader stage across all elements of `set_layouts` must be less
    ///    than or equal to [`VkPhysicalDeviceLimits::max_per_stage_descriptor_sampled_images`]
    ///  - The total number of descriptors in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::StorageImage`], and
    ///    [`VkDescriptorType::StorageTexelBuffer`] accessible to any given shader stage across all
    ///    elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_per_stage_descriptor_storage_images`]
    ///  - The total number of descriptors in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::InputAttachment`] accessible to any given
    ///    shader stage across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_per_stage_descriptor_input_attachments`]
    ///  - The total number of bindings in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set and with a
    ///    `descriptor_type` of [`VkDescriptorType::InlineUniformBlock`] accessible to any given
    ///    shader stage across all elements of `set_layouts`, must be less than or equal to
    ///    [`VkPhysicalDeviceInlineUniformBlockProperties::max_per_stage_descriptor_inline_uniform_blocks`]
    ///  - If the `descriptor_binding_sampled_image_update_after_bind` feature is supported on the
    ///    device, the total number of descriptors with a `descriptor_type` of
    ///    [`VkDescriptorType::Sampler`] and [`VkDescriptorType::CombinedImageSampler`] accessible
    ///    to any given shader stage across all elements of `set_layouts` must be less than or
    ///    equal to
    ///    [`VkPhysicalDeviceDescriptorIndexingProperties::max_per_stage_descriptor_update_after_bind_samplers`]
    ///  - If the `descriptor_binding_uniform_buffer_update_after_bind` feature is supported on the
    ///    device, the total number of descriptors with a `descriptor_type` of
    ///    [`VkDescriptorType::UniformBuffer`] and [`VkDescriptorType::UniformBufferDynamic`]
    ///    accessible to any given shader stage across all elements of `set_layouts` must be less
    ///    than or equal to
    ///    [`VkPhysicalDeviceDescriptorIndexingProperties::max_per_stage_descriptor_update_after_bind_uniform_buffers`]
    ///  - If the `descriptor_binding_storage_buffer_update_after_bind` feature is supported on the
    ///    device, the total number of descriptors with a `descriptor_type` of
    ///    [`VkDescriptorType::StorageBuffer`] and [`VkDescriptorType::StorageBufferDynamic`]
    ///    accessible to any given shader stage across all elements of `set_layouts` must be less
    ///    than or equal to
    ///    [`VkPhysicalDeviceDescriptorIndexingProperties::max_per_stage_descriptor_update_after_bind_storage_buffers`]
    ///  - If the `descriptor_binding_sampled_image_update_after_bind` feature is supported on the
    ///    device, the total number of descriptors with a `descriptor_type` of
    ///    [`VkDescriptorType::CombinedImageSampler`], [`VkDescriptorType::SampledImage`], and
    ///    [`VkDescriptorType::UniformTexelBuffer`] accessible to any given shader stage across all
    ///    elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceDescriptorIndexingProperties::max_per_stage_descriptor_update_after_bind_sampled_images`]
    ///  - If the `descriptor_binding_storage_image_update_after_bind` feature is supported on the
    ///    device, the total number of descriptors with a `descriptor_type` of
    ///    [`VkDescriptorType::StorageImage`], and [`VkDescriptorType::StorageTexelBuffer`]
    ///    accessible to any given shader stage across all elements of `set_layouts` must be less
    ///    than or equal to
    ///    [`VkPhysicalDeviceDescriptorIndexingProperties::max_per_stage_descriptor_update_after_bind_storage_images`]
    ///  - If any element of `set_layouts` is created with the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set, the total number of
    ///    descriptors with a `descriptor_type` of [`VkDescriptorType::InputAttachment`] accessible
    ///    to any given shader stage across all elements of `set_layouts` must be less than or
    ///    equal to
    ///    [`VkPhysicalDeviceDescriptorIndexingProperties::max_per_stage_descriptor_update_after_bind_input_attachments`]
    ///  - If the `descriptor_binding_inline_uniform_block_update_after_bind` feature is supported
    ///    on the device, the total number of bindings with a `descriptor_type` of
    ///    [`VkDescriptorType::InlineUniformBlock`] accessible to any given shader stage across all
    ///    elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceInlineUniformBlockProperties::max_per_stage_descriptor_update_after_bind_inline_uniform_blocks`]
    ///  - The total number of descriptors in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::Sampler`] and
    ///    [`VkDescriptorType::CombinedImageSampler`] accessible across all shader stages and
    ///    across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_descriptor_set_samplers`]
    ///  - The total number of descriptors in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::UniformBuffer`] accessible across all shader
    ///    stages and across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_descriptor_set_uniform_buffers`]
    ///  - If the `maintenance7` feature is not enabled, the total number of descriptors in
    ///    descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::UniformBufferDynamic`] accessible across all
    ///    shader stages and across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_descriptor_set_uniform_buffers_dynamic`]
    ///  - If the `maintenance7` feature is enabled, the total number of descriptors in descriptor
    ///    set layouts created without the [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`]
    ///    bit set with a `descriptor_type` of [`VkDescriptorType::UniformBufferDynamic`]
    ///    accessible across all shader stages and across all elements of `set_layouts` must be
    ///    less than or equal to
    ///    [`VkPhysicalDeviceMaintenance7PropertiesKhr::max_descriptor_set_total_uniform_buffers_dynamic`]
    ///  - The total number of descriptors in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::StorageBuffer`] accessible across all shader
    ///    stages and across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_descriptor_set_storage_buffers`]
    ///  - If the `maintenance7` feature is not enabled, the total number of descriptors in
    ///    descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::StorageBufferDynamic`] accessible across all
    ///    shader stages and across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_descriptor_set_storage_buffers_dynamic`]
    ///  - If the `maintenance7` feature is enabled, the total number of descriptors in descriptor
    ///    set layouts created without the [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`]
    ///    bit set with a `descriptor_type` of [`VkDescriptorType::StorageBufferDynamic`]
    ///    accessible across all shader stages and across all elements of `set_layouts` must be
    ///    less than or equal to
    ///    [`VkPhysicalDeviceMaintenance7PropertiesKhr::max_descriptor_set_total_storage_buffers_dynamic`]
    ///  - The total number of descriptors in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::UniformBufferDynamic`] or
    ///    [`VkDescriptorType::StorageBufferDynamic`] accessible across all shader stages and
    ///    across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceMaintenance7PropertiesKhr::max_descriptor_set_total_buffers_dynamic`]
    ///  - If either the `descriptor_binding_storage_buffer_update_after_bind` or
    ///    `descriptor_binding_uniform_buffer_update_after_bind` feature is supported on the
    ///    device, the total number of descriptors of the type
    ///    [`VkDescriptorType::UniformBufferDynamic`] or [`VkDescriptorType::StorageBufferDynamic`]
    ///    accessible across all shader stages and across all elements of `set_layouts` must be
    ///    less than or equal to
    ///    [`VkPhysicalDeviceMaintenance7PropertiesKhr::max_descriptor_set_update_after_bind_total_buffers_dynamic`]
    ///  - The total number of descriptors in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::CombinedImageSampler`],
    ///    [`VkDescriptorType::SampledImage`], and [`VkDescriptorType::UniformTexelBuffer`]
    ///    accessible across all shader stages and across all elements of `set_layouts` must be
    ///    less than or equal to [`VkPhysicalDeviceLimits::max_descriptor_set_sampled_images`]
    ///  - The total number of descriptors in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::StorageImage`], and
    ///    [`VkDescriptorType::StorageTexelBuffer`] accessible across all shader stages and across
    ///    all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_descriptor_set_storage_images`]
    ///  - The total number of descriptors in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::InputAttachment`] accessible across all shader
    ///    stages and across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_descriptor_set_input_attachments`]
    ///  - The total number of bindings in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::InlineUniformBlock`] accessible across all
    ///    shader stages and across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceInlineUniformBlockProperties::max_descriptor_set_inline_uniform_blocks`]
    ///  - If the `descriptor_binding_sampled_image_update_after_bind` feature is supported on the
    ///    device, the total number of descriptors of the type [`VkDescriptorType::Sampler`] and
    ///    [`VkDescriptorType::CombinedImageSampler`] accessible across all shader stages and
    ///    across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceDescriptorIndexingProperties::max_descriptor_set_update_after_bind_samplers`]
    ///  - If the `descriptor_binding_uniform_buffer_update_after_bind` feature is supported on the
    ///    device, the total number of descriptors of the type [`VkDescriptorType::UniformBuffer`]
    ///    accessible across all shader stages and across all elements of `set_layouts` must be
    ///    less than or equal to
    ///    [`VkPhysicalDeviceDescriptorIndexingProperties::max_descriptor_set_update_after_bind_uniform_buffers`]
    ///  - If the `descriptor_binding_uniform_buffer_update_after_bind` feature is supported on the
    ///    device, and if the `maintenance7` feature is not enabled, the total number of
    ///    descriptors of the type [`VkDescriptorType::UniformBufferDynamic`] accessible across all
    ///    shader stages and across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_descriptor_set_update_after_bind_uniform_buffers_dynamic`]
    ///  - If the `descriptor_binding_uniform_buffer_update_after_bind` feature is supported on the
    ///    device, and the `maintenance7` feature is enabled, the total number of descriptors of
    ///    the type [`VkDescriptorType::UniformBufferDynamic`] accessible across all shader stages
    ///    and across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceMaintenance7PropertiesKhr::max_descriptor_set_update_after_bind_total_uniform_buffers_dynamic`]
    ///  - If the `descriptor_binding_storage_buffer_update_after_bind` feature is supported on the
    ///    device, the total number of descriptors of the type [`VkDescriptorType::StorageBuffer`]
    ///    accessible across all shader stages and across all elements of `set_layouts` must be
    ///    less than or equal to
    ///    [`VkPhysicalDeviceDescriptorIndexingProperties::max_descriptor_set_update_after_bind_storage_buffers`]
    ///  - If the `descriptor_binding_storage_buffer_update_after_bind` feature is supported on the
    ///    device, and if the `maintenance7` feature is not enabled, the total number of
    ///    descriptors of the type [`VkDescriptorType::StorageBufferDynamic`] accessible across all
    ///    shader stages and across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_descriptor_set_update_after_bind_storage_buffers_dynamic`]
    ///  - If the `descriptor_binding_storage_buffer_update_after_bind` feature is supported on the
    ///    device, and if the `maintenance7` feature is enabled, the total number of descriptors of
    ///    the type [`VkDescriptorType::StorageBufferDynamic`] accessible across all shader stages
    ///    and across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceMaintenance7PropertiesKhr::max_descriptorSetUpdateAfterBindTotalStorageBuffersDynamic`]
    ///  - If the `descriptor_binding_sampled_image_update_after_bind` feature is supported on the
    ///    device, the total number of descriptors of the type
    ///    [`VkDescriptorType::CombinedImageSampler`], [`VkDescriptorType::SampledImage`], and
    ///    [`VkDescriptorType::UniformTexelBuffer`] accessible across all shader stages and across
    ///    all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceDescriptorIndexingProperties::max_descriptor_set_update_after_bind_sampled_images`]
    ///  - If the `descriptor_dinding_storage_image_update_after_bind` feature is supported on the
    ///    device, the total number of descriptors of the type [`VkDescriptorType::StorageImage`],
    ///    and [`VkDescriptorType::StorageTexelBuffer`] accessible across all shader stages and
    ///    across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceDescriptorIndexingProperties::max_descriptor_set_update_after_bind_storage_images`]
    ///  - If any element of `set_layouts` is created with the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set, the total number of
    ///    descriptors of the type [`VkDescriptorType::InputAttachment`] accessible across all
    ///    shader stages and across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceDescriptorIndexingProperties::max_descriptor_set_update_after_bind_input_attachments`]
    ///  - If the `descriptor_binding_inline_uniform_block_update_after_bind` feature is supported
    ///    on the device, the total number of bindings with a `descriptor_type` of
    ///    [`VkDescriptorType::InlineUniformBlock`] accessible across all shader stages and across
    ///    all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceInlineUniformBlockProperties::max_descriptor_set_update_after_bind_inline_uniform_blocks`]
    ///  - The total number of descriptors with a `descriptor_type` of
    ///    [`VkDescriptorType::InlineUniformBlock`] accessible across all shader stages and across
    ///    all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceVulkan13Properties::max_inline_uniform_total_size`]
    ///  - `set_layouts` must not contain more than one descriptor set layout that was created with
    ///    [`VkDescriptorSetLayoutCreateFlag::PushDescriptor`] set
    ///  - The total number of bindings in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::AccelerationStructureKhr`] accessible to any
    ///    given shader stage across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceAccelerationStructurePropertiesKhr::max_per_stage_descriptor_acceleration_structures`]
    ///  - The total number of bindings with a `descriptor_type` of
    ///    [`VkDescriptorType::AccelerationStructureKhr`] accessible to any given shader stage
    ///    across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceAccelerationStructurePropertiesKhr::max_per_stage_descriptor_update_after_bind_acceleration_structures`]
    ///  - The total number of bindings in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::AccelerationStructureKhr`] accessible across
    ///    all shader stages and across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceAccelerationStructurePropertiesKhr::max_descriptor_set_acceleration_structures`]
    ///  - The total number of bindings with a `descriptor_type` of
    ///    [`VkDescriptorType::AccelerationStructureKhr`] accessible across all shader stages and
    ///    across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceAccelerationStructurePropertiesKhr::max_descriptor_set_update_after_bind_acceleration_structures`]
    ///  - The total number of bindings with a `descriptor_type` of
    ///    [`VkDescriptorType::AccelerationStructureNv`] accessible across all shader stages and
    ///    across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceRayTracingPropertiesNv::max_descriptor_set_acceleration_structures`]
    ///  - The total number of `immutable_samplers` created with flags containing
    ///    [`VkSamplerCreateFlag::SubsampledExt`] or
    ///    [`VkSamplerCreateFlag::SubsampledCoarseReconstructionExt`] across all shader stages and
    ///    across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceFragmentDensityMap2PropertiesExt::max_descriptor_set_subsampled_samplers`]
    ///  - Any element of `set_layouts` must not have been created with the
    ///    [`VkDescriptorSetLayoutCreateFlag::HostOnlyPoolExt`] bit set
    ///  - If the `graphics_pipeline_library` feature is not enabled, elements of `set_layouts`
    ///    must be valid [`VkDescriptorSetLayout`] objects
    ///  - If any element of `set_layouts` was created with the
    ///    [`VkDescriptorSetLayoutCreateFlag::DescriptorBufferExt`] bit set, all elements of
    ///    `set_layouts` must have been created with the
    ///    [`VkDescriptorSetLayoutCreateFlag::DescriptorBufferExt`] bit set
    ///  - The total number of descriptors in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::TensorArm`] accessible to any given shader
    ///    stage across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceTensorPropertiesArm::max_per_stage_descriptor_set_storage_tensors`]
    ///  - The total number of descriptors in descriptor set layouts created without the
    ///    [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set with a
    ///    `descriptor_type` of [`VkDescriptorType::TensorArm`] accessible across all shader stages
    ///    and across all elements of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceTensorPropertiesArm::max_descriptor_set_storage_tensors`]
    ///  - The total number of descriptors of the type [`VkDescriptorType::TensorArm`] accessible
    ///    across all shader stages and across all elements of `set_layouts` must be less than or
    ///    equal to [`VkPhysicalDeviceTensorPropertiesArm::max_descriptor_set_update_after_bind_storage_tensors`]
    ///  - The total number of descriptors with a `descriptor_type` of
    ///    [`VkDescriptorType::TensorArm`] accessible to any given shader stage across all elements
    ///    of `set_layouts` must be less than or equal to
    ///    [`VkPhysicalDeviceTensorPropertiesArm::max_per_stage_descriptor_update_after_bind_storage_tensors`]
    ///
    /// # Valid Usage (Implicit)
    ///  - If `set_layout_count` is not 0, `set_layouts` must be a valid pointer to an array of
    ///    `set_layout_count` valid or [`VK_NULL_HANDLE`] [`VkDescriptorSetLayout`] handles
    pub set_layouts: *const VkDescriptorSetLayout,

    /// `push_constant_range_count` is the number of push constant ranges included in the pipeline
    /// layout.
    pub push_constant_range_count: u32,

    /// `push_constant_ranges` is a pointer to an array of [`VkPushConstantRange`] structures
    /// defining a set of push constant ranges for use in a single pipeline layout. In addition to
    /// descriptor set layouts, a pipeline layout also describes how many push constants can be
    /// accessed by each stage of the pipeline.
    ///
    /// # Valid Usage
    ///  - Any two elements of `push_constant_ranges` must not include the same stage in
    ///    `stage_flags`
    ///
    /// # Valid Usage (Implicit)
    ///  - If `push_constant_range_count` is not 0, `push_constant_ranges` must be a valid pointer
    ///    to an array of `push_constant_range_count` valid [`VkPushConstantRange`] structures
    pub push_constant_ranges: *const VkPushConstantRange,
}

const impl Default for VkPipelineLayoutCreateInfo {
    fn default() -> Self {
        VkPipelineLayoutCreateInfo {
            r#type: VkStructureType::PipelineLayoutCreateInfo,
            next: null(),
            flags: VkPipelineLayoutCreateFlags::empty(),
            set_layout_count: 0,
            set_layouts: null(),
            push_constant_range_count: 0,
            push_constant_ranges: null(),
        }
    }
}

impl NextChain for VkPipelineLayoutCreateInfo {
    fn structure_type(&self) -> VkStructureType {
        self.r#type
    }

    fn next(&self) -> *const c_void {
        self.next
    }

    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: Option<&dyn NextChain>) {
        self.next = next.map_or(null(), |n| n.as_ptr());
    }
}
