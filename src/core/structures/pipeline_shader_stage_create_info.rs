use crate::{
    VkPipelineShaderStageCreateFlags, VkShaderModule, VkShaderStageFlag, VkSpecializationInfo,
    VkStructureType,
};
use std::{
    ffi::{c_char, c_void},
    ptr::null,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VkPhysicalDeviceLimits, VkPipelineCreateFlag, VkPipelineShaderStageCreateFlag,
    VkResult, VkShaderModuleCreateInfo,
};

/// Structure specifying parameters of a newly created pipeline shader `stage`
///
/// # Description
/// If `module` is not [`VK_NULL_HANDLE`], the shader code used by the pipeline is defined by
/// `module`. If `module` is [`VK_NULL_HANDLE`], the shader code is defined by the chained
/// [`VkShaderModuleCreateInfo`] if present.
///
/// If the `shader_module_identifier` feature is enabled, applications can omit shader code for
/// `stage` and instead provide a `module` identifier. This is done by including a
/// [`VkPipelineShaderStageModuleIdentifierCreateInfoExt`] structure with `identifier_size` not
/// equal to 0 in the `next` chain. A shader `stage` created in this way is equivalent to one created
/// using a shader `module` with the same identifier. The identifier allows an implementation to
/// look up a pipeline without consuming a valid SPIR-V `module`. If a pipeline is not found,
/// pipeline compilation is not possible and the implementation must fail as specified by
/// [`VkPipelineCreateFlag::FailOnPipelineCompileRequired`].
///
/// When an identifier is used in lieu of a shader `module`, implementations may fail pipeline
/// compilation with [`VkResult::VkPipelineCompileRequired`] for any reason.
///
/// Applications can use identifiers when creating pipelines with
/// [`VkPipelineCreateFlag::LibraryKhr`]. When creating such pipelines, [`VkResult::VkSuccess`] may
/// be returned, but subsequently fail when referencing the pipeline in a
/// [`VkPipelineLibraryCreateInfoKhr`] struct. Applications must allow pipeline compilation to fail
/// during link steps with [`VkPipelineCreateFlag::FailOnPipelineCompileRequired`] as it may not be
/// possible to determine if a pipeline can be created from identifiers until the link step.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPipelineShaderStageCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PipelineShaderStageCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If the `descriptor_heap` feature is not enabled,
    ///    [`VkShaderDescriptorSetAndBindingMappingInfoExt::mapping_count`] must be 0
    ///  - If the `next` chain specifies a descriptor mapping using
    ///    [`VkDescriptorMappingSourceExt::PushDataExt`],
    ///    [`VkDescriptorMappingSourceExt::ShaderRecordDataExt`], or
    ///    [`VkDescriptorMappingSourceExt::ResourceHeapDataExt`], the mapped resource in the shader
    ///    must be a variable with a structure type decorated with Block in the Uniform Storage
    ///    Class
    ///  - If the `next` chain specifies a descriptor mapping using
    ///    [`VkDescriptorMappingSourceExt::PushDataExt`], the mapped structure must not be larger
    ///    than `max_push_data_size` minus the `push_data_offset` used in the mapping
    ///  - If the `next` chain specifies a descriptor mapping using
    ///    [`VkDescriptorMappingSourceExt::ShaderRecordDataExt`], the sum of mapped structure size
    ///    and `shader_record_data_offset` used in the mapping must not be larger than
    ///    `max_shader_group_stride`
    ///  - If the `next` chain specifies a descriptor mapping using
    ///    [`VkDescriptorMappingSourceExt::ShaderRecordAddressExt`],
    ///    [`VkDescriptorMappingSourceExt::PushAddressExt`], or
    ///    [`VkDescriptorMappingSourceExt::IndirectAddressExt`] the mapped resource in the shader
    ///    must be one of:
    ///    - A variable with a structure type decorated with Block in the Uniform Storage Class
    ///    - A variable with a structure type decorated with BufferBlock in the Uniform Storage
    ///      Class
    ///    - A variable with a structure type decorated with Block in the StorageBuffer Storage
    ///      Class
    ///    - A `OpTypeAccelerationStructureKhr` variable
    ///    - A `OpTypeAccelerationStructureNv` variable
    ///  - If the `next` chain specifies a descriptor mapping using
    ///    [`VkDescriptorMappingSourceExt::PushAddressExt`],
    ///    [`VkDescriptorMappingSourceExt::ShaderRecordAddressExt`], or
    ///    [`VkDescriptorMappingSourceExt::IndirectAddressExt`], the `OpArrayLength` or
    ///    `OpUntypedArrayLengthKHR` instruction must not be used on that resource
    ///  - If the `next` chain specifies a descriptor mapping using
    ///    [`VkDescriptorMappingSourceExt::HeapWithConstantOffsetExt`],
    ///    [`VkDescriptorMappingSourceExt::HeapWithPushIndexExt`],
    ///    [`VkDescriptorMappingSourceExt::HeapWithShaderRecordIndexExt`],
    ///    [`VkDescriptorMappingSourceExt::HeapWithIndirectIndexExt`], or
    ///    [`VkDescriptorMappingSourceExt::HeapWithIndirectIndexArrayExt`], and the mapped resource
    ///    declaration is an array, the `embedded_sampler` member of the corresponding mapping
    ///    structure must be [`null`]
    ///  - If a [`VkPipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure is included in
    ///    the `next` chain, `flags` must not have the
    ///    [`VkPipelineShaderStageCreateFlag::AllowVaryingSubgroupSize`] flag set
    ///  - If a [`VkPipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure is included in
    ///    the `next` chain, the `subgroup_size_control` feature must be enabled, and `stage` must be
    ///    a valid bit specified in `required_subgroup_size_stages`
    ///  - If a [`VkPipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure is included in
    ///    the `next` chain and `stage` is [`VkShaderStageFlag::Compute`],
    ///    [`VkShaderStageFlag::MeshExt`], or [`VkShaderStageFlag::TaskExt`], the local workgroup
    ///    size of the shader must be less than or equal to the product of
    ///    [`VkPipelineShaderStageRequiredSubgroupSizeCreateInfo::required_subgroup_size`] and
    ///    `max_compute_workgroup_subgroups`
    ///  - If a [`VkPipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure is included in
    ///    the `next` chain, and `flags` has the
    ///    [`VkPipelineShaderStageCreateFlag::RequireFullSubgroups`] flag set, the local workgroup
    ///    size in the X dimension of the pipeline must be a multiple of
    ///    [`VkPipelineShaderStageRequiredSubgroupSizeCreateInfo::required_subgroup_size`]
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of [`VkDebugUtilsObjectNameInfoExt`],
    ///    [`VkPipelineRobustnessCreateInfo`],
    ///    [`VkPipelineShaderStageModuleIdentifierCreateInfoExt`],
    ///    [`VkPipelineShaderStageNodeCreateInfoAmdx`],
    ///    [`VkPipelineShaderStageRequiredSubgroupSizeCreateInfo`],
    ///    [`VkShaderDescriptorSetAndBindingMappingInfoExt`], [`VkShaderModuleCreateInfo`], or
    ///    [`VkShaderModuleValidationCacheCreateInfoExt`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkPipelineShaderStageCreateFlag`]s specifying how the pipeline
    /// shader `stage` will be generated.
    ///
    /// # Valid Usage
    ///  - If `flags` has the [`VkPipelineShaderStageCreateFlag::AllowVaryingSubgroupSize`] flag
    ///    set, the `subgroup_size_control` feature must be enabled
    ///  - If `flags` has the [`VkPipelineShaderStageCreateFlag::RequireFullSubgroups`] flag set,
    ///    the `compute_full_subgroups` feature must be enabled
    ///  - If `flags` includes [`VkPipelineShaderStageCreateFlag::RequireFullSubgroups`], `stage`
    ///    must be one of [`VkShaderStageFlag::MeshExt`], [`VkShaderStageFlag::TaskExt`], or
    ///    [`VkShaderStageFlag::Compute`]
    ///  - If `flags` has both the [`VkPipelineShaderStageCreateFlag::RequireFullSubgroups`] and
    ///    [`VkPipelineShaderStageCreateFlag::AllowVaryingSubgroupSize`] `flags` set, the local
    ///    workgroup size in the X dimension of the pipeline must be a multiple of
    ///    `max_subgroup_size`
    ///  - If `flags` has the [`VkPipelineShaderStageCreateFlag::RequireFullSubgroups`] flag set
    ///    and `flags` does not have the
    ///    [`VkPipelineShaderStageCreateFlag::AllowVaryingSubgroupSize`] flag set and no
    ///    [`VkPipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure is included in the
    ///    `next` chain, the local workgroup size in the X dimension of the pipeline must be a
    ///    multiple of `subgroup_size`
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of [`VkPipelineShaderStageCreateFlag`] values
    pub flags: VkPipelineShaderStageCreateFlags,

    /// `stage` is a [`VkShaderStageFlag`] value specifying a single pipeline `stage`.
    ///
    /// # Valid Usage
    ///  - If the `geometry_shader` feature is not enabled, `stage` must not be
    ///    [`VkShaderStageFlag::Geometry`]
    ///  - If the `tessellation_shader` feature is not enabled, `stage` must not be
    ///    [`VkShaderStageFlag::TessellationControl`] or
    ///    [`VkShaderStageFlag::TessellationEvaluation`]
    ///  - If the `mesh_shaders` feature is not enabled, `stage` must not be
    ///    [`VkShaderStageFlag::MeshExt`]
    ///  - If the `task_shaders` feature is not enabled, `stage` must not be
    ///    [`VkShaderStageFlag::TaskExt`]
    ///  - If the `clusterculling_shader` feature is not enabled, `stage` must not be
    ///    [`VkShaderStageFlag::ClusterCullingHuawei`]
    ///  - `stage` must not be [`VkShaderStageFlag::AllGraphics`], or [`VkShaderStageFlag::All`]
    ///  - If `stage` is [`VkShaderStageFlag::TessellationControl`] or
    ///    [`VkShaderStageFlag::TessellationEvaluation`], and the identified entry point has an
    ///    `OpExecutionMode` instruction specifying a patch size with `OutputVertices`, the patch
    ///    size must be greater than 0 and less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_tessellation_patch_size`]
    ///  - If `stage` is [`VkShaderStageFlag::Geometry`], the identified entry point must have an
    ///    `OpExecutionMode` instruction specifying a maximum output vertex count that is greater
    ///    than 0 and less than or equal to
    ///    [`VkPhysicalDeviceLimits::max_geometry_output_vertices`]
    ///  - If `stage` is [`VkShaderStageFlag::Geometry`], the identified entry point must have an
    ///    `OpExecutionMode` instruction specifying an invocation count that is greater than 0 and
    ///    less than or equal to [`VkPhysicalDeviceLimits::max_geometry_shader_invocations`]
    ///  - If `stage` is either [`VkShaderStageFlag::Vertex`],
    ///    [`VkShaderStageFlag::TessellationControl`],
    ///    [`VkShaderStageFlag::TessellationEvaluation`], or [`VkShaderStageFlag::Geometry`], and
    ///    the identified entry point writes to `Layer` for any primitive, it must write the same
    ///    value to `Layer` for all vertices of a given primitive
    ///  - If `stage` is either [`VkShaderStageFlag::Vertex`],
    ///    [`VkShaderStageFlag::TessellationControl`],
    ///    [`VkShaderStageFlag::TessellationEvaluation`], or [`VkShaderStageFlag::Geometry`], and
    ///    the identified entry point writes to `ViewportIndex` for any primitive, it must write
    ///    the same value to `ViewportIndex` for all vertices of a given primitive
    ///  - If `stage` is [`VkShaderStageFlag::Fragment`], and the identified entry point writes to
    ///    `FragDepth` in any execution path, all execution paths that are not exclusive to helper
    ///    invocations must either discard the fragment, or write or initialize the value of
    ///    `FragDepth`
    ///  - If `stage` is [`VkShaderStageFlag::Fragment`], and the identified entry point writes to
    ///    `FragStencilRefEXT` in any execution path, all execution paths that are not exclusive to
    ///    helper invocations must either discard the fragment, or write or initialize the value of
    ///    `FragStencilRefEXT`
    ///  - If a shader `module` identifier is not specified for this `stage`, `module` must be a
    ///    valid [`VkShaderModule`], or the `next` chain of the parent `Vk*CreateInfo` structure
    ///    must set [`VkPipelineBinaryInfoKhr::binary_count`] to a value greater than 0, if none of
    ///    the following features are enabled:
    ///    - `graphics_pipeline_library`
    ///    - `maintenance5`
    ///  - If a shader `module` identifier is not specified for this `stage`, `module` must be a
    ///    valid [`VkShaderModule`], or there must be a valid [`VkShaderModuleCreateInfo`]
    ///    structure in the `next` chain , or the `next` chain of the parent `Vk*CreateInfo`
    ///    structure must set [`VkPipelineBinaryInfoKhr::binary_count`] to a value greater than 0.
    ///  - If a shader `module` identifier is specified for this `stage`, the `next` chain must not
    ///    include a [`VkShaderModuleCreateInfo`] structure
    ///  - If a shader `module` identifier is specified for this `stage`, `module` must be
    ///    [`VK_NULL_HANDLE`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `stage` must be a valid [`VkShaderStageFlag`] value
    pub stage: VkShaderStageFlag,

    /// `module` is optionally a [`VkShaderModule`] object containing the shader code for this
    /// `stage`. The implementation must not access this object outside of the duration of the
    /// command this structure is passed to.
    ///
    /// # Valid Usage
    ///  - If `module` uses the `OpTypeCooperativeMatrixKHR` instruction with a `Scope` equal to
    ///    `Subgroup`, then the local workgroup size in the X dimension of the pipeline must be a
    ///    multiple of the effective subgroup size
    ///  - If `module` uses the `OpTypeCooperativeMatrixKHR` instruction with a `Scope` equal to
    ///    `Workgroup`, then the local workgroup size in the X dimension of the pipeline must be a
    ///    multiple of the effective subgroup size and the total local workgroup size must be a
    ///    power of two multiple of the effective subgroup size and must be less than or equal to
    ///    `cooperative_matrix_workgroup_scope_max_workgroup_size`
    ///
    /// # Valid Usage (Implicit)
    ///  - If `module` is not [`VK_NULL_HANDLE`], `module` must be a valid [`VkShaderModule`]
    ///    handle
    pub module: VkShaderModule,

    /// `name` is a pointer to a null-terminated UTF-8 string specifying the entry point name of
    /// the shader for this `stage`.
    ///
    /// # Valid Usage
    ///  - `name` must be the name of an `OpEntryPoint` in `module` with an execution model that
    ///    matches `stage`
    ///  - If the identified entry point includes any variable in its interface that is declared
    ///    with the `ClipDistance BuiltIn` decoration, that variable must not have an array size
    ///    greater than [`VkPhysicalDeviceLimits::max_clip_distances`]
    ///  - If the identified entry point includes any variable in its interface that is declared
    ///    with the `CullDistance BuiltIn` decoration, that variable must not have an array size
    ///    greater than [`VkPhysicalDeviceLimits::max_cull_distances`]
    ///  - If the identified entry point includes variables in its interface that are declared with
    ///    the `ClipDistance BuiltIn` decoration and variables in its interface that are declared
    ///    with the `CullDistance BuiltIn` decoration, those variables must not have array sizes
    ///    which sum to more than [`VkPhysicalDeviceLimits::max_combined_clip_and_cull_distances`]
    ///  - If the identified entry point includes any variable in its interface that is declared
    ///    with the `SampleMask BuiltIn` decoration, that variable must not have an array size
    ///    greater than [`VkPhysicalDeviceLimits::max_sample_mask_words`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `name` must be a null-terminated UTF-8 string
    pub name: *const c_char,

    /// `specialization_info` is a pointer to a [`VkSpecializationInfo`] structure, as described in
    /// Specialization Constants, or [`null`].
    ///
    /// # Valid Usage
    ///  - If a shader `module` identifier is not specified, the shader code used by the pipeline
    ///    must be valid as described by the Khronos SPIR-V Specification after applying the
    ///    specializations provided in `specialization_info`, if any, and then converting all
    ///    specialization constants into fixed constants
    ///
    /// # Valid Usage (Implicit)
    ///  - If `specialization_info` is not [`null`], `specialization_info` must be a valid pointer
    ///    to a valid [`VkSpecializationInfo`] structure
    pub specialization_info: *const VkSpecializationInfo,
}

impl const Default for VkPipelineShaderStageCreateInfo {
    fn default() -> Self {
        VkPipelineShaderStageCreateInfo {
            r#type: VkStructureType::PipelineShaderStageCreateInfo,
            next: null(),
            flags: VkPipelineShaderStageCreateFlags::empty(),
            stage: VkShaderStageFlag::All,
            module: VkShaderModule::null(),
            name: null(),
            specialization_info: null(),
        }
    }
}
