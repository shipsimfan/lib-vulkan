use crate::{VkBool32, VkDeviceSize, VkSampleCountFlags};
use std::ffi::c_size_t;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_FALSE, VK_TRUE, VK_VERSION_1_0, VkAllocateMemory, VkBuffer, VkBufferCreateInfo,
    VkBufferUsageFlag, VkCreateSampler, VkDescriptorBufferInfo, VkDescriptorSetLayoutBinding,
    VkDescriptorSetLayoutCreateFlag, VkDescriptorType, VkDeviceMemory, VkDeviceQueueCreateInfo,
    VkImage, VkImageCreateFlag, VkImageCreateInfo, VkImageTiling, VkImageType, VkImageUsageFlag,
    VkMapMemory, VkPipelineLayoutCreateInfo, VkPipelineRasterizationStateCreateInfo,
    VkPipelineTessellationStateCreateInfo, VkPipelineVertexInputStateCreateInfo,
    VkPipelineViewportStateCreateInfo, VkQueueFamilyProperties, VkQueueFlag, VkSampleCountFlag,
    VkSamplerCreateInfo, VkUpdateDescriptorSets, VkVertexInputAttributeDescription,
    VkVertexInputBindingDescription,
};

/// Structure reporting implementation-dependent physical device limits
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct VkPhysicalDeviceLimits {
    /// `max_image_dimension_1d` is the largest dimension (`width`) that is guaranteed to be
    /// supported for all images created with an `image_type` of [`VkImageType::_1d`]. Some
    /// combinations of image parameters (format, usage, etc.) may allow support for larger
    /// dimensions, which can be queried using [`VkGetPhysicalDeviceImageFormatProperties`].
    pub max_image_dimension_1d: u32,

    /// `max_image_dimension_2d` is the largest dimension (`width` or `height`) that is guaranteed
    /// to be supported for all images created with an `image_type` of [`VkImageType::_2d`] and
    /// without [`VkImageCreateFlag::CubeCompatible`] set in `flags`. Some combinations of image
    /// parameters (format, usage, etc.) may allow support for larger dimensions, which can be
    /// queried using [`VkGetPhysicalDeviceImageFormatProperties`].
    pub max_image_dimension_2d: u32,

    /// `max_image_dimension_3d` is the largest dimension (`width`, `height`, or `depth`) that is
    /// guaranteed to be supported for all images created with an `image_type` of
    /// [`VkImageType::_3d`]. Some combinations of image parameters (format, usage, etc.) may allow
    /// support for larger dimensions, which can be queried using
    /// [`VkGetPhysicalDeviceImageFormatProperties`].
    pub max_image_dimension_3d: u32,

    /// `max_image_dimension_cube` is the largest dimension (`width` or `height`) that is
    /// guaranteed to be supported for all images created with an `image_type` of
    /// [`VkImageType::_2d`] and with [`VkImageCreateFlag::CubeCompatible`] set in `flags`. Some
    /// combinations of image parameters (format, usage, etc.) may allow support for larger
    /// dimensions, which can be queried using [`VkGetPhysicalDeviceImageFormatProperties`].
    pub max_image_dimension_cube: u32,

    /// `max_image_array_layers` is the maximum number of layers (`array_layers`) for an image.
    pub max_image_array_layers: u32,

    /// `max_texel_buffer_elements` is the maximum number of addressable texels for a buffer view
    /// created on a buffer which was created with the [`VkBufferUsageFlag::UniformTexelBuffer`] or
    /// [`VkBufferUsageFlag::StorageTexelBuffer`] usage flag set.
    pub max_texel_buffer_elements: u32,

    ///  `max_uniform_buffer_range` is the maximum value that can be specified in the range member
    /// of a [`VkDescriptorBufferInfo`] structure passed to [`VkUpdateDescriptorSets`] for
    /// descriptors of type [`VkDescriptorType::UniformBuffer`] or
    /// [`VkDescriptorType::UniformBufferDynamic`].
    pub max_uniform_buffer_range: u32,

    ///  `max_storage_buffer_range` is the maximum value that can be specified in the range member
    /// of a [`VkDescriptorBufferInfo`] structure passed to [`VkUpdateDescriptorSets`] for
    /// descriptors of type [`VkDescriptorType::StorageBuffer`] or
    /// [`VkDescriptorType::StorageBufferDynamic`]. If the `shader_64_bit_indexing` feature is
    /// enabled, this limit does not apply.
    pub max_storage_buffer_range: u32,

    /// `max_push_constants_size` is the maximum size, in bytes, of the pool of push constant
    /// memory. For each of the push constant ranges indicated by the `push_constant_ranges` member
    /// of the [`VkPipelineLayoutCreateInfo`] structure, `offset + size` must be less than or equal
    /// `max_push_constants_size` this limit.
    pub max_push_constants_size: u32,

    /// `max_memory_allocation_count` is the maximum number of device memory allocations, as
    /// `max_memory_allocation_count` by [`VkAllocateMemory`], which can simultaneously exist.
    pub max_memory_allocation_count: u32,

    /// `max_sampler_allocation_count` is the maximum number of sampler objects, as created by
    /// [`VkCreateSampler`], which can simultaneously exist on a device. If the `descriptor_heap`
    /// feature is enabled and the application intends to use embedded samplers, the number
    /// advertised here is effectively reduced by the quotient of
    /// `min_sampler_heap_reserved_range_with_embedded` divided by `sampler_descriptor_size`, to
    /// provide storage for embedded samplers when switching to heaps. If embedded samplers are not
    /// used, this can be ignored.
    pub max_sampler_allocation_count: u32,

    /// `buffer_image_granularity` is the granularity, in bytes, at which buffer or linear image
    /// resources, linear or optimal tensor resources, and optimal image resources can be bound to
    /// adjacent offsets in the same [`VkDeviceMemory`] object without aliasing.
    pub buffer_image_granularity: VkDeviceSize,

    /// `sparse_address_space_size` is the total amount of address space available, in bytes, for
    /// sparse memory resources. This is an upper bound on the sum of the sizes of all sparse
    /// resources, regardless of whether any memory is bound to them. If the
    /// `extended_sparse_address_space` feature is enabled, then the difference between
    /// `extended_sparse_address_space_size` and `sparse_address_space_size` can also be used, by
    /// [`VkImage`] created with the usage member of [`VkImageCreateInfo`] only containing bits in
    /// `extended_sparse_image_usage_flags` and [`VkBuffer`] created with the usage member of
    /// [`VkBufferCreateInfo`] only containing bits in `extended_sparse_buffer_usage_flags`.
    pub sparse_address_space_size: VkDeviceSize,

    /// `max_bound_descriptor_sets` is the maximum number of descriptor sets that can be
    /// simultaneously used by a pipeline.
    pub max_bound_descriptor_sets: u32,

    /// `max_per_stage_descriptor_samplers` is the maximum number of samplers that can be
    /// accessible to a single shader stage in a pipeline layout. Descriptors with a type of
    /// [`VkDescriptorType::Sampler`] or [`VkDescriptorType::CombinedImageSampler`] count against
    /// this limit. Only descriptors in descriptor set layouts created without the
    /// [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set count against this limit.
    /// A descriptor is accessible to a shader stage when the `stage_flags` member of the
    /// [`VkDescriptorSetLayoutBinding`] structure has the bit for that shader stage set.
    pub max_per_stage_descriptor_samplers: u32,

    /// `max_per_stage_descriptor_uniform_buffers` is the maximum number of uniform buffers that
    /// can be accessible to a single shader stage in a pipeline layout. Descriptors with a type of
    /// [`VkDescriptorType::UniformBuffer`] or [`VkDescriptorType::UniformBufferDynamic`] count
    /// against this limit. Only descriptors in descriptor set layouts created without the
    /// [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set count against this limit.
    /// A descriptor is accessible to a shader stage when the `stage_flags` member of the
    /// [`VkDescriptorSetLayoutBinding`] structure has the bit for that shader stage set.
    pub max_per_stage_descriptor_uniform_buffers: u32,

    /// `max_per_stage_descriptor_storage_buffers` is the maximum number of storage buffers that
    /// can be accessible to a single shader stage in a pipeline layout. Descriptors with a type of
    /// [`VkDescriptorType::StorageBuffer`] or [`VkDescriptorType::StorageBufferDynamic`] count
    /// against this limit. Only descriptors in descriptor set layouts created without the
    /// [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set count against this limit.
    /// A descriptor is accessible to a pipeline shader stage when the `stage_flags` member of the
    /// [`VkDescriptorSetLayoutBinding`] structure has the bit for that shader stage set.
    pub max_per_stage_descriptor_storage_buffers: u32,

    /// `max_per_stage_descriptor_sampled_images` is the maximum number of sampled images that can
    /// be accessible to a single shader stage in a pipeline layout. Descriptors with a type of
    /// [`VkDescriptorType::CombinedImageSampler`], [`VkDescriptorType::SampledImage`], or
    /// [`VkDescriptorType::UniformTexelBuffer`] count against this limit. Only descriptors in
    /// descriptor set layouts created without the
    /// [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set count against this limit.
    /// A descriptor is accessible to a pipeline shader stage when the `stage_flags` member of the
    /// [`VkDescriptorSetLayoutBinding`] structure has the bit for that shader stage set.
    pub max_per_stage_descriptor_sampled_images: u32,

    /// `max_per_stage_descriptor_storage_images` is the maximum number of storage images that can
    /// be accessible to a single shader stage in a pipeline layout. Descriptors with a type of
    /// [`VkDescriptorType::StorageImage`], or [`VkDescriptorType::StorageTexelBuffer`] count
    /// against this limit. Only descriptors in descriptor set layouts created without the
    /// [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set count against this limit.
    /// A descriptor is accessible to a pipeline shader stage when the `stage_flags` member of the
    /// [`VkDescriptorSetLayoutBinding`] structure has the bit for that shader stage set.
    pub max_per_stage_descriptor_storage_images: u32,

    /// `max_per_stage_descriptor_input_attachments` is the maximum number of input attachments
    /// that can be accessible to a single shader stage in a pipeline layout, as well as the
    /// maximum usable input attachment index. Descriptors with a type of
    /// [`VkDescriptorType::InputAttachment`] count against this limit. Only descriptors in
    /// descriptor set layouts created without the
    /// [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set count against this limit.
    /// A descriptor is accessible to a pipeline shader stage when the `stage_flags` member of the
    /// [`VkDescriptorSetLayoutBinding`] structure has the bit for that shader stage set. These are
    /// only supported for the fragment stage.
    pub max_per_stage_descriptor_input_attachments: u32,

    /// `max_per_stage_resources` is the maximum number of resources that can be accessible to a
    /// single shader stage in a pipeline layout. Descriptors with a type of
    /// [`VkDescriptorType::CombinedImageSampler`], [`VkDescriptorType::SampledImage`],
    /// [`VkDescriptorType::StorageImage`], [`VkDescriptorType::UniformTexelBuffer`],
    /// [`VkDescriptorType::StorageTexelBuffer`], [`VkDescriptorType::UniformBuffer`],
    /// [`VkDescriptorType::StorageBuffer`], [`VkDescriptorType::UniformBufferDynamic`],
    /// [`VkDescriptorType::StorageBufferDynamic`], or [`VkDescriptorType::InputAttachment`] count
    /// against this limit. Only descriptors in descriptor set layouts created without the
    /// [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set count against this limit.
    /// For the fragment shader stage the framebuffer color attachments also count against this
    /// limit.
    pub max_per_stage_resources: u32,

    /// `max_descriptor_set_samplers` is the maximum number of samplers that can be included in a
    /// pipeline layout. Descriptors with a type of [`VkDescriptorType::Sampler`] or
    /// [`VkDescriptorType::CombinedImageSampler`] count against this limit. Only descriptors in
    /// descriptor set layouts created without the
    /// [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set count against this limit.
    pub max_descriptor_set_samplers: u32,

    /// `max_descriptor_set_uniform_buffers` is the maximum number of uniform buffers that can be
    /// included in a pipeline layout. Descriptors with a type of
    /// [`VkDescriptorType::UniformBuffer`] or [`VkDescriptorType::UniformBufferDynamic`] count
    /// against this limit. Only descriptors in descriptor set layouts created without the
    /// [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set count against this limit.
    pub max_descriptor_set_uniform_buffers: u32,

    /// `max_descriptor_set_uniform_buffers_dynamic` is the maximum number of dynamic uniform
    /// buffers that can be included in a pipeline layout. Descriptors with a type of
    /// [`VkDescriptorType::UniformBufferDynamic`] count against this limit. Only descriptors in
    /// descriptor set layouts created without the
    /// [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set count against this limit.
    pub max_descriptor_set_uniform_buffers_dynamic: u32,

    /// `max_descriptor_set_storage_buffers` is the maximum number of storage buffers that can be
    /// included in a pipeline layout. Descriptors with a type of
    /// [`VkDescriptorType::StorageBuffer`] or [`VkDescriptorType::StorageBufferDynamic`] count
    /// against this limit. Only descriptors in descriptor set layouts created without the
    /// [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set count against this limit.
    pub max_descriptor_set_storage_buffers: u32,

    /// `max_descriptor_set_storage_buffers_dynamic` is the maximum number of dynamic storage
    /// buffers that can be included in a pipeline layout. Descriptors with a type of
    /// [`VkDescriptorType::StorageBufferDynamic`] count against this limit. Only descriptors in
    /// descriptor set layouts created without the
    /// [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set count against this limit.
    pub max_descriptor_set_storage_buffers_dynamic: u32,

    /// `max_descriptor_set_sampled_images` is the maximum number of sampled images that can be
    /// included in a pipeline layout. Descriptors with a type of
    /// [`VkDescriptorType::CombinedImageSampler`], [`VkDescriptorType::SampledImage`], or
    /// [`VkDescriptorType::UniformTexelBuffer`] count against this limit. Only descriptors in
    /// descriptor set layouts created without the
    /// [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set count against this limit.
    pub max_descriptor_set_sampled_images: u32,

    /// `max_descriptor_set_storage_images` is the maximum number of storage images that can be
    /// included in a pipeline layout. Descriptors with a type of
    /// [`VkDescriptorType::StorageImage`], or [`VkDescriptorType::StorageTexelBuffer`] count
    /// against this limit. Only descriptors in descriptor set layouts created without the
    /// [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set count against this limit.
    pub max_descriptor_set_storage_images: u32,

    /// `max_descriptor_set_input_attachments` is the maximum number of input attachments that can
    /// be included in a pipeline layout. Descriptors with a type of
    /// [`VkDescriptorType::InputAttachment`] count against this limit. Only descriptors in
    /// descriptor set layouts created without the
    /// [`VkDescriptorSetLayoutCreateFlag::UpdateAfterBindPool`] bit set count against this limit.
    pub max_descriptor_set_input_attachments: u32,

    /// `max_vertex_input_attributes` is the maximum number of vertex input attributes that can be
    /// specified for a graphics pipeline. These are described in the array of
    /// [`VkVertexInputAttributeDescription`] structures that are provided at graphics pipeline
    /// creation time via the `vertex_attribute_descriptions` member of the
    /// [`VkPipelineVertexInputStateCreateInfo`] structure.
    pub max_vertex_input_attributes: u32,

    /// `max_vertex_input_bindings` is the maximum number of vertex buffers that can be specified
    /// for providing vertex attributes to a graphics pipeline. These are described in the array of
    /// [`VkVertexInputBindingDescription`] structures that are provided at graphics pipeline
    /// creation time via the `vertex_binding_descriptions` member of the
    /// [`VkPipelineVertexInputStateCreateInfo`] structure. The binding member of
    /// [`VkVertexInputBindingDescription`] must be less than this limit.
    pub max_vertex_input_bindings: u32,

    /// `max_vertex_input_attribute_offset` is the maximum vertex input attribute offset that can
    /// be added to the vertex input binding stride. The offset member of the
    /// [`VkVertexInputAttributeDescription`] structure must be less than or equal to this limit.
    pub max_vertex_input_attribute_offset: u32,

    /// `max_vertex_input_binding_stride` is the maximum vertex input binding stride that can be
    /// specified in a vertex input binding. The stride member of the
    /// [`VkVertexInputBindingDescription`] structure must be less than or equal to this limit.
    pub max_vertex_input_binding_stride: u32,

    /// `max_vertex_output_components` is the maximum number of components of output variables
    /// which can be output by a vertex shader.
    pub max_vertex_output_components: u32,

    /// `max_tessellation_generation_level` is the maximum tessellation generation level supported
    /// by the fixed-function tessellation primitive generator.
    pub max_tessellation_generation_level: u32,

    /// `max_tessellation_patch_size` is the maximum patch size, in vertices, of patches that can
    /// be processed by the tessellation control shader and tessellation primitive generator. The
    /// `patch_control_points` member of the [`VkPipelineTessellationStateCreateInfo`] structure
    /// specified at pipeline creation time and the value provided in the `OutputVertices`
    /// execution mode of shader modules must be less than or equal to this limit.
    pub max_tessellation_patch_size: u32,

    /// `max_tessellation_control_per_vertex_input_components` is the maximum number of components
    /// of input variables which can be provided as per-vertex inputs to the tessellation control
    /// shader stage.
    pub max_tessellation_control_per_vertex_input_components: u32,

    /// `max_tessellation_control_per_vertex_output_components` is the maximum number of components
    /// of per-vertex output variables which can be output from the tessellation control shader
    /// stage.
    pub max_tessellation_control_per_vertex_output_components: u32,

    /// `max_tessellation_control_per_patch_output_components` is the maximum number of components
    /// of per-patch output variables which can be output from the tessellation control shader
    /// stage.
    pub max_tessellation_control_per_patch_output_components: u32,

    /// `max_tessellation_control_total_output_components` is the maximum total number of
    /// components of per-vertex and per-patch output variables which can be output from the
    /// tessellation control shader stage.
    pub max_tessellation_control_total_output_components: u32,

    /// `max_tessellation_evaluation_input_components` is the maximum number of components of input
    /// variables which can be provided as per-vertex inputs to the tessellation evaluation shader
    /// stage.
    pub max_tessellation_evaluation_input_components: u32,

    /// `max_tessellation_evaluation_output_components` is the maximum number of components of
    /// per-vertex output variables which can be output from the tessellation evaluation shader
    /// stage.
    pub max_tessellation_evaluation_output_components: u32,

    /// `max_geometry_shader_invocations` is the maximum invocation count supported for instanced
    /// geometry shaders. The value provided in the Invocations execution mode of shader modules
    /// must be less than or equal to this limit.
    pub max_geometry_shader_invocations: u32,

    /// `max_geometry_input_components` is the maximum number of components of input variables
    /// which can be provided as inputs to the geometry shader stage.
    pub max_geometry_input_components: u32,

    /// `max_geometry_output_components` is the maximum number of components of output variables
    /// which can be output from the geometry shader stage.
    pub max_geometry_output_components: u32,

    /// `max_geometry_output_vertices` is the maximum number of vertices which can be emitted by
    /// any geometry shader.
    pub max_geometry_output_vertices: u32,

    /// `max_geometry_total_output_components` is the maximum total number of components of output
    /// variables, across all emitted vertices, which can be output from the geometry shader stage.
    pub max_geometry_total_output_components: u32,

    /// `max_fragment_input_components` is the maximum number of components of input variables
    /// which can be provided as inputs to the fragment shader stage.
    pub max_fragment_input_components: u32,

    /// `max_fragment_output_attachments` is the maximum number of output attachments which can be
    /// written to by the fragment shader stage.
    pub max_fragment_output_attachments: u32,

    /// `max_fragment_dual_src_attachments` is the maximum number of output attachments which can
    /// be written to by the fragment shader stage when blending is enabled and one of the dual
    /// source blend modes is in use. See Dual-Source Blending and dualSrcBlend.
    pub max_fragment_dual_src_attachments: u32,

    /// `max_fragment_combined_output_resources` is the total number of storage buffers, storage
    /// images, and output Location decorated color attachments which can be used in the fragment
    /// shader stage.
    pub max_fragment_combined_output_resources: u32,

    /// `max_compute_shared_memory_size` is the maximum total storage size, in bytes, available for
    /// variables declared with the `Workgroup` storage class in shader modules (or with the shared
    /// storage qualifier in GLSL) in the compute shader stage.
    pub max_compute_shared_memory_size: u32,

    /// `max_compute_work_group_count` is the maximum number of local workgroups that can be
    /// dispatched by a single dispatching command. These three values represent the maximum number
    /// of local workgroups for the X, Y, and Z dimensions, respectively. The workgroup count
    /// parameters to the dispatching commands must be less than or equal to the corresponding
    /// limit.
    pub max_compute_work_group_count: [u32; 3],

    /// `max_compute_work_group_invocations` is the maximum total number of compute shader
    /// invocations in a single local workgroup. The product of the X, Y, and Z sizes, as specified
    /// by the `LocalSize` or `LocalSizeId` execution mode in shader modules or by the object
    /// decorated by the `WorkgroupSize` decoration, must be less than or equal to this limit.
    pub max_compute_work_group_invocations: u32,

    /// `max_compute_work_group_size` is the maximum size of a local compute workgroup, per
    /// dimension. These three values represent the maximum local workgroup size in the X, Y, and Z
    /// dimensions, respectively. The x, y, and z sizes, as specified by the `LocalSize` or
    /// `LocalSizeId` execution mode or by the object decorated by the `WorkgroupSize` decoration
    /// in shader modules, must be less than or equal to the corresponding limit.
    pub max_compute_work_group_size: [u32; 3],

    /// `sub_pixel_precision_bits` is the number of bits of subpixel precision in framebuffer
    /// coordinates xf and yf.
    pub sub_pixel_precision_bits: u32,

    /// `sub_texel_precision_bits` is the number of bits of precision in the division along an axis
    /// of an image used for minification and magnification filters. `2subTexelPrecisionBits` is
    /// the actual number of divisions along each axis of the image represented. Sub-texel values
    /// calculated during image sampling will snap to these locations when generating the filtered
    /// results.
    pub sub_texel_precision_bits: u32,

    /// `mipmap_precision_bits` is the number of bits of division that the LOD calculation for
    /// mipmap fetching get snapped to when determining the contribution from each mip level to the
    /// mip filtered results. `2mipmapPrecisionBits` is the actual number of divisions.
    pub mipmap_precision_bits: u32,

    /// `max_draw_indexed_index_value` is the maximum index value that can be used for indexed draw
    /// calls when using 32-bit indices. This excludes the primitive restart index value of
    /// 0xFFFFFFFF.
    pub max_draw_indexed_index_value: u32,

    /// `max_draw_indirect_count` is the maximum draw count that is supported for indirect drawing
    /// calls.
    pub max_draw_indirect_count: u32,

    /// `max_sampler_lod_bias` is the maximum absolute sampler LOD bias. The sum of the
    /// `mip_lod_bias` member of the [`VkSamplerCreateInfo`] structure and the `Bias` operand of
    /// image sampling operations in shader modules (or 0 if no `Bias` operand is provided to an
    /// image sampling operation) are clamped to the range
    /// `[-max_sampler_lod_bias, +max_sampler_lod_bias]`.
    pub max_sampler_lod_bias: f32,

    /// `max_sampler_anisotropy` is the maximum degree of sampler anisotropy. The maximum degree of
    /// anisotropic filtering used for an image sampling operation is the minimum of the
    /// `max_anisotropy` member of the [`VkSamplerCreateInfo`] structure and this limit.
    pub max_sampler_anisotropy: f32,

    /// `max_viewports` is the maximum number of active viewports. The `viewport_count` member of
    /// the [`VkPipelineViewportStateCreateInfo`] structure that is provided at pipeline creation
    /// must be less than or equal to this limit.
    pub max_viewports: u32,

    /// `max_viewport_dimensions` are the maximum viewport dimensions in the X (`width`) and Y
    /// (`height`) dimensions, respectively. The maximum viewport dimensions must be greater than
    /// or equal to the largest image which can be created and used as a framebuffer attachment.
    pub max_viewport_dimensions: [u32; 2],

    /// `viewport_bounds_range` is the `[minimum, maximum]` range that the corners of a viewport
    /// must be contained in. This range must be at least `[-2 × size, 2 × size - 1]`, where
    /// `size = max(max_viewport_dimensions[0], max_viewport_dimensions[1])`.
    pub viewport_bounds_range: [f32; 2],

    /// `viewport_sub_pixel_bits` is the number of bits of subpixel precision for viewport bounds.
    /// The subpixel precision that floating-point viewport bounds are interpreted at is given by
    /// this limit.
    pub viewport_sub_pixel_bits: u32,

    /// `min_memory_map_alignment` is the minimum required alignment, in bytes, of host visible
    /// memory allocations within the host address space. When mapping a memory allocation with
    /// [`VkMapMemory`], subtracting offset bytes from the returned pointer will always produce an
    /// integer multiple of this limit. The value must be a power of two.
    pub min_memory_map_alignment: c_size_t,

    /// `min_texel_buffer_offset_alignment` is the minimum required alignment, in bytes, for the
    /// offset member of the [`VkBufferViewCreateInfo`] structure for texel buffers. The value must
    /// be a power of two. This limit is equivalent to the maximum of the
    /// `uniform_texel_buffer_offset_alignment_bytes` and
    /// `storage_texel_buffer_offset_alignment_bytes` members of
    /// [`VkPhysicalDeviceTexelBufferAlignmentProperties`], but smaller alignment is optionally
    /// allowed by `storage_texel_buffer_offset_single_texel_alignment` and
    /// `uniform_texel_buffer_offset_single_texel_alignment`. For single texel alignment, a format
    /// has an alignment requirement which is the size of a single component if the size of the
    /// format is a multiple of three bytes, otherwise, it is the size of the format itself. The
    /// effective alignment requirement is the minimum of the per-format alignment and
    /// `uniform_texel_buffer_offset_alignment_bytes` or
    /// `storage_texel_buffer_offset_alignment_bytes` depending on the descriptor type. If the
    /// `texel_buffer_alignment` feature is not enabled, the effective alignment requirement for
    /// any format is `min_texel_buffer_offset_alignment`. [`VkBufferViewCreateInfo::offset`] must
    /// be a multiple of this value.
    pub min_texel_buffer_offset_alignment: VkDeviceSize,

    /// `min_uniform_buffer_offset_alignment` is the minimum required alignment, in bytes, for the
    /// offset member of the [`VkDescriptorBufferInfo`] structure for uniform buffers. When a
    /// descriptor of type [`VkDescriptorType::UniformBuffer`] or
    /// [`VkDescriptorType::UniformBufferDynamic`] is updated, the offset must be an integer
    /// multiple of this limit. Similarly, dynamic offsets for uniform buffers must be multiples of
    /// this limit. The value must be a power of two.
    pub min_uniform_buffer_offset_alignment: VkDeviceSize,

    /// `min_storage_buffer_offset_alignment` is the minimum required alignment, in bytes, for the
    /// offset member of the VkDescriptorBufferInfo structure for storage buffers. When a
    /// descriptor of type [`VkDescriptorType::StorageBuffer`] or
    /// [`VkDescriptorType::StorageBufferDynamic`] is updated, the offset must be an integer
    /// multiple of this limit. Similarly, dynamic offsets for storage buffers must be multiples of
    /// this limit. The value must be a power of two.
    pub min_storage_buffer_offset_alignment: VkDeviceSize,

    /// `min_texel_offset` is the minimum offset value for the `Offset` or `ConstOffset` image
    /// operand of any of the `OpImageSample*` or `OpImageFetch*` image instructions.
    pub min_texel_offset: i32,

    /// `max_texel_offset` is the maximum offset value for the `Offset` or `ConstOffset` image
    /// operand of any of the `OpImageSample*` or `OpImageFetch*` image instructions.
    pub max_texel_offset: u32,

    /// `min_texel_gather_offset` is the minimum offset value for the `Offset`, `ConstOffset`, or
    /// `ConstOffsets` image operands of any of the `OpImage*Gather` and `OpImageGatherQCOM` image
    /// instructions.
    pub min_texel_gather_offset: i32,

    /// `max_texel_gather_offset` is the maximum offset value for the `Offset`, `ConstOffset`, or
    /// `ConstOffsets` image operands of any of the `OpImage*Gather` and `OpImageGatherQCOM` image
    /// instructions.
    pub max_texel_gather_offset: u32,

    /// `min_interpolation_offset` is the base minimum (inclusive) negative offset value for the
    /// `Offset` operand of the `InterpolateAtOffset` extended instruction.
    pub min_interpolation_offset: f32,

    /// `max_interpolation_offset` is the base maximum (inclusive) positive offset value for the
    /// `Offset` operand of the `InterpolateAtOffset` extended instruction.
    pub max_interpolation_offset: f32,

    /// `sub_pixel_interpolation_offset_bits` is the number of fractional bits that the x and y
    /// offsets to the `InterpolateAtOffset` extended instruction may be rounded to as fixed-point
    /// values.
    pub sub_pixel_interpolation_offset_bits: u32,

    /// `max_framebuffer_width` is the maximum width for a framebuffer. The `width` member of the
    /// [`VkFramebufferCreateInfo`] structure must be less than or equal to this limit.
    pub max_framebuffer_width: u32,

    /// `max_framebuffer_height` is the maximum height for a framebuffer. The `height` member of
    /// the [`VkFramebufferCreateInfo`] structure must be less than or equal to this limit.
    pub max_framebuffer_height: u32,

    /// `max_framebuffer_layers` is the maximum layer count for a layered framebuffer. The layers
    /// member of the [`VkFramebufferCreateInfo`] structure must be less than or equal to this
    /// limit.
    pub max_framebuffer_layers: u32,

    /// `framebuffer_color_sample_counts` is a bitmask of [`VkSampleCountFlag`]s indicating the
    /// color sample counts that are supported for all framebuffer color attachments with floating-
    /// or fixed-point formats.
    pub framebuffer_color_sample_counts: VkSampleCountFlags,

    /// `framebuffer_depth_sample_counts` is a bitmask of [`VkSampleCountFlag`]s indicating the
    /// supported depth sample counts for all framebuffer depth/stencil attachments, when the
    /// format includes a depth component.
    pub framebuffer_depth_sample_counts: VkSampleCountFlags,

    /// `framebuffer_stencil_sample_counts` is a bitmask of [`VkSampleCountFlag`]s indicating the
    /// supported stencil sample counts for all framebuffer depth/stencil attachments, when the
    /// format includes a stencil component.
    pub framebuffer_stencil_sample_counts: VkSampleCountFlags,

    /// `framebuffer_no_attachments_sample_counts` is a bitmask of [`VkSampleCountFlag`]s
    /// indicating the supported sample counts for a subpass which uses no attachments.
    pub framebuffer_no_attachments_sample_counts: VkSampleCountFlags,

    /// `max_color_attachments` is the maximum number of color attachments that can be used by a
    /// subpass in a render pass. The `color_attachment_count` member of the
    /// [`VkSubpassDescription`] or [`VkSubpassDescription2`] structure must be less than or equal
    /// to this limit.
    pub max_color_attachments: u32,

    /// `sampled_image_color_sample_counts` is a bitmask of [`VkSampleCountFlag`]s indicating the
    /// sample counts supported for all 2D images created with [`VkImageTiling::Optimal`], the
    /// [`VkImageUsageFlag::Sampled`] usage flag set, and a non-integer color format.
    pub sampled_image_color_sample_counts: VkSampleCountFlags,

    /// `sampled_image_integer_sample_counts` is a bitmask of [`VkSampleCountFlag`]s indicating the
    /// sample counts supported for all 2D images created with [`VkImageTiling::Optimal`], the
    /// [`VkImageUsageFlag::Sampled`] usage flag set, and a non-integer color format.
    pub sampled_image_integer_sample_counts: VkSampleCountFlags,

    /// `sampled_image_depth_sample_counts` is a bitmask of [`VkSampleCountFlag`]s indicating the
    /// sample counts supported for all 2D images created with [`VkImageTiling::Optimal`], the
    /// [`VkImageUsageFlag::Sampled`] usage flag set, and a depth format.
    pub sampled_image_depth_sample_counts: VkSampleCountFlags,

    /// `sampled_image_stencil_sample_counts` is a bitmask of [`VkSampleCountFlag`]s indicating the
    /// sample counts supported for all 2D images created with [`VkImageTiling::Optimal`], the
    /// [`VkImageUsageFlag::Sampled`] usage flag set, and a stencil format.
    pub sampled_image_stencil_sample_counts: VkSampleCountFlags,

    /// `storage_image_sample_counts` is a bitmask of [`VkSampleCountFlag`]s indicating the sample
    /// counts supported for all 2D images created with [`VkImageTiling::Optimal`], and the
    /// [`VkImageUsageFlag::Storage`] usage flag set.
    pub storage_image_sample_counts: VkSampleCountFlags,

    /// `max_sample_mask_words` is the maximum number of array elements of a variable decorated
    /// with the `SampleMask` built-in decoration.
    pub max_sample_mask_words: u32,

    /// `timestamp_compute_and_graphics` specifies support for timestamps on all graphics and
    /// compute queues. If this limit is [`VK_TRUE`], all queues that advertise the
    /// [`VkQueueFlag::Graphics`] or [`VkQueueFlag::Compute`] in the
    /// [`VkQueueFamilyProperties::queue_flags`] support
    /// [`VkQueueFamilyProperties::timestamp_valid_bits`] of at least 36.
    pub timestamp_compute_and_graphics: VkBool32,

    /// `timestamp_period` is the number of nanoseconds required for a timestamp query to be
    /// incremented by 1.
    pub timestamp_period: f32,

    /// `max_clip_distances` is the maximum number of clip distances that can be used in a single
    /// shader stage. The size of any array declared with the `ClipDistance` built-in decoration in
    /// a shader module must be less than or equal to this limit.
    pub max_clip_distances: u32,

    /// `max_cull_distances` is the maximum number of clip distances that can be used in a single
    /// shader stage. The size of any array declared with the `ClipDistance` built-in decoration in
    /// a shader module must be less than or equal to this limit.
    pub max_cull_distances: u32,

    /// `max_combined_clip_and_cull_distances` is the maximum combined number of clip and cull
    /// distances that can be used in a single shader stage. The sum of the sizes of all arrays
    /// declared with the `ClipDistance` and `CullDistance` built-in decoration used by a single
    /// shader stage in a shader module must be less than or equal to this limit.
    pub max_combined_clip_and_cull_distances: u32,

    /// `discrete_queue_priorities` is the number of discrete priorities that can be assigned to a
    /// queue based on the value of each member of [`VkDeviceQueueCreateInfo::queue_priorities`].
    /// This must be at least 2, and levels must be spread evenly over the range, with at least one
    /// level at 1.0, and another at 0.0.
    pub discrete_queue_priorities: u32,

    /// `point_size_range` is the range `[minimum, maximum]` of supported sizes for points. Values
    /// written to variables decorated with the PointSize built-in decoration are clamped to this
    /// range.
    pub point_size_range: [f32; 2],

    /// `line_width_range` is the range `[minimum, maximum]` of supported widths for lines. Values
    /// specified by the `line_width` member of the [`VkPipelineRasterizationStateCreateInfo`] or
    /// the `line_width` parameter to [`VkCmdSetLineWidth`] are clamped to this range.
    pub line_width_range: [f32; 2],

    /// `point_size_granularity` is the granularity of supported point sizes. Not all point sizes
    /// in the range defined by `point_size_range` are supported. This limit specifies the
    /// granularity (or increment) between successive supported point sizes.
    pub point_size_granularity: f32,

    /// `line_width_granularity` is the granularity of supported line widths. Not all line widths
    /// in the range defined by `line_width_range` are supported. This limit specifies the
    /// granularity (or increment) between successive supported line widths.
    pub line_width_granularity: f32,

    /// `strict_lines` specifies whether lines are rasterized according to the preferred method of
    /// rasterization. If set to [`VK_FALSE`], lines may be rasterized under a relaxed set of
    /// rules. If set to [`VK_TRUE`], lines are rasterized as per the strict definition.
    pub strict_lines: VkBool32,

    /// `standard_sample_locations` specifies whether rasterization uses the standard sample
    /// locations. If set to [`VK_TRUE`], the implementation uses the documented sample locations.
    /// If set to [`VK_FALSE`], the implementation may use different sample locations.
    pub standard_sample_locations: VkBool32,

    /// `optimal_buffer_copy_offset_alignment` is the optimal buffer offset alignment in bytes for
    /// [`VkCmdCopyBufferToImage2`], [`VkCmdCopyBufferToImage`], [`VkCmdCopyImageToBuffer2`], and
    /// [`VkCmdCopyImageToBuffer`]. This value is also the optimal host memory offset alignment in
    /// bytes for [`VkCopyMemoryToImage`] and [`VkCopyImageToMemory`]. The per texel alignment
    /// requirements are enforced, but applications should use the optimal alignment for optimal
    /// performance and power use. The value must be a power of two.
    pub optimal_buffer_copy_offset_alignment: VkDeviceSize,

    /// `optimal_buffer_copy_row_pitch_alignment` is the optimal buffer row pitch alignment in
    /// bytes for [`VkCmdCopyBufferToImage2`], [`VkCmdCopyBufferToImage`],
    /// [`VkCmdCopyImageToBuffer2`], and [`VkCmdCopyImageToBuffer`]. This value is also the optimal
    /// host memory row pitch alignment in bytes for [`VkCopyMemoryToImage`] and
    /// [`VkCopyImageToMemory`]. Row pitch is the number of bytes between texels with the same X
    /// coordinate in adjacent rows (Y coordinates differ by one). The per texel alignment
    /// requirements are enforced, but applications should use the optimal alignment for optimal
    /// performance and power use. The value must be a power of two.
    pub optimal_buffer_copy_row_pitch_alignment: VkDeviceSize,

    /// `non_coherent_atom_size` is the size and alignment in bytes that bounds concurrent access
    /// to host-mapped device memory. The value must be a power of two.
    pub non_coherent_atom_size: VkDeviceSize,
}

const impl Default for VkPhysicalDeviceLimits {
    fn default() -> Self {
        VkPhysicalDeviceLimits {
            max_image_dimension_1d: 0,
            max_image_dimension_2d: 0,
            max_image_dimension_3d: 0,
            max_image_dimension_cube: 0,
            max_image_array_layers: 0,
            max_texel_buffer_elements: 0,
            max_uniform_buffer_range: 0,
            max_storage_buffer_range: 0,
            max_push_constants_size: 0,
            max_memory_allocation_count: 0,
            max_sampler_allocation_count: 0,
            buffer_image_granularity: 0,
            sparse_address_space_size: 0,
            max_bound_descriptor_sets: 0,
            max_per_stage_descriptor_samplers: 0,
            max_per_stage_descriptor_uniform_buffers: 0,
            max_per_stage_descriptor_storage_buffers: 0,
            max_per_stage_descriptor_sampled_images: 0,
            max_per_stage_descriptor_storage_images: 0,
            max_per_stage_descriptor_input_attachments: 0,
            max_per_stage_resources: 0,
            max_descriptor_set_samplers: 0,
            max_descriptor_set_uniform_buffers: 0,
            max_descriptor_set_uniform_buffers_dynamic: 0,
            max_descriptor_set_storage_buffers: 0,
            max_descriptor_set_storage_buffers_dynamic: 0,
            max_descriptor_set_sampled_images: 0,
            max_descriptor_set_storage_images: 0,
            max_descriptor_set_input_attachments: 0,
            max_vertex_input_attributes: 0,
            max_vertex_input_bindings: 0,
            max_vertex_input_attribute_offset: 0,
            max_vertex_input_binding_stride: 0,
            max_vertex_output_components: 0,
            max_tessellation_generation_level: 0,
            max_tessellation_patch_size: 0,
            max_tessellation_control_per_vertex_input_components: 0,
            max_tessellation_control_per_vertex_output_components: 0,
            max_tessellation_control_per_patch_output_components: 0,
            max_tessellation_control_total_output_components: 0,
            max_tessellation_evaluation_input_components: 0,
            max_tessellation_evaluation_output_components: 0,
            max_geometry_shader_invocations: 0,
            max_geometry_input_components: 0,
            max_geometry_output_components: 0,
            max_geometry_output_vertices: 0,
            max_geometry_total_output_components: 0,
            max_fragment_input_components: 0,
            max_fragment_output_attachments: 0,
            max_fragment_dual_src_attachments: 0,
            max_fragment_combined_output_resources: 0,
            max_compute_shared_memory_size: 0,
            max_compute_work_group_count: [0; 3],
            max_compute_work_group_invocations: 0,
            max_compute_work_group_size: [0; 3],
            sub_pixel_precision_bits: 0,
            sub_texel_precision_bits: 0,
            mipmap_precision_bits: 0,
            max_draw_indexed_index_value: 0,
            max_draw_indirect_count: 0,
            max_sampler_lod_bias: 0.0,
            max_sampler_anisotropy: 0.0,
            max_viewports: 0,
            max_viewport_dimensions: [0; 2],
            viewport_bounds_range: [0.0; 2],
            viewport_sub_pixel_bits: 0,
            min_memory_map_alignment: 0,
            min_texel_buffer_offset_alignment: 0,
            min_uniform_buffer_offset_alignment: 0,
            min_storage_buffer_offset_alignment: 0,
            min_texel_offset: 0,
            max_texel_offset: 0,
            min_texel_gather_offset: 0,
            max_texel_gather_offset: 0,
            min_interpolation_offset: 0.0,
            max_interpolation_offset: 0.0,
            sub_pixel_interpolation_offset_bits: 0,
            max_framebuffer_width: 0,
            max_framebuffer_height: 0,
            max_framebuffer_layers: 0,
            framebuffer_color_sample_counts: VkSampleCountFlags::empty(),
            framebuffer_depth_sample_counts: VkSampleCountFlags::empty(),
            framebuffer_stencil_sample_counts: VkSampleCountFlags::empty(),
            framebuffer_no_attachments_sample_counts: VkSampleCountFlags::empty(),
            max_color_attachments: 0,
            sampled_image_color_sample_counts: VkSampleCountFlags::empty(),
            sampled_image_integer_sample_counts: VkSampleCountFlags::empty(),
            sampled_image_depth_sample_counts: VkSampleCountFlags::empty(),
            sampled_image_stencil_sample_counts: VkSampleCountFlags::empty(),
            storage_image_sample_counts: VkSampleCountFlags::empty(),
            max_sample_mask_words: 0,
            timestamp_compute_and_graphics: 0,
            timestamp_period: 0.0,
            max_clip_distances: 0,
            max_cull_distances: 0,
            max_combined_clip_and_cull_distances: 0,
            discrete_queue_priorities: 0,
            point_size_range: [0.0; 2],
            line_width_range: [0.0; 2],
            point_size_granularity: 0.0,
            line_width_granularity: 0.0,
            strict_lines: 0,
            standard_sample_locations: 0,
            optimal_buffer_copy_offset_alignment: 0,
            optimal_buffer_copy_row_pitch_alignment: 0,
            non_coherent_atom_size: 0,
        }
    }
}
