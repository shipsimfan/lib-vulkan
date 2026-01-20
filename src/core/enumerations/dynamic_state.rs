// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_TRUE, VK_VERSION_1_0, VK_VERSION_1_3, VK_VERSION_1_4};

/// Indicate which dynamic state is taken from dynamic state commands
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkDynamicState {
    /// [`VkDynamicState::Viewport`] specifies that the `viewports` state in
    /// [`VkPipelineViewportStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetViewport`] before any drawing commands. The number of viewports used by a
    /// pipeline is still specified by the `viewport_count` member of
    /// [`VkPipelineViewportStateCreateInfo`].
    Viewport = 0,

    /// [`VkDynamicState::Scissor`] specifies that the `scissors` state in
    /// [`VkPipelineViewportStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetScissor`] before any drawing commands. The number of scissor rectangles used by a
    /// pipeline is still specified by the `scissor_count` member of
    /// [`VkPipelineViewportStateCreateInfo`].
    Scissor = 1,

    /// [`VkDynamicState::LineWidth`] specifies that the `line_width` state in
    /// [`VkPipelineRasterizationStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetLineWidth`] before any drawing commands that generate line primitives for the
    /// rasterizer.
    LineWidth = 2,

    /// [`VkDynamicState::DepthBias`] specifies that any instance of
    /// [`VkDepthBiasRepresentationInfoExt`] included in the `next` chain of
    /// [`VkPipelineRasterizationStateCreateInfo`] as well as the `depth_bias_constant_factor`,
    /// `depth_bias_clamp` and `depth_bias_slope_factor` states in
    /// [`VkPipelineRasterizationStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetDepthBias`] or [`VkCmdSetDepthBias2Ext`] before any draws are performed with
    /// depth bias enabled.
    DepthBias = 3,

    /// [`VkDynamicState::BlendConstants`] specifies that the `blend_constants` state in
    /// [`VkPipelineColorBlendStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetBlendConstants`] before any draws are performed with a pipeline state with
    /// [`VkPipelineColorBlendAttachmentState`] member `blend_enable` set to [`VK_TRUE`] and any of
    /// the blend functions using a constant blend color.
    BlendConstants = 4,

    /// [`VkDynamicState::DepthBounds`] specifies that the `min_depth_bounds` and
    /// `max_depth_bounds` states of [`VkPipelineDepthStencilStateCreateInfo`] will be ignored and
    /// must be set dynamically with [`VkCmdSetDepthBounds`] before any draws are performed with a
    /// pipeline state with [`VkPipelineDepthStencilStateCreateInfo`] member
    /// `depth_bounds_test_enable` set to [`VK_TRUE`].
    DepthBounds = 5,

    /// [`VkDynamicState::StencilCompareMask`] specifies that the `compare_mask` state in
    /// [`VkPipelineDepthStencilStateCreateInfo`] for both front and back will be ignored and must
    /// be set dynamically with [`VkCmdSetStencilCompareMask`] before any draws are performed with
    /// a pipeline state with [`VkPipelineDepthStencilStateCreateInfo`] member
    /// `stencil_test_enable` set to [`VK_TRUE`]
    StencilCompareMask = 6,

    /// [`VkDynamicState::StencilWriteMask`] specifies that the `write_mask` state in
    /// [`VkPipelineDepthStencilStateCreateInfo`] for both front and back will be ignored and must
    /// be set dynamically with [`VkCmdSetStencilWriteMask`] before any draws are performed with a
    /// pipeline state with [`VkPipelineDepthStencilStateCreateInfo`] member `stencil_test_enable`
    /// set to [`VK_TRUE`]
    StencilWriteMask = 7,

    /// [`VkDynamicState::StencilReference`] specifies that the reference state in
    /// [`VkPipelineDepthStencilStateCreateInfo`] for both front and back will be ignored and must
    /// be set dynamically with [`VkCmdSetStencilReference`] before any draws are performed with a
    /// pipeline state with [`VkPipelineDepthStencilStateCreateInfo`] member `stencil_test_enable`
    /// set to [`VK_TRUE`]
    StencilReference = 8,

    /// [`VkDynamicState::CullMode`] specifies that the cullMode state in VkPipelineRasterizationStateCreateInfo will be ignored and must be set dynamically with vkCmdSetCullMode before any drawing commands.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    CullMode = 1000267000,

    /// [`VkDynamicState::FrontFace`] specifies that the frontFace state in VkPipelineRasterizationStateCreateInfo will be ignored and must be set dynamically with vkCmdSetFrontFace before any drawing commands.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    FrontFace = 1000267001,

    /// [`VkDynamicState::PrimitiveTopology`] specifies that the topology state in VkPipelineInputAssemblyStateCreateInfo only specifies the topology class, and the specific topology order and adjacency must be set dynamically with vkCmdSetPrimitiveTopology before any drawing commands.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    PrimitiveTopology = 1000267002,

    /// [`VkDynamicState::ViewportWithCount`] specifies that the viewportCount and `viewports` state in VkPipelineViewportStateCreateInfo will be ignored and must be set dynamically with vkCmdSetViewportWithCount before any draw call.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    ViewportWithCount = 1000267003,

    /// [`VkDynamicState::ScissorWithCount`] specifies that the scissorCount and `scissors` state in VkPipelineViewportStateCreateInfo will be ignored and must be set dynamically with vkCmdSetScissorWithCount before any draw call.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    ScissorWithCount = 1000267004,

    /// [`VkDynamicState::VertexInputBindingStride`] specifies that the stride state in VkVertexInputBindingDescription will be ignored and must be set dynamically with vkCmdBindVertexBuffers2 before any draw call.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    VertexInputBindingStride = 1000267005,

    /// [`VkDynamicState::DepthTestEnable`] specifies that the depthTestEnable state in VkPipelineDepthStencilStateCreateInfo will be ignored and must be set dynamically with vkCmdSetDepthTestEnable before any draw call.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    DepthTestEnable = 1000267006,

    /// [`VkDynamicState::DepthWriteEnable`] specifies that the depthWriteEnable state in VkPipelineDepthStencilStateCreateInfo will be ignored and must be set dynamically with vkCmdSetDepthWriteEnable before any draw call.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    DepthWriteEnable = 1000267007,

    /// [`VkDynamicState::DepthCompareOp`] specifies that the depthCompareOp state in VkPipelineDepthStencilStateCreateInfo will be ignored and must be set dynamically with vkCmdSetDepthCompareOp before any draw call.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    DepthCompareOp = 1000267008,

    /// [`VkDynamicState::DepthBoundsTestEnable`] specifies that the depthBoundsTestEnable state in VkPipelineDepthStencilStateCreateInfo will be ignored and must be set dynamically with vkCmdSetDepthBoundsTestEnable before any draw call.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    DepthBoundsTestEnable = 1000267009,

    /// [`VkDynamicState::StencilTestEnable`] specifies that the stencilTestEnable state in VkPipelineDepthStencilStateCreateInfo will be ignored and must be set dynamically with vkCmdSetStencilTestEnable before any draw call.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    StencilTestEnable = 1000267010,

    /// [`VkDynamicState::StencilOp`] specifies that the failOp, passOp, depthFailOp, and compareOp states in VkPipelineDepthStencilStateCreateInfo for both front and back will be ignored and must be set dynamically with vkCmdSetStencilOp before any draws are performed with a pipeline state with VkPipelineDepthStencilStateCreateInfo member stencilTestEnable set to VK_TRUE
    ///
    /// Provided by [`VK_VERSION_1_3`]
    StencilOp = 1000267011,

    /// [`VkDynamicState::RasterizerDiscardEnable`] specifies that the rasterizerDiscardEnable state in VkPipelineRasterizationStateCreateInfo will be ignored and must be set dynamically with vkCmdSetRasterizerDiscardEnable before any drawing commands.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    RasterizerDiscardEnable = 1000377001,

    /// [`VkDynamicState::DepthBiasEnable`] specifies that the depthBiasEnable state in VkPipelineRasterizationStateCreateInfo will be ignored and must be set dynamically with vkCmdSetDepthBiasEnable before any drawing commands.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    DepthBiasEnable = 1000377002,

    /// [`VkDynamicState::PrimitiveRestartEnable`] specifies that the primitiveRestartEnable state in VkPipelineInputAssemblyStateCreateInfo will be ignored and must be set dynamically with vkCmdSetPrimitiveRestartEnable before any drawing commands.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    PrimitiveRestartEnable = 1000377004,

    /// [`VkDynamicState::LineStipple`] specifies that the lineStippleFactor and lineStipplePattern state in VkPipelineRasterizationLineStateCreateInfo will be ignored and must be set dynamically with vkCmdSetLineStipple before any draws are performed with a pipeline state with VkPipelineRasterizationLineStateCreateInfo member stippledLineEnable set to VK_TRUE.
    ///
    /// Provided by [`VK_VERSION_1_4`]
    LineStipple = 1000259000,

    /// [`VkDynamicState::ViewportWScalingNv`] specifies that the pViewportWScalings state in VkPipelineViewportWScalingStateCreateInfoNv will be ignored and must be set dynamically with vkCmdSetViewportWScalingNv before any draws are performed with a pipeline state with VkPipelineViewportWScalingStateCreateInfoNv member viewportScalingEnable set to VK_TRUE
    ///
    /// Provided by [`nv_clip_space_w_scaling`]
    ViewportWScalingNv = 1000087000,

    /// [`VkDynamicState::DiscardRectangleExt`] specifies that the pDiscardRectangles state in VkPipelineDiscardRectangleStateCreateInfoExt will be ignored and must be set dynamically with vkCmdSetDiscardRectangleExt before any draw or clear commands.
    ///
    /// Provided by [`ext_discard_rectangles`]
    DiscardRectangleExt = 1000099000,

    /// [`VkDynamicState::DiscardRectangleEnableExt`] specifies that the presence of the VkPipelineDiscardRectangleStateCreateInfoExt structure in the VkGraphicsPipelineCreateInfo chain with a discardRectangleCount greater than zero does not implicitly enable discard rectangles and they must be enabled dynamically with vkCmdSetDiscardRectangleEnableExt before any draw commands. This is available on implementations that support at least specVersion 2 of the VK_Ext_discard_rectangles extension.
    ///
    /// Provided by [`ext_discard_rectangles`]
    DiscardRectangleEnableExt = 1000099001,

    /// [`VkDynamicState::DiscardRectangleModeExt`] specifies that the discardRectangleMode state in VkPipelineDiscardRectangleStateCreateInfoExt will be ignored and must be set dynamically with vkCmdSetDiscardRectangleModeExt before any draw commands. This is available on implementations that support at least specVersion 2 of the VK_Ext_discard_rectangles extension.
    ///
    /// Provided by [`ext_discard_rectangles`]
    DiscardRectangleModeExt = 1000099002,

    /// [`VkDynamicState::SampleLocationsExt`] specifies that the sampleLocationsInfo state in VkPipelineSampleLocationsStateCreateInfoExt will be ignored and must be set dynamically with vkCmdSetSampleLocationsExt before any draw or clear commands. Enabling custom sample locations is still indicated by the sampleLocationsEnable member of VkPipelineSampleLocationsStateCreateInfoExt.
    ///
    /// Provided by [`ext_sample_locations`]
    SampleLocationsExt = 1000143000,

    /// [`VkDynamicState::RayTracingPipelineStackSizeKhr`] specifies that the default stack size computation for the pipeline will be ignored and must be set dynamically with vkCmdSetRayTracingPipelineStackSizeKhr before any ray tracing calls are performed.
    ///
    /// Provided by [`khr_ray_tracing_pipeline`]
    RayTracingPipelineStackSizeKhr = 1000347000,

    /// [`VkDynamicState::ViewportShadingRatePaletteNv`] specifies that the pShadingRatePalettes state in VkPipelineViewportShadingRateImageStateCreateInfoNv will be ignored and must be set dynamically with vkCmdSetViewportShadingRatePaletteNv before any drawing commands.
    ///
    /// Provided by [`nv_shading_rate_image`]
    ViewportShadingRatePaletteNv = 1000164004,

    /// [`VkDynamicState::ViewportCoarseSampleOrderNv`] specifies that the coarse sample order state in VkPipelineViewportCoarseSampleOrderStateCreateInfoNv will be ignored and must be set dynamically with vkCmdSetCoarseSampleOrderNv before any drawing commands.
    ///
    /// Provided by [`nv_shading_rate_image`]
    ViewportCoarseSampleOrderNv = 1000164006,

    /// [`VkDynamicState::ExclusiveScissorEnableNv`] specifies that the exclusive scissors must be explicitly enabled with vkCmdSetExclusiveScissorEnableNv and the exclusiveScissorCount value in VkPipelineViewportExclusiveScissorStateCreateInfoNv will not implicitly enable them. This is available on implementations that support at least specVersion 2 of the VK_Nvscissor_exclusive extension.
    ///
    /// Provided by [`nv_scissor_exclusive`]
    ExclusiveScissorEnableNv = 1000205000,

    /// [`VkDynamicState::ExclusiveScissorNv`] specifies that the pExclusiveScissors state in VkPipelineViewportExclusiveScissorStateCreateInfoNv will be ignored and must be set dynamically with vkCmdSetExclusiveScissorNv before any drawing commands.
    ///
    /// Provided by [`nv_scissor_exclusive`]
    ExclusiveScissorNv = 1000205001,

    /// [`VkDynamicState::FragmentShadingRateKhr`] specifies that state in VkPipelineFragmentShadingRateStateCreateInfoKhr and VkPipelineFragmentShadingRateEnumStateCreateInfoNv will be ignored and must be set dynamically with vkCmdSetFragmentShadingRateKhr or vkCmdSetFragmentShadingRateEnumNv before any drawing commands.
    ///
    /// Provided by [`khr_fragment_shading_rate`]
    FragmentShadingRateKhr = 1000226000,

    /// [`VkDynamicState::VertexInputExt`] specifies that the pVertexInputState state will be ignored and must be set dynamically with vkCmdSetVertexInputExt before any drawing commands
    ///
    /// Provided by [`ext_vertex_input_dynamic_state`]
    VertexInputExt = 1000352000,

    /// [`VkDynamicState::PatchControlPointsExt`] specifies that the patchControlPoints state in VkPipelineTessellationStateCreateInfo will be ignored and must be set dynamically with vkCmdSetPatchControlPointsExt before any drawing commands.
    ///
    /// Provided by [`ext_extended_dynamic_state2`]
    PatchControlPointsExt = 1000377000,

    /// [`VkDynamicState::LogicOpExt`] specifies that the logicOp state in VkPipelineColorBlendStateCreateInfo will be ignored and must be set dynamically with vkCmdSetLogicOpExt before any drawing commands.
    ///
    /// Provided by [`ext_extended_dynamic_state2`]
    LogicOpExt = 1000377003,

    /// [`VkDynamicState::ColorWriteEnableExt`] specifies that the pColorWriteEnables state in VkPipelineColorWriteCreateInfoExt will be ignored and must be set dynamically with vkCmdSetColorWriteEnableExt before any draw call.
    ///
    /// Provided by [`ext_color_write_enable`]
    ColorWriteEnableExt = 1000381000,

    /// [`VkDynamicState::DepthClampEnableExt`] specifies that the depthClampEnable state in VkPipelineRasterizationStateCreateInfo will be ignored and must be set dynamically with vkCmdSetDepthClampEnableExt before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    DepthClampEnableExt = 1000455003,

    /// [`VkDynamicState::PolygonModeExt`] specifies that the polygonMode state in VkPipelineRasterizationStateCreateInfo will be ignored and must be set dynamically with vkCmdSetPolygonModeExt before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    PolygonModeExt = 1000455004,

    /// [`VkDynamicState::RasterizationSamplesExt`] specifies that the rasterizationSamples state in VkPipelineMultisampleStateCreateInfo will be ignored and must be set dynamically with vkCmdSetRasterizationSamplesExt before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    RasterizationSamplesExt = 1000455005,

    /// [`VkDynamicState::SampleMaskExt`] specifies that the pSampleMask state in VkPipelineMultisampleStateCreateInfo will be ignored and must be set dynamically with vkCmdSetSampleMaskExt before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    SampleMaskExt = 1000455006,

    /// [`VkDynamicState::AlphaToCoverageEnableExt`] specifies that the alphaToCoverageEnable state in VkPipelineMultisampleStateCreateInfo will be ignored and must be set dynamically with vkCmdSetAlphaToCoverageEnableExt before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    AlphaToCoverageEnableExt = 1000455007,

    /// [`VkDynamicState::AlphaToOneEnableExt`] specifies that the alphaToOneEnable state in VkPipelineMultisampleStateCreateInfo will be ignored and must be set dynamically with vkCmdSetAlphaToOneEnableExt before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    AlphaToOneEnableExt = 1000455008,

    /// [`VkDynamicState::LogicOpEnableExt`] specifies that the logicOpEnable state in VkPipelineColorBlendStateCreateInfo will be ignored and must be set dynamically with vkCmdSetLogicOpEnableExt before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    LogicOpEnableExt = 1000455009,

    /// [`VkDynamicState::ColorBlendEnableExt`] specifies that the blendEnable state in VkPipelineColorBlendAttachmentState will be ignored and must be set dynamically with vkCmdSetColorBlendEnableExt before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    ColorBlendEnableExt = 1000455010,

    /// [`VkDynamicState::ColorBlendEquationExt`] specifies that the srcColorBlendFactor, dstColorBlendFactor, colorBlendOp, srcAlphaBlendFactor, dstAlphaBlendFactor, and alphaBlendOp states in VkPipelineColorBlendAttachmentState will be ignored and must be set dynamically with vkCmdSetColorBlendEquationExt before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    ColorBlendEquationExt = 1000455011,

    /// [`VkDynamicState::ColorWriteMaskExt`] specifies that the colorWriteMask state in VkPipelineColorBlendAttachmentState will be ignored and must be set dynamically with vkCmdSetColorWriteMaskExt before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    ColorWriteMaskExt = 1000455012,

    /// [`VkDynamicState::TessellationDomainOriginExt`] specifies that the domainOrigin state in VkPipelineTessellationDomainOriginStateCreateInfo will be ignored and must be set dynamically with vkCmdSetTessellationDomainOriginExt before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with [`khr_maintenance2`] or [`VK_VERSION_1_1`]
    TessellationDomainOriginExt = 1000455002,

    /// [`VkDynamicState::RasterizationStreamExt`] specifies that the rasterizationStream state in VkPipelineRasterizationStateStreamCreateInfoExt will be ignored and must be set dynamically with vkCmdSetRasterizationStreamExt before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with [`ext_transform_feedback`]
    RasterizationStreamExt = 1000455013,

    /// [`VkDynamicState::ConservativeRasterizationModeExt`] specifies that the conservativeRasterizationMode state in VkPipelineRasterizationConservativeStateCreateInfoExt will be ignored and must be set dynamically with vkCmdSetConservativeRasterizationModeExt before any draw call.
    ///
    /// Provided by [`ext_conservative_rasterization`] with [`ext_extended_dynamic_state3`]
    ConservativeRasterizationModeExt = 1000455014,

    /// [`VkDynamicState::ExtraPrimitiveOverestimationSizeExt`] specifies that the extraPrimitiveOverestimationSize state in VkPipelineRasterizationConservativeStateCreateInfoExt will be ignored and must be set dynamically with vkCmdSetExtraPrimitiveOverestimationSizeExt before any draw call.
    ///
    /// Provided by [`ext_conservative_rasterization`] with [`ext_extended_dynamic_state3`]
    ExtraPrimitiveOverestimationSizeExt = 1000455015,

    /// [`VkDynamicState::DepthClipEnableExt`] specifies that the depthClipEnable state in VkPipelineRasterizationDepthClipStateCreateInfoExt will be ignored and must be set dynamically with vkCmdSetDepthClipEnableExt before any draw call.
    ///
    /// Provided by VK_EXT_depth_clip_enable with VK_EXT_extended_dynamic_state3
    DepthClipEnableExt = 1000455016,

    /// [`VkDynamicState::SampleLocationsEnableExt`] specifies that the sampleLocationsEnable state in VkPipelineSampleLocationsStateCreateInfoExt will be ignored and must be set dynamically with vkCmdSetSampleLocationsEnableExt before any draw call.
    ///
    /// Provided by VK_EXT_extended_dynamic_state3 with VK_EXT_sample_locations
    SampleLocationsEnableExt = 1000455017,

    /// [`VkDynamicState::ColorBlendAdvancedExt`] specifies that the colorBlendOp state in VkPipelineColorBlendAttachmentState, and srcPremultiplied, dstPremultiplied, and blendOverlap states in VkPipelineColorBlendAdvancedStateCreateInfoExt will be ignored and must be set dynamically with vkCmdSetColorBlendAdvancedExt before any draw call.
    ///
    /// Provided by VK_EXT_blend_operation_advanced with VK_EXT_extended_dynamic_state3
    ColorBlendAdvancedExt = 1000455018,

    /// [`VkDynamicState::ProvokingVertexModeExt`] specifies that the provokingVertexMode state in VkPipelineRasterizationProvokingVertexStateCreateInfoExt will be ignored and must be set dynamically with vkCmdSetProvokingVertexModeExt before any draw call.
    ///
    /// Provided by VK_EXT_extended_dynamic_state3 with VK_EXT_provoking_vertex
    ProvokingVertexModeExt = 1000455019,

    /// [`VkDynamicState::LineRasterizationModeExt`] specifies that the lineRasterizationMode state in VkPipelineRasterizationLineStateCreateInfo will be ignored and must be set dynamically with vkCmdSetLineRasterizationModeExt before any draw call.
    ///
    /// Provided by VK_EXT_extended_dynamic_state3 with VK_EXT_line_rasterization
    LineRasterizationModeExt = 1000455020,

    /// [`VkDynamicState::LineStippleEnableExt`] specifies that the stippledLineEnable state in VkPipelineRasterizationLineStateCreateInfo will be ignored and must be set dynamically with vkCmdSetLineStippleEnableExt before any draw call.
    ///
    /// Provided by VK_EXT_extended_dynamic_state3 with VK_EXT_line_rasterization
    LineStippleEnableExt = 1000455021,

    /// [`VkDynamicState::DepthClipNegativeOneToOneExt`] specifies that the negativeOneToOne state in VkPipelineViewportDepthClipControlCreateInfoExt will be ignored and must be set dynamically with vkCmdSetDepthClipNegativeOneToOneExt before any draw call.
    ///
    /// Provided by VK_EXT_depth_clip_control with VK_EXT_extended_dynamic_state3
    DepthClipNegativeOneToOneExt = 1000455022,

    /// [`VkDynamicState::ViewportWScalingEnableNv`] specifies that the viewportWScalingEnable state in VkPipelineViewportWScalingStateCreateInfoNv will be ignored and must be set dynamically with vkCmdSetViewportWScalingEnableNv before any draw call.
    ///
    /// Provided by VK_EXT_extended_dynamic_state3 with VK_NV_clip_space_w_scaling
    ViewportWScalingEnableNv = 1000455023,

    /// [`VkDynamicState::ViewportSwizzleNv`] specifies that the viewportCount, and pViewportSwizzles states in VkPipelineViewportSwizzleStateCreateInfoNv will be ignored and must be set dynamically with vkCmdSetViewportSwizzleNv before any draw call.
    ///
    /// Provided by VK_EXT_extended_dynamic_state3 with VK_NV_viewport_swizzle
    ViewportSwizzleNv = 1000455024,

    /// [`VkDynamicState::CoverageToColorEnableNv`] specifies that the coverageToColorEnable state in VkPipelineCoverageToColorStateCreateInfoNv will be ignored and must be set dynamically with vkCmdSetCoverageToColorEnableNv before any draw call.
    ///
    /// Provided by VK_EXT_extended_dynamic_state3 with VK_NV_fragment_coverage_to_color
    CoverageToColorEnableNv = 1000455025,

    /// [`VkDynamicState::CoverageToColorLocationNv`] specifies that the coverageToColorLocation state in VkPipelineCoverageToColorStateCreateInfoNv will be ignored and must be set dynamically with vkCmdSetCoverageToColorLocationNv before any draw call.
    ///
    /// Provided by VK_EXT_extended_dynamic_state3 with VK_NV_fragment_coverage_to_color
    CoverageToColorLocationNv = 1000455026,

    /// [`VkDynamicState::CoverageModulationModeNv`] specifies that the coverageModulationMode state in VkPipelineCoverageModulationStateCreateInfoNv will be ignored and must be set dynamically with vkCmdSetCoverageModulationModeNv before any draw call.
    ///
    /// Provided by VK_EXT_extended_dynamic_state3 with VK_NV_framebuffer_mixed_samples
    CoverageModulationModeNv = 1000455027,

    /// [`VkDynamicState::CoverageModulationTableEnableNv`] specifies that the coverageModulationTableEnable state in VkPipelineCoverageModulationStateCreateInfoNv will be ignored and must be set dynamically with vkCmdSetCoverageModulationTableEnableNv before any draw call.
    ///
    /// Provided by VK_EXT_extended_dynamic_state3 with VK_NV_framebuffer_mixed_samples
    CoverageModulationTableEnableNv = 1000455028,

    /// [`VkDynamicState::CoverageModulationTableNv`] specifies that the coverageModulationTableCount, and pCoverageModulationTable states in VkPipelineCoverageModulationStateCreateInfoNv will be ignored and must be set dynamically with vkCmdSetCoverageModulationTableNv before any draw call.
    ///
    /// Provided by VK_EXT_extended_dynamic_state3 with VK_NV_framebuffer_mixed_samples
    CoverageModulationTableNv = 1000455029,

    /// [`VkDynamicState::ShadingRateImageEnableNv`] specifies that the shadingRateImageEnable state in VkPipelineViewportShadingRateImageStateCreateInfoNv will be ignored and must be set dynamically with vkCmdSetShadingRateImageEnableNv before any draw call.
    ///
    /// Provided by VK_EXT_extended_dynamic_state3 with VK_NV_shading_rate_image
    ShadingRateImageEnableNv = 1000455030,

    /// [`VkDynamicState::RepresentativeFragmentTestEnableNv`] specifies that the representativeFragmentTestEnable state in VkPipelineRepresentativeFragmentTestStateCreateInfoNv will be ignored and must be set dynamically with vkCmdSetRepresentativeFragmentTestEnableNv before any draw call.
    ///
    /// Provided by VK_EXT_extended_dynamic_state3 with VK_NV_representative_fragment_test
    RepresentativeFragmentTestEnableNv = 1000455031,

    /// [`VkDynamicState::CoverageReductionModeNv`] specifies that the coverageReductionMode state in VkPipelineCoverageReductionStateCreateInfoNv will be ignored and must be set dynamically with vkCmdSetCoverageReductionModeNv before any draw call.
    ///
    /// Provided by VK_EXT_extended_dynamic_state3 with VK_NV_coverage_reduction_mode
    CoverageReductionModeNv = 1000455032,

    /// [`VkDynamicState::AttachmentFeedbackLoopEnableExt`] specifies that the VK_PipelineCREATE_ColorAttachmentFeedbackLoopBIT_Ext and VK_PipelineCREATE_DepthStencilAttachmentFeedbackLoopBIT_Ext flags will be ignored and must be set dynamically with vkCmdSetAttachmentFeedbackLoopEnableExt before any draw call.
    ///
    /// Provided by VK_EXT_attachment_feedback_loop_dynamic_state
    AttachmentFeedbackLoopEnableExt = 1000524000,

    /// [`VkDynamicState::DepthClampRangeExt`] specifies that the depthClampMode and pDepthClampRange state in VkPipelineViewportDepthClampControlCreateInfoExt will be ignored and must be set dynamically with vkCmdSetDepthClampRangeExt before any draw call.
    ///
    /// Provided by VK_EXT_depth_clamp_control
    DepthClampRangeExt = 1000582000,
}
