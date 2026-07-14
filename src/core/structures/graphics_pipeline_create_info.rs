use crate::{
    VkPipeline, VkPipelineColorBlendStateCreateInfo, VkPipelineCreateFlags,
    VkPipelineDepthStencilStateCreateInfo, VkPipelineDynamicStateCreateInfo,
    VkPipelineInputAssemblyStateCreateInfo, VkPipelineLayout, VkPipelineMultisampleStateCreateInfo,
    VkPipelineRasterizationStateCreateInfo, VkPipelineShaderStageCreateInfo,
    VkPipelineTessellationStateCreateInfo, VkPipelineVertexInputStateCreateInfo,
    VkPipelineViewportStateCreateInfo, VkRenderPass, VkStructureType, util::NextChain,
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_FALSE, VK_NULL_HANDLE, VK_TRUE, VK_VERSION_1_0, VkDevice, VkDynamicState,
    VkPhysicalDeviceProperties, VkPipelineCreateFlag, VkPipelineLayoutCreateFlag, VkResult,
};

/// Structure specifying parameters of a newly created graphics pipeline
///
/// # Description
/// If any shader stage fails to compile, the compile log will be reported back to the application,
/// and [`VkResult::VkErrorInvalidShaderNv`] will be generated.
///
/// The state required for a graphics pipeline is divided into vertex input state,
/// pre-rasterization shader state, fragment shader state, and fragment output state.
///
/// ## Vertex Input State
/// Vertex input state is defined by:
///  - [`VkPipelineVertexInputStateCreateInfo`]
///  - [`VkPipelineInputAssemblyStateCreateInfo`]
///
/// If this pipeline specifies pre-rasterization state either directly or by including it as a
/// pipeline library and its pStages includes a vertex shader, this state must be specified to
/// create a complete graphics pipeline.
///
/// If a pipeline includes [`VkGraphicsPipelineLibraryFlagExt::VertexInputInterfaceExt`] in
/// [`VkGraphicsPipelineLibraryCreateInfoExt::flags`] either explicitly or as a default, and either
/// the conditions requiring this state for a complete graphics pipeline are met or this pipeline
/// does not specify pre-rasterization state in any way, that pipeline must specify this state
/// directly.
///
/// ## Pre-Rasterization Shader State
/// Pre-rasterization shader state is defined by:
///  - [`VkPipelineShaderStageCreateInfo`] entries for:
///    - Vertex shaders
///    - Tessellation control shaders
///    - Tessellation evaluation shaders
///    - Geometry shaders
///    - Task shaders
///    - Mesh shaders
///  - Within the [`VkPipelineLayout`], all descriptor sets with pre-rasterization shader bindings
///    if [`VkPipelineLayoutCreateFlag::IndependentSetsExt`] was specified.
///    - If [`VkPipelineLayoutCreateFlag::IndependentSetsExt`] was not specified, the full pipeline
///      layout must be specified.
///  - [`VkPipelineViewportStateCreateInfo`]
///  - [`VkPipelineRasterizationStateCreateInfo`]
///  - [`VkPipelineTessellationStateCreateInfo`]
///  - [`VkRenderPass`] and subpass parameter
///  - The `view_mask` parameter of [`VkPipelineRenderingCreateInfo`] (formats are ignored)
///  - [`VkPipelineDiscardRectangleStateCreateInfoExt`]
///  - [`VkPipelineFragmentShadingRateStateCreateInfoKhr`]
///  - Inclusion/omission of the [`VkPipelineCreateFlag2::PerLayerFragmentDensityValve`] flag
///
/// This state must be specified to create a complete graphics pipeline.
///
/// If either the `next` chain includes a [`VkGraphicsPipelineLibraryCreateInfoExt`] structure with
/// [`VkGraphicsPipelineLibraryFlagExt::PreRasterizationShadersExt`] included in `flags`, or it is
/// not specified and would default to include that value, this state must be specified in the
/// pipeline.
///
/// ## Fragment Shader State
/// Fragment shader state is defined by:
///  - A [`VkPipelineShaderStageCreateInfo`] entry for the fragment shader
///  - Within the [`VkPipelineLayout`], all descriptor sets with fragment shader bindings if
///    [`VkPipelineLayoutCreateFlag::IndependentSetsExt`] was specified.
///    - If [`VkPipelineLayoutCreateFlag::IndependentSetsExt`] was not specified, the full pipeline
///      layout must be specified.
///  - [`VkPipelineMultisampleStateCreateInfo`] if sample shading is enabled or renderpass is not
///    [`VK_NULL_HANDLE`]
///  - [`VkPipelineDepthStencilStateCreateInfo`]
///  - [`VkRenderPass`] and subpass parameter
///  - The `view_mask` parameter of [`VkPipelineRenderingCreateInfo`] (formats are ignored)
///  - [`VkPipelineFragmentShadingRateStateCreateInfoKhr`]
///  - [`VkPipelineFragmentShadingRateEnumStateCreateInfoNv`]
///  - [`VkPipelineRepresentativeFragmentTestStateCreateInfoNv`]
///  - Inclusion/omission of the
///    [`VkPipelineCreateFlag::RasterizationFragmentShadingRateAttachmentKhr`] flag
///  - Inclusion/omission of the
///    [`VkPipelineCreateFlag::RasterizationFragmentDensityMapAttachmentExt`] flag
///  - [`VkRenderingInputAttachmentIndexInfo`]
///  - Inclusion/omission of the [`VkPipelineCreateFlag2::PerLayerFragmentDensityValve`] flag
///  - The customResolve parameter of [`VkCustomResolveCreateInfoExt`]. Formats are ignored, and
///    not including the structure behaves identically to setting customResolve to [`VK_FALSE`],
///    unlike in fragment output interface state.
///
/// If a pipeline specifies pre-rasterization state either directly or by including it as a
/// pipeline library and rasterizerDiscardEnable is [`VK_FALSE`] or
/// [`VkDynamicState::RasterizerDiscardEnable`] is used, this state must be specified to create a
/// complete graphics pipeline.
///
/// If a pipeline includes [`VkGraphicsPipelineLibraryFlagExt::FragmentShaderExt`] in
/// [`VkGraphicsPipelineLibraryCreateInfoExt::flags`] either explicitly or as a default, and either
/// the conditions requiring this state for a complete graphics pipeline are met or this pipeline
/// does not specify pre-rasterization state in any way, that pipeline must specify this state
/// directly.
///
/// ## Fragment Output State
/// Fragment output state is defined by:
///  - [`VkPipelineColorBlendStateCreateInfo`]
///  - [`VkRenderPass`] and subpass parameter
///  - [`VkPipelineMultisampleStateCreateInfo`]
///  - The format parameters of [`VkPipelineRenderingCreateInfo`] (`view_mask` is ignored)
///  - [`VkAttachmentSampleCountInfoAmd`]
///  - [`VkAttachmentSampleCountInfoNv`]
///  - [`VkExternalFormatAndroid`]
///  - [`VkCustomResolveCreateInfoExt`]
///  - Inclusion/omission of the [`VkPipelineCreateFlag::CreateColorAttachmentFeedbackLoopExt`] and
///    [`VkPipelineCreateFlag::CreateDepthStencilAttachmentFeedbackLoopExt`] flags
///  - Inclusion/omission of the [`VkPipelineCreateFlag2::EnableLegacyDitheringBitExt`] flag
///  - [`VkRenderingAttachmentLocationInfo`]
///
/// If a pipeline specifies pre-rasterization state either directly or by including it as a
/// pipeline library and `rasterizer_discard_enable` is [`VK_FALSE`] or
/// [`VkDynamicState::RasterizerDiscardEnable`] is used, this state must be specified to create a
/// complete graphics pipeline.
///
/// If a pipeline includes [`VkGraphicsPipelineLibraryFlagExt::FragmentOutputInterfaceExt`] in
/// [`VkGraphicsPipelineLibraryCreateInfoExt::flags`] either explicitly or as a default, and either
/// the conditions requiring this state for a complete graphics pipeline are met or this pipeline
/// does not specify pre-rasterization state in any way, that pipeline must specify this state
/// directly.
///
/// ## Dynamic State
/// Dynamic state values set via `dynamic_state` must be ignored if the state they correspond to is
/// not otherwise statically set by one of the state subsets used to create the pipeline.
/// Additionally, setting dynamic state values must not modify whether state in a linked library is
/// static or dynamic; this is set and unchangeable when the library is created. For example, if a
/// pipeline only included pre-rasterization shader state, then any dynamic state value
/// corresponding to depth or stencil testing has no effect. Any linked library that has dynamic
/// state enabled that same dynamic state must also be enabled in all the other linked libraries to
/// which that dynamic state applies.
///
/// ### Complete Graphics Pipelines
/// A complete graphics pipeline always includes pre-rasterization shader state, with other subsets
/// included depending on that state as specified in the above sections.
///
/// ### Graphics Pipeline Library Layouts
/// If different subsets are linked together with pipeline layouts created with
/// [`VkPipelineLayoutCreateFlag::IndependentSetsExt`], the final effective pipeline layout is
/// effectively the union of the linked pipeline layouts. When binding descriptor sets for this
/// pipeline, the pipeline layout used must be compatible with this union. This pipeline layout can
/// be overridden when linking with [`VkPipelineCreateFlag::CreateLinkTimeOptimizationExt`] by
/// providing a [`VkPipelineLayout`] that is compatible with this union other than
/// [`VkPipelineLayoutCreateFlag::IndependentSetsExt`], or when linking without
/// [`VkPipelineCreateFlag::CreateLinkTimeOptimizationExt`] by providing a [`VkPipelineLayout`]
/// that is fully compatible with this union.
///
/// If the `next` chain includes a [`VkPipelineCreateFlags2CreateInfo`] structure,
/// [`VkPipelineCreateFlags2CreateInfo::flags`] from that structure is used instead of flags from
/// this structure.
///
/// TODO: Add Valid Usage
///
/// # Valid Usage (Implicit)
///  - Each of `base_pipeline_handle`, `layout`, and `render_pass` that are valid handles of
///    non-ignored parameters must have been created, allocated, or retrieved from the same
///    [`VkDevice`]
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkGraphicsPipelineCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::GraphicsPipelineCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of [`VkAttachmentSampleCountInfoAmd`],
    ///    [`VkCustomResolveCreateInfoExt`], [`VkExternalFormatAndroid`], [`VkExternalFormatOhos`],
    ///    [`VkGraphicsPipelineLibraryCreateInfoExt`],
    ///    [`VkGraphicsPipelineShaderGroupsCreateInfoNv`], [`VkMultiviewPerViewAttributesInfoNvx`],
    ///    [`VkPipelineBinaryInfoKhr`], [`VkPipelineCompilerControlCreateInfoAmd`],
    ///    [`VkPipelineCreateFlags2CreateInfo`], [`VkPipelineCreationFeedbackCreateInfo`],
    ///    [`VkPipelineDiscardRectangleStateCreateInfoExt`],
    ///    [`VkPipelineFragmentDensityMapLayeredCreateInfoValve`],
    ///    [`VkPipelineFragmentShadingRateEnumStateCreateInfoNv`],
    ///    [`VkPipelineFragmentShadingRateStateCreateInfoKhr`], [`VkPipelineLibraryCreateInfoKhr`],
    ///    [`VkPipelineRenderingCreateInfo`],
    ///    [`VkPipelineRepresentativeFragmentTestStateCreateInfoNv`],
    ///    [`VkPipelineRobustnessCreateInfo`], [`VkRenderingAttachmentLocationInfo`], or
    ///    [`VkRenderingInputAttachmentIndexInfo`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkPipelineCreateFlag`] specifying how the pipeline will be
    /// generated.
    pub flags: VkPipelineCreateFlags,

    /// `stage_count` is the number of entries in the pStages array.
    pub stage_count: u32,

    /// `stages` is a pointer to an array of `stage_count` [`VkPipelineShaderStageCreateInfo`]
    /// structures describing the set of the shader stages to be included in the graphics pipeline.
    pub stages: *const VkPipelineShaderStageCreateInfo,

    /// `vertex_input_state` is a pointer to a [`VkPipelineVertexInputStateCreateInfo`] structure.
    /// It is ignored if the pipeline includes a mesh shader stage. It can be [`null`] if the
    /// pipeline is created with the [`VkDynamicState::VertexInputExt`] dynamic state set.
    pub vertex_input_state: *const VkPipelineVertexInputStateCreateInfo,

    /// `input_assembly_state` is a pointer to a [`VkPipelineInputAssemblyStateCreateInfo`]
    /// structure which determines input assembly behavior for vertex shading. If the
    /// [`ext_extended_dynamic_state3`] extension is enabled, it can be [`null`] if the pipeline is
    /// created with both [`VkDynamicState::PrimitiveRestartEnable`], and
    /// [`VkDynamicState::PrimitiveTopology`] dynamic states set and
    /// `dynamic_primitive_topology_unrestricted` is [`VK_TRUE`]. It is ignored if the pipeline
    /// includes a mesh shader stage.
    pub input_assembly_state: *const VkPipelineInputAssemblyStateCreateInfo,

    /// `tessellation_state` is a pointer to a [`VkPipelineTessellationStateCreateInfo`] structure
    /// defining tessellation state used by tessellation shaders. It can be [`null`] if the
    /// pipeline is created with the [`VkDynamicState::PatchControlPointsExt`] dynamic state set.
    pub tessellation_state: *const VkPipelineTessellationStateCreateInfo,

    /// `viewport_state` is a pointer to a [`VkPipelineViewportStateCreateInfo`] structure defining
    /// viewport state used when rasterization is enabled. If the [`ext_extended_dynamic_state3`]
    /// extension is enabled, it can be [`null`] if the pipeline is created with both
    /// [`VkDynamicState::ViewportWithCount`], and [`VkDynamicState::ScissorWithCount`] dynamic
    /// states set.
    pub viewport_state: *const VkPipelineViewportStateCreateInfo,

    /// `rasterization_state` is a pointer to a [`VkPipelineRasterizationStateCreateInfo`]
    /// structure defining rasterization state. If the [`ext_extended_dynamic_state3`] extension is
    /// enabled, it can be [`null`] if the pipeline is created with all of
    /// [`VkDynamicState::DepthClampEnableExt`], [`VkDynamicState::RasterizerDiscardEnable`],
    /// [`VkDynamicState::PolygonModeExt`], [`VkDynamicState::CullMode`],
    /// [`VkDynamicState::FrontFace`], [`VkDynamicState::DepthBiasEnable`],
    /// [`VkDynamicState::DepthBias`], and [`VkDynamicState::LineWidth`] dynamic states set.
    pub rasterization_state: *const VkPipelineRasterizationStateCreateInfo,

    /// `multisample_state` is a pointer to a VkPipelineMultisampleStateCreateInfo structure
    /// defining multisample state used when rasterization is enabled. If the
    /// [`ext_extended_dynamic_state3`] extension is enabled, it can be [`null`] if the pipeline is
    /// created with all of [`VkDynamicState::RasterizationSamplesExt`],
    /// [`VkDynamicState::SampleMaskExt`], and [`VkDynamicState::AlphaToCoverageEnableExt`] dynamic
    /// states set, and either the alphaToOne feature is not enabled or
    /// [`VkDynamicState::AlphaToOneEnableExt`] is set, in which case
    /// [`VkPipelineMultisampleStateCreateInfo::sample_shading_enable`] is assumed to be
    /// [`VK_FALSE`].
    pub multisample_state: *const VkPipelineMultisampleStateCreateInfo,

    /// `depth_stencil_state` is a pointer to a [`VkPipelineDepthStencilStateCreateInfo`] structure
    /// defining depth/stencil state used when rasterization is enabled for depth or stencil
    /// attachments accessed during rendering. If the [`ext_extended_dynamic_state3`] extension is
    /// enabled, it can be [`null`] if the pipeline is created with all of
    /// [`VkDynamicState::DepthTestEnable`], [`VkDynamicState::DepthWriteEnable`],
    /// [`VkDynamicState::DepthCompareOp`], [`VkDynamicState::DepthBoundsTestEnable`],
    /// [`VkDynamicState::StencilTestEnable`], [`VkDynamicState::StencilOp`], and
    /// [`VkDynamicState::DepthBounds`] dynamic states set.
    pub depth_stencil_state: *const VkPipelineDepthStencilStateCreateInfo,

    /// `color_blend_state` is a pointer to a [`VkPipelineColorBlendStateCreateInfo`] structure
    /// defining color blend state used when rasterization is enabled for any color attachments
    /// accessed during rendering. If the [`ext_extended_dynamic_state3`] extension is enabled, it
    /// can be [`null`] if the pipeline is created with all of
    /// [`VkDynamicState::LogicOpEnableExt`], [`VkDynamicState::LogicOpExt`],
    /// [`VkDynamicState::ColorBlendEnableExt`], [`VkDynamicState::ColorBlendEquationExt`],
    /// [`VkDynamicState::ColorWriteMaskExt`], and [`VkDynamicState::BlendConstants`] dynamic
    /// states set.
    pub color_blend_state: *const VkPipelineColorBlendStateCreateInfo,

    /// `dynamic_state` is a pointer to a [`VkPipelineDynamicStateCreateInfo`] structure defining
    /// which properties of the pipeline state object are dynamic and can be changed independently
    /// of the pipeline state. This can be [`null`], which means no state in the pipeline is
    /// considered dynamic.
    ///
    /// # Valid Usage (Implicit)
    ///  - If `dynamic_state` is not [`null`], `dynamic_state` must be a valid pointer to a valid
    ///    [`VkPipelineDynamicStateCreateInfo`] structure
    pub dynamic_state: *const VkPipelineDynamicStateCreateInfo,

    /// `layout` is the description of binding locations used by both the pipeline and descriptor
    /// sets used with the pipeline. If [`VkPhysicalDeviceProperties::api_version`] is greater than
    /// or equal to Vulkan 1.3 or [`khr_maintenance4`] is enabled layout must not be accessed by
    /// the implementation outside of the duration of the command this structure is passed to.
    pub layout: VkPipelineLayout,

    /// `render_pass` is a handle to a render pass object describing the environment in which the
    /// pipeline will be used. The pipeline must only be used with a render pass instance
    /// compatible with the one provided. See Render Pass Compatibility for more information. The
    /// implementation must not access this object outside of the duration of the command this
    /// structure is passed to.
    pub render_pass: VkRenderPass,

    /// `subpass` is the index of the subpass in the render pass where this pipeline will be used.
    pub subpass: u32,

    /// `base_pipeline_handle` is a pipeline to derive from.
    pub base_pipeline_handle: VkPipeline,

    /// `base_pipeline_index` is an index into the `create_infos` parameter to use as a pipeline to
    /// derive from.
    pub base_pipeline_index: i32,
}

impl const Default for VkGraphicsPipelineCreateInfo {
    fn default() -> Self {
        VkGraphicsPipelineCreateInfo {
            r#type: VkStructureType::GraphicsPipelineCreateInfo,
            next: null(),
            flags: VkPipelineCreateFlags::empty(),
            stage_count: 0,
            stages: null(),
            vertex_input_state: null(),
            input_assembly_state: null(),
            tessellation_state: null(),
            viewport_state: null(),
            rasterization_state: null(),
            multisample_state: null(),
            depth_stencil_state: null(),
            color_blend_state: null(),
            dynamic_state: null(),
            layout: VkPipelineLayout::null(),
            render_pass: VkRenderPass::null(),
            subpass: 0,
            base_pipeline_handle: VkPipeline::null(),
            base_pipeline_index: 0,
        }
    }
}

impl NextChain for VkGraphicsPipelineCreateInfo {
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
