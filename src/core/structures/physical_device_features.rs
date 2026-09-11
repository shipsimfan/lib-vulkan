use crate::{VK_FALSE, VkBool32};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_TRUE, VK_VERSION_1_0, VkBindBufferMemory, VkBindImageMemory, VkBlendFactor,
    VkBufferCreateFlag, VkBufferCreateInfo, VkDescriptorType, VkDynamicState, VkFormat,
    VkImageCreateFlag, VkImageCreateInfo, VkImageType, VkImageUsageFlag, VkImageViewType,
    VkIndexType, VkPhysicalDeviceLimits, VkPipelineColorBlendAttachmentState,
    VkPipelineColorBlendStateCreateInfo, VkPipelineDepthStencilStateCreateInfo,
    VkPipelineMultisampleStateCreateInfo, VkPipelineRasterizationStateCreateInfo,
    VkPipelineStageFlag, VkPipelineViewportStateCreateInfo, VkPolygonMode,
    VkQueryPipelineStatisticFlag, VkSampleCountFlag, VkSamplerCreateInfo, VkShaderStageFlag,
    VkStructureType,
};

/// Structure describing the fine-grained features that can be supported by an implementation
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceFeatures {
    /// `robust_buffer_access` enables Robust Buffer Access guarantees for shader buffer accesses.
    pub robust_buffer_access: VkBool32,

    /// `full_draw_index_uint32` specifies the full 32-bit range of indices is supported for
    /// indexed draw calls when using a [`VkIndexType`] of [`VkIndexType::Uint32`].
    /// `max_draw_indexed_index_value` is the maximum index value that may be used (aside from the
    /// primitive restart index, which is always `2^32 - 1` when the [`VkIndexType`] is
    /// [`VkIndexType::Uint32`]). If this feature is supported, `max_draw_indexed_index_value` must
    /// be `2^32 - 1`; otherwise it must be no smaller than `2^24 - 1`.
    pub full_draw_index_uint32: VkBool32,

    /// `image_cube_array` specifies whether image views with a [`VkImageViewType`] of
    /// [`VkImageViewType::CubeArray`] can be created, and that the corresponding
    /// `SampledCubeArray` and `ImageCubeArray` SPIR-V capabilities can be used in shader code.
    pub image_cube_array: VkBool32,

    /// `independent_blend` specifies whether the [`VkPipelineColorBlendAttachmentState`] settings
    /// are controlled independently per-attachment. If this feature is not enabled, the
    /// [`VkPipelineColorBlendAttachmentState`] settings for all color attachments must be
    /// identical. Otherwise, a different [`VkPipelineColorBlendAttachmentState`] can be provided
    /// for each bound color attachment.
    pub independent_blend: VkBool32,

    /// `geometry_shader` specifies whether geometry shaders are supported. If this feature is not
    /// enabled, the [`VkShaderStageFlag::Geometry`] and [`VkPipelineStageFlag::GeometryShader`]
    /// enum values must not be used. This also specifies whether shader modules can declare the
    /// `Geometry` capability.
    pub geometry_shader: VkBool32,

    /// `tessellation_shader` specifies whether tessellation control and evaluation shaders are
    /// supported. If this feature is not enabled, the [`VkShaderStageFlag::TessellationControl`],
    /// [`VkShaderStageFlag::TessellationEvaluation`],
    /// [`VkPipelineStageFlag::TessellationControlShader`],
    /// [`VkPipelineStageFlag::TessellationEvaluationShader`], and
    /// [`VkStructureType::PipelineTessellationStateCreateInfo`] enum values must not be used. This
    /// also specifies whether shader modules can declare the `Tessellation` capability.
    pub tessellation_shader: VkBool32,

    /// `sample_rate_shading` specifies whether Sample Shading and multisample interpolation are
    /// supported. If this feature is not enabled, the `sample_shading_enable` member of the
    /// [`VkPipelineMultisampleStateCreateInfo`] structure must be [`VK_FALSE`] and the
    /// `min_sample_shading` member is ignored. This also specifies whether shader modules can
    /// declare the `SampleRateShading` capability.
    pub sample_rate_shading: VkBool32,

    /// `dual_src_blend` specifies whether blend operations which take two sources are supported.
    /// If this feature is not enabled, the [`VkBlendFactor::Src1Color`],
    /// [`VkBlendFactor::OneMinusSrc1Color`], [`VkBlendFactor::Src1Alpha`], and
    /// [`VkBlendFactor::OneMinusSrc1Alpha`] enum values must not be used as source or destination
    /// blending factors.
    pub dual_src_blend: VkBool32,

    /// `logic_op` specifies whether logic operations are supported. If this feature is not
    /// enabled, the `logic_op_enable` member of the [`VkPipelineColorBlendStateCreateInfo`]
    /// structure must be [`VK_FALSE`], and the `logic_op` member is ignored.
    pub logic_op: VkBool32,

    /// `multi_draw_indirect` specifies whether multiple draw indirect is supported. If this
    /// feature is not enabled, the `draw_count` parameter to the [`VkCmdDrawIndirect`] and
    /// [`VkCmdDrawIndexedIndirect`] commands must be 0 or 1. The `max_draw_indirect_count` member
    /// of the [`VkPhysicalDeviceLimits`] structure must also be 1 if this feature is not
    /// supported.
    pub multi_draw_indirect: VkBool32,

    /// `draw_indirect_first_instance` specifies whether indirect drawing calls support the
    /// `first_instance` parameter. If this feature is not enabled, the `first_instance` member of
    /// all [`VkDrawIndirectCommand`] and [`VkDrawIndexedIndirectCommand`] structures that are
    /// provided to the [`VkCmdDrawIndirect`] and [`VkCmdDrawIndexedIndirect`] commands must be 0.
    pub draw_indirect_first_instance: VkBool32,

    /// `depth_clamp` specifies whether depth clamping is supported. If this feature is not
    /// enabled, the `depth_clamp_enable` member of the [`VkPipelineRasterizationStateCreateInfo`]
    /// structure must be [`VK_FALSE`]. Otherwise, setting `depth_clamp_enable` to [`VK_TRUE`] will
    /// enable depth clamping.
    pub depth_clamp: VkBool32,

    /// `depth_bias_clamp` specifies whether depth bias clamping is supported. If this feature is
    /// not enabled, the `depth_bias_clamp` member of the
    /// [`VkPipelineRasterizationStateCreateInfo`] structure must be 0.0 unless the
    /// [`VkDynamicState::DepthBias`] dynamic state is enabled, in which case the
    /// `depth_bias_clamp` parameter to [`VkCmdSetDepthBias`] must be 0.0.
    pub depth_bias_clamp: VkBool32,

    /// `fill_mode_non_solid` specifies whether point and wireframe fill modes are supported. If
    /// this feature is not enabled, the [`VkPolygonMode::Point`] and [`VkPolygonMode::Line`] enum
    /// values must not be used.
    pub fill_mode_non_solid: VkBool32,

    /// `depth_bounds` specifies whether depth bounds tests are supported. If this feature is not
    /// enabled, the `depth_bounds_test_enable` member of the
    /// [`VkPipelineDepthStencilStateCreateInfo`] structure must be [`VK_FALSE`] unless the
    /// [`VkDynamicState::DepthBoundsTestEnable`] dynamic state is enabled, in which case the
    /// `depth_bounds_test_enable` parameter to [`VkCmdSetDepthBoundsTestEnable`] must be
    /// [`VK_FALSE`]. When `depth_bounds_test_enable` is [`VK_FALSE`], the `min_depth_bounds` and
    /// `max_depth_bounds` members of the [`VkPipelineDepthStencilStateCreateInfo`] structure are
    /// ignored.
    pub depth_bounds: VkBool32,

    /// `wide_lines` specifies whether lines with width other than 1.0 are supported. If this
    /// feature is not enabled, the `line_width` member of the
    /// [`VkPipelineRasterizationStateCreateInfo`] structure must be 1.0 unless the
    /// [`VkDynamicState::LineWidth`] dynamic state is enabled, in which case the `line_width`
    /// parameter to [`VkCmdSetLineWidth`] must be 1.0. When this feature is supported, the range
    /// and granularity of supported line widths are indicated by the `line_width_range` and
    /// `line_width_granularity` members of the [`VkPhysicalDeviceLimits`] structure, respectively.
    pub wide_lines: VkBool32,

    /// `large_points` specifies whether points with size greater than 1.0 are supported. If this
    /// feature is not enabled, only a point size of 1.0 written by a shader is supported. The
    /// range and granularity of supported point sizes are indicated by the `point_size_range` and
    /// `point_size_granularity` members of the [`VkPhysicalDeviceLimits`] structure, respectively.
    pub large_points: VkBool32,

    /// `alpha_to_one` specifies whether the implementation is able to replace the alpha value of
    /// the fragment shader color output in the Multisample Coverage fragment operation. If this
    /// feature is not enabled, then the `alpha_to_one_enable` member of the
    /// [`VkPipelineMultisampleStateCreateInfo`] structure must be [`VK_FALSE`]. Otherwise setting
    /// `alpha_to_one_enable` to [`VK_TRUE`] will enable alpha-to-one behavior.
    pub alpha_to_one: VkBool32,

    /// `multi_viewport` specifies whether more than one viewport is supported. If this feature is
    /// not enabled:
    ///  - The `viewport_count` and `scissor_count` members of the
    ///    [`VkPipelineViewportStateCreateInfo`] structure must be 1.
    ///  - The `first_viewport` and `viewport_count` parameters to the [`VkCmdSetViewport`] command
    ///    must be 0 and 1, respectively.
    ///  - The `first_scissor` and `scissor_count` parameters to the [`VkCmdSetScissor`] command
    ///    must be 0 and 1, respectively.
    ///  - The `exclusive_scissor_count` member of the
    ///    [`VkPipelineViewportExclusiveScissorStateCreateInfoNv`] structure must be 0 or 1.
    ///  - The `first_exclusive_scissor` and `exclusive_scissor_count` parameters to the
    ///    [`VkCmdSetExclusiveScissorNv`] command must be 0 and 1, respectively.
    pub multi_viewport: VkBool32,

    /// `sampler_anisotropy` specifies whether anisotropic filtering is supported. If this feature
    /// is not enabled, the `anisotropy_enable` member of the [`VkSamplerCreateInfo`] structure
    /// must be [`VK_FALSE`].
    pub sampler_anisotropy: VkBool32,

    /// `texture_compression_etc2` specifies whether all of the ETC2 and EAC compressed texture
    /// formats are supported. If this feature is enabled, then the
    /// [`VkFormatFeatureFlag::SampledImage`], [`VkFormatFeatureFlag::BlitSrc`] and
    /// [`VkFormatFeatureFlag::SampledImageFilterLinear`] features must be supported in
    /// `optimal_tiling_features` for the following formats:
    ///  - [`VkFormat::Etc2R8G8B8UnormBlock`]
    ///  - [`VkFormat::Etc2R8G8B8SrgbBlock`]
    ///  - [`VkFormat::Etc2R8G8B8A1UnormBlock`]
    ///  - [`VkFormat::Etc2R8G8B8A1SrgbBlock`]
    ///  - [`VkFormat::Etc2R8G8B8A8UnormBlock`]
    ///  - [`VkFormat::Etc2R8G8B8A8SrgbBlock`]
    ///  - [`VkFormat::EacR11UnormBlock`]
    ///  - [`VkFormat::EacR11SnormBlock`]
    ///  - [`VkFormat::EacR11G11UnormBlock`]
    ///  - [`VkFormat::EacR11G11SnormBlock`]
    pub texture_compression_etc2: VkBool32,

    /// `texture_compression_astc_ldr` specifies whether all of the ASTC LDR compressed texture
    /// formats are supported. If this feature is enabled, then the
    /// [`VkFormatFeatureFlag::SampledImage`], [`VkFormatFeatureFlag::BlitSrc`] and
    /// [`VkFormatFeatureFlag::SampledImageFilterLinear`] features must be supported in
    /// `optimal_tiling_features` for the following formats:
    ///  - [`VkFormat::Astc4x4UnormBlock`]
    ///  - [`VkFormat::Astc4x4SrgbBlock`]
    ///  - [`VkFormat::Astc5x4UnormBlock`]
    ///  - [`VkFormat::Astc5x4SrgbBlock`]
    ///  - [`VkFormat::Astc5x5UnormBlock`]
    ///  - [`VkFormat::Astc5x5SrgbBlock`]
    ///  - [`VkFormat::Astc6x5UnormBlock`]
    ///  - [`VkFormat::Astc6x5SrgbBlock`]
    ///  - [`VkFormat::Astc6x6UnormBlock`]
    ///  - [`VkFormat::Astc6x6SrgbBlock`]
    ///  - [`VkFormat::Astc8x5UnormBlock`]
    ///  - [`VkFormat::Astc8x5SrgbBlock`]
    ///  - [`VkFormat::Astc8x6UnormBlock`]
    ///  - [`VkFormat::Astc8x6SrgbBlock`]
    ///  - [`VkFormat::Astc8x8UnormBlock`]
    ///  - [`VkFormat::Astc8x8SrgbBlock`]
    ///  - [`VkFormat::Astc10x5UnormBlock`]
    ///  - [`VkFormat::Astc10x5SrgbBlock`]
    ///  - [`VkFormat::Astc10x6UnormBlock`]
    ///  - [`VkFormat::Astc10x6SrgbBlock`]
    ///  - [`VkFormat::Astc10x8UnormBlock`]
    ///  - [`VkFormat::Astc10x8SrgbBlock`]
    ///  - [`VkFormat::Astc10x10UnormBlock`]
    ///  - [`VkFormat::Astc10x10SrgbBlock`]
    ///  - [`VkFormat::Astc12x10UnormBlock`]
    ///  - [`VkFormat::Astc12x10SrgbBlock`]
    ///  - [`VkFormat::Astc12x12UnormBlock`]
    ///  - [`VkFormat::Astc12x12SrgbBlock`]
    pub texture_compression_astcldr: VkBool32,

    /// `texture_compression_bc` specifies whether all of the BC compressed texture formats are
    /// supported. If this feature is enabled, then the [`VkFormatFeatureFlag::SampledImage`],
    /// [`VkFormatFeatureFlag::BlitSrc`] and [`VkFormatFeatureFlag::SampledImageFilterLinear`]
    /// features must be supported in `optimal_tiling_features` for the following formats:
    ///  - [`VkFormat::Bc1RgbUnormBlock`]
    ///  - [`VkFormat::Bc1RgbSrgbBlock`]
    ///  - [`VkFormat::Bc1RgbaUnormBlock`]
    ///  - [`VkFormat::Bc1RgbaSrgbBlock`]
    ///  - [`VkFormat::Bc2UnormBlock`]
    ///  - [`VkFormat::Bc2SrgbBlock`]
    ///  - [`VkFormat::Bc3UnormBlock`]
    ///  - [`VkFormat::Bc3SrgbBlock`]
    ///  - [`VkFormat::Bc4UnormBlock`]
    ///  - [`VkFormat::Bc4SnormBlock`]
    ///  - [`VkFormat::Bc5UnormBlock`]
    ///  - [`VkFormat::Bc5SnormBlock`]
    ///  - [`VkFormat::Bc6HUfloatBlock`]
    ///  - [`VkFormat::Bc6HSfloatBlock`]
    ///  - [`VkFormat::Bc7UnormBlock`]
    ///  - [`VkFormat::Bc7SrgbBlock`]
    pub texture_compression_bc: VkBool32,

    /// `occlusion_query_precise` specifies whether occlusion queries returning actual sample
    /// counts are supported. Occlusion queries are created in a [`VkQueryPool`] by specifying the
    /// `query_type` of [`VkQueryTypeOcclusion`] in the [`VkQueryPoolCreateInfo`] structure which
    /// is passed to [`VkCreateQueryPool`]. If this feature is enabled, queries of this type can
    /// enable [`VkQueryFlag::ControlPrecise`] in the `flags` parameter to [`VkCmdBeginQuery`]. If
    /// this feature is not supported, the implementation supports only boolean occlusion queries.
    /// When any samples are passed, boolean queries will return a non-zero result value, otherwise
    /// a result value of zero is returned. When this feature is enabled and
    /// [`VkQueryFlag::ControlPrecise`] is set, occlusion queries will report the actual number of
    /// samples passed.
    pub occlusion_query_precise: VkBool32,

    /// `pipeline_statistics_query` specifies whether the pipeline statistics queries are
    /// supported. If this feature is not enabled, queries of type
    /// [`VkQueryType::PipelineStatistics`] cannot be created, and none of the
    /// [`VkQueryPipelineStatisticFlag`] bits can be set in the `pipeline_statistics` member of the
    /// [`VkQueryPoolCreateInfo`] structure.
    pub pipeline_statistics_query: VkBool32,

    /// `vertex_pipeline_stores_and_atomics` specifies whether storage buffers and images support
    /// stores and atomic operations in the vertex, tessellation, and geometry shader stages. If
    /// this feature is not enabled, all storage image, storage texel buffer, and storage buffer
    /// variables used by these stages in shader modules must be decorated with the `NonWritable`
    /// decoration (or the readonly memory qualifier in GLSL).
    pub vertex_pipeline_stores_and_atomics: VkBool32,

    /// `fragment_stores_and_atomics` specifies whether storage buffers and images support stores
    /// and atomic operations in the fragment shader stage. If this feature is not enabled, all
    /// storage image, storage texel buffer, and storage buffer variables used by the fragment
    /// stage in shader modules must be decorated with the `NonWritable` decoration (or the
    /// readonly memory qualifier in GLSL).
    pub fragment_stores_and_atomics: VkBool32,

    /// `shader_tessellation_and_geometry_point_size` specifies whether the `PointSize` built-in
    /// decoration is available in the tessellation control, tessellation evaluation, and geometry
    /// shader stages. If this feature is not enabled, members decorated with the `PointSize`
    /// built-in decoration must not be read from or written to and all points written from a
    /// tessellation or geometry shader will have a size of 1.0. This also specifies whether shader
    /// modules can declare the `TessellationPointSize` capability for tessellation control and
    /// evaluation shaders, or if the shader modules can declare the `GeometryPointSize` capability
    /// for geometry shaders. An implementation supporting this feature must also support one or
    /// both of the `tessellation_shader` or `geometry_shader` features.
    pub shader_tessellation_and_geometry_point_size: VkBool32,

    /// `shader_image_gather_extended` specifies whether the extended set of image gather
    /// instructions are available in shader code. If this feature is not enabled, the
    /// `OpImage*Gather` and `OpImageGatherQCOM` instructions do not support the `Offset` and
    /// `ConstOffsets` operands. The `OpImageGatherQCOM` instruction does not support the
    /// `ConstOffsets` operand regardless of the value of this feature. This also specifies whether
    /// shader modules can declare the `ImageGatherExtended` capability.
    pub shader_image_gather_extended: VkBool32,

    /// `shader_storage_image_extended_formats` specifies whether all the “storage image extended
    /// formats” below are supported; if this feature is supported, then the
    /// [`VkFormatFeatureFlag::StorageImage`] must be supported in `optimal_tiling_features` for
    /// the following formats:
    ///  - [`VkFormat::R16G16Sfloat`]
    ///  - [`VkFormat::B10G11R11UfloatPack32`]
    ///  - [`VkFormat::R16Sfloat`]
    ///  - [`VkFormat::R16G16B16A16Unorm`]
    ///  - [`VkFormat::A2B10G10R10UnormPack32`]
    ///  - [`VkFormat::R16G16Unorm`]
    ///  - [`VkFormat::R8G8Unorm`]
    ///  - [`VkFormat::R16Unorm`]
    ///  - [`VkFormat::R8Unorm`]
    ///  - [`VkFormat::R16G16B16A16Snorm`]
    ///  - [`VkFormat::R16G16Snorm`]
    ///  - [`VkFormat::R8G8Snorm`]
    ///  - [`VkFormat::R16Snorm`]
    ///  - [`VkFormat::R8Snorm`]
    ///  - [`VkFormat::R16G16Sint`]
    ///  - [`VkFormat::R8G8Sint`]
    ///  - [`VkFormat::R16Sint`]
    ///  - [`VkFormat::R8Sint`]
    ///  - [`VkFormat::A2B10G10R10UintPack32`]
    ///  - [`VkFormat::R16G16Uint`]
    ///  - [`VkFormat::R8G8Uint`]
    ///  - [`VkFormat::R16Uint`]
    ///  - [`VkFormat::R8Uint`]
    pub shader_storage_image_extended_formats: VkBool32,

    /// `shader_storage_image_multisample` specifies whether multisampled storage images are
    /// supported. If this feature is not enabled, images that are created with the
    /// [`VkImageUsageFlag::Storage`] usage flag set must be created with samples equal to
    /// [`VkSampleCountFlag::_1`]. This also specifies whether shader modules can declare the
    /// `StorageImageMultisample` and `ImageMSArray` capabilities.
    pub shader_storage_image_multisample: VkBool32,

    /// `shader_storage_image_read_without_format` specifies whether storage images and storage
    /// texel buffers require a format qualifier to be specified when reading.
    /// `shader_storage_image_read_without_format` applies only to formats listed in the storage
    /// without format list.
    pub shader_storage_image_read_without_format: VkBool32,

    /// `shader_storage_image_write_without_format` specifies whether storage images and storage
    /// texel buffers require a format qualifier to be specified when writing.
    /// `shader_storage_image_read_without_format` applies only to formats listed in the storage
    /// without format list.
    pub shader_storage_image_write_without_format: VkBool32,

    /// `shader_uniform_buffer_array_dynamic_indexing` specifies whether arrays of uniform buffers
    /// can be indexed by integer expressions that are dynamically uniform within either the
    /// subgroup or the invocation group in shader code. If this feature is not enabled, resources
    /// with a descriptor type of [`VkDescriptorType::UniformBuffer`] or
    /// [`VkDescriptorType::UniformBufferDynamic`] must be indexed only by constant integral
    /// expressions when aggregated into arrays in shader code. This also specifies whether shader
    /// modules can declare the `UniformBufferArrayDynamicIndexing` capability.
    pub shader_uniform_buffer_array_dynamic_indexing: VkBool32,

    /// `shader_sampled_image_array_dynamic_indexing` specifies whether arrays of samplers or
    /// sampled images can be indexed by integer expressions that are dynamically uniform within
    /// either the subgroup or the invocation group in shader code. If this feature is not enabled,
    /// resources with a descriptor type of [`VkDescriptorType::Sampler`],
    /// [`VkDescriptorType::CombinedImageSampler`], or [`VkDescriptorType::SampledImage`] must be
    /// indexed only by constant integral expressions when aggregated into arrays in shader code.
    /// This also specifies whether shader modules can declare the
    /// `SampledImageArrayDynamicIndexing` capability.
    pub shader_sampled_image_array_dynamic_indexing: VkBool32,

    /// `shader_storage_buffer_array_dynamic_indexing` specifies whether arrays of storage buffers
    /// can be indexed by integer expressions that are dynamically uniform within either the
    /// subgroup or the invocation group in shader code. If this feature is not enabled, resources
    /// with a descriptor type of [`VkDescriptorType::StorageBuffer`] or
    /// [`VkDescriptorType::StorageBufferDynamic`] must be indexed only by constant integral
    /// expressions when aggregated into arrays in shader code. This also specifies whether shader
    /// modules can declare the `StorageBufferArrayDynamicIndexing` capability.
    pub shader_storage_buffer_array_dynamic_indexing: VkBool32,

    /// `shader_storage_image_array_dynamic_indexing` specifies whether arrays of storage images
    /// can be indexed by integer expressions that are dynamically uniform within either the
    /// subgroup or the invocation group in shader code. If this feature is not enabled, resources
    /// with a descriptor type of [`VkDescriptorType::StorageImage`] must be indexed only by
    /// constant integral expressions when aggregated into arrays in shader code. This also
    /// specifies whether shader modules can declare the `StorageImageArrayDynamicIndexing`
    /// capability.
    pub shader_storage_image_array_dynamic_indexing: VkBool32,

    /// `shader_clip_distance` specifies whether clip distances are supported in shader code. If
    /// this feature is not enabled, any members decorated with the `ClipDistance` built-in
    /// decoration must not be read from or written to in shader modules. This also specifies
    /// whether shader modules can declare the `ClipDistance` capability.
    pub shader_clip_distance: VkBool32,

    /// `shader_cull_distance` specifies whether cull distances are supported in shader code. If
    /// this feature is not enabled, any members decorated with the `CullDistance` built-in
    /// decoration must not be read from or written to in shader modules. This also specifies
    /// whether shader modules can declare the `CullDistance` capability.
    pub shader_cull_distance: VkBool32,

    /// `shader_float64` specifies whether 64-bit floats (doubles) are supported in shader code. If
    /// this feature is not enabled, 64-bit floating-point types must not be used in shader code.
    /// This also specifies whether shader modules can declare the `Float64` capability. Declaring
    /// and using 64-bit floats is enabled for all storage classes that SPIR-V allows with the
    /// `Float64` capability.
    pub shader_float64: VkBool32,

    /// `shader_int64` specifies whether 64-bit integers (signed and unsigned) are supported in
    /// shader code. If this feature is not enabled, 64-bit integer types must not be used in
    /// shader code. This also specifies whether shader modules can declare the `Int64` capability.
    /// Declaring and using 64-bit integers is enabled for all storage classes that SPIR-V allows
    /// with the `Int64` capability.
    pub shader_int64: VkBool32,

    /// `shader_int16` specifies whether 16-bit integers (signed and unsigned) are supported in
    /// shader code. If this feature is not enabled, 16-bit integer types must not be used in
    /// shader code. This also specifies whether shader modules can declare the `Int16` capability.
    /// However, this only enables a subset of the storage classes that SPIR-V allows for the
    /// `Int16` SPIR-V capability: Declaring and using 16-bit integers in the `Private`,
    /// `Workgroup` (for non-`Block` variables), and `Function` storage classes is enabled, while
    /// declaring them in the interface storage classes (e.g., `UniformConstant`, `Uniform`,
    /// `StorageBuffer`, `Input`, `Output`, and `PushConstant`) is not enabled.
    pub shader_int16: VkBool32,

    /// `shader_resource_residency` specifies whether image operations that return resource
    /// residency information are supported in shader code. If this feature is not enabled, the
    /// `OpImageSparse*` instructions must not be used in shader code. This also specifies whether
    /// shader modules can declare the `SparseResidency` capability. The feature requires at least
    /// one of the `sparse_residency_*` features to be supported.
    pub shader_resource_residency: VkBool32,

    /// `shader_resource_min_lod` specifies whether image operations specifying the minimum
    /// resource LOD are supported in shader code. If this feature is not enabled, the `MinLod`
    /// image operand must not be used in shader code. This also specifies whether shader modules
    /// can declare the `MinLod` capability.
    pub shader_resource_min_lod: VkBool32,

    /// `sparse_binding` specifies whether resource memory can be managed at opaque sparse block
    /// level instead of at the object level. If this feature is not enabled, resource memory must
    /// be bound only on a per-object basis using the [`VkBindBufferMemory`] and
    /// [`VkBindImageMemory`] commands. In this case, buffers and images must not be created with
    /// [`VkBufferCreateFlag::SparseBinding`] and [`VkImageCreateFlag::SparseBinding`] set in the
    /// `flags` member of the [`VkBufferCreateInfo`] and [`VkImageCreateInfo`] structures,
    /// respectively.
    pub sparse_binding: VkBool32,

    /// `sparse_residency_buffer` specifies whether the device can access partially resident
    /// buffers. If this feature is not enabled, buffers must not be created with
    /// [`VkBufferCreateFlag::SparseResidency`] set in the `flags` member of the
    /// [`VkBufferCreateInfo`] structure.
    pub sparse_residency_buffer: VkBool32,

    /// `sparse_residency_image_2d` specifies whether the device can access partially resident 2D
    /// images with 1 sample per pixel. If this feature is not enabled, images with an `image_type`
    /// of [`VkImageType::_2d`] and samples set to [`VkSampleCountFlag::_1`] must not be created
    /// with [`VkImageCreateFlag::SparseResidency`] set in the `flags` member of the
    /// [`VkImageCreateInfo`] structure.
    pub sparse_residency_image_2d: VkBool32,

    /// `sparse_residency_image_3d` specifies whether the device can access partially resident 3D
    /// images. If this feature is not enabled, images with an `image_type` of [`VkImageType::_3d`]
    /// must not be created with [`VkImageCreateFlag::SparseResidency`] set in the `flags` member
    /// of the [`VkImageCreateInfo`] structure.
    pub sparse_residency_image_3d: VkBool32,

    /// `sparse_residency_2_samples` specifies whether the physical device can access partially
    /// resident 2D images with 2 samples per pixel. If this feature is not enabled, images with an
    /// `image_type` of [`VkImageType::_2d`] and samples set to [`VkSampleCountFlag::_2`] must not
    /// be created with [`VkImageCreateFlag::SparseResidency`] set in the `flags` member of the
    /// [`VkImageCreateInfo`] structure.
    pub sparse_residency_2_samples: VkBool32,

    /// `sparse_residency_4_samples` specifies whether the physical device can access partially
    /// resident 2D images with 4 samples per pixel. If this feature is not enabled, images with an
    /// `image_type` of [`VkImageType::_2d`] and samples set to [`VkSampleCountFlag::_4`] must not
    /// be created with [`VkImageCreateFlag::SparseResidency`] set in the `flags` member of the
    /// [`VkImageCreateInfo`] structure.
    pub sparse_residency_4_samples: VkBool32,

    /// `sparse_residency_8_samples` specifies whether the physical device can access partially
    /// resident 2D images with 8 samples per pixel. If this feature is not enabled, images with an
    /// `image_type` of [`VkImageType::_2d`] and samples set to [`VkSampleCountFlag::_8`] must not
    /// be created with [`VkImageCreateFlag::SparseResidency`] set in the `flags` member of the
    /// [`VkImageCreateInfo`] structure.
    pub sparse_residency_8_samples: VkBool32,

    /// `sparse_residency_16_samples` specifies whether the physical device can access partially
    /// resident 2D images with 16 samples per pixel. If this feature is not enabled, images with
    /// an `image_type` of [`VkImageType::_2d`] and samples set to [`VkSampleCountFlag::_16`] must
    /// not be created with [`VkImageCreateFlag::SparseResidency`] set in the `flags` member of the
    /// [`VkImageCreateInfo`] structure.
    pub sparse_residency_16_samples: VkBool32,

    /// `sparse_residency_aliased` specifies whether the physical device can correctly access data
    /// aliased into multiple locations. If this feature is not enabled, the
    /// [`VkBufferCreateFlag::SparseAliased`] and [`VkImageCreateFlag::SparseAliased`] enum values
    /// must not be used in `flags` members of the [`VkBufferCreateInfo`] and [`VkImageCreateInfo`]
    /// structures, respectively.
    pub sparse_residency_aliased: VkBool32,

    /// `variable_multisample_rate` specifies whether all pipelines that will be bound to a command
    /// buffer during a subpass which uses no attachments must have the same value for
    /// [`VkPipelineMultisampleStateCreateInfo::rasterization_samples`]. If set to [`VK_TRUE`], the
    /// implementation supports variable multisample rates in a subpass which uses no attachments.
    /// If set to [`VK_FALSE`], then all pipelines bound in such a subpass must have the same
    /// multisample rate. This has no effect in situations where a subpass uses any attachments.
    pub variable_multisample_rate: VkBool32,

    /// `inherited_queries` specifies whether a secondary command buffer may be executed while a
    /// query is active.
    pub inherited_queries: VkBool32,
}

const impl Default for VkPhysicalDeviceFeatures {
    fn default() -> Self {
        VkPhysicalDeviceFeatures {
            robust_buffer_access: VK_FALSE,
            full_draw_index_uint32: VK_FALSE,
            image_cube_array: VK_FALSE,
            independent_blend: VK_FALSE,
            geometry_shader: VK_FALSE,
            tessellation_shader: VK_FALSE,
            sample_rate_shading: VK_FALSE,
            dual_src_blend: VK_FALSE,
            logic_op: VK_FALSE,
            multi_draw_indirect: VK_FALSE,
            draw_indirect_first_instance: VK_FALSE,
            depth_clamp: VK_FALSE,
            depth_bias_clamp: VK_FALSE,
            fill_mode_non_solid: VK_FALSE,
            depth_bounds: VK_FALSE,
            wide_lines: VK_FALSE,
            large_points: VK_FALSE,
            alpha_to_one: VK_FALSE,
            multi_viewport: VK_FALSE,
            sampler_anisotropy: VK_FALSE,
            texture_compression_etc2: VK_FALSE,
            texture_compression_astcldr: VK_FALSE,
            texture_compression_bc: VK_FALSE,
            occlusion_query_precise: VK_FALSE,
            pipeline_statistics_query: VK_FALSE,
            vertex_pipeline_stores_and_atomics: VK_FALSE,
            fragment_stores_and_atomics: VK_FALSE,
            shader_tessellation_and_geometry_point_size: VK_FALSE,
            shader_image_gather_extended: VK_FALSE,
            shader_storage_image_extended_formats: VK_FALSE,
            shader_storage_image_multisample: VK_FALSE,
            shader_storage_image_read_without_format: VK_FALSE,
            shader_storage_image_write_without_format: VK_FALSE,
            shader_uniform_buffer_array_dynamic_indexing: VK_FALSE,
            shader_sampled_image_array_dynamic_indexing: VK_FALSE,
            shader_storage_buffer_array_dynamic_indexing: VK_FALSE,
            shader_storage_image_array_dynamic_indexing: VK_FALSE,
            shader_clip_distance: VK_FALSE,
            shader_cull_distance: VK_FALSE,
            shader_float64: VK_FALSE,
            shader_int64: VK_FALSE,
            shader_int16: VK_FALSE,
            shader_resource_residency: VK_FALSE,
            shader_resource_min_lod: VK_FALSE,
            sparse_binding: VK_FALSE,
            sparse_residency_buffer: VK_FALSE,
            sparse_residency_image_2d: VK_FALSE,
            sparse_residency_image_3d: VK_FALSE,
            sparse_residency_2_samples: VK_FALSE,
            sparse_residency_4_samples: VK_FALSE,
            sparse_residency_8_samples: VK_FALSE,
            sparse_residency_16_samples: VK_FALSE,
            sparse_residency_aliased: VK_FALSE,
            variable_multisample_rate: VK_FALSE,
            inherited_queries: VK_FALSE,
        }
    }
}
