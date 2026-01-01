// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VK_VERSION_1_1, VK_VERSION_1_2, VK_VERSION_1_3, VK_VERSION_1_4, ext_debug_utils,
};

/// Vulkan structure types
///
/// # Description
/// Each value corresponds to a particular structure with a `r#type` member with a matching name.
///
/// The values [`VkStructureType::LoaderInstanceCreateInfo`] and
/// [`VkStructureType::LoaderDeviceCreateInfo`] are reserved for internal use by the loader, and do
/// not have corresponding Vulkan structures in this Specification.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum VkStructureType {
    ApplicationInfo = 0,
    InstanceCreateInfo = 1,
    DeviceQueueCreateInfo = 2,
    DeviceCreateInfo = 3,
    SubmitInfo = 4,
    MemoryAllocateInfo = 5,
    MappedMemoryRange = 6,
    BindSparseInfo = 7,
    FenceCreateInfo = 8,
    SemaphoreCreateInfo = 9,
    EventCreateInfo = 10,
    QueryPoolCreateInfo = 11,
    BufferCreateInfo = 12,
    BufferViewCreateInfo = 13,
    ImageCreateInfo = 14,
    ImageViewCreateInfo = 15,
    ShaderModuleCreateInfo = 16,
    PipelineCacheCreateInfo = 17,
    PipelineShaderStageCreateInfo = 18,
    PipelineVertexInputStateCreateInfo = 19,
    PipelineInputAssemblyStateCreateInfo = 20,
    PipelineTessellationStateCreateInfo = 21,
    PipelineViewportStateCreateInfo = 22,
    PipelineRasterizationStateCreateInfo = 23,
    PipelineMultisampleStateCreateInfo = 24,
    PipelineDepthStencilStateCreateInfo = 25,
    PipelineColorBlendStateCreateInfo = 26,
    PipelineDynamicStateCreateInfo = 27,
    GraphicsPipelineCreateInfo = 28,
    ComputePipelineCreateInfo = 29,
    PipelineLayoutCreateInfo = 30,
    SamplerCreateInfo = 31,
    DescriptorSetLayoutCreateInfo = 32,
    DescriptorPoolCreateInfo = 33,
    DescriptorSetAllocateInfo = 34,
    WriteDescriptorSet = 35,
    CopyDescriptorSet = 36,
    FramebufferCreateInfo = 37,
    RenderPassCreateInfo = 38,
    CommandPoolCreateInfo = 39,
    CommandBufferAllocateInfo = 40,
    CommandBufferInheritanceInfo = 41,
    CommandBufferBeginInfo = 42,
    RenderPassBeginInfo = 43,
    BufferMemoryBarrier = 44,
    ImageMemoryBarrier = 45,
    MemoryBarrier = 46,
    LoaderInstanceCreateInfo = 47,
    LoaderDeviceCreateInfo = 48,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceSubgroupProperties = 1000094000,

    /// Provided by [`VK_VERSION_1_1`]
    BindBufferMemoryInfo = 1000157000,

    /// Provided by [`VK_VERSION_1_1`]
    BindImageMemoryInfo = 1000157001,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDevice16BitStorageFeatures = 1000083000,

    /// Provided by [`VK_VERSION_1_1`]
    MemoryDedicatedRequirements = 1000127000,

    /// Provided by [`VK_VERSION_1_1`]
    MemoryDedicatedAllocateInfo = 1000127001,

    /// Provided by [`VK_VERSION_1_1`]
    MemoryAllocateFlagsInfo = 1000060000,

    /// Provided by [`VK_VERSION_1_1`]
    DeviceGroupRenderPassBeginInfo = 1000060003,

    /// Provided by [`VK_VERSION_1_1`]
    DeviceGroupCommandBufferBeginInfo = 1000060004,

    /// Provided by [`VK_VERSION_1_1`]
    DeviceGroupSubmitInfo = 1000060005,

    /// Provided by [`VK_VERSION_1_1`]
    DeviceGroupBindSparseInfo = 1000060006,

    /// Provided by [`VK_VERSION_1_1`]
    BindBufferMemoryDeviceGroupInfo = 1000060013,

    /// Provided by [`VK_VERSION_1_1`]
    BindImageMemoryDeviceGroupInfo = 1000060014,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceGroupProperties = 1000070000,

    /// Provided by [`VK_VERSION_1_1`]
    DeviceGroupDeviceCreateInfo = 1000070001,

    /// Provided by [`VK_VERSION_1_1`]
    BufferMemoryRequirementsInfo2 = 1000146000,

    /// Provided by [`VK_VERSION_1_1`]
    ImageMemoryRequirementsInfo2 = 1000146001,

    /// Provided by [`VK_VERSION_1_1`]
    ImageSparseMemoryRequirementsInfo2 = 1000146002,

    /// Provided by [`VK_VERSION_1_1`]
    MemoryRequirements2 = 1000146003,

    /// Provided by [`VK_VERSION_1_1`]
    SparseImageMemoryRequirements2 = 1000146004,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceFeatures2 = 1000059000,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceProperties2 = 1000059001,

    /// Provided by [`VK_VERSION_1_1`]
    FormatProperties2 = 1000059002,

    /// Provided by [`VK_VERSION_1_1`]
    ImageFormatProperties2 = 1000059003,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceImageFormatInfo2 = 1000059004,

    /// Provided by [`VK_VERSION_1_1`]
    QueueFamilyProperties2 = 1000059005,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceMemoryProperties2 = 1000059006,

    /// Provided by [`VK_VERSION_1_1`]
    SparseImageFormatProperties2 = 1000059007,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceSparseImageFormatInfo2 = 1000059008,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDevicePointClippingProperties = 1000117000,

    /// Provided by [`VK_VERSION_1_1`]
    RenderPassInputAttachmentAspectCreateInfo = 1000117001,

    /// Provided by [`VK_VERSION_1_1`]
    ImageViewUsageCreateInfo = 1000117002,

    /// Provided by [`VK_VERSION_1_1`]
    PipelineTessellationDomainOriginStateCreateInfo = 1000117003,

    /// Provided by [`VK_VERSION_1_1`]
    RenderPassMultiviewCreateInfo = 1000053000,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceMultiviewFeatures = 1000053001,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceMultiviewProperties = 1000053002,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceVariablePointersFeatures = 1000120000,

    /// Provided by [`VK_VERSION_1_1`]
    ProtectedSubmitInfo = 1000145000,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceProtectedMemoryFeatures = 1000145001,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceProtectedMemoryProperties = 1000145002,

    /// Provided by [`VK_VERSION_1_1`]
    DeviceQueueInfo2 = 1000145003,

    /// Provided by [`VK_VERSION_1_1`]
    SamplerYcbcrConversionCreateInfo = 1000156000,

    /// Provided by [`VK_VERSION_1_1`]
    SamplerYcbcrConversionInfo = 1000156001,

    /// Provided by [`VK_VERSION_1_1`]
    BindImagePlaneMemoryInfo = 1000156002,

    /// Provided by [`VK_VERSION_1_1`]
    ImagePlaneMemoryRequirementsInfo = 1000156003,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceSamplerYcbcrConversionFeatures = 1000156004,

    /// Provided by [`VK_VERSION_1_1`]
    SamplerYcbcrConversionImageFormatProperties = 1000156005,

    /// Provided by [`VK_VERSION_1_1`]
    DescriptorUpdateTemplateCreateInfo = 1000085000,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceExternalImageFormatInfo = 1000071000,

    /// Provided by [`VK_VERSION_1_1`]
    ExternalImageFormatProperties = 1000071001,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceExternalBufferInfo = 1000071002,

    /// Provided by [`VK_VERSION_1_1`]
    ExternalBufferProperties = 1000071003,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceIdProperties = 1000071004,

    /// Provided by [`VK_VERSION_1_1`]
    ExternalMemoryBufferCreateInfo = 1000072000,

    /// Provided by [`VK_VERSION_1_1`]
    ExternalMemoryImageCreateInfo = 1000072001,

    /// Provided by [`VK_VERSION_1_1`]
    ExportMemoryAllocateInfo = 1000072002,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceExternalFenceInfo = 1000112000,

    /// Provided by [`VK_VERSION_1_1`]
    ExternalFenceProperties = 1000112001,

    /// Provided by [`VK_VERSION_1_1`]
    ExportFenceCreateInfo = 1000113000,

    /// Provided by [`VK_VERSION_1_1`]
    ExportSemaphoreCreateInfo = 1000077000,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceExternalSemaphoreInfo = 1000076000,

    /// Provided by [`VK_VERSION_1_1`]
    ExternalSemaphoreProperties = 1000076001,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceMaintenance3Properties = 1000168000,

    /// Provided by [`VK_VERSION_1_1`]
    DescriptorSetLayoutSupport = 1000168001,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceShaderDrawParametersFeatures = 1000063000,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceVulkan11Features = 49,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceVulkan11Properties = 50,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceVulkan12Features = 51,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceVulkan12Properties = 52,

    /// Provided by [`VK_VERSION_1_2`]
    ImageFormatListCreateInfo = 1000147000,

    /// Provided by [`VK_VERSION_1_2`]
    AttachmentDescription2 = 1000109000,

    /// Provided by [`VK_VERSION_1_2`]
    AttachmentReference2 = 1000109001,

    /// Provided by [`VK_VERSION_1_2`]
    SubpassDescription2 = 1000109002,

    /// Provided by [`VK_VERSION_1_2`]
    SubpassDependency2 = 1000109003,

    /// Provided by [`VK_VERSION_1_2`]
    RenderPassCreateInfo2 = 1000109004,

    /// Provided by [`VK_VERSION_1_2`]
    SubpassBeginInfo = 1000109005,

    /// Provided by [`VK_VERSION_1_2`]
    SubpassEndInfo = 1000109006,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDevice8BitStorageFeatures = 1000177000,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceDriverProperties = 1000196000,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceShaderAtomicInt64Features = 1000180000,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceShaderFloat16Int8Features = 1000082000,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceFloatControlsProperties = 1000197000,

    /// Provided by [`VK_VERSION_1_2`]
    DescriptorSetLayoutBindingFlagsCreateInfo = 1000161000,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceDescriptorIndexingFeatures = 1000161001,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceDescriptorIndexingProperties = 1000161002,

    /// Provided by [`VK_VERSION_1_2`]
    DescriptorSetVariableDescriptorCountAllocateInfo = 1000161003,

    /// Provided by [`VK_VERSION_1_2`]
    DescriptorSetVariableDescriptorCountLayoutSupport = 1000161004,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceDepthStencilResolveProperties = 1000199000,

    /// Provided by [`VK_VERSION_1_2`]
    SubpassDescriptionDepthStencilResolve = 1000199001,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceScalarBlockLayoutFeatures = 1000221000,

    /// Provided by [`VK_VERSION_1_2`]
    ImageStencilUsageCreateInfo = 1000246000,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceSamplerFilterMinmaxProperties = 1000130000,

    /// Provided by [`VK_VERSION_1_2`]
    SamplerReductionModeCreateInfo = 1000130001,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceVulkanMemoryModelFeatures = 1000211000,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceImagelessFramebufferFeatures = 1000108000,

    /// Provided by [`VK_VERSION_1_2`]
    FramebufferAttachmentsCreateInfo = 1000108001,

    /// Provided by [`VK_VERSION_1_2`]
    FramebufferAttachmentImageInfo = 1000108002,

    /// Provided by [`VK_VERSION_1_2`]
    RenderPassAttachmentBeginInfo = 1000108003,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceUniformBufferStandardLayoutFeatures = 1000253000,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceShaderSubgroupExtendedTypesFeatures = 1000175000,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceSeparateDepthStencilLayoutsFeatures = 1000241000,

    /// Provided by [`VK_VERSION_1_2`]
    AttachmentReferenceStencilLayout = 1000241001,

    /// Provided by [`VK_VERSION_1_2`]
    AttachmentDescriptionStencilLayout = 1000241002,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceHostQueryResetFeatures = 1000261000,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceTimelineSemaphoreFeatures = 1000207000,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceTimelineSemaphoreProperties = 1000207001,

    /// Provided by [`VK_VERSION_1_2`]
    SemaphoreTypeCreateInfo = 1000207002,

    /// Provided by [`VK_VERSION_1_2`]
    TimelineSemaphoreSubmitInfo = 1000207003,

    /// Provided by [`VK_VERSION_1_2`]
    SemaphoreWaitInfo = 1000207004,

    /// Provided by [`VK_VERSION_1_2`]
    SemaphoreSignalInfo = 1000207005,

    /// Provided by [`VK_VERSION_1_2`]
    PhysicalDeviceBufferDeviceAddressFeatures = 1000257000,

    /// Provided by [`VK_VERSION_1_2`]
    BufferDeviceAddressInfo = 1000244001,

    /// Provided by [`VK_VERSION_1_2`]
    BufferOpaqueCaptureAddressCreateInfo = 1000257002,

    /// Provided by [`VK_VERSION_1_2`]
    MemoryOpaqueCaptureAddressAllocateInfo = 1000257003,

    /// Provided by [`VK_VERSION_1_2`]
    DeviceMemoryOpaqueCaptureAddressInfo = 1000257004,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceVulkan13Features = 53,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceVulkan13Properties = 54,

    /// Provided by [`VK_VERSION_1_3`]
    PipelineCreationFeedbackCreateInfo = 1000192000,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceShaderTerminateInvocationFeatures = 1000215000,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceToolProperties = 1000245000,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceShaderDemoteToHelperInvocationFeatures = 1000276000,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDevicePrivateDataFeatures = 1000295000,

    /// Provided by [`VK_VERSION_1_3`]
    DevicePrivateDataCreateInfo = 1000295001,

    /// Provided by [`VK_VERSION_1_3`]
    PrivateDataSlotCreateInfo = 1000295002,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDevicePipelineCreationCacheControlFeatures = 1000297000,

    /// Provided by [`VK_VERSION_1_3`]
    MemoryBarrier2 = 1000314000,

    /// Provided by [`VK_VERSION_1_3`]
    BufferMemoryBarrier2 = 1000314001,

    /// Provided by [`VK_VERSION_1_3`]
    ImageMemoryBarrier2 = 1000314002,

    /// Provided by [`VK_VERSION_1_3`]
    DependencyInfo = 1000314003,

    /// Provided by [`VK_VERSION_1_3`]
    SubmitInfo2 = 1000314004,

    /// Provided by [`VK_VERSION_1_3`]
    SemaphoreSubmitInfo = 1000314005,

    /// Provided by [`VK_VERSION_1_3`]
    CommandBufferSubmitInfo = 1000314006,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceSynchronization2Features = 1000314007,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures = 1000325000,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceImageRobustnessFeatures = 1000335000,

    /// Provided by [`VK_VERSION_1_3`]
    CopyBufferInfo2 = 1000337000,

    /// Provided by [`VK_VERSION_1_3`]
    CopyImageInfo2 = 1000337001,

    /// Provided by [`VK_VERSION_1_3`]
    CopyBufferToImageInfo2 = 1000337002,

    /// Provided by [`VK_VERSION_1_3`]
    CopyImageToBufferInfo2 = 1000337003,

    /// Provided by [`VK_VERSION_1_3`]
    BlitImageInfo2 = 1000337004,

    /// Provided by [`VK_VERSION_1_3`]
    ResolveImageInfo2 = 1000337005,

    /// Provided by [`VK_VERSION_1_3`]
    BufferCopy2 = 1000337006,

    /// Provided by [`VK_VERSION_1_3`]
    ImageCopy2 = 1000337007,

    /// Provided by [`VK_VERSION_1_3`]
    ImageBlit2 = 1000337008,

    /// Provided by [`VK_VERSION_1_3`]
    BufferImageCopy2 = 1000337009,

    /// Provided by [`VK_VERSION_1_3`]
    ImageResolve2 = 1000337010,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceSubgroupSizeControlProperties = 1000225000,

    /// Provided by [`VK_VERSION_1_3`]
    PipelineShaderStageRequiredSubgroupSizeCreateInfo = 1000225001,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceSubgroupSizeControlFeatures = 1000225002,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceInlineUniformBlockFeatures = 1000138000,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceInlineUniformBlockProperties = 1000138001,

    /// Provided by [`VK_VERSION_1_3`]
    WriteDescriptorSetInlineUniformBlock = 1000138002,

    /// Provided by [`VK_VERSION_1_3`]
    DescriptorPoolInlineUniformBlockCreateInfo = 1000138003,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceTextureCompressionAstcHdrFeatures = 1000066000,

    /// Provided by [`VK_VERSION_1_3`]
    RenderingInfo = 1000044000,

    /// Provided by [`VK_VERSION_1_3`]
    RenderingAttachmentInfo = 1000044001,

    /// Provided by [`VK_VERSION_1_3`]
    PipelineRenderingCreateInfo = 1000044002,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceDynamicRenderingFeatures = 1000044003,

    /// Provided by [`VK_VERSION_1_3`]
    CommandBufferInheritanceRenderingInfo = 1000044004,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceShaderIntegerDotProductFeatures = 1000280000,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceShaderIntegerDotProductProperties = 1000280001,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceTexelBufferAlignmentProperties = 1000281001,

    /// Provided by [`VK_VERSION_1_3`]
    FormatProperties3 = 1000360000,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceMaintenance4Features = 1000413000,

    /// Provided by [`VK_VERSION_1_3`]
    PhysicalDeviceMaintenance4Properties = 1000413001,

    /// Provided by [`VK_VERSION_1_3`]
    DeviceBufferMemoryRequirements = 1000413002,

    /// Provided by [`VK_VERSION_1_3`]
    DeviceImageMemoryRequirements = 1000413003,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceVulkan1_4Features = 55,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceVulkan1_4Properties = 56,

    /// Provided by [`VK_VERSION_1_4`]
    DeviceQueueGlobalPriorityCreateInfo = 1000174000,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceGlobalPriorityQueryFeatures = 1000388000,

    /// Provided by [`VK_VERSION_1_4`]
    QueueFamilyGlobalPriorityProperties = 1000388001,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceIndexTypeUInt8Features = 1000265000,

    /// Provided by [`VK_VERSION_1_4`]
    MemoryMapInfo = 1000271000,

    /// Provided by [`VK_VERSION_1_4`]
    MemoryUnmapInfo = 1000271001,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceMaintenance5Features = 1000470000,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceMaintenance5Properties = 1000470001,

    /// Provided by [`VK_VERSION_1_4`]
    DeviceImageSubresourceInfo = 1000470004,

    /// Provided by [`VK_VERSION_1_4`]
    SubresourceLayout2 = 1000338002,

    /// Provided by [`VK_VERSION_1_4`]
    ImageSubresource2 = 1000338003,

    /// Provided by [`VK_VERSION_1_4`]
    BufferUsageFlags2CreateInfo = 1000470006,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceMaintenance6Features = 1000545000,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceMaintenance6Properties = 1000545001,

    /// Provided by [`VK_VERSION_1_4`]
    BindMemoryStatus = 1000545002,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceHostImageCopyFeatures = 1000270000,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceHostImageCopyProperties = 1000270001,

    /// Provided by [`VK_VERSION_1_4`]
    MemoryToImageCopy = 1000270002,

    /// Provided by [`VK_VERSION_1_4`]
    ImageToMemoryCopy = 1000270003,

    /// Provided by [`VK_VERSION_1_4`]
    CopyImageToMemoryInfo = 1000270004,

    /// Provided by [`VK_VERSION_1_4`]
    CopyMemoryToImageInfo = 1000270005,

    /// Provided by [`VK_VERSION_1_4`]
    HostImageLayoutTransitionInfo = 1000270006,

    /// Provided by [`VK_VERSION_1_4`]
    CopyImageToImageInfo = 1000270007,

    /// Provided by [`VK_VERSION_1_4`]
    SubresourceHostMemcpySize = 1000270008,

    /// Provided by [`VK_VERSION_1_4`]
    HostImageCopyDevicePerformanceQuery = 1000270009,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceShaderSubgroupRotateFeatures = 1000416000,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceShaderFloatControls2Features = 1000528000,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceShaderExpectAssumeFeatures = 1000544000,

    /// Provided by [`VK_VERSION_1_4`]
    PipelineCreateFlags2CreateInfo = 1000470005,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDevicePushDescriptorProperties = 1000080000,

    /// Provided by [`VK_VERSION_1_4`]
    BindDescriptorSetsInfo = 1000545003,

    /// Provided by [`VK_VERSION_1_4`]
    PushConstantsInfo = 1000545004,

    /// Provided by [`VK_VERSION_1_4`]
    PushDescriptorSetInfo = 1000545005,

    /// Provided by [`VK_VERSION_1_4`]
    PushDescriptorSetWithTemplateInfo = 1000545006,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDevicePipelineProtectedAccessFeatures = 1000466000,

    /// Provided by [`VK_VERSION_1_4`]
    PipelineRobustnessCreateInfo = 1000068000,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDevicePipelineRobustnessFeatures = 1000068001,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDevicePipelineRobustnessProperties = 1000068002,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceLineRasterizationFeatures = 1000259000,

    /// Provided by [`VK_VERSION_1_4`]
    PipelineRasterizationLineStateCreateInfo = 1000259001,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceLineRasterizationProperties = 1000259002,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceVertexAttributeDivisorProperties = 1000525000,

    /// Provided by [`VK_VERSION_1_4`]
    PipelineVertexInputDivisorStateCreateInfo = 1000190001,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceVertexAttributeDivisorFeatures = 1000190002,

    /// Provided by [`VK_VERSION_1_4`]
    RenderingAreaInfo = 1000470003,

    /// Provided by [`VK_VERSION_1_4`]
    PhysicalDeviceDynamicRenderingLocalReadFeatures = 1000232000,

    /// Provided by [`VK_VERSION_1_4`]
    RenderingAttachmentLocationInfo = 1000232001,

    /// Provided by [`VK_VERSION_1_4`]
    RenderingInputAttachmentIndexInfo = 1000232002,

    /// Provided by [`khr_swapchain`]
    SwapchainCreateInfoKhr = 1000001000,

    /// Provided by [`khr_swapchain`]
    PresentInfoKhr = 1000001001,

    /// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with [`khr_surface`]
    DeviceGroupPresentCapabilitiesKhr = 1000060007,

    /// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with [`khr_swapchain`]
    ImageSwapchainCreateInfoKhr = 1000060008,

    /// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with [`khr_swapchain`]
    BindImageMemorySwapchainInfoKhr = 1000060009,

    /// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with [`khr_swapchain`]
    AcquireNextImageInfoKhr = 1000060010,

    /// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with [`khr_swapchain`]
    DeviceGroupPresentInfoKhr = 1000060011,

    /// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with [`khr_swapchain`]
    DeviceGroupSwapchainCreateInfoKhr = 1000060012,

    /// Provided by [`khr_display`]
    DisplayModeCreateInfoKhr = 1000002000,

    /// Provided by [`khr_display`]
    DisplaySurfaceCreateInfoKhr = 1000002001,

    /// Provided by [`khr_display_swapchain`]
    DisplayPresentInfoKhr = 1000003000,

    /// Provided by [`khr_xlib_surface`]
    XlibSurfaceCreateInfoKhr = 1000004000,

    /// Provided by [`khr_xcb_surface`]
    XcbSurfaceCreateInfoKhr = 1000005000,

    /// Provided by [`khr_wayland_surface`]
    WaylandSurfaceCreateInfoKhr = 1000006000,

    /// Provided by [`khr_android_surface`]
    AndroidSurfaceCreateInfoKhr = 1000008000,

    /// Provided by [`khr_win32_surface`]
    Win32SurfaceCreateInfoKhr = 1000009000,

    /// Provided by [`ext_debug_report`]
    DebugReportCallbackCreateInfoExt = 1000011000,

    /// Provided by [`amd_raserization_order`]
    PipelineRasterizationStateRasterizationOrderAmd = 1000018000,

    /// Provided by [`ext_debug_marker`]
    DebugMarkerObjectNameInfoExt = 1000022000,

    /// Provided by [`ext_debug_marker`]
    DebugMarkerObjectTagInfoExt = 1000022001,

    /// Provided by [`ext_debug_marker`]
    DebugMarkerMarkerInfoExt = 1000022002,

    /// Provided by [`khr_video_queue`]
    VideoProfileInfoKhr = 1000023000,

    /// Provided by [`khr_video_queue`]
    VideoCapabilitiesKhr = 1000023001,

    /// Provided by [`khr_video_queue`]
    VideoPictureResourceInfoKhr = 1000023002,

    /// Provided by [`khr_video_queue`]
    VideoSessionMemoryRequirementsKhr = 1000023003,

    /// Provided by [`khr_video_queue`]
    BindVideoSessionMemoryInfoKhr = 1000023004,

    /// Provided by [`khr_video_queue`]
    VideoSessionCreateInfoKhr = 1000023005,

    /// Provided by [`khr_video_queue`]
    VideoSessionParametersCreateInfoKhr = 1000023006,

    /// Provided by [`khr_video_queue`]
    VideoSessionParametersUpdateInfoKhr = 1000023007,

    /// Provided by [`khr_video_queue`]
    VideoBeginCodingInfoKhr = 1000023008,

    /// Provided by [`khr_video_queue`]
    VideoEndCodingInfoKhr = 1000023009,

    /// Provided by [`khr_video_queue`]
    VideoCodingControlInfoKhr = 1000023010,

    /// Provided by [`khr_video_queue`]
    VideoReferenceSlotInfoKhr = 1000023011,

    /// Provided by [`khr_video_queue`]
    QueueFamilyVideoPropertiesKhr = 1000023012,

    /// Provided by [`khr_video_queue`]
    VideoProfileListInfoKhr = 1000023013,

    /// Provided by [`khr_video_queue`]
    PhysicalDeviceVideoFormatInfoKhr = 1000023014,

    /// Provided by [`khr_video_queue`]
    VideoFormatPropertiesKhr = 1000023015,

    /// Provided by [`khr_video_queue`]
    QueueFamilyQueryResultStatusPropertiesKhr = 1000023016,

    /// Provided by [`khr_video_decode_queue`]
    VideoDecodeInfoKhr = 1000024000,

    /// Provided by [`khr_video_decode_queue`]
    VideoDecodeCapabilitiesKhr = 1000024001,

    /// Provided by [`khr_video_decode_queue`]
    VideoDecodeUsageInfoKhr = 1000024002,

    /// Provided by [`nv_dedicated_allocation`]
    DedicatedAllocationImageCreateInfoNv = 1000026000,

    /// Provided by [`nv_dedicated_allocation`]
    DedicatedAllocationBufferCreateInfoNv = 1000026001,

    /// Provided by [`nv_dedicated_allocation`]
    DedicatedAllocationMemoryAllocateInfoNv = 1000026002,

    /// Provided by [`ext_transform_feedback`]
    PhysicalDeviceTransformFeedbackFeaturesExt = 1000028000,

    /// Provided by [`ext_transform_feedback`]
    PhysicalDeviceTransformFeedbackPropertiesExt = 1000028001,

    /// Provided by [`ext_transform_feedback`]
    PipelineRasterizationStateStreamCreateInfoExt = 1000028002,

    /// Provided by [`nvx_binary_import`]
    CuModuleCreateInfoNvx = 1000029000,

    /// Provided by [`nvx_binary_import`]
    CuFunctionCreateInfoNvx = 1000029001,

    /// Provided by [`nvx_binary_import`]
    CuLaunchInfoNvx = 1000029002,

    /// Provided by [`nvx_image_view_handle`]
    ImageViewHandleInfoNvx = 1000030000,

    /// Provided by [`nvx_image_view_handle`]
    ImageViewAddressPropertiesNvx = 1000030001,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264CapabilitiesKhr = 1000038000,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264SessionParametersCreateInfoKhr = 1000038001,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264SessionParametersAddInfoKhr = 1000038002,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264PictureInfoKhr = 1000038003,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264DpbSlotInfoKhr = 1000038004,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264NaluSliceInfoKhr = 1000038005,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264GopRemainingFrameInfoKhr = 1000038006,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264ProfileInfoKhr = 1000038007,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264RateControlInfoKhr = 1000038008,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264RateControlLayerInfoKhr = 1000038009,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264SessionCreateInfoKhr = 1000038010,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264QualityLevelPropertiesKhr = 1000038011,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264SessionParametersGetInfoKhr = 1000038012,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264SessionParametersFeedbackInfoKhr = 1000038013,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265CapabilitiesKhr = 1000039000,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265SessionParametersCreateInfoKhr = 1000039001,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265SessionParametersAddInfoKhr = 1000039002,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265PictureInfoKhr = 1000039003,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265DpbSlotInfoKhr = 1000039004,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265NaluSliceSegmentInfoKhr = 1000039005,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265GopRemainingFrameInfoKhr = 1000039006,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265ProfileInfoKhr = 1000039007,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265RateControlInfoKhr = 1000039009,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265RateControlLayerInfoKhr = 1000039010,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265SessionCreateInfoKhr = 1000039011,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265QualityLevelPropertiesKhr = 1000039012,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265SessionParametersGetInfoKhr = 1000039013,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265SessionParametersFeedbackInfoKhr = 1000039014,

    /// Provided by [`khr_video_decode_h264`]
    VideoDecodeH264CapabilitiesKhr = 1000040000,

    /// Provided by [`khr_video_decode_h264`]
    VideoDecodeH264PictureInfoKhr = 1000040001,

    /// Provided by [`khr_video_decode_h264`]
    VideoDecodeH264ProfileInfoKhr = 1000040003,

    /// Provided by [`khr_video_decode_h264`]
    VideoDecodeH264SessionParametersCreateInfoKhr = 1000040004,

    /// Provided by [`khr_video_decode_h264`]
    VideoDecodeH264SessionParametersAddInfoKhr = 1000040005,

    /// Provided by [`khr_video_decode_h264`]
    VideoDecodeH264DpbSlotInfoKhr = 1000040006,

    /// Provided by [`amd_texture_gather_bias_lod`]
    TextureLodGatherFormatPropertiesAmd = 1000041000,

    /// Provided by [`khr_dynamic_rendering`] with [`khr_fragment_shading_rate`]
    RenderingFragmentShadingRateAttachmentInfoKhr = 1000044006,

    /// Provided by [`khr_dynamic_rendering`] with [`ext_fragment_density_map`]
    RenderingFragmentDensityMapAttachmentInfoExt = 1000044007,

    /// Provided by [`khr_dynamic_rendering`] with [`amd_mixed_attachment_samples`]
    AttachmentSampleCountInfoAmd = 1000044008,

    /// Provided by [`khr_dynamic_rendering`] with [`nvx_multiview_per_view_attributes`]
    MultiviewPerViewAttributesInfoNvx = 1000044009,

    /// Provided by [`gpgstream_descriptor_surface`]
    StreamDescriptorSurfaceCreateInfoGgp = 1000049000,

    /// Provided by [`nv_corner_sampled_image`]
    PhysicalDeviceCornerSampledImageFeaturesNv = 1000050000,

    /// Provided by [`nv_external_memory`]
    ExternalMemoryImageCreateInfoNv = 1000056000,

    /// Provided by [`nv_external_memory`]
    ExportMemoryAllocateInfoNv = 1000056001,

    /// Provided by [`nv_external_memory_win32`]
    ImportMemoryWin32HandleInfoNv = 1000057000,

    /// Provided by [`nv_external_memory_win32`]
    ExportMemoryWin32HandleInfoNv = 1000057001,

    /// Provided by [`nv_win32_keyed_mutex`]
    Win32KeyedMutexAcquireReleaseInfoNv = 1000058000,

    /// Provided by [`ext_validation_flags`]
    ValidationFlagsExt = 1000061000,

    /// Provided by [`nnvi_surface`]
    ViSurfaceCreateInfoNn = 1000062000,

    /// Provided by [`ext_astc_decode_mode`]
    ImageViewAstcDecodeModeExt = 1000067000,

    /// Provided by [`ext_astc_decode_mode`]
    PhysicalDeviceAstcDecodeFeaturesExt = 1000067001,

    /// Provided by [`khr_external_memory_win32`]
    ImportMemoryWin32HandleInfoKhr = 1000073000,

    /// Provided by [`khr_external_memory_win32`]
    ExportMemoryWin32HandleInfoKhr = 1000073001,

    /// Provided by [`khr_external_memory_win32`]
    MemoryWin32HandlePropertiesKhr = 1000073002,

    /// Provided by [`khr_external_memory_win32`]
    MemoryGetWin32HandleInfoKhr = 1000073003,

    /// Provided by [`khr_external_memory_fd`]
    ImportMemoryFdInfoKhr = 1000074000,

    /// Provided by [`khr_external_memory_fd`]
    MemoryFdPropertiesKhr = 1000074001,

    /// Provided by [`khr_external_memory_fd`]
    MemoryGetFdInfoKhr = 1000074002,

    /// Provided by [`khr_win32_keyed_mutex`]
    Win32KeyedMutexAcquireReleaseInfoKhr = 1000075000,

    /// Provided by [`khr_external_semaphore_win32`]
    ImportSemaphoreWin32HandleInfoKhr = 1000078000,

    /// Provided by [`khr_external_semaphore_win32`]
    ExportSemaphoreWin32HandleInfoKhr = 1000078001,

    /// Provided by [`khr_external_semaphore_win32`]
    D3D12FenceSubmitInfoKhr = 1000078002,

    /// Provided by [`khr_external_semaphore_win32`]
    SemaphoreGetWin32HandleInfoKhr = 1000078003,

    /// Provided by [`khr_external_semaphore_fd`]
    ImportSemaphoreFdInfoKhr = 1000079000,

    /// Provided by [`khr_external_semaphore_fd`]
    SemaphoreGetFdInfoKhr = 1000079001,

    /// Provided by [`ext_conditional_rendering`]
    CommandBufferInheritanceConditionalRenderingInfoExt = 1000081000,

    /// Provided by [`ext_conditional_rendering`]
    PhysicalDeviceConditionalRenderingFeaturesExt = 1000081001,

    /// Provided by [`ext_conditional_rendering`]
    ConditionalRenderingBeginInfoExt = 1000081002,

    /// Provided by [`khr_incremental_present`]
    PresentRegionsKhr = 1000084000,

    /// Provided by [`nv_clip_space_w_scaling`]
    PipelineViewportWScalingStateCreateInfoNv = 1000087000,

    /// Provided by [`ext_display_surface_counter`]
    SurfaceCapabilities2Ext = 1000090000,

    /// Provided by [`ext_display_control`]
    DisplayPowerInfoExt = 1000091000,

    /// Provided by [`ext_display_control`]
    DeviceEventInfoExt = 1000091001,

    /// Provided by [`ext_display_control`]
    DisplayEventInfoExt = 1000091002,

    /// Provided by [`ext_display_control`]
    SwapchainCounterCreateInfoExt = 1000091003,

    /// Provided by [`googledisplay_timing`]
    PresentTimesInfoGoogle = 1000092000,

    /// Provided by [`nvx_multiview_per_view_attributes`]
    PhysicalDeviceMultiviewPerViewAttributesPropertiesNvx = 1000097000,

    /// Provided by [`nv_viewport_swizzle`]
    PipelineViewportSwizzleStateCreateInfoNv = 1000098000,

    /// Provided by [`ext_discard_rectangles`]
    PhysicalDeviceDiscardRectanglePropertiesExt = 1000099000,

    /// Provided by [`ext_discard_rectangles`]
    PipelineDiscardRectangleStateCreateInfoExt = 1000099001,

    /// Provided by [`ext_conservative_rasterization`]
    PhysicalDeviceConservativeRasterizationPropertiesExt = 1000101000,

    /// Provided by [`ext_conservative_rasterization`]
    PipelineRasterizationConservativeStateCreateInfoExt = 1000101001,

    /// Provided by [`ext_depth_clip_enable`]
    PhysicalDeviceDepthClipEnableFeaturesExt = 1000102000,

    /// Provided by [`ext_depth_clip_enable`]
    PipelineRasterizationDepthClipStateCreateInfoExt = 1000102001,

    /// Provided by [`ext_hdr_metadata`]
    HdrMetadataExt = 1000105000,

    /// Provided by [`imgrelaxed_line_rasterization`]
    PhysicalDeviceRelaxedLineRasterizationFeaturesImg = 1000110000,

    /// Provided by [`khr_shared_presentable_image`]
    SharedPresentSurfaceCapabilitiesKhr = 1000111000,

    /// Provided by [`khr_external_fence_win32`]
    ImportFenceWin32HandleInfoKhr = 1000114000,

    /// Provided by [`khr_external_fence_win32`]
    ExportFenceWin32HandleInfoKhr = 1000114001,

    /// Provided by [`khr_external_fence_win32`]
    FenceGetWin32HandleInfoKhr = 1000114002,

    /// Provided by [`khr_external_fence_fd`]
    ImportFenceFdInfoKhr = 1000115000,

    /// Provided by [`khr_external_fence_fd`]
    FenceGetFdInfoKhr = 1000115001,

    /// Provided by [`khr_performance_query`]
    PhysicalDevicePerformanceQueryFeaturesKhr = 1000116000,

    /// Provided by [`khr_performance_query`]
    PhysicalDevicePerformanceQueryPropertiesKhr = 1000116001,

    /// Provided by [`khr_performance_query`]
    QueryPoolPerformanceCreateInfoKhr = 1000116002,

    /// Provided by [`khr_performance_query`]
    PerformanceQuerySubmitInfoKhr = 1000116003,

    /// Provided by [`khr_performance_query`]
    AcquireProfilingLockInfoKhr = 1000116004,

    /// Provided by [`khr_performance_query`]
    PerformanceCounterKhr = 1000116005,

    /// Provided by [`khr_performance_query`]
    PerformanceCounterDescriptionKhr = 1000116006,

    /// Provided by [`khr_get_surface_capabilities2`]
    PhysicalDeviceSurfaceInfo2Khr = 1000119000,

    /// Provided by [`khr_get_surface_capabilities2`]
    SurfaceCapabilities2Khr = 1000119001,

    /// Provided by [`khr_get_surface_capabilities2`]
    SurfaceFormat2Khr = 1000119002,

    /// Provided by [`khr_get_display_properties2`]
    DisplayProperties2Khr = 1000121000,

    /// Provided by [`khr_get_display_properties2`]
    DisplayPlaneProperties2Khr = 1000121001,

    /// Provided by [`khr_get_display_properties2`]
    DisplayModeProperties2Khr = 1000121002,

    /// Provided by [`khr_get_display_properties2`]
    DisplayPlaneInfo2Khr = 1000121003,

    /// Provided by [`khr_get_display_properties2`]
    DisplayPlaneCapabilities2Khr = 1000121004,

    /// Provided by [`mvk_ios_surface`]
    IosSurfaceCreateInfoMVK = 1000122000,

    /// Provided by [`mvk_macos_surface`]
    MacosSurfaceCreateInfoMVK = 1000123000,

    /// Provided by [`ext_debug_utils`]
    DebugUtilsObjectNameInfoExt = 1000128000,

    /// Provided by [`ext_debug_utils`]
    DebugUtilsObjectTagInfoExt = 1000128001,

    /// Provided by [`ext_debug_utils`]
    DebugUtilsLabelExt = 1000128002,

    /// Provided by [`ext_debug_utils`]
    DebugUtilsMessengerCallbackDataExt = 1000128003,

    /// Provided by [`ext_debug_utils`]
    DebugUtilsMessengerCreateInfoExt = 1000128004,

    /// Provided by [`android_external_memory_android_hardware_buffer`]
    AndroidHardwareBufferUsageAndroid = 1000129000,

    /// Provided by [`android_external_memory_android_hardware_buffer`]
    AndroidHardwareBufferPropertiesAndroid = 1000129001,

    /// Provided by [`android_external_memory_android_hardware_buffer`]
    AndroidHardwareBufferFormatPropertiesAndroid = 1000129002,

    /// Provided by [`android_external_memory_android_hardware_buffer`]
    ImportAndroidHardwareBufferInfoAndroid = 1000129003,

    /// Provided by [`android_external_memory_android_hardware_buffer`]
    MemoryGetAndroidHardwareBufferInfoAndroid = 1000129004,

    /// Provided by [`android_external_memory_android_hardware_buffer`]
    ExternalFormatAndroid = 1000129005,

    /// Provided by [`android_external_memory_android_hardware_buffer`] with [`khr_format_feature_flags2`] or [`VK_VERSION_1_3`]
    AndroidHardwareBufferFormatProperties2Android = 1000129006,

    /// Provided by [`amd_xshader_enqueue`]
    PhysicalDeviceShaderEnqueueFeaturesAmdx = 1000134000,

    /// Provided by [`amd_xshader_enqueue`]
    PhysicalDeviceShaderEnqueuePropertiesAmdx = 1000134001,

    /// Provided by [`amd_xshader_enqueue`]
    ExecutionGraphPipelineScratchSizeAmdx = 1000134002,

    /// Provided by [`amd_xshader_enqueue`]
    ExecutionGraphPipelineCreateInfoAmdx = 1000134003,

    /// Provided by [`amd_xshader_enqueue`]
    PipelineShaderStageNodeCreateInfoAmdx = 1000134004,

    /// Provided by [`ext_sample_locations`]
    SampleLocationsInfoExt = 1000143000,

    /// Provided by [`ext_sample_locations`]
    RenderPassSampleLocationsBeginInfoExt = 1000143001,

    /// Provided by [`ext_sample_locations`]
    PipelineSampleLocationsStateCreateInfoExt = 1000143002,

    /// Provided by [`ext_sample_locations`]
    PhysicalDeviceSampleLocationsPropertiesExt = 1000143003,

    /// Provided by [`ext_sample_locations`]
    MultisamplePropertiesExt = 1000143004,

    /// Provided by [`ext_blend_operation_advanced`]
    PhysicalDeviceBlendOperationAdvancedFeaturesExt = 1000148000,

    /// Provided by [`ext_blend_operation_advanced`]
    PhysicalDeviceBlendOperationAdvancedPropertiesExt = 1000148001,

    /// Provided by [`ext_blend_operation_advanced`]
    PipelineColorBlendAdvancedStateCreateInfoExt = 1000148002,

    /// Provided by [`nv_fragment_coverage_to_color`]
    PipelineCoverageToColorStateCreateInfoNv = 1000149000,

    /// Provided by [`khr_acceleration_structure`]
    WriteDescriptorSetAccelerationStructureKhr = 1000150007,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureBuildGeometryInfoKhr = 1000150000,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureDeviceAddressInfoKhr = 1000150002,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureGeometryAabbsDataKhr = 1000150003,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureGeometryInstancesDataKhr = 1000150004,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureGeometryTrianglesDataKhr = 1000150005,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureGeometryKhr = 1000150006,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureVersionInfoKhr = 1000150009,

    /// Provided by [`khr_acceleration_structure`]
    CopyAccelerationStructureInfoKhr = 1000150010,

    /// Provided by [`khr_acceleration_structure`]
    CopyAccelerationStructureToMemoryInfoKhr = 1000150011,

    /// Provided by [`khr_acceleration_structure`]
    CopyMemoryToAccelerationStructureInfoKhr = 1000150012,

    /// Provided by [`khr_acceleration_structure`]
    PhysicalDeviceAccelerationStructureFeaturesKhr = 1000150013,

    /// Provided by [`khr_acceleration_structure`]
    PhysicalDeviceAccelerationStructurePropertiesKhr = 1000150014,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureCreateInfoKhr = 1000150017,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureBuildSizesInfoKhr = 1000150020,

    /// Provided by [`khr_ray_tracing_pipeline`]
    PhysicalDeviceRayTracingPipelineFeaturesKhr = 1000347000,

    /// Provided by [`khr_ray_tracing_pipeline`]
    PhysicalDeviceRayTracingPipelinePropertiesKhr = 1000347001,

    /// Provided by [`khr_ray_tracing_pipeline`]
    RayTracingPipelineCreateInfoKhr = 1000150015,

    /// Provided by [`khr_ray_tracing_pipeline`]
    RayTracingShaderGroupCreateInfoKhr = 1000150016,

    /// Provided by [`khr_ray_tracing_pipeline`]
    RayTracingPipelineInterfaceCreateInfoKhr = 1000150018,

    /// Provided by [`khr_ray_query`]
    PhysicalDeviceRayQueryFeaturesKhr = 1000348013,

    /// Provided by [`nv_framebuffer_mixed_samples`]
    PipelineCoverageModulationStateCreateInfoNv = 1000152000,

    /// Provided by [`nv_shader_sm_builtins`]
    PhysicalDeviceShaderSmBuiltinsFeaturesNv = 1000154000,

    /// Provided by [`nv_shader_sm_builtins`]
    PhysicalDeviceShaderSmBuiltinsPropertiesNv = 1000154001,

    /// Provided by [`ext_image_drm_format_modifier`]
    DrmFormatModifierPropertiesListExt = 1000158000,

    /// Provided by [`ext_image_drm_format_modifier`]
    PhysicalDeviceImageDrmFormatModifierInfoExt = 1000158002,

    /// Provided by [`ext_image_drm_format_modifier`]
    ImageDrmFormatModifierListCreateInfoExt = 1000158003,

    /// Provided by [`ext_image_drm_format_modifier`]
    ImageDrmFormatModifierExplicitCreateInfoExt = 1000158004,

    /// Provided by [`ext_image_drm_format_modifier`]
    ImageDrmFormatModifierPropertiesExt = 1000158005,

    /// Provided by [`ext_image_drm_format_modifier`] with [`khr_format_feature_flags2`] or [`VK_VERSION_1_3`]
    DrmFormatModifierPropertiesList2Ext = 1000158006,

    /// Provided by [`ext_validation_cache`]
    ValidationCacheCreateInfoExt = 1000160000,

    /// Provided by [`ext_validation_cache`]
    ShaderModuleValidationCacheCreateInfoExt = 1000160001,

    /// Provided by [`khr_portability_subset`]
    PhysicalDevicePortabilitySubsetFeaturesKhr = 1000163000,

    /// Provided by [`khr_portability_subset`]
    PhysicalDevicePortabilitySubsetPropertiesKhr = 1000163001,

    /// Provided by [`nv_shading_rate_image`]
    PipelineViewportShadingRateImageStateCreateInfoNv = 1000164000,

    /// Provided by [`nv_shading_rate_image`]
    PhysicalDeviceShadingRateImageFeaturesNv = 1000164001,

    /// Provided by [`nv_shading_rate_image`]
    PhysicalDeviceShadingRateImagePropertiesNv = 1000164002,

    /// Provided by [`nv_shading_rate_image`]
    PipelineViewportCoarseSampleOrderStateCreateInfoNv = 1000164005,

    /// Provided by [`nv_ray_tracing`]
    RayTracingPipelineCreateInfoNv = 1000165000,

    /// Provided by [`nv_ray_tracing`]
    AccelerationStructureCreateInfoNv = 1000165001,

    /// Provided by [`nv_ray_tracing`]
    GeometryNv = 1000165003,

    /// Provided by [`nv_ray_tracing`]
    GeometryTrianglesNv = 1000165004,

    /// Provided by [`nv_ray_tracing`]
    GeometryAabbNv = 1000165005,

    /// Provided by [`nv_ray_tracing`]
    BindAccelerationStructureMemoryInfoNv = 1000165006,

    /// Provided by [`nv_ray_tracing`]
    WriteDescriptorSetAccelerationStructureNv = 1000165007,

    /// Provided by [`nv_ray_tracing`]
    AccelerationStructureMemoryRequirementsInfoNv = 1000165008,

    /// Provided by [`nv_ray_tracing`]
    PhysicalDeviceRayTracingPropertiesNv = 1000165009,

    /// Provided by [`nv_ray_tracing`]
    RayTracingShaderGroupCreateInfoNv = 1000165011,

    /// Provided by [`nv_ray_tracing`]
    AccelerationStructureInfoNv = 1000165012,

    /// Provided by [`nv_representative_fragment_test`]
    PhysicalDeviceRepresentativeFragmentTestFeaturesNv = 1000166000,

    /// Provided by [`nv_representative_fragment_test`]
    PipelineRepresentativeFragmentTestStateCreateInfoNv = 1000166001,

    /// Provided by [`ext_filter_cubic`]
    PhysicalDeviceImageViewImageFormatInfoExt = 1000170000,

    /// Provided by [`ext_filter_cubic`]
    FilterCubicImageViewImageFormatPropertiesExt = 1000170001,

    /// Provided by [`ext_external_memory_host`]
    ImportMemoryHostPointerInfoExt = 1000178000,

    /// Provided by [`ext_external_memory_host`]
    MemoryHostPointerPropertiesExt = 1000178001,

    /// Provided by [`ext_external_memory_host`]
    PhysicalDeviceExternalMemoryHostPropertiesExt = 1000178002,

    /// Provided by [`khr_shader_clock`]
    PhysicalDeviceShaderClockFeaturesKhr = 1000181000,

    /// Provided by [`amd_pipeline_compiler_control`]
    PipelineCompilerControlCreateInfoAmd = 1000183000,

    /// Provided by [`amd_shader_core_properties`]
    PhysicalDeviceShaderCorePropertiesAmd = 1000185000,

    /// Provided by [`khr_video_decode_h265`]
    VideoDecodeH265CapabilitiesKhr = 1000187000,

    /// Provided by [`khr_video_decode_h265`]
    VideoDecodeH265SessionParametersCreateInfoKhr = 1000187001,

    /// Provided by [`khr_video_decode_h265`]
    VideoDecodeH265SessionParametersAddInfoKhr = 1000187002,

    /// Provided by [`khr_video_decode_h265`]
    VideoDecodeH265ProfileInfoKhr = 1000187003,

    /// Provided by [`khr_video_decode_h265`]
    VideoDecodeH265PictureInfoKhr = 1000187004,

    /// Provided by [`khr_video_decode_h265`]
    VideoDecodeH265DpbSlotInfoKhr = 1000187005,

    /// Provided by [`amd_memory_overallocation_behavior`]
    DeviceMemoryOverallocationCreateInfoAmd = 1000189000,

    /// Provided by [`ext_vertex_attribute_divisor`]
    PhysicalDeviceVertexAttributeDivisorPropertiesExt = 1000190000,

    /// Provided by [`gpgframe_token`]
    PresentFrameTokenGgp = 1000191000,

    /// Provided by [`nv_compute_shader_derivatives`]
    PhysicalDeviceComputeShaderDerivativesFeaturesNv = 1000201000,

    /// Provided by [`nv_mesh_shader`]
    PhysicalDeviceMeshShaderFeaturesNv = 1000202000,

    /// Provided by [`nv_mesh_shader`]
    PhysicalDeviceMeshShaderPropertiesNv = 1000202001,

    /// Provided by [`nv_shader_image_footprint`]
    PhysicalDeviceShaderImageFootprintFeaturesNv = 1000204000,

    /// Provided by [`nv_scissor_exclusive`]
    PipelineViewportExclusiveScissorStateCreateInfoNv = 1000205000,

    /// Provided by [`nv_scissor_exclusive`]
    PhysicalDeviceExclusiveScissorFeaturesNv = 1000205002,

    /// Provided by [`nv_device_diagnostic_checkpoints`]
    CheckpointDataNv = 1000206000,

    /// Provided by [`nv_device_diagnostic_checkpoints`]
    QueueFamilyCheckpointPropertiesNv = 1000206001,

    /// Provided by [`intel_shader_integer_functions2`]
    PhysicalDeviceShaderIntegerFunctions2FeaturesIntel = 1000209000,

    /// Provided by [`intel_performance_query`]
    QueryPoolPerformanceQueryCreateInfoIntel = 1000210000,

    /// Provided by [`intel_performance_query`]
    InitializePerformanceApiInfoIntel = 1000210001,

    /// Provided by [`intel_performance_query`]
    PerformanceMarkerInfoIntel = 1000210002,

    /// Provided by [`intel_performance_query`]
    PerformanceStreamMarkerInfoIntel = 1000210003,

    /// Provided by [`intel_performance_query`]
    PerformanceOverrideInfoIntel = 1000210004,

    /// Provided by [`intel_performance_query`]
    PerformanceConfigurationAcquireInfoIntel = 1000210005,

    /// Provided by [`ext_pci_bus_info`]
    PhysicalDevicePciBusInfoPropertiesExt = 1000212000,

    /// Provided by [`amd_display_native_hdr`]
    DisplayNativeHdrSurfaceCapabilitiesAmd = 1000213000,

    /// Provided by [`amd_display_native_hdr`]
    SwapchainDisplayNativeHdrCreateInfoAmd = 1000213001,

    /// Provided by [`fuchsia_imagepipe_surface`]
    ImagepipeSurfaceCreateInfoFuchsia = 1000214000,

    /// Provided by [`ext_metal_surface`]
    MetalSurfaceCreateInfoExt = 1000217000,

    /// Provided by [`ext_fragment_density_map`]
    PhysicalDeviceFragmentDensityMapFeaturesExt = 1000218000,

    /// Provided by [`ext_fragment_density_map`]
    PhysicalDeviceFragmentDensityMapPropertiesExt = 1000218001,

    /// Provided by [`ext_fragment_density_map`]
    RenderPassFragmentDensityMapCreateInfoExt = 1000218002,

    /// Provided by [`khr_fragment_shading_rate`]
    FragmentShadingRateAttachmentInfoKhr = 1000226000,

    /// Provided by [`khr_fragment_shading_rate`]
    PipelineFragmentShadingRateStateCreateInfoKhr = 1000226001,

    /// Provided by [`khr_fragment_shading_rate`]
    PhysicalDeviceFragmentShadingRatePropertiesKhr = 1000226002,

    /// Provided by [`khr_fragment_shading_rate`]
    PhysicalDeviceFragmentShadingRateFeaturesKhr = 1000226003,

    /// Provided by [`khr_fragment_shading_rate`]
    PhysicalDeviceFragmentShadingRateKhr = 1000226004,

    /// Provided by [`amd_shader_core_properties2`]
    PhysicalDeviceShaderCoreProperties2Amd = 1000227000,

    /// Provided by [`amd_device_coherent_memory`]
    PhysicalDeviceCoherentMemoryFeaturesAmd = 1000229000,

    /// Provided by [`ext_shader_image_atomic_int64`]
    PhysicalDeviceShaderImageAtomicInt64FeaturesExt = 1000234000,

    /// Provided by [`khr_shader_quad_control`]
    PhysicalDeviceShaderQuadControlFeaturesKhr = 1000235000,

    /// Provided by [`ext_memory_budget`]
    PhysicalDeviceMemoryBudgetPropertiesExt = 1000237000,

    /// Provided by [`ext_memory_priority`]
    PhysicalDeviceMemoryPriorityFeaturesExt = 1000238000,

    /// Provided by [`ext_memory_priority`]
    MemoryPriorityAllocateInfoExt = 1000238001,

    /// Provided by [`khr_surface_protected_capabilities`]
    SurfaceProtectedCapabilitiesKhr = 1000239000,

    /// Provided by [`nv_dedicated_allocation_image_aliasing`]
    PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNv = 1000240000,

    /// Provided by [`ext_buffer_device_address`]
    PhysicalDeviceBufferDeviceAddressFeaturesExt = 1000244000,

    /// Provided by [`ext_buffer_device_address`]
    BufferDeviceAddressCreateInfoExt = 1000244002,

    /// Provided by [`ext_validation_features`]
    ValidationFeaturesExt = 1000247000,

    /// Provided by [`khr_present_wait`]
    PhysicalDevicePresentWaitFeaturesKhr = 1000248000,

    /// Provided by [`nv_cooperative_matrix`]
    PhysicalDeviceCooperativeMatrixFeaturesNv = 1000249000,

    /// Provided by [`nv_cooperative_matrix`]
    CooperativeMatrixPropertiesNv = 1000249001,

    /// Provided by [`nv_cooperative_matrix`]
    PhysicalDeviceCooperativeMatrixPropertiesNv = 1000249002,

    /// Provided by [`nv_coverage_reduction_mode`]
    PhysicalDeviceCoverageReductionModeFeaturesNv = 1000250000,

    /// Provided by [`nv_coverage_reduction_mode`]
    PipelineCoverageReductionStateCreateInfoNv = 1000250001,

    /// Provided by [`nv_coverage_reduction_mode`]
    FramebufferMixedSamplesCombinationNv = 1000250002,

    /// Provided by [`ext_fragment_shader_interlock`]
    PhysicalDeviceFragmentShaderInterlockFeaturesExt = 1000251000,

    /// Provided by [`ext_ycbcr_image_arrays`]
    PhysicalDeviceYcbcrImageArraysFeaturesExt = 1000252000,

    /// Provided by [`ext_provoking_vertex`]
    PhysicalDeviceProvokingVertexFeaturesExt = 1000254000,

    /// Provided by [`ext_provoking_vertex`]
    PipelineRasterizationProvokingVertexStateCreateInfoExt = 1000254001,

    /// Provided by [`ext_provoking_vertex`]
    PhysicalDeviceProvokingVertexPropertiesExt = 1000254002,

    /// Provided by [`ext_full_screen_exclusive`]
    SurfaceFullScreenExclusiveInfoExt = 1000255000,

    /// Provided by [`ext_full_screen_exclusive`]
    SurfaceCapabilitiesFullScreenExclusiveExt = 1000255002,

    /// Provided by [`khr_win32_surface`] with [`ext_full_screen_exclusive`]
    SurfaceFullScreenExclusiveWin32InfoExt = 1000255001,

    /// Provided by [`ext_headless_surface`]
    HeadlessSurfaceCreateInfoExt = 1000256000,

    /// Provided by [`ext_shader_atomic_float`]
    PhysicalDeviceShaderAtomicFloatFeaturesExt = 1000260000,

    /// Provided by [`ext_extended_dynamic_state`]
    PhysicalDeviceExtendedDynamicStateFeaturesExt = 1000267000,

    /// Provided by [`khr_pipeline_executable_properties`]
    PhysicalDevicePipelineExecutablePropertiesFeaturesKhr = 1000269000,

    /// Provided by [`khr_pipeline_executable_properties`]
    PipelineInfoKhr = 1000269001,

    /// Provided by [`khr_pipeline_executable_properties`]
    PipelineExecutablePropertiesKhr = 1000269002,

    /// Provided by [`khr_pipeline_executable_properties`]
    PipelineExecutableInfoKhr = 1000269003,

    /// Provided by [`khr_pipeline_executable_properties`]
    PipelineExecutableStatisticKhr = 1000269004,

    /// Provided by [`khr_pipeline_executable_properties`]
    PipelineExecutableInternalRepresentationKhr = 1000269005,

    /// Provided by [`ext_map_memory_placed`]
    PhysicalDeviceMapMemoryPlacedFeaturesExt = 1000272000,

    /// Provided by [`ext_map_memory_placed`]
    PhysicalDeviceMapMemoryPlacedPropertiesExt = 1000272001,

    /// Provided by [`ext_map_memory_placed`]
    MemoryMapPlacedInfoExt = 1000272002,

    /// Provided by [`ext_shader_atomic_float2`]
    PhysicalDeviceShaderAtomicFloat2FeaturesExt = 1000273000,

    /// Provided by [`ext_surface_maintenance1`]
    SurfacePresentModeExt = 1000274000,

    /// Provided by [`ext_surface_maintenance1`]
    SurfacePresentScalingCapabilitiesExt = 1000274001,

    /// Provided by [`ext_surface_maintenance1`]
    SurfacePresentModeCompatibilityExt = 1000274002,

    /// Provided by [`ext_swapchain_maintenance1`]
    PhysicalDeviceSwapchainMaintenance1FeaturesExt = 1000275000,

    /// Provided by [`ext_swapchain_maintenance1`]
    SwapchainPresentFenceInfoExt = 1000275001,

    /// Provided by [`ext_swapchain_maintenance1`]
    SwapchainPresentModesCreateInfoExt = 1000275002,

    /// Provided by [`ext_swapchain_maintenance1`]
    SwapchainPresentModeInfoExt = 1000275003,

    /// Provided by [`ext_swapchain_maintenance1`]
    SwapchainPresentScalingCreateInfoExt = 1000275004,

    /// Provided by [`ext_swapchain_maintenance1`]
    ReleaseSwapchainImagesInfoExt = 1000275005,

    /// Provided by [`nv_device_generated_commands`]
    PhysicalDeviceDeviceGeneratedCommandsPropertiesNv = 1000277000,

    /// Provided by [`nv_device_generated_commands`]
    GraphicsShaderGroupCreateInfoNv = 1000277001,

    /// Provided by [`nv_device_generated_commands`]
    GraphicsPipelineShaderGroupsCreateInfoNv = 1000277002,

    /// Provided by [`nv_device_generated_commands`]
    IndirectCommandsLayoutTokenNv = 1000277003,

    /// Provided by [`nv_device_generated_commands`]
    IndirectCommandsLayoutCreateInfoNv = 1000277004,

    /// Provided by [`nv_device_generated_commands`]
    GeneratedCommandsInfoNv = 1000277005,

    /// Provided by [`nv_device_generated_commands`]
    GeneratedCommandsMemoryRequirementsInfoNv = 1000277006,

    /// Provided by [`nv_device_generated_commands`]
    PhysicalDeviceDeviceGeneratedCommandsFeaturesNv = 1000277007,

    /// Provided by [`nv_inherited_viewport_scissor`]
    PhysicalDeviceInheritedViewportScissorFeaturesNv = 1000278000,

    /// Provided by [`nv_inherited_viewport_scissor`]
    CommandBufferInheritanceViewportScissorInfoNv = 1000278001,

    /// Provided by [`ext_texel_buffer_alignment`]
    PhysicalDeviceTexelBufferAlignmentFeaturesExt = 1000281000,

    /// Provided by [`qcom_render_pass_transform`]
    CommandBufferInheritanceRenderPassTransformInfoQcom = 1000282000,

    /// Provided by [`qcom_render_pass_transform`]
    RenderPassTransformBeginInfoQcom = 1000282001,

    /// Provided by [`ext_depth_bias_control`]
    PhysicalDeviceDepthBiasControlFeaturesExt = 1000283000,

    /// Provided by [`ext_depth_bias_control`]
    DepthBiasInfoExt = 1000283001,

    /// Provided by [`ext_depth_bias_control`]
    DepthBiasRepresentationInfoExt = 1000283002,

    /// Provided by [`ext_device_memory_report`]
    PhysicalDeviceDeviceMemoryReportFeaturesExt = 1000284000,

    /// Provided by [`ext_device_memory_report`]
    DeviceDeviceMemoryReportCreateInfoExt = 1000284001,

    /// Provided by [`ext_device_memory_report`]
    DeviceMemoryReportCallbackDataExt = 1000284002,

    /// Provided by [`ext_robustness2`]
    PhysicalDeviceRobustness2FeaturesExt = 1000286000,

    /// Provided by [`ext_robustness2`]
    PhysicalDeviceRobustness2PropertiesExt = 1000286001,

    /// Provided by [`ext_custom_border_color`]
    SamplerCustomBorderColorCreateInfoExt = 1000287000,

    /// Provided by [`ext_custom_border_color`]
    PhysicalDeviceCustomBorderColorPropertiesExt = 1000287001,

    /// Provided by [`ext_custom_border_color`]
    PhysicalDeviceCustomBorderColorFeaturesExt = 1000287002,

    /// Provided by [`khr_pipeline_library`]
    PipelineLibraryCreateInfoKhr = 1000290000,

    /// Provided by [`nv_present_barrier`]
    PhysicalDevicePresentBarrierFeaturesNv = 1000292000,

    /// Provided by [`nv_present_barrier`]
    SurfaceCapabilitiesPresentBarrierNv = 1000292001,

    /// Provided by [`nv_present_barrier`]
    SwapchainPresentBarrierCreateInfoNv = 1000292002,

    /// Provided by [`khr_present_id`]
    PresentIdKhr = 1000294000,

    /// Provided by [`khr_present_id`]
    PhysicalDevicePresentIdFeaturesKhr = 1000294001,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeInfoKhr = 1000299000,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeRateControlInfoKhr = 1000299001,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeRateControlLayerInfoKhr = 1000299002,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeCapabilitiesKhr = 1000299003,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeUsageInfoKhr = 1000299004,

    /// Provided by [`khr_video_encode_queue`]
    QueryPoolVideoEncodeFeedbackCreateInfoKhr = 1000299005,

    /// Provided by [`khr_video_encode_queue`]
    PhysicalDeviceVideoEncodeQualityLevelInfoKhr = 1000299006,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeQualityLevelPropertiesKhr = 1000299007,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeQualityLevelInfoKhr = 1000299008,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeSessionParametersGetInfoKhr = 1000299009,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeSessionParametersFeedbackInfoKhr = 1000299010,

    /// Provided by [`nv_device_diagnostics_config`]
    PhysicalDeviceDiagnosticsConfigFeaturesNv = 1000300000,

    /// Provided by [`nv_device_diagnostics_config`]
    DeviceDiagnosticsConfigCreateInfoNv = 1000300001,

    /// Provided by [`nv_cuda_kernel_launch`]
    CudaModuleCreateInfoNv = 1000307000,

    /// Provided by [`nv_cuda_kernel_launch`]
    CudaFunctionCreateInfoNv = 1000307001,

    /// Provided by [`nv_cuda_kernel_launch`]
    CudaLaunchInfoNv = 1000307002,

    /// Provided by [`nv_cuda_kernel_launch`]
    PhysicalDeviceCudaKernelLaunchFeaturesNv = 1000307003,

    /// Provided by [`nv_cuda_kernel_launch`]
    PhysicalDeviceCudaKernelLaunchPropertiesNv = 1000307004,

    /// Provided by [`nv_low_latency`]
    QueryLowLatencySupportNv = 1000310000,

    /// Provided by [`ext_metal_objects`]
    ExportMetalObjectCreateInfoExt = 1000311000,

    /// Provided by [`ext_metal_objects`]
    ExportMetalObjectsInfoExt = 1000311001,

    /// Provided by [`ext_metal_objects`]
    ExportMetalDeviceInfoExt = 1000311002,

    /// Provided by [`ext_metal_objects`]
    ExportMetalCommandQueueInfoExt = 1000311003,

    /// Provided by [`ext_metal_objects`]
    ExportMetalBufferInfoExt = 1000311004,

    /// Provided by [`ext_metal_objects`]
    ImportMetalBufferInfoExt = 1000311005,

    /// Provided by [`ext_metal_objects`]
    ExportMetalTextureInfoExt = 1000311006,

    /// Provided by [`ext_metal_objects`]
    ImportMetalTextureInfoExt = 1000311007,

    /// Provided by [`ext_metal_objects`]
    ExportMetalIoSurfaceInfoExt = 1000311008,

    /// Provided by [`ext_metal_objects`]
    ImportMetalIoSurfaceInfoExt = 1000311009,

    /// Provided by [`ext_metal_objects`]
    ExportMetalSharedEventInfoExt = 1000311010,

    /// Provided by [`ext_metal_objects`]
    ImportMetalSharedEventInfoExt = 1000311011,

    /// Provided by [`khr_synchronization2`] with [`nv_device_diagnostic_checkpoints`]
    QueueFamilyCheckpointProperties2Nv = 1000314008,

    /// Provided by [`khr_synchronization2`] with [`nv_device_diagnostic_checkpoints`]
    CheckpointData2Nv = 1000314009,

    /// Provided by [`ext_descriptor_buffer`]
    PhysicalDeviceDescriptorBufferPropertiesExt = 1000316000,

    /// Provided by [`ext_descriptor_buffer`]
    PhysicalDeviceDescriptorBufferDensityMapPropertiesExt = 1000316001,

    /// Provided by [`ext_descriptor_buffer`]
    PhysicalDeviceDescriptorBufferFeaturesExt = 1000316002,

    /// Provided by [`ext_descriptor_buffer`]
    DescriptorAddressInfoExt = 1000316003,

    /// Provided by [`ext_descriptor_buffer`]
    DescriptorGetInfoExt = 1000316004,

    /// Provided by [`ext_descriptor_buffer`]
    BufferCaptureDescriptorDataInfoExt = 1000316005,

    /// Provided by [`ext_descriptor_buffer`]
    ImageCaptureDescriptorDataInfoExt = 1000316006,

    /// Provided by [`ext_descriptor_buffer`]
    ImageViewCaptureDescriptorDataInfoExt = 1000316007,

    /// Provided by [`ext_descriptor_buffer`]
    SamplerCaptureDescriptorDataInfoExt = 1000316008,

    /// Provided by [`ext_descriptor_buffer`]
    OpaqueCaptureDescriptorDataCreateInfoExt = 1000316010,

    /// Provided by [`ext_descriptor_buffer`]
    DescriptorBufferBindingInfoExt = 1000316011,

    /// Provided by [`ext_descriptor_buffer`]
    DescriptorBufferBindingPushDescriptorBufferHandleExt = 1000316012,

    /// Provided by [`ext_descriptor_buffer`] with [`khr_acceleration_structure`] or [`nv_ray_tracing`]
    AccelerationStructureCaptureDescriptorDataInfoExt = 1000316009,

    /// Provided by [`ext_graphics_pipeline_library`]
    PhysicalDeviceGraphicsPipelineLibraryFeaturesExt = 1000320000,

    /// Provided by [`ext_graphics_pipeline_library`]
    PhysicalDeviceGraphicsPipelineLibraryPropertiesExt = 1000320001,

    /// Provided by [`ext_graphics_pipeline_library`]
    GraphicsPipelineLibraryCreateInfoExt = 1000320002,

    /// Provided by [`amd_shader_early_and_late_fragment_tests`]
    PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAmd = 1000321000,

    /// Provided by [`khr_fragment_shader_barycentric`]
    PhysicalDeviceFragmentShaderBarycentricFeaturesKhr = 1000203000,

    /// Provided by [`khr_fragment_shader_barycentric`]
    PhysicalDeviceFragmentShaderBarycentricPropertiesKhr = 1000322000,

    /// Provided by [`khr_shader_subgroup_uniform_control_flow`]
    PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKhr = 1000323000,

    /// Provided by [`nv_fragment_shading_rate_enums`]
    PhysicalDeviceFragmentShadingRateEnumsPropertiesNv = 1000326000,

    /// Provided by [`nv_fragment_shading_rate_enums`]
    PhysicalDeviceFragmentShadingRateEnumsFeaturesNv = 1000326001,

    /// Provided by [`nv_fragment_shading_rate_enums`]
    PipelineFragmentShadingRateEnumStateCreateInfoNv = 1000326002,

    /// Provided by [`nv_ray_tracing_motion_blur`]
    AccelerationStructureGeometryMotionTrianglesDataNv = 1000327000,

    /// Provided by [`nv_ray_tracing_motion_blur`]
    PhysicalDeviceRayTracingMotionBlurFeaturesNv = 1000327001,

    /// Provided by [`nv_ray_tracing_motion_blur`]
    AccelerationStructureMotionInfoNv = 1000327002,

    /// Provided by [`ext_mesh_shader`]
    PhysicalDeviceMeshShaderFeaturesExt = 1000328000,

    /// Provided by [`ext_mesh_shader`]
    PhysicalDeviceMeshShaderPropertiesExt = 1000328001,

    /// Provided by [`ext_ycbcr_2plane_444_formats`]
    PhysicalDeviceYcbcr2Plane444FormatsFeaturesExt = 1000330000,

    /// Provided by [`ext_fragment_density_map2`]
    PhysicalDeviceFragmentDensityMap2FeaturesExt = 1000332000,

    /// Provided by [`ext_fragment_density_map2`]
    PhysicalDeviceFragmentDensityMap2PropertiesExt = 1000332001,

    /// Provided by [`qcom_rotated_copy_commands`]
    CopyCommandTransformInfoQcom = 1000333000,

    /// Provided by [`khr_workgroup_memory_explicit_layout`]
    PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKhr = 1000336000,

    /// Provided by [`ext_image_compression_control`]
    PhysicalDeviceImageCompressionControlFeaturesExt = 1000338000,

    /// Provided by [`ext_image_compression_control`]
    ImageCompressionControlExt = 1000338001,

    /// Provided by [`ext_image_compression_control`]
    ImageCompressionPropertiesExt = 1000338004,

    /// Provided by [`ext_attachment_feedback_loop_layout`]
    PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesExt = 1000339000,

    /// Provided by [`ext_4444_formats`]
    PhysicalDevice4444FormatsFeaturesExt = 1000340000,

    /// Provided by [`ext_device_fault`]
    PhysicalDeviceFaultFeaturesExt = 1000341000,

    /// Provided by [`ext_device_fault`]
    DeviceFaultCountsExt = 1000341001,

    /// Provided by [`ext_device_fault`]
    DeviceFaultInfoExt = 1000341002,

    /// Provided by [`ext_rgba10x6_formats`]
    PhysicalDeviceRgba10X6FormatsFeaturesExt = 1000344000,

    /// Provided by [`ext_directfb_surface`]
    DirectfbSurfaceCreateInfoExt = 1000346000,

    /// Provided by [`ext_vertex_input_dynamic_state`]
    PhysicalDeviceVertexInputDynamicStateFeaturesExt = 1000352000,

    /// Provided by [`ext_shader_object`], [`ext_vertex_input_dynamic_state`]
    VertexInputBindingDescription2Ext = 1000352001,

    /// Provided by [`ext_shader_object`], [`ext_vertex_input_dynamic_state`]
    VertexInputAttributeDescription2Ext = 1000352002,

    /// Provided by [`ext_physical_device_drm`]
    PhysicalDeviceDrmPropertiesExt = 1000353000,

    /// Provided by [`ext_device_address_binding_report`]
    PhysicalDeviceAddressBindingReportFeaturesExt = 1000354000,

    /// Provided by [`ext_device_address_binding_report`]
    DeviceAddressBindingCallbackDataExt = 1000354001,

    /// Provided by [`ext_depth_clip_control`]
    PhysicalDeviceDepthClipControlFeaturesExt = 1000355000,

    /// Provided by [`ext_depth_clip_control`]
    PipelineViewportDepthClipControlCreateInfoExt = 1000355001,

    /// Provided by [`ext_primitive_topology_list_restart`]
    PhysicalDevicePrimitiveTopologyListRestartFeaturesExt = 1000356000,

    /// Provided by [`fuchsia_external_memory`]
    ImportMemoryZirconHandleInfoFuchsia = 1000364000,

    /// Provided by [`fuchsia_external_memory`]
    MemoryZirconHandlePropertiesFuchsia = 1000364001,

    /// Provided by [`fuchsia_external_memory`]
    MemoryGetZirconHandleInfoFuchsia = 1000364002,

    /// Provided by [`fuchsia_external_semaphore`]
    ImportSemaphoreZirconHandleInfoFuchsia = 1000365000,

    /// Provided by [`fuchsia_external_semaphore`]
    SemaphoreGetZirconHandleInfoFuchsia = 1000365001,

    /// Provided by [`fuchsia_buffer_collection`]
    BufferCollectionCreateInfoFuchsia = 1000366000,

    /// Provided by [`fuchsia_buffer_collection`]
    ImportMemoryBufferCollectionFuchsia = 1000366001,

    /// Provided by [`fuchsia_buffer_collection`]
    BufferCollectionImageCreateInfoFuchsia = 1000366002,

    /// Provided by [`fuchsia_buffer_collection`]
    BufferCollectionPropertiesFuchsia = 1000366003,

    /// Provided by [`fuchsia_buffer_collection`]
    BufferConstraintsInfoFuchsia = 1000366004,

    /// Provided by [`fuchsia_buffer_collection`]
    BufferCollectionBufferCreateInfoFuchsia = 1000366005,

    /// Provided by [`fuchsia_buffer_collection`]
    ImageConstraintsInfoFuchsia = 1000366006,

    /// Provided by [`fuchsia_buffer_collection`]
    ImageFormatConstraintsInfoFuchsia = 1000366007,

    /// Provided by [`fuchsia_buffer_collection`]
    SysmemColorSpaceFuchsia = 1000366008,

    /// Provided by [`fuchsia_buffer_collection`]
    BufferCollectionConstraintsInfoFuchsia = 1000366009,

    /// Provided by [`huawei_subpass_shading`]
    SubpassShadingPipelineCreateInfoHuawei = 1000369000,

    /// Provided by [`huawei_subpass_shading`]
    PhysicalDeviceSubpassShadingFeaturesHuawei = 1000369001,

    /// Provided by [`huawei_subpass_shading`]
    PhysicalDeviceSubpassShadingPropertiesHuawei = 1000369002,

    /// Provided by [`huawei_invocation_mask`]
    PhysicalDeviceInvocationMaskFeaturesHuawei = 1000370000,

    /// Provided by [`nv_external_memory_rdma`]
    MemoryGetRemoteAddressInfoNv = 1000371000,

    /// Provided by [`nv_external_memory_rdma`]
    PhysicalDeviceExternalMemoryRdmaFeaturesNv = 1000371001,

    /// Provided by [`ext_pipeline_properties`]
    PipelinePropertiesIdentifierExt = 1000372000,

    /// Provided by [`ext_pipeline_properties`]
    PhysicalDevicePipelinePropertiesFeaturesExt = 1000372001,

    /// Provided by [`ext_frame_boundary`]
    PhysicalDeviceFrameBoundaryFeaturesExt = 1000375000,

    /// Provided by [`ext_frame_boundary`]
    FrameBoundaryExt = 1000375001,

    /// Provided by [`ext_multisampled_render_to_single_sampled`]
    PhysicalDeviceMultisampledRenderToSingleSampledFeaturesExt = 1000376000,

    /// Provided by [`ext_multisampled_render_to_single_sampled`]
    SubpassResolvePerformanceQueryExt = 1000376001,

    /// Provided by [`ext_multisampled_render_to_single_sampled`]
    MultisampledRenderToSingleSampledInfoExt = 1000376002,

    /// Provided by [`ext_extended_dynamic_state2`]
    PhysicalDeviceExtendedDynamicState2FeaturesExt = 1000377000,

    /// Provided by [`qnx_screen_surface`]
    ScreenSurfaceCreateInfoQnx = 1000378000,

    /// Provided by [`ext_color_write_enable`]
    PhysicalDeviceColorWriteEnableFeaturesExt = 1000381000,

    /// Provided by [`ext_color_write_enable`]
    PipelineColorWriteCreateInfoExt = 1000381001,

    /// Provided by [`ext_primitives_generated_query`]
    PhysicalDevicePrimitivesGeneratedQueryFeaturesExt = 1000382000,

    /// Provided by [`khr_ray_tracing_maintenance1`]
    PhysicalDeviceRayTracingMaintenance1FeaturesKhr = 1000386000,

    /// Provided by [`ext_image_view_min_lod`]
    PhysicalDeviceImageViewMinLodFeaturesExt = 1000391000,

    /// Provided by [`ext_image_view_min_lod`]
    ImageViewMinLodCreateInfoExt = 1000391001,

    /// Provided by [`ext_multi_draw`]
    PhysicalDeviceMultiDrawFeaturesExt = 1000392000,

    /// Provided by [`ext_multi_draw`]
    PhysicalDeviceMultiDrawPropertiesExt = 1000392001,

    /// Provided by [`ext_image_2d_view_of_3d`]
    PhysicalDeviceImage2DViewOf3DFeaturesExt = 1000393000,

    /// Provided by [`ext_shader_tile_image`]
    PhysicalDeviceShaderTileImageFeaturesExt = 1000395000,

    /// Provided by [`ext_shader_tile_image`]
    PhysicalDeviceShaderTileImagePropertiesExt = 1000395001,

    /// Provided by [`ext_opacity_micromap`]
    MicromapBuildInfoExt = 1000396000,

    /// Provided by [`ext_opacity_micromap`]
    MicromapVersionInfoExt = 1000396001,

    /// Provided by [`ext_opacity_micromap`]
    CopyMicromapInfoExt = 1000396002,

    /// Provided by [`ext_opacity_micromap`]
    CopyMicromapToMemoryInfoExt = 1000396003,

    /// Provided by [`ext_opacity_micromap`]
    CopyMemoryToMicromapInfoExt = 1000396004,

    /// Provided by [`ext_opacity_micromap`]
    PhysicalDeviceOpacityMicromapFeaturesExt = 1000396005,

    /// Provided by [`ext_opacity_micromap`]
    PhysicalDeviceOpacityMicromapPropertiesExt = 1000396006,

    /// Provided by [`ext_opacity_micromap`]
    MicromapCreateInfoExt = 1000396007,

    /// Provided by [`ext_opacity_micromap`]
    MicromapBuildSizesInfoExt = 1000396008,

    /// Provided by [`ext_opacity_micromap`]
    AccelerationStructureTrianglesOpacityMicromapExt = 1000396009,

    /// Provided by [`nv_displacement_micromap`]
    PhysicalDeviceDisplacementMicromapFeaturesNv = 1000397000,

    /// Provided by [`nv_displacement_micromap`]
    PhysicalDeviceDisplacementMicromapPropertiesNv = 1000397001,

    /// Provided by [`nv_displacement_micromap`]
    AccelerationStructureTrianglesDisplacementMicromapNv = 1000397002,

    /// Provided by [`huawei_cluster_culling_shader`]
    PhysicalDeviceClusterCullingShaderFeaturesHuawei = 1000404000,

    /// Provided by [`huawei_cluster_culling_shader`]
    PhysicalDeviceClusterCullingShaderPropertiesHuawei = 1000404001,

    /// Provided by [`huawei_cluster_culling_shader`]
    PhysicalDeviceClusterCullingShaderVrsFeaturesHuawei = 1000404002,

    /// Provided by [`ext_border_color_swizzle`]
    PhysicalDeviceBorderColorSwizzleFeaturesExt = 1000411000,

    /// Provided by [`ext_border_color_swizzle`]
    SamplerBorderColorComponentMappingCreateInfoExt = 1000411001,

    /// Provided by [`ext_pageable_device_local_memory`]
    PhysicalDevicePageableDeviceLocalMemoryFeaturesExt = 1000412000,

    /// Provided by [`arm_shader_core_properties`]
    PhysicalDeviceShaderCorePropertiesArm = 1000415000,

    /// Provided by [`arm_scheduling_controls`]
    DeviceQueueShaderCoreControlCreateInfoArm = 1000417000,

    /// Provided by [`arm_scheduling_controls`]
    PhysicalDeviceSchedulingControlsFeaturesArm = 1000417001,

    /// Provided by [`arm_scheduling_controls`]
    PhysicalDeviceSchedulingControlsPropertiesArm = 1000417002,

    /// Provided by [`ext_image_sliced_view_of_3d`]
    PhysicalDeviceImageSlicedViewOf3DFeaturesExt = 1000418000,

    /// Provided by [`ext_image_sliced_view_of_3d`]
    ImageViewSlicedCreateInfoExt = 1000418001,

    /// Provided by [`valve_descriptor_set_host_mapping`]
    PhysicalDeviceDescriptorSetHostMappingFeaturesValve = 1000420000,

    /// Provided by [`valve_descriptor_set_host_mapping`]
    DescriptorSetBindingReferenceValve = 1000420001,

    /// Provided by [`valve_descriptor_set_host_mapping`]
    DescriptorSetLayoutHostMappingInfoValve = 1000420002,

    /// Provided by [`ext_depth_clamp_zero_one`]
    PhysicalDeviceDepthClampZeroOneFeaturesExt = 1000421000,

    /// Provided by [`ext_non_seamless_cube_map`]
    PhysicalDeviceNonSeamlessCubeMapFeaturesExt = 1000422000,

    /// Provided by [`arm_render_pass_striped`]
    PhysicalDeviceRenderPassStripedFeaturesArm = 1000424000,

    /// Provided by [`arm_render_pass_striped`]
    PhysicalDeviceRenderPassStripedPropertiesArm = 1000424001,

    /// Provided by [`arm_render_pass_striped`]
    RenderPassStripeBeginInfoArm = 1000424002,

    /// Provided by [`arm_render_pass_striped`]
    RenderPassStripeInfoArm = 1000424003,

    /// Provided by [`arm_render_pass_striped`]
    RenderPassStripeSubmitInfoArm = 1000424004,

    /// Provided by [`qcom_fragment_density_map_offset`]
    PhysicalDeviceFragmentDensityMapOffsetFeaturesQcom = 1000425000,

    /// Provided by [`qcom_fragment_density_map_offset`]
    PhysicalDeviceFragmentDensityMapOffsetPropertiesQcom = 1000425001,

    /// Provided by [`qcom_fragment_density_map_offset`]
    SubpassFragmentDensityMapOffsetEndInfoQcom = 1000425002,

    /// Provided by [`nv_copy_memory_indirect`]
    PhysicalDeviceCopyMemoryIndirectFeaturesNv = 1000426000,

    /// Provided by [`nv_copy_memory_indirect`]
    PhysicalDeviceCopyMemoryIndirectPropertiesNv = 1000426001,

    /// Provided by [`nv_memory_decompression`]
    PhysicalDeviceMemoryDecompressionFeaturesNv = 1000427000,

    /// Provided by [`nv_memory_decompression`]
    PhysicalDeviceMemoryDecompressionPropertiesNv = 1000427001,

    /// Provided by [`nv_device_generated_commands_compute`]
    PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNv = 1000428000,

    /// Provided by [`nv_device_generated_commands_compute`]
    ComputePipelineIndirectBufferInfoNv = 1000428001,

    /// Provided by [`nv_device_generated_commands_compute`]
    PipelineIndirectDeviceAddressInfoNv = 1000428002,

    /// Provided by [`nv_linear_color_attachment`]
    PhysicalDeviceLinearColorAttachmentFeaturesNv = 1000430000,

    /// Provided by [`khr_shader_maximal_reconvergence`]
    PhysicalDeviceShaderMaximalReconvergenceFeaturesKhr = 1000434000,

    /// Provided by [`ext_image_compression_control_swapchain`]
    PhysicalDeviceImageCompressionControlSwapchainFeaturesExt = 1000437000,

    /// Provided by [`qcom_image_processing`]
    PhysicalDeviceImageProcessingFeaturesQcom = 1000440000,

    /// Provided by [`qcom_image_processing`]
    PhysicalDeviceImageProcessingPropertiesQcom = 1000440001,

    /// Provided by [`qcom_image_processing`]
    ImageViewSampleWeightCreateInfoQcom = 1000440002,

    /// Provided by [`ext_nested_command_buffer`]
    PhysicalDeviceNestedCommandBufferFeaturesExt = 1000451000,

    /// Provided by [`ext_nested_command_buffer`]
    PhysicalDeviceNestedCommandBufferPropertiesExt = 1000451001,

    /// Provided by [`ext_external_memory_acquire_unmodified`]
    ExternalMemoryAcquireUnmodifiedExt = 1000453000,

    /// Provided by [`ext_extended_dynamic_state3`]
    PhysicalDeviceExtendedDynamicState3FeaturesExt = 1000455000,

    /// Provided by [`ext_extended_dynamic_state3`]
    PhysicalDeviceExtendedDynamicState3PropertiesExt = 1000455001,

    /// Provided by [`ext_subpass_merge_feedback`]
    PhysicalDeviceSubpassMergeFeedbackFeaturesExt = 1000458000,

    /// Provided by [`ext_subpass_merge_feedback`]
    RenderPassCreationControlExt = 1000458001,

    /// Provided by [`ext_subpass_merge_feedback`]
    RenderPassCreationFeedbackCreateInfoExt = 1000458002,

    /// Provided by [`ext_subpass_merge_feedback`]
    RenderPassSubpassFeedbackCreateInfoExt = 1000458003,

    /// Provided by [`lunarg_direct_driver_loading`]
    DirectDriverLoadingInfoLunarG = 1000459000,

    /// Provided by [`lunarg_direct_driver_loading`]
    DirectDriverLoadingListLunarG = 1000459001,

    /// Provided by [`ext_shader_module_identifier`]
    PhysicalDeviceShaderModuleIdentifierFeaturesExt = 1000462000,

    /// Provided by [`ext_shader_module_identifier`]
    PhysicalDeviceShaderModuleIdentifierPropertiesExt = 1000462001,

    /// Provided by [`ext_shader_module_identifier`]
    PipelineShaderStageModuleIdentifierCreateInfoExt = 1000462002,

    /// Provided by [`ext_shader_module_identifier`]
    ShaderModuleIdentifierExt = 1000462003,

    /// Provided by [`ext_rasterization_order_attachment_access`]
    PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesExt = 1000342000,

    /// Provided by [`nv_optical_flow`]
    PhysicalDeviceOpticalFlowFeaturesNv = 1000464000,

    /// Provided by [`nv_optical_flow`]
    PhysicalDeviceOpticalFlowPropertiesNv = 1000464001,

    /// Provided by [`nv_optical_flow`]
    OpticalFlowImageFormatInfoNv = 1000464002,

    /// Provided by [`nv_optical_flow`]
    OpticalFlowImageFormatPropertiesNv = 1000464003,

    /// Provided by [`nv_optical_flow`]
    OpticalFlowSessionCreateInfoNv = 1000464004,

    /// Provided by [`nv_optical_flow`]
    OpticalFlowExecuteInfoNv = 1000464005,

    /// Provided by [`nv_optical_flow`]
    OpticalFlowSessionCreatePrivateDataInfoNv = 1000464010,

    /// Provided by [`ext_legacy_dithering`]
    PhysicalDeviceLegacyDitheringFeaturesExt = 1000465000,

    /// Provided by [`android_external_format_resolve`]
    PhysicalDeviceExternalFormatResolveFeaturesAndroid = 1000468000,

    /// Provided by [`android_external_format_resolve`]
    PhysicalDeviceExternalFormatResolvePropertiesAndroid = 1000468001,

    /// Provided by [`android_external_format_resolve`]
    AndroidHardwareBufferFormatResolvePropertiesAndroid = 1000468002,

    /// Provided by [`khr_ray_tracing_position_fetch`]
    PhysicalDeviceRayTracingPositionFetchFeaturesKhr = 1000481000,

    /// Provided by [`ext_shader_object`]
    PhysicalDeviceShaderObjectFeaturesExt = 1000482000,

    /// Provided by [`ext_shader_object`]
    PhysicalDeviceShaderObjectPropertiesExt = 1000482001,

    /// Provided by [`ext_shader_object`]
    ShaderCreateInfoExt = 1000482002,

    /// Provided by [`qcom_tile_properties`]
    PhysicalDeviceTilePropertiesFeaturesQcom = 1000484000,

    /// Provided by [`qcom_tile_properties`]
    TilePropertiesQcom = 1000484001,

    /// Provided by [`secamigo_profiling`]
    PhysicalDeviceAmigoProfilingFeaturesSec = 1000485000,

    /// Provided by [`secamigo_profiling`]
    AmigoProfilingSubmitInfoSec = 1000485001,

    /// Provided by [`qcom_multiview_per_view_viewports`]
    PhysicalDeviceMultiviewPerViewViewportsFeaturesQcom = 1000488000,

    /// Provided by [`nv_ray_tracing_invocation_reorder`]
    PhysicalDeviceRayTracingInvocationReorderFeaturesNv = 1000490000,

    /// Provided by [`nv_ray_tracing_invocation_reorder`]
    PhysicalDeviceRayTracingInvocationReorderPropertiesNv = 1000490001,

    /// Provided by [`nv_extended_sparse_address_space`]
    PhysicalDeviceExtendedSparseAddressSpaceFeaturesNv = 1000492000,

    /// Provided by [`nv_extended_sparse_address_space`]
    PhysicalDeviceExtendedSparseAddressSpacePropertiesNv = 1000492001,

    /// Provided by [`ext_mutable_descriptor_type`]
    PhysicalDeviceMutableDescriptorTypeFeaturesExt = 1000351000,

    /// Provided by [`ext_mutable_descriptor_type`]
    MutableDescriptorTypeCreateInfoExt = 1000351002,

    /// Provided by [`ext_legacy_vertex_attributes`]
    PhysicalDeviceLegacyVertexAttributesFeaturesExt = 1000495000,

    /// Provided by [`ext_legacy_vertex_attributes`]
    PhysicalDeviceLegacyVertexAttributesPropertiesExt = 1000495001,

    /// Provided by [`ext_layer_settings`]
    LayerSettingsCreateInfoExt = 1000496000,

    /// Provided by [`arm_shader_core_builtins`]
    PhysicalDeviceShaderCoreBuiltinsFeaturesArm = 1000497000,

    /// Provided by [`arm_shader_core_builtins`]
    PhysicalDeviceShaderCoreBuiltinsPropertiesArm = 1000497001,

    /// Provided by [`ext_pipeline_library_group_handles`]
    PhysicalDevicePipelineLibraryGroupHandlesFeaturesExt = 1000498000,

    /// Provided by [`ext_dynamic_rendering_unused_attachments`]
    PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesExt = 1000499000,

    /// Provided by [`nv_low_latency2`]
    LatencySleepModeInfoNv = 1000505000,

    /// Provided by [`nv_low_latency2`]
    LatencySleepInfoNv = 1000505001,

    /// Provided by [`nv_low_latency2`]
    SetLatencyMarkerInfoNv = 1000505002,

    /// Provided by [`nv_low_latency2`]
    GetLatencyMarkerInfoNv = 1000505003,

    /// Provided by [`nv_low_latency2`]
    LatencyTimingsFrameReportNv = 1000505004,

    /// Provided by [`nv_low_latency2`]
    LatencySubmissionPresentIdNv = 1000505005,

    /// Provided by [`nv_low_latency2`]
    OutOfBandQueueTypeInfoNv = 1000505006,

    /// Provided by [`nv_low_latency2`]
    SwapchainLatencyCreateInfoNv = 1000505007,

    /// Provided by [`nv_low_latency2`]
    LatencySurfaceCapabilitiesNv = 1000505008,

    /// Provided by [`khr_cooperative_matrix`]
    PhysicalDeviceCooperativeMatrixFeaturesKhr = 1000506000,

    /// Provided by [`khr_cooperative_matrix`]
    CooperativeMatrixPropertiesKhr = 1000506001,

    /// Provided by [`khr_cooperative_matrix`]
    PhysicalDeviceCooperativeMatrixPropertiesKhr = 1000506002,

    /// Provided by [`qcom_multiview_per_view_render_areas`]
    PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQcom = 1000510000,

    /// Provided by [`qcom_multiview_per_view_render_areas`]
    MultiviewPerViewRenderAreasRenderPassBeginInfoQcom = 1000510001,

    /// Provided by [`khr_video_decode_av1`]
    VideoDecodeAv1CapabilitiesKhr = 1000512000,

    /// Provided by [`khr_video_decode_av1`]
    VideoDecodeAv1PictureInfoKhr = 1000512001,

    /// Provided by [`khr_video_decode_av1`]
    VideoDecodeAv1ProfileInfoKhr = 1000512003,

    /// Provided by [`khr_video_decode_av1`]
    VideoDecodeAv1SessionParametersCreateInfoKhr = 1000512004,

    /// Provided by [`khr_video_decode_av1`]
    VideoDecodeAv1DpbSlotInfoKhr = 1000512005,

    /// Provided by [`khr_video_maintenance1`]
    PhysicalDeviceVideoMaintenance1FeaturesKhr = 1000515000,

    /// Provided by [`khr_video_maintenance1`]
    VideoInlineQueryInfoKhr = 1000515001,

    /// Provided by [`nv_per_stage_descriptor_set`]
    PhysicalDevicePerStageDescriptorSetFeaturesNv = 1000516000,

    /// Provided by [`qcom_image_processing2`]
    PhysicalDeviceImageProcessing2FeaturesQcom = 1000518000,

    /// Provided by [`qcom_image_processing2`]
    PhysicalDeviceImageProcessing2PropertiesQcom = 1000518001,

    /// Provided by [`qcom_image_processing2`]
    SamplerBlockMatchWindowCreateInfoQcom = 1000518002,

    /// Provided by [`qcom_filter_cubic_weights`]
    SamplerCubicWeightsCreateInfoQcom = 1000519000,

    /// Provided by [`qcom_filter_cubic_weights`]
    PhysicalDeviceCubicWeightsFeaturesQcom = 1000519001,

    /// Provided by [`qcom_filter_cubic_weights`]
    BlitImageCubicWeightsInfoQcom = 1000519002,

    /// Provided by [`qcom_ycbcr_degamma`]
    PhysicalDeviceYcbcrDegammaFeaturesQcom = 1000520000,

    /// Provided by [`qcom_ycbcr_degamma`]
    SamplerYcbcrConversionYcbcrDegammaCreateInfoQcom = 1000520001,

    /// Provided by [`qcom_filter_cubic_clamp`]
    PhysicalDeviceCubicClampFeaturesQcom = 1000521000,

    /// Provided by [`ext_attachment_feedback_loop_dynamic_state`]
    PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesExt = 1000524000,

    /// Provided by [`qnx_external_memory_screen_buffer`]
    ScreenBufferPropertiesQnx = 1000529000,

    /// Provided by [`qnx_external_memory_screen_buffer`]
    ScreenBufferFormatPropertiesQnx = 1000529001,

    /// Provided by [`qnx_external_memory_screen_buffer`]
    ImportScreenBufferInfoQnx = 1000529002,

    /// Provided by [`qnx_external_memory_screen_buffer`]
    ExternalFormatQnx = 1000529003,

    /// Provided by [`qnx_external_memory_screen_buffer`]
    PhysicalDeviceExternalMemoryScreenBufferFeaturesQnx = 1000529004,

    /// Provided by [`msft_layered_driver`]
    PhysicalDeviceLayeredDriverPropertiesMsft = 1000530000,

    /// Provided by [`khr_calibrated_timestamps`]
    CalibratedTimestampInfoKhr = 1000184000,

    /// Provided by [`khr_maintenance6`] with [`ext_descriptor_buffer`]
    SetDescriptorBufferOffsetsInfoExt = 1000545007,

    /// Provided by [`khr_maintenance6`] with [`ext_descriptor_buffer`]
    BindDescriptorBufferEmbeddedSamplersInfoExt = 1000545008,

    /// Provided by [`nv_descriptor_pool_overallocation`]
    PhysicalDeviceDescriptorPoolOverallocationFeaturesNv = 1000546000,

    /// Provided by [`nv_raw_access_chains`]
    PhysicalDeviceRawAccessChainsFeaturesNv = 1000555000,

    /// Provided by [`nv_shader_atomic_float16_vector`]
    PhysicalDeviceShaderAtomicFloat16VectorFeaturesNv = 1000563000,

    /// Provided by [`ext_shader_replicated_composites`]
    PhysicalDeviceShaderReplicatedCompositesFeaturesExt = 1000564000,

    /// Provided by [`nv_ray_tracing_validation`]
    PhysicalDeviceRayTracingValidationFeaturesNv = 1000568000,

    /// Provided by [`mesa_image_alignment_control`]
    PhysicalDeviceImageAlignmentControlFeaturesMesa = 1000575000,

    /// Provided by [`mesa_image_alignment_control`]
    PhysicalDeviceImageAlignmentControlPropertiesMesa = 1000575001,

    /// Provided by [`mesa_image_alignment_control`]
    ImageAlignmentControlCreateInfoMesa = 1000575002,
}
