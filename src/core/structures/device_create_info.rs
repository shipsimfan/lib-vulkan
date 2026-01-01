use crate::{
    VkDeviceCreateFlags, VkDeviceQueueCreateInfo, VkPhysicalDeviceFeatures, VkStructureType,
};
use std::{
    ffi::{c_char, c_void},
    ptr::null,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_FALSE, VK_TRUE, VK_VERSION_1_0, VkEnumerateDeviceExtensionProperties,
    VkGetPhysicalDeviceQueueFamilyProperties, VkPhysicalDeviceProperties, VkQueueFamilyProperties,
    VkQueueFlag,
};

/// Structure specifying parameters of a newly created device
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::DeviceCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If the `next` chain includes a [`VkPhysicalDeviceVulkan11Features`] structure, then it
    ///    must not include a [`VkPhysicalDevice16BitStorageFeatures`],
    ///    [`VkPhysicalDeviceMultiviewFeatures`], [`VkPhysicalDeviceVariablePointersFeatures`],
    ///    [`VkPhysicalDeviceProtectedMemoryFeatures`],
    ///    [`VkPhysicalDeviceSamplerYcbcrConversionFeatures`], or
    ///    [`VkPhysicalDeviceShaderDrawParametersFeatures`] structure
    ///  - If the `next` chain includes a [`VkPhysicalDeviceVulkan12Features`] structure, then it
    ///    must not include a [`VkPhysicalDevice8BitStorageFeatures`],
    ///    [`VkPhysicalDeviceShaderAtomicInt64Features`],
    ///    [`VkPhysicalDeviceShaderFloat16Int8Features`],
    ///    [`VkPhysicalDeviceDescriptorIndexingFeatures`],
    ///    [`VkPhysicalDeviceScalarBlockLayoutFeatures`],
    ///    [`VkPhysicalDeviceImagelessFramebufferFeatures`],
    ///    [`VkPhysicalDeviceUniformBufferStandardLayoutFeatures`],
    ///    [`VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures`],
    ///    [`VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures`],
    ///    [`VkPhysicalDeviceHostQueryResetFeatures`],
    ///    [`VkPhysicalDeviceTimelineSemaphoreFeatures`],
    ///    [`VkPhysicalDeviceBufferDeviceAddressFeatures`], or
    ///    [`VkPhysicalDeviceVulkanMemoryModelFeatures`] structure
    ///  - If `enabled_extension_names` contains "VK_Khr_shader_draw_parameters" and the `next`
    ///    chain includes a [`VkPhysicalDeviceVulkan11Features`] structure, then
    ///    [`VkPhysicalDeviceVulkan11Features::shader_draw_parameters`] must be [`VK_TRUE`]
    ///  - If `enabled_extension_names` contains "VK_Khr_draw_indirect_count" and the `next` chain
    ///    includes a [`VkPhysicalDeviceVulkan12Features`] structure, then
    ///    [`VkPhysicalDeviceVulkan12Features::draw_indirect_count`] must be [`VK_TRUE`]
    ///  - If `enabled_extension_names` contains "VK_Khr_sampler_mirror_clamp_to_edge" and the
    ///    `next` chain includes a [`VkPhysicalDeviceVulkan12Features`] structure, then
    ///    [`VkPhysicalDeviceVulkan12Features::sampler_mirror_clamp_to_edge`] must be [`VK_TRUE`]
    ///  - If `enabled_extension_names` contains "VK_Ext_descriptor_indexing" and the `next` chain
    ///    includes a [`VkPhysicalDeviceVulkan12Features`] structure, then
    ///    [`VkPhysicalDeviceVulkan12Features::descriptor_indexing`] must be [`VK_TRUE`]
    ///  - If `enabled_extension_names` contains "VK_Ext_sampler_filter_minmax" and the `next`
    ///    chain includes a [`VkPhysicalDeviceVulkan12Features`] structure, then
    ///    [`VkPhysicalDeviceVulkan12Features::sampler_filter_minmax`] must be [`VK_TRUE`]
    ///  - If `enabled_extension_names` contains "VK_Ext_shader_viewport_index_layer" and the
    ///    `next` chain includes a [`VkPhysicalDeviceVulkan12Features`] structure, then
    ///    [`VkPhysicalDeviceVulkan12Features::shader_output_viewport_index`] and
    ///    [`VkPhysicalDeviceVulkan12Features::shader_output_layer`] must both be [`VK_TRUE`]
    ///  - If the `next` chain includes a [`VkPhysicalDeviceVulkan13Features`] structure, then it
    ///    must not include a [`VkPhysicalDeviceDynamicRenderingFeatures`],
    ///    [`VkPhysicalDeviceImageRobustnessFeatures`],
    ///    [`VkPhysicalDeviceInlineUniformBlockFeatures`], [`VkPhysicalDeviceMaintenance4Features`],
    ///    [`VkPhysicalDevicePipelineCreationCacheControlFeatures`],
    ///    [`VkPhysicalDevicePrivateDataFeatures`],
    ///    [`VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures`],
    ///    [`VkPhysicalDeviceShaderIntegerDotProductFeatures`],
    ///    [`VkPhysicalDeviceShaderTerminateInvocationFeatures`],
    ///    [`VkPhysicalDeviceSubgroupSizeControlFeatures`],
    ///    [`VkPhysicalDeviceSynchronization2Features`],
    ///    [`VkPhysicalDeviceTextureCompressionASTCHDRFeatures`], or
    ///    [`VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures`] structure
    ///  - If the `next` chain includes a [`VkPhysicalDeviceVulkan14Features`] structure, then it
    ///    must not include a [`VkPhysicalDeviceGlobalPriorityQueryFeatures`],
    ///    [`VkPhysicalDeviceShaderSubgroupRotateFeatures`],
    ///    [`VkPhysicalDeviceShaderFloatControls2Features`],
    ///    [`VkPhysicalDeviceShaderExpectAssumeFeatures`],
    ///    [`VkPhysicalDeviceLineRasterizationFeatures`],
    ///    [`VkPhysicalDeviceVertexAttributeDivisorFeatures`],
    ///    [`VkPhysicalDeviceIndexTypeUint8Features`],
    ///    [`VkPhysicalDeviceDynamicRenderingLocalReadFeatures`],
    ///    [`VkPhysicalDeviceMaintenance5Features`], [`VkPhysicalDeviceMaintenance6Features`],
    ///    [`VkPhysicalDevicePipelineProtectedAccessFeatures`],
    ///    [`VkPhysicalDevicePipelineRobustnessFeatures`], or
    ///    [`VkPhysicalDeviceHostImageCopyFeatures`] structure
    ///  - If `enabled_extension_names` contains "VK_Khr_push_descriptor" and the `next` chain
    ///    includes a [`VkPhysicalDeviceVulkan14Features`] structure, then
    ///    [`VkPhysicalDeviceVulkan14Features::push_descriptor`] must be [`VK_TRUE`]
    ///  - If the "VK_Khr_portability_subset" extension is included in `properties` of
    ///    [`VkEnumerateDeviceExtensionProperties`], `enabled_extension_names` must include
    ///    "VK_Khr_portability_subset"
    ///  - If the `shading_rate_image` feature is enabled, the `pipeline_fragment_shading_rate`
    ///    feature must not be enabled
    ///  - If the `shading_rate_image` feature is enabled, the `primitive_fragment_shading_rate`
    ///    feature must not be enabled
    ///  - If the `shading_rate_image` feature is enabled, the `attachment_fragment_shading_rate`
    ///    feature must not be enabled
    ///  - If the `fragment_density_map` feature is enabled, the `pipeline_fragment_shading_rate`
    ///    feature must not be enabled
    ///  - If the `fragment_density_map` feature is enabled, the `primitive_fragment_shading_rate`
    ///    feature must not be enabled
    ///  - If the `fragment_density_map` feature is enabled, the `attachment_fragment_shading_rate`
    ///    feature must not be enabled
    ///  - If the `sparse_image_int64_atomics` feature is enabled, `shader_image_int64_atomics`
    ///    must be enabled
    ///  - If the `sparse_image_float32_atomics` feature is enabled, `shader_image_float32_atomics`
    ///    must be enabled
    ///  - If the `sparse_image_float32_atomic_add` feature is enabled,
    ///    `shader_image_float32_atomic_add` must be enabled
    ///  - If the `sparse_image_float32_atomic_min_max` feature is enabled,
    ///    `shader_image_float32_atomic_min_max` must be enabled
    ///  - If the `robust_buffer_access` feature is enabled, and
    ///    `robust_buffer_access_update_after_bind` is [`VK_FALSE`], then
    ///    `descriptor_binding_uniform_buffer_update_after_bind`,
    ///    `descriptor_binding_storage_buffer_update_after_bind`,
    ///    `descriptor_binding_uniform_texel_buffer_update_after_bind`, and
    ///    `descriptor_binding_storage_texel_buffer_update_after_bind` must not be enabled
    ///  - If the `descriptor_buffer` feature is enabled, `enabled_extension_names` must not
    ///    contain "VK_Amd_shader_fragment_mask"
    ///  - If the `next` chain includes a [`VkDeviceQueueShaderCoreControlCreateInfoArm`]
    ///    structure, then it must not be included in the `next` chain of any of the
    ///    [`VkDeviceQueueCreateInfo`] structures in `queue_create_infos`
    ///  - If the `next` chain includes a [`VkDeviceQueueShaderCoreControlCreateInfoArm`] structure
    ///    then [`VkPhysicalDeviceSchedulingControlsPropertiesArm::scheduling_controls_flags`] must
    ///    contain [`VkPhysicalDeviceSchedulingControlsFlag::ShaderCoreCountBitArm`]
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of
    ///    [`VkDeviceDeviceMemoryReportCreateInfoExt`], [`VkDeviceDiagnosticsConfigCreateInfoNv`],
    ///    [`VkDeviceGroupDeviceCreateInfo`], [`VkDeviceMemoryOverallocationCreateInfoAmd`],
    ///    [`VkDevicePipelineBinaryInternalCacheControlKhr`], [`VkDevicePrivateDataCreateInfo`],
    ///    [`VkDeviceQueueShaderCoreControlCreateInfoArm`],
    ///    [`VkExternalComputeQueueDeviceCreateInfoNv`], [`VkPhysicalDevice16BitStorageFeatures`],
    ///    [`VkPhysicalDevice4444FormatsFeaturesExt`], [`VkPhysicalDevice8BitStorageFeatures`],
    ///    [`VkPhysicalDeviceASTCDecodeFeaturesExt`],
    ///    [`VkPhysicalDeviceAccelerationStructureFeaturesKhr`],
    ///    [`VkPhysicalDeviceAddressBindingReportFeaturesExt`],
    ///    [`VkPhysicalDeviceAmigoProfilingFeaturesSec`], [`VkPhysicalDeviceAntiLagFeaturesAmd`],
    ///    [`VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesExt`],
    ///    [`VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesExt`],
    ///    [`VkPhysicalDeviceBlendOperationAdvancedFeaturesExt`],
    ///    [`VkPhysicalDeviceBorderColorSwizzleFeaturesExt`],
    ///    [`VkPhysicalDeviceBufferDeviceAddressFeatures`],
    ///    [`VkPhysicalDeviceBufferDeviceAddressFeaturesExt`],
    ///    [`VkPhysicalDeviceClusterAccelerationStructureFeaturesNv`],
    ///    [`VkPhysicalDeviceClusterCullingShaderFeaturesHuawei`],
    ///    [`VkPhysicalDeviceCoherentMemoryFeaturesAmd`],
    ///    [`VkPhysicalDeviceColorWriteEnableFeaturesExt`],
    ///    [`VkPhysicalDeviceCommandBufferInheritanceFeaturesNv`],
    ///    [`VkPhysicalDeviceComputeOccupancyPriorityFeaturesNv`],
    ///    [`VkPhysicalDeviceComputeShaderDerivativesFeaturesKhr`],
    ///    [`VkPhysicalDeviceConditionalRenderingFeaturesExt`],
    ///    [`VkPhysicalDeviceCooperativeMatrix2FeaturesNv`],
    ///    [`VkPhysicalDeviceCooperativeMatrixFeaturesKhr`],
    ///    [`VkPhysicalDeviceCooperativeMatrixFeaturesNv`],
    ///    [`VkPhysicalDeviceCooperativeVectorFeaturesNv`],
    ///    [`VkPhysicalDeviceCopyMemoryIndirectFeaturesKhr`],
    ///    [`VkPhysicalDeviceCopyMemoryIndirectFeaturesNv`],
    ///    [`VkPhysicalDeviceCornerSampledImageFeaturesNv`],
    ///    [`VkPhysicalDeviceCoverageReductionModeFeaturesNv`],
    ///    [`VkPhysicalDeviceCubicClampFeaturesQcom`],
    ///    [`VkPhysicalDeviceCubicWeightsFeaturesQcom`],
    ///    [`VkPhysicalDeviceCudaKernelLaunchFeaturesNv`],
    ///    [`VkPhysicalDeviceCustomBorderColorFeaturesExt`],
    ///    [`VkPhysicalDeviceCustomResolveFeaturesExt`], [`VkPhysicalDeviceDataGraphFeaturesArm`],
    ///    [`VkPhysicalDeviceDataGraphModelFeaturesQcom`],
    ///    [`VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNv`],
    ///    [`VkPhysicalDeviceDenseGeometryFormatFeaturesAmdX`],
    ///    [`VkPhysicalDeviceDepthBiasControlFeaturesExt`],
    ///    [`VkPhysicalDeviceDepthClampControlFeaturesExt`],
    ///    [`VkPhysicalDeviceDepthClampZeroOneFeaturesKhr`],
    ///    [`VkPhysicalDeviceDepthClipControlFeaturesExt`],
    ///    [`VkPhysicalDeviceDepthClipEnableFeaturesExt`],
    ///    [`VkPhysicalDeviceDescriptorBufferFeaturesExt`],
    ///    [`VkPhysicalDeviceDescriptorBufferTensorFeaturesArm`],
    ///    [`VkPhysicalDeviceDescriptorIndexingFeatures`],
    ///    [`VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNv`],
    ///    [`VkPhysicalDeviceDescriptorSetHostMappingFeaturesValve`],
    ///    [`VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNv`],
    ///    [`VkPhysicalDeviceDeviceGeneratedCommandsFeaturesExt`],
    ///    [`VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNv`],
    ///    [`VkPhysicalDeviceDeviceMemoryReportFeaturesExt`],
    ///    [`VkPhysicalDeviceDiagnosticsConfigFeaturesNv`],
    ///    [`VkPhysicalDeviceDisplacementMicromapFeaturesNv`],
    ///    [`VkPhysicalDeviceDynamicRenderingFeatures`],
    ///    [`VkPhysicalDeviceDynamicRenderingLocalReadFeatures`],
    ///    [`VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesExt`],
    ///    [`VkPhysicalDeviceExclusiveScissorFeaturesNv`],
    ///    [`VkPhysicalDeviceExtendedDynamicState2FeaturesExt`],
    ///    [`VkPhysicalDeviceExtendedDynamicState3FeaturesExt`],
    ///    [`VkPhysicalDeviceExtendedDynamicStateFeaturesExt`],
    ///    [`VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNv`],
    ///    [`VkPhysicalDeviceExternalFormatResolveFeaturesAndroid`],
    ///    [`VkPhysicalDeviceExternalMemoryRDMAFeaturesNv`],
    ///    [`VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQnx`],
    ///    [`VkPhysicalDeviceFaultFeaturesExt`], [`VkPhysicalDeviceFeatures2`],
    ///    [`VkPhysicalDeviceFormatPackFeaturesArm`],
    ///    [`VkPhysicalDeviceFragmentDensityMap2FeaturesExt`],
    ///    [`VkPhysicalDeviceFragmentDensityMapFeaturesExt`],
    ///    [`VkPhysicalDeviceFragmentDensityMapLayeredFeaturesValve`],
    ///    [`VkPhysicalDeviceFragmentDensityMapOffsetFeaturesExt`],
    ///    [`VkPhysicalDeviceFragmentShaderBarycentricFeaturesKhr`],
    ///    [`VkPhysicalDeviceFragmentShaderInterlockFeaturesExt`],
    ///    [`VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNv`],
    ///    [`VkPhysicalDeviceFragmentShadingRateFeaturesKhr`],
    ///    [`VkPhysicalDeviceFrameBoundaryFeaturesExt`],
    ///    [`VkPhysicalDeviceGlobalPriorityQueryFeatures`],
    ///    [`VkPhysicalDeviceGraphicsPipelineLibraryFeaturesExt`],
    ///    [`VkPhysicalDeviceHdrVividFeaturesHuawei`], [`VkPhysicalDeviceHostImageCopyFeatures`],
    ///    [`VkPhysicalDeviceHostQueryResetFeatures`],
    ///    [`VkPhysicalDeviceImage2DViewOf3DFeaturesExt`],
    ///    [`VkPhysicalDeviceImageAlignmentControlFeaturesMesa`],
    ///    [`VkPhysicalDeviceImageCompressionControlFeaturesExt`],
    ///    [`VkPhysicalDeviceImageCompressionControlSwapchainFeaturesExt`],
    ///    [`VkPhysicalDeviceImageProcessing2FeaturesQcom`],
    ///    [`VkPhysicalDeviceImageProcessingFeaturesQcom`],
    ///    [`VkPhysicalDeviceImageRobustnessFeatures`],
    ///    [`VkPhysicalDeviceImageSlicedViewOf3DFeaturesExt`],
    ///    [`VkPhysicalDeviceImageViewMinLodFeaturesExt`],
    ///    [`VkPhysicalDeviceImagelessFramebufferFeatures`],
    ///    [`VkPhysicalDeviceIndexTypeUint8Features`],
    ///    [`VkPhysicalDeviceInheritedViewportScissorFeaturesNv`],
    ///    [`VkPhysicalDeviceInlineUniformBlockFeatures`],
    ///    [`VkPhysicalDeviceInvocationMaskFeaturesHuawei`],
    ///    [`VkPhysicalDeviceLegacyDitheringFeaturesExt`],
    ///    [`VkPhysicalDeviceLegacyVertexAttributesFeaturesExt`],
    ///    [`VkPhysicalDeviceLineRasterizationFeatures`],
    ///    [`VkPhysicalDeviceLinearColorAttachmentFeaturesNv`],
    ///    [`VkPhysicalDeviceMaintenance10FeaturesKhr`], [`VkPhysicalDeviceMaintenance4Features`],
    ///    [`VkPhysicalDeviceMaintenance5Features`], [`VkPhysicalDeviceMaintenance6Features`],
    ///    [`VkPhysicalDeviceMaintenance7FeaturesKhr`],
    ///    [`VkPhysicalDeviceMaintenance8FeaturesKhr`],
    ///    [`VkPhysicalDeviceMaintenance9FeaturesKhr`],
    ///    [`VkPhysicalDeviceMapMemoryPlacedFeaturesExt`],
    ///    [`VkPhysicalDeviceMemoryDecompressionFeaturesExt`],
    ///    [`VkPhysicalDeviceMemoryPriorityFeaturesExt`],
    ///    [`VkPhysicalDeviceMeshShaderFeaturesExt`], [`VkPhysicalDeviceMeshShaderFeaturesNv`],
    ///    [`VkPhysicalDeviceMultiDrawFeaturesExt`],
    ///    [`VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesExt`],
    ///    [`VkPhysicalDeviceMultiviewFeatures`],
    ///    [`VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQcom`],
    ///    [`VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQcom`],
    ///    [`VkPhysicalDeviceMutableDescriptorTypeFeaturesExt`],
    ///    [`VkPhysicalDeviceNestedCommandBufferFeaturesExt`],
    ///    [`VkPhysicalDeviceNonSeamlessCubeMapFeaturesExt`],
    ///    [`VkPhysicalDeviceOpacityMicromapFeaturesExt`],
    ///    [`VkPhysicalDeviceOpticalFlowFeaturesNv`],
    ///    [`VkPhysicalDevicePageableDeviceLocalMemoryFeaturesExt`],
    ///    [`VkPhysicalDevicePartitionedAccelerationStructureFeaturesNv`],
    ///    [`VkPhysicalDevicePerStageDescriptorSetFeaturesNv`],
    ///    [`VkPhysicalDevicePerformanceCountersByRegionFeaturesArm`],
    ///    [`VkPhysicalDevicePerformanceQueryFeaturesKhr`],
    ///    [`VkPhysicalDevicePipelineBinaryFeaturesKhr`],
    ///    [`VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSec`],
    ///    [`VkPhysicalDevicePipelineCreationCacheControlFeatures`],
    ///    [`VkPhysicalDevicePipelineExecutablePropertiesFeaturesKhr`],
    ///    [`VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesExt`],
    ///    [`VkPhysicalDevicePipelineOpacityMicromapFeaturesArm`],
    ///    [`VkPhysicalDevicePipelinePropertiesFeaturesExt`],
    ///    [`VkPhysicalDevicePipelineProtectedAccessFeatures`],
    ///    [`VkPhysicalDevicePipelineRobustnessFeatures`],
    ///    [`VkPhysicalDevicePortabilitySubsetFeaturesKhr`],
    ///    [`VkPhysicalDevicePresentBarrierFeaturesNv`], [`VkPhysicalDevicePresentId2FeaturesKhr`],
    ///    [`VkPhysicalDevicePresentIdFeaturesKhr`], [`VkPhysicalDevicePresentMeteringFeaturesNv`],
    ///    [`VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKhr`],
    ///    [`VkPhysicalDevicePresentTimingFeaturesExt`],
    ///    [`VkPhysicalDevicePresentWait2FeaturesKhr`], [`VkPhysicalDevicePresentWaitFeaturesKhr`],
    ///    [`VkPhysicalDevicePrimitiveTopologyListRestartFeaturesExt`],
    ///    [`VkPhysicalDevicePrimitivesGeneratedQueryFeaturesExt`],
    ///    [`VkPhysicalDevicePrivateDataFeatures`], [`VkPhysicalDeviceProtectedMemoryFeatures`],
    ///    [`VkPhysicalDeviceProvokingVertexFeaturesExt`],
    ///    [`VkPhysicalDeviceRGBA10X6FormatsFeaturesExt`],
    ///    [`VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesExt`],
    ///    [`VkPhysicalDeviceRawAccessChainsFeaturesNv`], [`VkPhysicalDeviceRayQueryFeaturesKhr`],
    ///    [`VkPhysicalDeviceRayTracingInvocationReorderFeaturesExt`],
    ///    [`VkPhysicalDeviceRayTracingInvocationReorderFeaturesNv`],
    ///    [`VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNv`],
    ///    [`VkPhysicalDeviceRayTracingMaintenance1FeaturesKhr`],
    ///    [`VkPhysicalDeviceRayTracingMotionBlurFeaturesNv`],
    ///    [`VkPhysicalDeviceRayTracingPipelineFeaturesKhr`],
    ///    [`VkPhysicalDeviceRayTracingPositionFetchFeaturesKhr`],
    ///    [`VkPhysicalDeviceRayTracingValidationFeaturesNv`],
    ///    [`VkPhysicalDeviceRelaxedLineRasterizationFeaturesImg`],
    ///    [`VkPhysicalDeviceRenderPassStripedFeaturesArm`],
    ///    [`VkPhysicalDeviceRepresentativeFragmentTestFeaturesNv`],
    ///    [`VkPhysicalDeviceRobustness2FeaturesKhr`],
    ///    [`VkPhysicalDeviceSamplerYcbcrConversionFeatures`],
    ///    [`VkPhysicalDeviceScalarBlockLayoutFeatures`],
    ///    [`VkPhysicalDeviceSchedulingControlsFeaturesArm`],
    ///    [`VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures`],
    ///    [`VkPhysicalDeviceShader64BitIndexingFeaturesExt`],
    ///    [`VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNv`],
    ///    [`VkPhysicalDeviceShaderAtomicFloat2FeaturesExt`],
    ///    [`VkPhysicalDeviceShaderAtomicFloatFeaturesExt`],
    ///    [`VkPhysicalDeviceShaderAtomicInt64Features`],
    ///    [`VkPhysicalDeviceShaderBfloat16FeaturesKhr`],
    ///    [`VkPhysicalDeviceShaderClockFeaturesKhr`],
    ///    [`VkPhysicalDeviceShaderCoreBuiltinsFeaturesArm`],
    ///    [`VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures`],
    ///    [`VkPhysicalDeviceShaderDrawParametersFeatures`],
    ///    [`VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAmd`],
    ///    [`VkPhysicalDeviceShaderEnqueueFeaturesAmdX`],
    ///    [`VkPhysicalDeviceShaderExpectAssumeFeatures`],
    ///    [`VkPhysicalDeviceShaderFloat16Int8Features`],
    ///    [`VkPhysicalDeviceShaderFloat8FeaturesExt`],
    ///    [`VkPhysicalDeviceShaderFloatControls2Features`],
    ///    [`VkPhysicalDeviceShaderFmaFeaturesKhr`],
    ///    [`VkPhysicalDeviceShaderImageAtomicInt64FeaturesExt`],
    ///    [`VkPhysicalDeviceShaderImageFootprintFeaturesNv`],
    ///    [`VkPhysicalDeviceShaderIntegerDotProductFeatures`],
    ///    [`VkPhysicalDeviceShaderIntegerFunctions2FeaturesIntel`],
    ///    [`VkPhysicalDeviceShaderLongVectorFeaturesExt`],
    ///    [`VkPhysicalDeviceShaderMaximalReconvergenceFeaturesKhr`],
    ///    [`VkPhysicalDeviceShaderModuleIdentifierFeaturesExt`],
    ///    [`VkPhysicalDeviceShaderObjectFeaturesExt`],
    ///    [`VkPhysicalDeviceShaderQuadControlFeaturesKhr`],
    ///    [`VkPhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKhr`],
    ///    [`VkPhysicalDeviceShaderReplicatedCompositesFeaturesExt`],
    ///    [`VkPhysicalDeviceShaderSMBuiltinsFeaturesNv`],
    ///    [`VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures`],
    ///    [`VkPhysicalDeviceShaderSubgroupRotateFeatures`],
    ///    [`VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKhr`],
    ///    [`VkPhysicalDeviceShaderTerminateInvocationFeatures`],
    ///    [`VkPhysicalDeviceShaderTileImageFeaturesExt`],
    ///    [`VkPhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesExt`],
    ///    [`VkPhysicalDeviceShaderUntypedPointersFeaturesKhr`],
    ///    [`VkPhysicalDeviceShadingRateImageFeaturesNv`],
    ///    [`VkPhysicalDeviceSubgroupSizeControlFeatures`],
    ///    [`VkPhysicalDeviceSubpassMergeFeedbackFeaturesExt`],
    ///    [`VkPhysicalDeviceSubpassShadingFeaturesHuawei`],
    ///    [`VkPhysicalDeviceSwapchainMaintenance1FeaturesKhr`],
    ///    [`VkPhysicalDeviceSynchronization2Features`], [`VkPhysicalDeviceTensorFeaturesArm`],
    ///    [`VkPhysicalDeviceTexelBufferAlignmentFeaturesExt`],
    ///    [`VkPhysicalDeviceTextureCompressionASTC3DFeaturesExt`],
    ///    [`VkPhysicalDeviceTextureCompressionASTCHDRFeatures`],
    ///    [`VkPhysicalDeviceTileMemoryHeapFeaturesQcom`],
    ///    [`VkPhysicalDeviceTilePropertiesFeaturesQcom`],
    ///    [`VkPhysicalDeviceTileShadingFeaturesQcom`],
    ///    [`VkPhysicalDeviceTimelineSemaphoreFeatures`],
    ///    [`VkPhysicalDeviceTransformFeedbackFeaturesExt`],
    ///    [`VkPhysicalDeviceUnifiedImageLayoutsFeaturesKhr`],
    ///    [`VkPhysicalDeviceUniformBufferStandardLayoutFeatures`],
    ///    [`VkPhysicalDeviceVariablePointersFeatures`],
    ///    [`VkPhysicalDeviceVertexAttributeDivisorFeatures`],
    ///    [`VkPhysicalDeviceVertexAttributeRobustnessFeaturesExt`],
    ///    [`VkPhysicalDeviceVertexInputDynamicStateFeaturesExt`],
    ///    [`VkPhysicalDeviceVideoDecodeVP9FeaturesKhr`],
    ///    [`VkPhysicalDeviceVideoEncodeAV1FeaturesKhr`],
    ///    [`VkPhysicalDeviceVideoEncodeIntraRefreshFeaturesKhr`],
    ///    [`VkPhysicalDeviceVideoEncodeQuantizationMapFeaturesKhr`],
    ///    [`VkPhysicalDeviceVideoEncodeRgbConversionFeaturesValve`],
    ///    [`VkPhysicalDeviceVideoMaintenance1FeaturesKhr`],
    ///    [`VkPhysicalDeviceVideoMaintenance2FeaturesKhr`], [`VkPhysicalDeviceVulkan11Features`],
    ///    [`VkPhysicalDeviceVulkan12Features`], [`VkPhysicalDeviceVulkan13Features`],
    ///    [`VkPhysicalDeviceVulkan14Features`], [`VkPhysicalDeviceVulkanMemoryModelFeatures`],
    ///    [`VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKhr`],
    ///    [`VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesExt`],
    ///    [`VkPhysicalDeviceYcbcrDegammaFeaturesQcom`],
    ///    [`VkPhysicalDeviceYcbcrImageArraysFeaturesExt`],
    ///    [`VkPhysicalDeviceZeroInitializeDeviceMemoryFeaturesExt`], or
    ///    [`VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique, with the
    ///    exception of structures of type [`VkDeviceDeviceMemoryReportCreateInfoExt`] or
    ///    [`VkDevicePrivateDataCreateInfo`]
    pub next: *const c_void,

    /// `flags` is reserved for future use.
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be 0
    pub flags: VkDeviceCreateFlags,

    /// `queue_create_info_count` is the unsigned integer size of the `queue_create_infos` array.
    ///
    /// # Valid Usage
    ///  - If the `maintenance9` feature is not supported, `queue_create_info_count` must be
    ///    greater than 0
    pub queue_create_info_count: u32,

    /// `queue_create_infos` is a pointer to an array of [`VkDeviceQueueCreateInfo`] structures
    /// describing the queues that are requested to be created along with the logical device.
    ///
    /// # Valid Usage
    ///  - The `queue_family_index` member of each element of `queue_create_infos` must be unique
    ///    within `queue_create_infos` , except that two members can share the same
    ///    `queue_family_index` if one describes protected-capable queues and one describes queues
    ///    that are not protected-capable
    ///  - If multiple elements of `queue_create_infos` share the same `queue_family_index`, the
    ///    sum of their `queue_count` members must be less than or equal to the `queue_count`
    ///    member of the [`VkQueueFamilyProperties`] structure, as returned by
    ///    [`VkGetPhysicalDeviceQueueFamilyProperties`] in the
    ///    `queue_family_properties[queue_family_index]`
    ///  - If multiple elements of `queue_create_infos` share the same `queue_family_index`, then
    ///    all of such elements must have the same global priority level, which can be specified
    ///    explicitly by the including a [`VkDeviceQueueGlobalPriorityCreateInfo`] structure in the
    ///    `next` chain, or by the implicit default value
    ///  - If any element of `queue_create_infos` specifies a `queue_family_index` that supports
    ///    [`VkQueueFlag::DataGraphBitArm`] and that `queue_family_index` enumerates an engine
    ///    through [`VkGetPhysicalDeviceQueueFamilyDataGraphPropertiesArm`] with type
    ///    [`VkPhysicalDeviceDataGraphProcessingEngineTypeArm::NeuralQcom`] or
    ///    [`VkPhysicalDeviceDataGraphProcessingEngineTypeArm::ComputeQcom`], a
    ///    [`VkPhysicalDeviceDataGraphModelFeaturesQcom`] structure must be included in `next` with
    ///    `data_graph_model` set to [`VK_TRUE`]
    ///
    /// # Valid Usage (Implicit)
    ///  - If `queue_create_info_count` is not 0, `queue_create_infos` must be a valid pointer to
    ///    an array of `queue_create_info_count` valid [`VkDeviceQueueCreateInfo`] structures
    pub queue_create_infos: *const VkDeviceQueueCreateInfo,

    /// `enabled_layer_count` is deprecated and ignored.
    pub enabled_layer_count: u32,

    /// `enabled_layer_names` is deprecated and ignored.
    ///
    /// # Valid Usage (Implicit)
    ///  - If `enabled_layer_count` is not 0, `enabled_layer_names` must be a valid pointer to an
    ///    array of `enabled_layer_count` null-terminated UTF-8 strings
    pub enabled_layer_names: *const *const c_char,

    /// `enabled_extension_count` is the number of device extensions to enable.
    pub enabled_extension_count: u32,

    /// `enabled_extension_names` is a pointer to an array of `enabled_extension_count`
    /// null-terminated UTF-8 strings containing the names of extensions to enable for the created
    /// device.
    ///
    /// # Valid Usage
    ///  - If [`VkPhysicalDeviceProperties::api_version`] advertises Vulkan 1.1 or later,
    ///    `enabled_extension_names` must not contain "VK_Amd_negative_viewport_height"
    ///  - `enabled_extension_names` must not contain both "VK_Khr_maintenance1" and
    ///    "VK_Amd_negative_viewport_height"
    ///  - `enabled_extension_names` must not contain both "VK_Khr_buffer_device_address" and
    ///    "VK_Ext_buffer_device_address"
    ///  - If the `next` chain includes a [`VkPhysicalDeviceVulkan12Features`] structure and
    ///    [`VkPhysicalDeviceVulkan12Features::buffer_device_address`] is [`VK_TRUE`],
    ///    `enabled_extension_names` must not contain "VK_Ext_buffer_device_address"
    ///
    /// # Valid Usage (Implicit)
    ///  - If `enabled_extension_count` is not 0, `enabled_extension_names` must be a valid pointer
    ///    to an array of `enabled_extension_count` null-terminated UTF-8 strings
    pub enabled_extension_names: *const *const c_char,

    /// `enabled_features` is [`null`] or a pointer to a [`VkPhysicalDeviceFeatures`] structure
    /// containing boolean indicators of all the features to be enabled.
    ///
    /// # Valid Usage
    ///  - If the `next` chain includes a [`VkPhysicalDeviceFeatures2`] structure, then
    ///    `enabled_features` must be [`null`]
    ///
    /// # Valid Usage (Implicit)
    ///  - If `enabled_features` is not [`null`], `enabled_features` must be a valid pointer to a
    ///    valid [`VkPhysicalDeviceFeatures`] structure
    pub enabled_features: *const VkPhysicalDeviceFeatures,
}

impl const Default for VkDeviceCreateInfo {
    fn default() -> Self {
        VkDeviceCreateInfo {
            r#type: VkStructureType::DeviceCreateInfo,
            next: null(),
            flags: VkDeviceCreateFlags::new(),
            queue_create_info_count: 0,
            queue_create_infos: null(),
            enabled_layer_count: 0,
            enabled_layer_names: null(),
            enabled_extension_count: 0,
            enabled_extension_names: null(),
            enabled_features: null(),
        }
    }
}
