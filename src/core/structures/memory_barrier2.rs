use crate::{VkAccessFlags2, VkPipelineStageFlags2, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_3, VkAccessFlag2, VkPipelineStageFlag2};

/// Structure specifying a global memory barrier
///
/// # Description
/// This structure defines a memory dependency affecting all device memory.
///
/// The first synchronization scope and access scope described by this structure include only
/// operations and memory accesses specified by the source stage mask and source access mask.
///
/// The second synchronization scope and access scope described by this structure include only
/// operations and memory accesses specified by destination stage mask and destination access mask.
///
/// Provided by [`VK_VERSION_1_3`]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkMemoryBarrier2 {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::MemoryBarrier2`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `src_stage_mask` is a [`VkPipelineStageFlags2`] mask of pipeline stages to be included in
    /// the first synchronization scope.
    ///
    /// # Valid Usage
    ///  - If the `geometry_shader` feature is not enabled, `src_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::GeometryShader`]
    ///  - If the `tessellation_shader` feature is not enabled, `src_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::TessellationControlShader`] or
    ///    [`VkPipelineStageFlag2::TessellationEvaluationShader`]
    ///  - If the `conditional_rendering` feature is not enabled, `src_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::ConditionalRenderingExt`]
    ///  - If the `fragment_density_map` feature is not enabled, `src_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::FragmentDensityProcessExt`]
    ///  - If the `transform_feedback` feature is not enabled, `src_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::TransformFeedbackExt`]
    ///  - If the `mesh_shader` feature is not enabled, `src_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::MeshShaderExt`]
    ///  - If the `task_shader` feature is not enabled, `src_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::TaskShaderExt`]
    ///  - If neither of the `shading_rate_image` or the `attachment_fragment_shading_rate`
    ///    features are enabled, `src_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::FragmentShadingRateAttachmentKhr`]
    ///  - If the `subpass_shading` feature is not enabled, `src_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::SubpassShaderHuawei`]
    ///  - If the `invocation_mask` feature is not enabled, `src_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::InvocationMaskHuawei`]
    ///  - If neither the [`nv_ray_tracing`] extension or the `ray_tracing_pipeline` feature are
    ///    enabled, `src_stage_mask` must not contain [`VkPipelineStageFlag2::RayTracingShaderKhr`]
    ///  - If the `acceleration_structure` feature is not enabled, `src_stage_mask` must not
    ///    contain [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`]
    ///  - If the `ray_tracing_maintenance1` feature is not enabled, `src_stage_mask` must not
    ///    contain [`VkPipelineStageFlag2::AccelerationStructureCopyKhr`]
    ///  - If the [`VkPhysicalDeviceOpacityMicromapFeaturesExt::micromap`] feature is not enabled,
    ///    `src_stage_mask` must not contain [`VkPipelineStageFlag2::MicromapBuildExt`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `src_stage_mask` must be a valid combination of [`VkPipelineStageFlag2`] values
    pub src_stage_mask: VkPipelineStageFlags2,

    /// `src_access_mask` is a [`VkAccessFlags2`] mask of access flags to be included in the first
    /// access scope.
    ///
    /// # Valid Usage
    ///  - If `src_access_mask` includes [`VkAccessFlag2::IndirectCommandRead`], `src_stage_mask`
    ///    must include [`VkPipelineStageFlag2::DrawIndirect`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`],
    ///    [`VkPipelineStageFlag2::CopyIndirectKhr`], [`VkPipelineStageFlag2::AllGraphics`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::IndexRead`], `src_stage_mask` must
    ///    include [`VkPipelineStageFlag2::IndexInput`], [`VkPipelineStageFlag2::VertexInput`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::VertexAttributeRead`], `src_stage_mask`
    ///    must include [`VkPipelineStageFlag2::VertexAttributeInput`],
    ///    [`VkPipelineStageFlag2::VertexInput`], [`VkPipelineStageFlag2::AllGraphics`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::InputAttachmentRead`], `src_stage_mask`
    ///    must include [`VkPipelineStageFlag2::FragmentShader`],
    ///    [`VkPipelineStageFlag2::SubpassShaderHuawei`], [`VkPipelineStageFlag2::AllGraphics`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::UniformRead`], `src_stage_mask` must
    ///    include [`VkPipelineStageFlag2::AllGraphics`], [`VkPipelineStageFlag2::AllCommands`], or
    ///    one of the `VkPipelineStageFlag::*Stage` stages
    ///  - If `src_access_mask` includes [`VkAccessFlag2::ShaderSampledRead`], `src_stage_mask`
    ///    must include [`VkPipelineStageFlag2::AllGraphics`],
    ///    [`VkPipelineStageFlag2::AllCommands`], or one of the `VkPipelineStageFlag::*Stage`
    ///    stages
    ///  - If `src_access_mask` includes [`VkAccessFlag2::ShaderStorageRead`], `src_stage_mask`
    ///    must include [`VkPipelineStageFlag2::AllGraphics`],
    ///    [`VkPipelineStageFlag2::AllCommands`], or one of the `VkPipelineStageFlag::*Stage`
    ///    stages
    ///  - If `src_access_mask` includes [`VkAccessFlag2::ShaderStorageWrite`], `src_stage_mask`
    ///    must include [`VkPipelineStageFlag2::AllGraphics`],
    ///    [`VkPipelineStageFlag2::AllCommands`], or one of the `VkPipelineStageFlag::*Stage`
    ///    stages
    ///  - If `src_access_mask` includes [`VkAccessFlag2::ShaderRead`], `src_stage_mask` must
    ///    include [`VkPipelineStageFlag2::AllGraphics`], [`VkPipelineStageFlag2::AllCommands`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`],
    ///    [`VkPipelineStageFlag2::MicromapBuildExt`], or one of the `VkPipelineStageFlag::*Stage`
    ///    stages
    ///  - If `src_access_mask` includes [`VkAccessFlag2::ShaderWrite`], `src_stage_mask` must
    ///    include [`VkPipelineStageFlag2::AllGraphics`], [`VkPipelineStageFlag2::AllCommands`], or
    ///    one of the `VkPipelineStageFlag::*Stage` stages
    ///  - If `src_access_mask` includes [`VkAccessFlag2::ColorAttachmentRead`], `src_stage_mask`
    ///    must include [`VkPipelineStageFlag2::ColorAttachmentOutput`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::ColorAttachmentWrite`], `src_stage_mask`
    ///    must include [`VkPipelineStageFlag2::ColorAttachmentOutput`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::DepthStencilAttachmentRead`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::EarlyFragmentTests`],
    ///    [`VkPipelineStageFlag2::LateFragmentTests`], [`VkPipelineStageFlag2::AllGraphics`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::DepthStencilAttachmentWrite`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::EarlyFragmentTests`],
    ///    [`VkPipelineStageFlag2::LateFragmentTests`], [`VkPipelineStageFlag2::AllGraphics`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::TransferRead`], `src_stage_mask` must
    ///    include [`VkPipelineStageFlag2::Copy`], [`VkPipelineStageFlag2::Blit`],
    ///    [`VkPipelineStageFlag2::Resolve`], [`VkPipelineStageFlag2::AllTransfer`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureCopyKhr`],
    ///    [`VkPipelineStageFlag2::ConvertCooperativeVectorMatrixNv`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::TransferWrite`], `src_stage_mask` must
    ///    include [`VkPipelineStageFlag2::Copy`], [`VkPipelineStageFlag2::Blit`],
    ///    [`VkPipelineStageFlag2::Resolve`], [`VkPipelineStageFlag2::Clear`],
    ///    [`VkPipelineStageFlag2::AllTransfer`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureCopyKhr`],
    ///    [`VkPipelineStageFlag2::ConvertCooperativeVectorMatrixNv`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::HostRead`], `src_stage_mask` must include
    ///    [`VkPipelineStageFlag2::Host`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::HostWrite`], `src_stage_mask` must
    ///    include [`VkPipelineStageFlag2::Host`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::ConditionalRenderingReadExt`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::ConditionalRenderingExt`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::FragmentDensityMapReadExt`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::FragmentDensityProcessExt`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::TransformFeedbackWriteExt`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::TransformFeedbackExt`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::TransformFeedbackCounterReadExt`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::DrawIndirect`],
    ///    [`VkPipelineStageFlag2::TransformFeedbackExt`], [`VkPipelineStageFlag2::AllGraphics`],
    ///    or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::TransformFeedbackCounterWriteExt`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::TransformFeedbackExt`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::ShadingRateImageReadNv`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::ShadingRateImageNv`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::InvocationMaskReadHuawei`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::InvocationMaskHuawei`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::CommandPreprocessReadExt`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::CommandPreprocessExt`] or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::CommandPreprocessWriteExt`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::CommandPreprocessExt`] or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::ColorAttachmentReadNoncoherentExt`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::ColorAttachmentOutput`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::AccelerationStructureReadKhr`], and the
    ///    `ray_query` feature is enabled, `src_stage_mask` must include
    ///    [`VkPipelineStageFlag2::RayTracingShaderKhr`], one of the `VkPipelineStageFlag::*Stage`
    ///    stages, [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureCopyKhr`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::AccelerationStructureWriteKhr`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::AccelerationStructureCopyKhr`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`] or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::AccelerationStructureReadKhr`], and the
    ///    `ray_query` feature is not enabled, `src_stage_mask` must include
    ///    [`VkPipelineStageFlag2::RayTracingShaderKhr`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureCopyKhr`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::ShaderBindingTableReadKhr`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::AllCommands`] or
    ///    [`VkPipelineStageFlag2::RayTracingShaderKhr`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::VideoDecodeReadKhr`], `src_stage_mask`
    ///    must include [`VkPipelineStageFlag2::VideoDecodeKhr`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::VideoDecodeWriteKhr`], `src_stage_mask`
    ///    must include [`VkPipelineStageFlag2::VideoDecodeKhr`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::VideoEncodeReadKhr`], `src_stage_mask`
    ///    must include [`VkPipelineStageFlag2::VideoEncodeKhr`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::VideoEncodeWriteKhr`], `src_stage_mask`
    ///    must include [`VkPipelineStageFlag2::VideoEncodeKhr`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::OpticalFlowReadNv`], `src_stage_mask`
    ///    must include [`VkPipelineStageFlag2::OpticalFlowNv`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::OpticalFlowWriteNv`], `src_stage_mask`
    ///    must include [`VkPipelineStageFlag2::OpticalFlowNv`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::MicromapWriteExt`], `src_stage_mask` must
    ///    include [`VkPipelineStageFlag2::MicromapBuildExt`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::MicromapReadExt`], `src_stage_mask` must
    ///    include [`VkPipelineStageFlag2::MicromapBuildExt`] or
    ///    [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::DescriptorBufferReadExt`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::AllGraphics`],
    ///    [`VkPipelineStageFlag2::AllCommands`], or one of `VkPipelineStageFlag::*Stage` stages
    ///  - If `src_access_mask` includes [`VkAccessFlag2::ShaderTileAttachmentReadQcom`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::FragmentShader`] or
    ///    [`VkPipelineStageFlag2::ComputeShader`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::ShaderTileAttachmentWriteQcom`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::FragmentShader`] or
    ///    [`VkPipelineStageFlag2::ComputeShader`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::MemoryDecompressionReadExt`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::MemoryDecompressionExt`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::MemoryDecompressionWriteExt`],
    ///    `src_stage_mask` must include [`VkPipelineStageFlag2::MemoryDecompressionExt`]
    ///  - If `src_access_mask` includes [`VkAccessFlag2::SamplerHeadReadExt`] or
    ///    [`VkAccessFlag2::ResourceHeapReadExt`], `src_stage_mask` must include
    ///    [`VkPipelineStageFlag2::AllGraphics`], [`VkPipelineStageFlag2::AllCommands`], or one of
    ///    `VkPipelineStageFlag::*Stage` stages
    ///
    /// # Valid Usage (Implicit)
    ///  - `src_access_mask` must be a valid combination of [`VkAccessFlag2`] values
    pub src_access_mask: VkAccessFlags2,

    /// `dst_stage_mask` is a [`VkPipelineStageFlags2`] mask of pipeline stages to be included in
    /// the second synchronization scope.
    ///
    /// # Valid Usage
    ///  - If the `geometry_shader` feature is not enabled, `dst_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::GeometryShader`]
    ///  - If the `tessellation_shader` feature is not enabled, `dst_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::TessellationControlShader`] or
    ///    [`VkPipelineStageFlag2::TessellationEvaluationShader`]
    ///  - If the `conditional_rendering` feature is not enabled, `dst_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::ConditionalRenderingExt`]
    ///  - If the `fragment_density_map` feature is not enabled, `dst_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::FragmentDensityProcessExt`]
    ///  - If the `transform_feedback` feature is not enabled, `dst_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::TransformFeedbackExt`]
    ///  - If the `mesh_shader` feature is not enabled, `dst_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::MeshShaderExt`]
    ///  - If the `task_shader` feature is not enabled, `dst_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::TaskShaderExt`]
    ///  - If neither of the `shading_rate_image` or the `attachment_fragment_shading_rate`
    ///    features are enabled, `dst_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::FragmentShadingRateAttachmentKhr`]
    ///  - If the `subpass_shading` feature is not enabled, `dst_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::SubpassShaderHuawei`]
    ///  - If the `invocation_mask` feature is not enabled, `dst_stage_mask` must not contain
    ///    [`VkPipelineStageFlag2::InvocationMaskHuawei`]
    ///  - If neither the [`nv_ray_tracing`] extension or the `ray_tracing_pipeline` feature are
    ///    enabled, `dst_stage_mask` must not contain [`VkPipelineStageFlag2::RayTracingShaderKhr`]
    ///  - If the `acceleration_structure` feature is not enabled, `dst_stage_mask` must not
    ///    contain [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`]
    ///  - If the `ray_tracing_maintenance1` feature is not enabled, `dst_stage_mask` must not
    ///    contain [`VkPipelineStageFlag2::AccelerationStructureCopyKhr`]
    ///  - If the VkPhysicalDeviceOpacityMicromapFeaturesEXT::micromap feature is not enabled,
    ///    `dst_stage_mask` must not contain [`VkPipelineStageFlag2::MicromapBuildExt`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `dst_stage_mask` must be a valid combination of [`VkPipelineStageFlag2`] values
    pub dst_stage_mask: VkPipelineStageFlags2,

    /// `dst_access_mask` is a [`VkAccessFlags2`] mask of access flags to be included in the second
    /// access scope.
    ///
    /// # Valid Usage
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::IndirectCommandRead`], `dst_stage_mask`
    ///    must include [`VkPipelineStageFlag2::DrawIndirect`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`],
    ///    [`VkPipelineStageFlag2::CopyIndirectKhr`], [`VkPipelineStageFlag2::AllGraphics`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::IndexRead`], `dst_stage_mask` must
    ///    include [`VkPipelineStageFlag2::IndexInput`], [`VkPipelineStageFlag2::VertexInput`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::VertexAttributeRead`], `dst_stage_mask`
    ///    must include [`VkPipelineStageFlag2::VertexAttributeInput`],
    ///    [`VkPipelineStageFlag2::VertexInput`], [`VkPipelineStageFlag2::AllGraphics`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::InputAttachmentRead`], `dst_stage_mask`
    ///    must include [`VkPipelineStageFlag2::FragmentShader`],
    ///    [`VkPipelineStageFlag2::SubpassShaderHuawei`], [`VkPipelineStageFlag2::AllGraphics`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::UniformRead`], `dst_stage_mask` must
    ///    include [`VkPipelineStageFlag2::AllGraphics`], [`VkPipelineStageFlag2::AllCommands`], or
    ///    one of the `VkPipelineStageFlag::*Stage` stages
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::ShaderSampledRead`], `dst_stage_mask`
    ///    must include [`VkPipelineStageFlag2::AllGraphics`],
    ///    [`VkPipelineStageFlag2::AllCommands`], or one of the `VkPipelineStageFlag::*Stage`
    ///    stages
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::ShaderStorageRead`], `dst_stage_mask`
    ///    must include [`VkPipelineStageFlag2::AllGraphics`],
    ///    [`VkPipelineStageFlag2::AllCommands`], or one of the `VkPipelineStageFlag::*Stage`
    ///    stages
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::ShaderStorageWrite`], `dst_stage_mask`
    ///    must include [`VkPipelineStageFlag2::AllGraphics`],
    ///    [`VkPipelineStageFlag2::AllCommands`], or one of the `VkPipelineStageFlag::*Stage`
    ///    stages
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::ShaderRead`], `dst_stage_mask` must
    ///    include [`VkPipelineStageFlag2::AllGraphics`], [`VkPipelineStageFlag2::AllCommands`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`],
    ///    [`VkPipelineStageFlag2::MicromapBuildExt`], or one of the `VkPipelineStageFlag::*Stage`
    ///    stages
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::ShaderWrite`], `dst_stage_mask` must
    ///    include [`VkPipelineStageFlag2::AllGraphics`], [`VkPipelineStageFlag2::AllCommands`], or
    ///    one of the `VkPipelineStageFlag::*Stage` stages
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::ColorAttachmentRead`], `dst_stage_mask`
    ///    must include [`VkPipelineStageFlag2::ColorAttachmentOutput`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::ColorAttachmentWrite`], `dst_stage_mask`
    ///    must include [`VkPipelineStageFlag2::ColorAttachmentOutput`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::DepthStencilAttachmentRead`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::EarlyFragmentTests`],
    ///    [`VkPipelineStageFlag2::LateFragmentTests`], [`VkPipelineStageFlag2::AllGraphics`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::DepthStencilAttachmentWrite`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::EarlyFragmentTests`],
    ///    [`VkPipelineStageFlag2::LateFragmentTests`], [`VkPipelineStageFlag2::AllGraphics`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::TransferRead`], `dst_stage_mask` must
    ///    include [`VkPipelineStageFlag2::Copy`], [`VkPipelineStageFlag2::Blit`],
    ///    [`VkPipelineStageFlag2::Resolve`], [`VkPipelineStageFlag2::AllTransfer`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`],     
    ///    [`VkPipelineStageFlag2::AccelerationStructureCopyKhr`],
    ///    [`VkPipelineStageFlag2::ConvertCooperativeVectorMatrixNv`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::TransferWrite`], `dst_stage_mask` must
    ///    include [`VkPipelineStageFlag2::Copy`], [`VkPipelineStageFlag2::Blit`],
    ///    [`VkPipelineStageFlag2::Resolve`], [`VkPipelineStageFlag2::Clear`],
    ///    [`VkPipelineStageFlag2::AllTransfer`],  
    ///    [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureCopyKhr`],
    ///    [`VkPipelineStageFlag2::ConvertCooperativeVectorMatrixNv`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::HostRead`], `dst_stage_mask` must include
    ///    [`VkPipelineStageFlag2::Host`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::HostWrite`], `dst_stage_mask` must
    ///    include [`VkPipelineStageFlag2::Host`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::ConditionalRenderingReadExt`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::ConditionalRenderingExt`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::FragmentDensityMapReadExt`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::FragmentDensityProcessExt`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::TransformFeedbackWriteExt`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::TransformFeedbackExt`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::TransformFeedbackCounterReadExt`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::DrawIndirect`],
    ///    [`VkPipelineStageFlag2::TransformFeedbackExt`], [`VkPipelineStageFlag2::AllGraphics`],
    ///    or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::TransformFeedbackCounterWriteExt`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::TransformFeedbackExt`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::ShadingRateImageReadNv`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::ShadingRateImageNv`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::InvocationMaskReadHuawei`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::InvocationMaskHuawei`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::CommandPreprocessReadNv`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::CommandPreprocessExt`] or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::CommandPreprocessWriteExt`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::CommandPreprocessExt`] or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::ColorAttachmentReadNoncoherentExt`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::ColorAttachmentOutput`],
    ///    [`VkPipelineStageFlag2::AllGraphics`], or [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::AccelerationStructureReadKhr`], and the
    ///    `ray_query` feature is enabled, `dst_stage_mask` must include
    ///    [`VkPipelineStageFlag2::RayTracingShaderKhr`], one of the `VkPipelineStageFlag::*Stage`
    ///    stages, [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureCopyKhr`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::AccelerationStructureWriteKhr`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::AccelerationStructureCopyKhr`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`] or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::AccelerationStructureReadKhr`], and the
    ///    `ray_query` feature is not enabled, `dst_stage_mask` must include
    ///    [`VkPipelineStageFlag2::RayTracingShaderKhr`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`],
    ///    [`VkPipelineStageFlag2::AccelerationStructureCopyKhr`], or
    ///    [`VkPipelineStageFlag2::AllCommands`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::ShaderBindingTableReadKhr`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::AllCommands`] or
    ///    [`VkPipelineStageFlag2::RayTracingShaderKhr`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::VideoDecodeReadKhr`], `dst_stage_mask`
    ///    must include [`VkPipelineStageFlag2::VideoDecodeKhr`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::VideoDecodeWriteKhr`], `dst_stage_mask`
    ///    must include [`VkPipelineStageFlag2::VideoDecodeKhr`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::VideoEncodeReadKhr`], `dst_stage_mask`
    ///    must include [`VkPipelineStageFlag2::VideoEncodeKhr`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::VideoEncodeWriteKhr`], `dst_stage_mask`
    ///    must include [`VkPipelineStageFlag2::VideoEncodeKhr`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::OpticalFlowReadNv`], `dst_stage_mask`
    ///    must include [`VkPipelineStageFlag2::OpticalFlowNv`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::OpticalFlowWriteNv`], `dst_stage_mask`
    ///    must include [`VkPipelineStageFlag2::OpticalFlowNv`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::MicromapWriteExt`], `dst_stage_mask` must
    ///    include [`VkPipelineStageFlag2::MicromapBuildExt`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::MicromapReadExt`], `dst_stage_mask` must
    ///    include [`VkPipelineStageFlag2::MicromapBuildExt`] or
    ///    [`VkPipelineStageFlag2::AccelerationStructureBuildKhr`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::DescriptorBufferReadExt`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::AllGraphics`],
    ///    [`VkPipelineStageFlag2::AllCommands`], or one of `VkPipelineStageFlag::*Stage` stages
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::ShaderTileAttachmentReadQcom`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::FragmentShader`] or
    ///    [`VkPipelineStageFlag2::ComputeShader`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::ShaderTileAttachmentWriteQcom`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::FragmentShader`] or
    ///    [`VkPipelineStageFlag2::ComputeShader`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::MemoryDecompressionReadExt`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::MemoryDecompressionExt`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::MemoryDecompressionWriteExt`],
    ///    `dst_stage_mask` must include [`VkPipelineStageFlag2::MemoryDecompressionExt`]
    ///  - If `dst_access_mask` includes [`VkAccessFlag2::SamplerHeadReadExt`] or
    ///    [`VkAccessFlag2::ResourceHeapReadExt`], `dst_stage_mask` must include
    ///    [`VkPipelineStageFlag2::AllGraphics`], [`VkPipelineStageFlag2::AllCommands`], or one of
    ///    `VkPipelineStageFlag::*Stage` stages
    ///
    /// # Valid Usage (Implicit)
    ///  - `dst_access_mask` must be a valid combination of [`VkAccessFlag2`] values
    pub dst_access_mask: VkAccessFlags2,
}

const impl Default for VkMemoryBarrier2 {
    fn default() -> Self {
        VkMemoryBarrier2 {
            r#type: VkStructureType::MemoryBarrier2,
            next: null(),
            src_stage_mask: VkPipelineStageFlags2::default(),
            src_access_mask: VkAccessFlags2::default(),
            dst_stage_mask: VkPipelineStageFlags2::default(),
            dst_access_mask: VkAccessFlags2::default(),
        }
    }
}

impl NextChain for VkMemoryBarrier2 {
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
