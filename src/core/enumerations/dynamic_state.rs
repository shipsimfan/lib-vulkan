// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_TRUE, VK_VERSION_1_0, VK_VERSION_1_1, VK_VERSION_1_3, VK_VERSION_1_4};

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

    /// [`VkDynamicState::CullMode`] specifies that the `cull_mode` state in
    /// [`VkPipelineRasterizationStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetCullMode`] before any drawing commands.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    CullMode = 1000267000,

    /// [`VkDynamicState::FrontFace`] specifies that the `front_face` state in
    /// [`VkPipelineRasterizationStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetFrontFace`] before any drawing commands.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    FrontFace = 1000267001,

    /// [`VkDynamicState::PrimitiveTopology`] specifies that the `topology` state in
    /// [`VkPipelineInputAssemblyStateCreateInfo`] only specifies the topology class, and the
    /// specific topology order and adjacency must be set dynamically with
    /// [`VkCmdSetPrimitiveTopology`] before any drawing commands.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    PrimitiveTopology = 1000267002,

    /// [`VkDynamicState::ViewportWithCount`] specifies that the `viewport_count` and `viewports`
    /// state in [`VkPipelineViewportStateCreateInfo`] will be ignored and must be set dynamically
    /// with [`VkCmdSetViewportWithCount`] before any draw call.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    ViewportWithCount = 1000267003,

    /// [`VkDynamicState::ScissorWithCount`] specifies that the `scissor_count` and `scissors`
    /// state in [`VkPipelineViewportStateCreateInfo`] will be ignored and must be set dynamically
    /// with [`VkCmdSetScissorWithCount`] before any draw call.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    ScissorWithCount = 1000267004,

    /// [`VkDynamicState::VertexInputBindingStride`] specifies that the `stride` state in
    /// [`VkVertexInputBindingDescription`] will be ignored and must be set dynamically with
    /// [`VkCmdBindVertexBuffers2`] before any draw call.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    VertexInputBindingStride = 1000267005,

    /// [`VkDynamicState::DepthTestEnable`] specifies that the `depth_test_enable` state in
    /// [`VkPipelineDepthStencilStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetDepthTestEnable`] before any draw call.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    DepthTestEnable = 1000267006,

    /// [`VkDynamicState::DepthWriteEnable`] specifies that the `depth_write_enable` state in
    /// [`VkPipelineDepthStencilStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetDepthWriteEnable`] before any draw call.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    DepthWriteEnable = 1000267007,

    /// [`VkDynamicState::DepthCompareOp`] specifies that the `depth_compare_op` state in
    /// [`VkPipelineDepthStencilStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetDepthCompareOp`] before any draw call.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    DepthCompareOp = 1000267008,

    /// [`VkDynamicState::DepthBoundsTestEnable`] specifies that the `depth_bounds_test_enable`
    /// state in [`VkPipelineDepthStencilStateCreateInfo`] will be ignored and must be set
    /// dynamically with [`VkCmdSetDepthBoundsTestEnable`] before any draw call.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    DepthBoundsTestEnable = 1000267009,

    /// [`VkDynamicState::StencilTestEnable`] specifies that the `stencil_test_enable` state in
    /// [`VkPipelineDepthStencilStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetStencilTestEnable`] before any draw call.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    StencilTestEnable = 1000267010,

    /// [`VkDynamicState::StencilOp`] specifies that the `fail_op`, `pass_op`, `depth_fail_op`, and
    /// `compare_op` states in [`VkPipelineDepthStencilStateCreateInfo`] for both front and back
    /// will be ignored and must be set dynamically with [`VkCmdSetStencilOp`] before any draws are
    /// performed with a pipeline state with [`VkPipelineDepthStencilStateCreateInfo`] member
    /// `stencil_test_enable` set to [`VK_TRUE`]
    ///
    /// Provided by [`VK_VERSION_1_3`]
    StencilOp = 1000267011,

    /// [`VkDynamicState::RasterizerDiscardEnable`] specifies that the `rasterizer_discard_enable`
    /// state in [`VkPipelineRasterizationStateCreateInfo`] will be ignored and must be set
    /// dynamically with [`VkCmdSetRasterizerDiscardEnable`] before any drawing commands.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    RasterizerDiscardEnable = 1000377001,

    /// [`VkDynamicState::DepthBiasEnable`] specifies that the `depth_bias_enable` state in
    /// [`VkPipelineRasterizationStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetDepthBiasEnable`] before any drawing commands.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    DepthBiasEnable = 1000377002,

    /// [`VkDynamicState::PrimitiveRestartEnable`] specifies that the `primitive_restart_enable`
    /// state in [`VkPipelineInputAssemblyStateCreateInfo`] will be ignored and must be set
    /// dynamically with [`VkCmdSetPrimitiveRestartEnable`] before any drawing commands.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    PrimitiveRestartEnable = 1000377004,

    /// [`VkDynamicState::LineStipple`] specifies that the `line_stipple_factor` and
    /// `line_stipple_pattern` state in [`VkPipelineRasterizationLineStateCreateInfo`] will be
    /// ignored and must be set dynamically with [`VkCmdSetLineStipple`] before any draws are
    /// performed with a pipeline state with [`VkPipelineRasterizationLineStateCreateInfo`] member
    /// `stippled_line_enable` set to [`VK_TRUE`].
    ///
    /// Provided by [`VK_VERSION_1_4`]
    LineStipple = 1000259000,

    /// [`VkDynamicState::ViewportWScalingNv`] specifies that the `viewport_w_scalings` state in
    /// [`VkPipelineViewportWScalingStateCreateInfoNv`] will be ignored and must be set dynamically
    /// with [`VkCmdSetViewportWScalingNv`] before any draws are performed with a pipeline state
    /// with [`VkPipelineViewportWScalingStateCreateInfoNv`] member `viewport_scaling_enable` set
    /// to [`VK_TRUE`]
    ///
    /// Provided by [`nv_clip_space_w_scaling`]
    ViewportWScalingNv = 1000087000,

    /// [`VkDynamicState::DiscardRectangleExt`] specifies that the `discard_rectangles` state in
    /// [`VkPipelineDiscardRectangleStateCreateInfoExt`] will be ignored and must be set
    /// dynamically with [`VkCmdSetDiscardRectangleExt`] before any draw or clear commands.
    ///
    /// Provided by [`ext_discard_rectangles`]
    DiscardRectangleExt = 1000099000,

    /// [`VkDynamicState::DiscardRectangleEnableExt`] specifies that the `presence` of the
    /// [`VkPipelineDiscardRectangleStateCreateInfoExt`] structure in the
    /// [`VkGraphicsPipelineCreateInfo`] chain with a `discard_rectangle_count` greater than zero
    /// does not implicitly enable discard rectangles and they must be enabled dynamically with
    /// [`VkCmdSetDiscardRectangleEnableExt`] before any draw commands. This is available on
    /// implementations that support at least `spec_version` 2 of the [`ext_discard_rectangles`]
    /// extension.
    ///
    /// Provided by [`ext_discard_rectangles`]
    DiscardRectangleEnableExt = 1000099001,

    /// [`VkDynamicState::DiscardRectangleModeExt`] specifies that the `discard_rectangle_mode`
    /// state in [`VkPipelineDiscardRectangleStateCreateInfoExt`] will be ignored and must be set
    /// dynamically with [`VkCmdSetDiscardRectangleModeExt`] before any draw commands. This is
    /// available on implementations that support at least `spec_version` 2 of the
    /// [`ext_discard_rectangles`] extension.
    ///
    /// Provided by [`ext_discard_rectangles`]
    DiscardRectangleModeExt = 1000099002,

    /// [`VkDynamicState::SampleLocationsExt`] specifies that the `sample_locations_info` state in
    /// [`VkPipelineSampleLocationsStateCreateInfoExt`] will be ignored and must be set dynamically
    /// with [`VkCmdSetSampleLocationsExt`] before any draw or clear commands. Enabling custom
    /// sample locations is still indicated by the `sample_locations_enable` member of
    /// [`VkPipelineSampleLocationsStateCreateInfoExt`].
    ///
    /// Provided by [`ext_sample_locations`]
    SampleLocationsExt = 1000143000,

    /// [`VkDynamicState::RayTracingPipelineStackSizeKhr`] specifies that the default stack size
    /// computation for the pipeline will be ignored and must be set dynamically with
    /// [`VkCmdSetRayTracingPipelineStackSizeKhr`] before any ray tracing calls are performed.
    ///
    /// Provided by [`khr_ray_tracing_pipeline`]
    RayTracingPipelineStackSizeKhr = 1000347000,

    /// [`VkDynamicState::ViewportShadingRatePaletteNv`] specifies that the
    /// `p_shading_rate_palettes` state in
    /// [`VkPipelineViewportShadingRateImageStateCreateInfoNv`] will be ignored and must be set
    /// dynamically with [`VkCmdSetViewportShadingRatePaletteNv`] before any drawing commands.
    ///
    /// Provided by [`nv_shading_rate_image`]
    ViewportShadingRatePaletteNv = 1000164004,

    /// [`VkDynamicState::ViewportCoarseSampleOrderNv`] specifies that the coarse sample order state
    /// in [`VkPipelineViewportCoarseSampleOrderStateCreateInfoNv`] will be ignored and must be set
    /// dynamically with [`VkCmdSetCoarseSampleOrderNv`] before any drawing commands.
    ///
    /// Provided by [`nv_shading_rate_image`]
    ViewportCoarseSampleOrderNv = 1000164006,

    /// [`VkDynamicState::ExclusiveScissorEnableNv`] specifies that the exclusive scissors must be
    /// explicitly enabled with [`VkCmdSetExclusiveScissorEnableNv`] and the
    /// `exclusive_scissor_count` value in [`VkPipelineViewportExclusiveScissorStateCreateInfoNv`]
    /// will not implicitly enable them. This is available on implementations that support at least
    /// `spec_version` 2 of the [`nv_scissor_exclusive`] extension.
    ///
    /// Provided by [`nv_scissor_exclusive`]
    ExclusiveScissorEnableNv = 1000205000,

    /// [`VkDynamicState::ExclusiveScissorNv`] specifies that the `p_exclusive_scissors` state in
    /// [`VkPipelineViewportExclusiveScissorStateCreateInfoNv`] will be ignored and must be set
    /// dynamically with [`VkCmdSetExclusiveScissorNv`] before any drawing commands.
    ///
    /// Provided by [`nv_scissor_exclusive`]
    ExclusiveScissorNv = 1000205001,

    /// [`VkDynamicState::FragmentShadingRateKhr`] specifies that state in
    /// [`VkPipelineFragmentShadingRateStateCreateInfoKhr`] and
    /// [`VkPipelineFragmentShadingRateEnumStateCreateInfoNv`] will be ignored and must be set
    /// dynamically with [`VkCmdSetFragmentShadingRateKhr`] or
    /// [`VkCmdSetFragmentShadingRateEnumNv`] before any drawing commands.
    ///
    /// Provided by [`khr_fragment_shading_rate`]
    FragmentShadingRateKhr = 1000226000,

    /// [`VkDynamicState::VertexInputExt`] specifies that the `p_vertex_input_state` state will be
    /// ignored and must be set dynamically with [`VkCmdSetVertexInputExt`] before any drawing
    /// commands.
    ///
    /// Provided by [`ext_vertex_input_dynamic_state`]
    VertexInputExt = 1000352000,

    /// [`VkDynamicState::PatchControlPointsExt`] specifies that the `patch_control_points` state in
    /// [`VkPipelineTessellationStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetPatchControlPointsExt`] before any drawing commands.
    ///
    /// Provided by [`ext_extended_dynamic_state2`]
    PatchControlPointsExt = 1000377000,

    /// [`VkDynamicState::LogicOpExt`] specifies that the `logic_op` state in
    /// [`VkPipelineColorBlendStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetLogicOpExt`] before any drawing commands.
    ///
    /// Provided by [`ext_extended_dynamic_state2`]
    LogicOpExt = 1000377003,

    /// [`VkDynamicState::ColorWriteEnableExt`] specifies that the `p_color_write_enables` state in
    /// [`VkPipelineColorWriteCreateInfoExt`] will be ignored and must be set dynamically with
    /// [`VkCmdSetColorWriteEnableExt`] before any draw call.
    ///
    /// Provided by [`ext_color_write_enable`]
    ColorWriteEnableExt = 1000381000,

    /// [`VkDynamicState::DepthClampEnableExt`] specifies that the `depth_clamp_enable` state in
    /// [`VkPipelineRasterizationStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetDepthClampEnableExt`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    DepthClampEnableExt = 1000455003,

    /// [`VkDynamicState::PolygonModeExt`] specifies that the `polygon_mode` state in
    /// [`VkPipelineRasterizationStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetPolygonModeExt`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    PolygonModeExt = 1000455004,

    /// [`VkDynamicState::RasterizationSamplesExt`] specifies that the `rasterization_samples`
    /// state in [`VkPipelineMultisampleStateCreateInfo`] will be ignored and must be set
    /// dynamically with [`VkCmdSetRasterizationSamplesExt`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    RasterizationSamplesExt = 1000455005,

    /// [`VkDynamicState::SampleMaskExt`] specifies that the `p_sample_mask` state in
    /// [`VkPipelineMultisampleStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetSampleMaskExt`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    SampleMaskExt = 1000455006,

    /// [`VkDynamicState::AlphaToCoverageEnableExt`] specifies that the `alpha_to_coverage_enable`
    /// state in [`VkPipelineMultisampleStateCreateInfo`] will be ignored and must be set
    /// dynamically with [`VkCmdSetAlphaToCoverageEnableExt`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    AlphaToCoverageEnableExt = 1000455007,

    /// [`VkDynamicState::AlphaToOneEnableExt`] specifies that the `alpha_to_one_enable` state in
    /// [`VkPipelineMultisampleStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetAlphaToOneEnableExt`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    AlphaToOneEnableExt = 1000455008,

    /// [`VkDynamicState::LogicOpEnableExt`] specifies that the `logic_op_enable` state in
    /// [`VkPipelineColorBlendStateCreateInfo`] will be ignored and must be set dynamically with
    /// [`VkCmdSetLogicOpEnableExt`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    LogicOpEnableExt = 1000455009,

    /// [`VkDynamicState::ColorBlendEnableExt`] specifies that the `blend_enable` state in
    /// [`VkPipelineColorBlendAttachmentState`] will be ignored and must be set dynamically with
    /// [`VkCmdSetColorBlendEnableExt`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    ColorBlendEnableExt = 1000455010,

    /// [`VkDynamicState::ColorBlendEquationExt`] specifies that the `src_color_blend_factor`,
    /// `dst_color_blend_factor`, `color_blend_op`, `src_alpha_blend_factor`,
    /// `dst_alpha_blend_factor`, and `alpha_blend_op` states in
    /// [`VkPipelineColorBlendAttachmentState`] will be ignored and must be set dynamically with
    /// [`VkCmdSetColorBlendEquationExt`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    ColorBlendEquationExt = 1000455011,

    /// [`VkDynamicState::ColorWriteMaskExt`] specifies that the `color_write_mask` state in
    /// [`VkPipelineColorBlendAttachmentState`] will be ignored and must be set dynamically with
    /// [`VkCmdSetColorWriteMaskExt`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`]
    ColorWriteMaskExt = 1000455012,

    /// [`VkDynamicState::TessellationDomainOriginExt`] specifies that the `domain_origin` state in
    /// [`VkPipelineTessellationDomainOriginStateCreateInfo`] will be ignored and must be set
    /// dynamically with [`VkCmdSetTessellationDomainOriginExt`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with [`khr_maintenance2`] or
    /// [`VK_VERSION_1_1`]
    TessellationDomainOriginExt = 1000455002,

    /// [`VkDynamicState::RasterizationStreamExt`] specifies that the `rasterization_stream` state
    /// in [`VkPipelineRasterizationStateStreamCreateInfoExt`] will be ignored and must be set
    /// dynamically with [`VkCmdSetRasterizationStreamExt`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with [`ext_transform_feedback`]
    RasterizationStreamExt = 1000455013,

    /// [`VkDynamicState::ConservativeRasterizationModeExt`] specifies that the
    /// `conservative_rasterization_mode` state in
    /// [`VkPipelineRasterizationConservativeStateCreateInfoExt`] will be ignored and must be set
    /// dynamically with [`VkCmdSetConservativeRasterizationModeExt`] before any draw call.
    ///
    /// Provided by [`ext_conservative_rasterization`] with [`ext_extended_dynamic_state3`]
    ConservativeRasterizationModeExt = 1000455014,

    /// [`VkDynamicState::ExtraPrimitiveOverestimationSizeExt`] specifies that the
    /// `extra_primitive_overestimation_size` state in
    /// [`VkPipelineRasterizationConservativeStateCreateInfoExt`] will be ignored and must be set
    /// dynamically with [`VkCmdSetExtraPrimitiveOverestimationSizeExt`] before any draw call.
    ///
    /// Provided by [`ext_conservative_rasterization`] with [`ext_extended_dynamic_state3`]
    ExtraPrimitiveOverestimationSizeExt = 1000455015,

    /// [`VkDynamicState::DepthClipEnableExt`] specifies that the `depth_clip_enable` state in
    /// [`VkPipelineRasterizationDepthClipStateCreateInfoExt`] will be ignored and must be set
    /// dynamically with [`VkCmdSetDepthClipEnableExt`] before any draw call.
    ///
    /// Provided by [`ext_depth_clip_enable`] with [`ext_extended_dynamic_state3`]
    DepthClipEnableExt = 1000455016,

    /// [`VkDynamicState::SampleLocationsEnableExt`] specifies that the `sample_locations_enable`
    /// state in [`VkPipelineSampleLocationsStateCreateInfoExt`] will be ignored and must be set
    /// dynamically with [`VkCmdSetSampleLocationsEnableExt`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with [`ext_sample_locations`]
    SampleLocationsEnableExt = 1000455017,

    /// [`VkDynamicState::ColorBlendAdvancedExt`] specifies that the `color_blend_op` state in
    /// [`VkPipelineColorBlendAttachmentState`], and `src_premultiplied`, `dst_premultiplied`, and
    /// `blend_overlap` states in [`VkPipelineColorBlendAdvancedStateCreateInfoExt`] will be
    /// ignored and must be set dynamically with [`VkCmdSetColorBlendAdvancedExt`] before any draw
    /// call.
    ///
    /// Provided by [`ext_blend_operation_advanced`] with [`ext_extended_dynamic_state3`]
    ColorBlendAdvancedExt = 1000455018,

    /// [`VkDynamicState::ProvokingVertexModeExt`] specifies that the `provoking_vertex_mode` state
    /// in [`VkPipelineRasterizationProvokingVertexStateCreateInfoExt`] will be ignored and must be
    /// set dynamically with [`VkCmdSetProvokingVertexModeExt`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with [`ext_provoking_vertex`]
    ProvokingVertexModeExt = 1000455019,

    /// [`VkDynamicState::LineRasterizationModeExt`] specifies that the `line_rasterization_mode`
    /// state in [`VkPipelineRasterizationLineStateCreateInfo`] will be ignored and must be set
    /// dynamically with [`VkCmdSetLineRasterizationModeExt`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with [`ext_line_rasterization`]
    LineRasterizationModeExt = 1000455020,

    /// [`VkDynamicState::LineStippleEnableExt`] specifies that the `stippled_line_enable` state in
    /// [`VkPipelineRasterizationLineStateCreateInfo`] will be ignored and must be set dynamically
    /// with [`VkCmdSetLineStippleEnableExt`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with [`ext_line_rasterization`]
    LineStippleEnableExt = 1000455021,

    /// [`VkDynamicState::DepthClipNegativeOneToOneExt`] specifies that the `negative_one_to_one`
    /// state in [`VkPipelineViewportDepthClipControlCreateInfoExt`] will be ignored and must be
    /// set dynamically with [`VkCmdSetDepthClipNegativeOneToOneExt`] before any draw call.
    ///
    /// Provided by [`ext_depth_clip_control`] with [`ext_extended_dynamic_state3`]
    DepthClipNegativeOneToOneExt = 1000455022,

    /// [`VkDynamicState::ViewportWScalingEnableNv`] specifies that the `viewport_w_scaling_enable`
    /// state in [`VkPipelineViewportWScalingStateCreateInfoNv`] will be ignored and must be set
    /// dynamically with [`VkCmdSetViewportWScalingEnableNv`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with [`nv_clip_space_w_scaling`]
    ViewportWScalingEnableNv = 1000455023,

    /// [`VkDynamicState::ViewportSwizzleNv`] specifies that the `viewport_count`, and
    /// `p_viewport_swizzles` states in [`VkPipelineViewportSwizzleStateCreateInfoNv`] will be
    /// ignored and must be set dynamically with [`VkCmdSetViewportSwizzleNv`] before any draw
    /// call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with [`nv_viewport_swizzle`]
    ViewportSwizzleNv = 1000455024,

    /// [`VkDynamicState::CoverageToColorEnableNv`] specifies that the `coverage_to_color_enable`
    /// state in [`VkPipelineCoverageToColorStateCreateInfoNv`] will be ignored and must be set
    /// dynamically with [`VkCmdSetCoverageToColorEnableNv`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with
    /// [`nv_fragment_coverage_to_color`]
    CoverageToColorEnableNv = 1000455025,

    /// [`VkDynamicState::CoverageToColorLocationNv`] specifies that the
    /// `coverage_to_color_location` state in [`VkPipelineCoverageToColorStateCreateInfoNv`] will
    /// be ignored and must be set dynamically with [`VkCmdSetCoverageToColorLocationNv`] before
    /// any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with
    /// [`nv_fragment_coverage_to_color`]
    CoverageToColorLocationNv = 1000455026,

    /// [`VkDynamicState::CoverageModulationModeNv`] specifies that the `coverage_modulation_mode`
    /// state in [`VkPipelineCoverageModulationStateCreateInfoNv`] will be ignored and must be set
    /// dynamically with [`VkCmdSetCoverageModulationModeNv`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with [`nv_framebuffer_mixed_samples`]
    CoverageModulationModeNv = 1000455027,

    /// [`VkDynamicState::CoverageModulationTableEnableNv`] specifies that the
    /// `coverage_modulation_table_enable` state in
    /// [`VkPipelineCoverageModulationStateCreateInfoNv`] will be ignored and must be set
    /// dynamically with [`VkCmdSetCoverageModulationTableEnableNv`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with [`nv_framebuffer_mixed_samples`]
    CoverageModulationTableEnableNv = 1000455028,

    /// [`VkDynamicState::CoverageModulationTableNv`] specifies that the
    /// `coverage_modulation_table_count`, and `p_coverage_modulation_table` states in
    /// [`VkPipelineCoverageModulationStateCreateInfoNv`] will be ignored and must be set
    /// dynamically with [`VkCmdSetCoverageModulationTableNv`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with [`nv_framebuffer_mixed_samples`]
    CoverageModulationTableNv = 1000455029,

    /// [`VkDynamicState::ShadingRateImageEnableNv`] specifies that the `shading_rate_image_enable`
    /// state in [`VkPipelineViewportShadingRateImageStateCreateInfoNv`] will be ignored and must
    /// be set dynamically with [`VkCmdSetShadingRateImageEnableNv`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with [`nv_shading_rate_image`]
    ShadingRateImageEnableNv = 1000455030,

    /// [`VkDynamicState::RepresentativeFragmentTestEnableNv`] specifies that the
    /// `representative_fragment_test_enable` state in
    /// [`VkPipelineRepresentativeFragmentTestStateCreateInfoNv`] will be ignored and must be set
    /// dynamically with [`VkCmdSetRepresentativeFragmentTestEnableNv`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with [`nv_representative_fragment_test`]
    RepresentativeFragmentTestEnableNv = 1000455031,

    /// [`VkDynamicState::CoverageReductionModeNv`] specifies that the `coverage_reduction_mode`
    /// state in [`VkPipelineCoverageReductionStateCreateInfoNv`] will be ignored and must be set
    /// dynamically with [`VkCmdSetCoverageReductionModeNv`] before any draw call.
    ///
    /// Provided by [`ext_extended_dynamic_state3`] with [`nv_coverage_reduction_mode`]
    CoverageReductionModeNv = 1000455032,

    /// [`VkDynamicState::AttachmentFeedbackLoopEnableExt`] specifies that the
    /// [`VkPipelineCreateFlag::ColorAttachmentFeedbackLoopExt`] and
    /// [`VkPipelineCreateFlag::DepthStencilAttachmentFeedbackLoopExt`] flags will be ignored
    /// and must be set dynamically with [`VkCmdSetAttachmentFeedbackLoopEnableExt`] before any
    /// draw call.
    ///
    /// Provided by [`ext_attachment_feedback_loop_dynamic_state`]
    AttachmentFeedbackLoopEnableExt = 1000524000,

    /// [`VkDynamicState::DepthClampRangeExt`] specifies that the `depth_clamp_mode` and
    /// `p_depth_clamp_range` state in [`VkPipelineViewportDepthClampControlCreateInfoExt`] will be
    /// ignored and must be set dynamically with [`VkCmdSetDepthClampRangeExt`] before any draw
    /// call.
    ///
    /// Provided by [`ext_depth_clamp_control`]
    DepthClampRangeExt = 1000582000,
}
