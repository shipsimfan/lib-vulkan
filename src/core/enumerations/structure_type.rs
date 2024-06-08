// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_1, VK_VERSION_1_2, VK_VERSION_1_3};

/// Vulkan structure types
///
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
    PhysicalDeviceEXTernalImageFormatInfo = 1000071000,

    /// Provided by [`VK_VERSION_1_1`]
    EXTernalImageFormatProperties = 1000071001,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceEXTernalBufferInfo = 1000071002,

    /// Provided by [`VK_VERSION_1_1`]
    EXTernalBufferProperties = 1000071003,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceIdProperties = 1000071004,

    /// Provided by [`VK_VERSION_1_1`]
    EXTernalMemoryBufferCreateInfo = 1000072000,

    /// Provided by [`VK_VERSION_1_1`]
    EXTernalMemoryImageCreateInfo = 1000072001,

    /// Provided by [`VK_VERSION_1_1`]
    ExportMemoryAllocateInfo = 1000072002,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceEXTernalFenceInfo = 1000112000,

    /// Provided by [`VK_VERSION_1_1`]
    EXTernalFenceProperties = 1000112001,

    /// Provided by [`VK_VERSION_1_1`]
    ExportFenceCreateInfo = 1000113000,

    /// Provided by [`VK_VERSION_1_1`]
    ExportSemaphoreCreateInfo = 1000077000,

    /// Provided by [`VK_VERSION_1_1`]
    PhysicalDeviceEXTernalSemaphoreInfo = 1000076000,

    /// Provided by [`VK_VERSION_1_1`]
    EXTernalSemaphoreProperties = 1000076001,

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
    PhysicalDeviceShaderSubgroupEXTendedTypesFeatures = 1000175000,

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

    /// Provided by [`VK_KHR_SWAPCHAIN`]
    SwapchainCreateInfoKHR = 1000001000,

    /// Provided by [`VK_KHR_SWAPCHAIN`]
    PresentInfoKHR = 1000001001,

    /// Provided by [`VK_VERSION_1_1`] with [`VK_KHR_SWAPCHAIN`], [`VK_KHR_DEVICE_GROUP`] with [`VK_KHR_SURFACE`]
    DeviceGroupPresentCapabilitiesKHR = 1000060007,

    /// Provided by [`VK_VERSION_1_1`] with [`VK_KHR_SWAPCHAIN`], [`VK_KHR_DEVICE_GROUP`] with [`VK_KHR_SWAPCHAIN`]
    ImageSwapchainCreateInfoKHR = 1000060008,

    /// Provided by [`VK_VERSION_1_1`] with [`VK_KHR_SWAPCHAIN`], [`VK_KHR_DEVICE_GROUP`] with [`VK_KHR_SWAPCHAIN`]
    BindImageMemorySwapchainInfoKHR = 1000060009,

    /// Provided by [`VK_VERSION_1_1`] with [`VK_KHR_SWAPCHAIN`], [`VK_KHR_DEVICE_GROUP`] with [`VK_KHR_SWAPCHAIN`]
    AcquireNextImageInfoKHR = 1000060010,

    /// Provided by [`VK_VERSION_1_1`] with [`VK_KHR_SWAPCHAIN`], [`VK_KHR_DEVICE_GROUP`] with [`VK_KHR_SWAPCHAIN`]
    DeviceGroupPresentInfoKHR = 1000060011,

    /// Provided by [`VK_VERSION_1_1`] with [`VK_KHR_SWAPCHAIN`], [`VK_KHR_DEVICE_GROUP`] with [`VK_KHR_SWAPCHAIN`]
    DeviceGroupSwapchainCreateInfoKHR = 1000060012,

    /// Provided by [`VK_KHR_DISPLAY`]
    DisplayModeCreateInfoKHR = 1000002000,

    /// Provided by [`VK_KHR_DISPLAY`]
    DisplaySurfaceCreateInfoKHR = 1000002001,

    /// Provided by [`VK_KHR_DISPLAY_SWAPCHAIN`]
    DisplayPresentInfoKHR = 1000003000,

    /// Provided by [`VK_KHR_XLIB_SURFACE`]
    XlibSurfaceCreateInfoKHR = 1000004000,

    /// Provided by [`VK_KHR_XCB_SURFACE`]
    XcbSurfaceCreateInfoKHR = 1000005000,

    /// Provided by [`VK_KHR_WAYLAND_SURFACE`]
    WaylandSurfaceCreateInfoKHR = 1000006000,

    /// Provided by [`VK_KHR_ANDROID_SURFACE`]
    AndroidSurfaceCreateInfoKHR = 1000008000,

    /// Provided by [`VK_KHR_WIN32_SURFACE`]
    Win32SurfaceCreateInfoKHR = 1000009000,

    /// Provided by [`VK_EXT_DEBUG_REPORT`]
    DebugReportCallbackCreateInfoEXT = 1000011000,

    /// Provided by [`VK_AMD_RASERIZATION_ORDER`]
    PipelineRasterizationStateRasterizationOrderAMD = 1000018000,

    /// Provided by [`VK_EXT_DEBUG_MARKER`]
    DebugMarkerObjectNameInfoEXT = 1000022000,

    /// Provided by [`VK_EXT_DEBUG_MARKER`]
    DebugMarkerObjectTagInfoEXT = 1000022001,

    /// Provided by [`VK_EXT_DEBUG_MARKER`]
    DebugMarkerMarkerInfoEXT = 1000022002,

    /// Provided by [`VK_KHR_VIDEO_QUEUE`]
    VideoProfileInfoKHR = 1000023000,

    /// Provided by [`VK_KHR_VIDEO_QUEUE`]
    VideoCapabilitiesKHR = 1000023001,

    /// Provided by [`VK_KHR_VIDEO_QUEUE`]
    VideoPictureResourceInfoKHR = 1000023002,

    /// Provided by [`VK_KHR_VIDEO_QUEUE`]
    VideoSessionMemoryRequirementsKHR = 1000023003,

    /// Provided by [`VK_KHR_VIDEO_QUEUE`]
    BindVideoSessionMemoryInfoKHR = 1000023004,

    /// Provided by [`VK_KHR_VIDEO_QUEUE`]
    VideoSessionCreateInfoKHR = 1000023005,

    /// Provided by [`VK_KHR_VIDEO_QUEUE`]
    VideoSessionParametersCreateInfoKHR = 1000023006,

    /// Provided by [`VK_KHR_VIDEO_QUEUE`]
    VideoSessionParametersUpdateInfoKHR = 1000023007,

    /// Provided by [`VK_KHR_VIDEO_QUEUE`]
    VideoBeginCodingInfoKHR = 1000023008,

    /// Provided by [`VK_KHR_VIDEO_QUEUE`]
    VideoEndCodingInfoKHR = 1000023009,

    /// Provided by [`VK_KHR_VIDEO_QUEUE`]
    VideoCodingControlInfoKHR = 1000023010,

    /// Provided by [`VK_KHR_VIDEO_QUEUE`]
    VideoReferenceSlotInfoKHR = 1000023011,

    /// Provided by [`VK_KHR_VIDEO_QUEUE`]
    QueueFamilyVideoPropertiesKHR = 1000023012,

    /// Provided by [`VK_KHR_VIDEO_QUEUE`]
    VideoProfileListInfoKHR = 1000023013,

    /// Provided by [`VK_KHR_VIDEO_QUEUE`]
    PhysicalDeviceVideoFormatInfoKHR = 1000023014,

    /// Provided by [`VK_KHR_VIDEO_QUEUE`]
    VideoFormatPropertiesKHR = 1000023015,

    /// Provided by [`VK_KHR_VIDEO_QUEUE`]
    QueueFamilyQueryResultStatusPropertiesKHR = 1000023016,

    /// Provided by [`VK_KHR_VIDEO_DECODE_QUEUE`]
    VideoDecodeInfoKHR = 1000024000,

    /// Provided by [`VK_KHR_VIDEO_DECODE_QUEUE`]
    VideoDecodeCapabilitiesKHR = 1000024001,

    /// Provided by [`VK_KHR_VIDEO_DECODE_QUEUE`]
    VideoDecodeUsageInfoKHR = 1000024002,

    /// Provided by [`VK_NV_DEDICATED_ALLOCATION`]
    DedicatedAllocationImageCreateInfoNV = 1000026000,

    /// Provided by [`VK_NV_DEDICATED_ALLOCATION`]
    DedicatedAllocationBufferCreateInfoNV = 1000026001,

    /// Provided by [`VK_NV_DEDICATED_ALLOCATION`]
    DedicatedAllocationMemoryAllocateInfoNV = 1000026002,

    /// Provided by [`VK_EXT_TRANSFORM_FEEDBACK`]
    PhysicalDeviceTransformFeedbackFeaturesEXT = 1000028000,

    /// Provided by [`VK_EXT_TRANSFORM_FEEDBACK`]
    PhysicalDeviceTransformFeedbackPropertiesEXT = 1000028001,

    /// Provided by [`VK_EXT_TRANSFORM_FEEDBACK`]
    PipelineRasterizationStateStreamCreateInfoEXT = 1000028002,

    /// Provided by [`VK_NVX_BINARY_IMPORT`]
    CuModuleCreateInfoNVx = 1000029000,

    /// Provided by [`VK_NVX_BINARY_IMPORT`]
    CuFunctionCreateInfoNVx = 1000029001,

    /// Provided by [`VK_NVX_BINARY_IMPORT`]
    CuLaunchInfoNVx = 1000029002,

    /// Provided by [`VK_NVX_IMAGE_VIEW_HANDLE`]
    ImageViewHandleInfoNVx = 1000030000,

    /// Provided by [`VK_NVX_IMAGE_VIEW_HANDLE`]
    ImageViewAddressPropertiesNVx = 1000030001,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H264`]
    VideoEncodeH264CapabilitiesKHR = 1000038000,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H264`]
    VideoEncodeH264SessionParametersCreateInfoKHR = 1000038001,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H264`]
    VideoEncodeH264SessionParametersAddInfoKHR = 1000038002,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H264`]
    VideoEncodeH264PictureInfoKHR = 1000038003,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H264`]
    VideoEncodeH264DpbSlotInfoKHR = 1000038004,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H264`]
    VideoEncodeH264NaluSliceInfoKHR = 1000038005,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H264`]
    VideoEncodeH264GopRemainingFrameInfoKHR = 1000038006,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H264`]
    VideoEncodeH264ProfileInfoKHR = 1000038007,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H264`]
    VideoEncodeH264RateControlInfoKHR = 1000038008,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H264`]
    VideoEncodeH264RateControlLayerInfoKHR = 1000038009,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H264`]
    VideoEncodeH264SessionCreateInfoKHR = 1000038010,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H264`]
    VideoEncodeH264QualityLevelPropertiesKHR = 1000038011,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H264`]
    VideoEncodeH264SessionParametersGetInfoKHR = 1000038012,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H264`]
    VideoEncodeH264SessionParametersFeedbackInfoKHR = 1000038013,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H265`]
    VideoEncodeH265CapabilitiesKHR = 1000039000,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H265`]
    VideoEncodeH265SessionParametersCreateInfoKHR = 1000039001,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H265`]
    VideoEncodeH265SessionParametersAddInfoKHR = 1000039002,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H265`]
    VideoEncodeH265PictureInfoKHR = 1000039003,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H265`]
    VideoEncodeH265DpbSlotInfoKHR = 1000039004,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H265`]
    VideoEncodeH265NaluSliceSegmentInfoKHR = 1000039005,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H265`]
    VideoEncodeH265GopRemainingFrameInfoKHR = 1000039006,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H265`]
    VideoEncodeH265ProfileInfoKHR = 1000039007,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H265`]
    VideoEncodeH265RateControlInfoKHR = 1000039009,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H265`]
    VideoEncodeH265RateControlLayerInfoKHR = 1000039010,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H265`]
    VideoEncodeH265SessionCreateInfoKHR = 1000039011,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H265`]
    VideoEncodeH265QualityLevelPropertiesKHR = 1000039012,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H265`]
    VideoEncodeH265SessionParametersGetInfoKHR = 1000039013,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_H265`]
    VideoEncodeH265SessionParametersFeedbackInfoKHR = 1000039014,

    /// Provided by [`VK_KHR_VIDEO_DECODE_H264`]
    VideoDecodeH264CapabilitiesKHR = 1000040000,

    /// Provided by [`VK_KHR_VIDEO_DECODE_H264`]
    VideoDecodeH264PictureInfoKHR = 1000040001,

    /// Provided by [`VK_KHR_VIDEO_DECODE_H264`]
    VideoDecodeH264ProfileInfoKHR = 1000040003,

    /// Provided by [`VK_KHR_VIDEO_DECODE_H264`]
    VideoDecodeH264SessionParametersCreateInfoKHR = 1000040004,

    /// Provided by [`VK_KHR_VIDEO_DECODE_H264`]
    VideoDecodeH264SessionParametersAddInfoKHR = 1000040005,

    /// Provided by [`VK_KHR_VIDEO_DECODE_H264`]
    VideoDecodeH264DpbSlotInfoKHR = 1000040006,

    /// Provided by [`VkAMDtexture_GATHER_BIAS_LOD`]
    TextureLodGatherFormatPropertiesAMD = 1000041000,

    /// Provided by [`VK_KHR_DYNAMIC_RENDERING`] with [`VK_KHR_FRAGMENT_SHADING_RATE`]
    RenderingFragmentShadingRateAttachmentInfoKHR = 1000044006,

    /// Provided by [`VK_KHR_DYNAMIC_RENDERING`] with [`VK_EXT_FRAGMENT_DENSITY_MAP`]
    RenderingFragmentDensityMapAttachmentInfoEXT = 1000044007,

    /// Provided by [`VK_KHR_DYNAMIC_RENDERING`] with [`VK_AMD_MIXED_ATTACHMENT_SAMPLES`]
    AttachmentSampleCountInfoAMD = 1000044008,

    /// Provided by [`VK_KHR_DYNAMIC_RENDERING`] with [`VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES`]
    MultiviewPerViewAttributesInfoNVx = 1000044009,

    /// Provided by [`VkGgpstream_DESCRIPTOR_SURFACE`]
    StreamDescriptorSurfaceCreateInfoGgp = 1000049000,

    /// Provided by [`VK_NV_CORNER_SAMPLED_IMAGE`]
    PhysicalDeviceCornerSampledImageFeaturesNV = 1000050000,

    /// Provided by [`VK_NV_EXTERNAL_MEMORY`]
    EXTernalMemoryImageCreateInfoNV = 1000056000,

    /// Provided by [`VK_NV_EXTERNAL_MEMORY`]
    ExportMemoryAllocateInfoNV = 1000056001,

    /// Provided by [`VK_NV_EXTERNAL_MEMORY_WIN32`]
    ImportMemoryWin32HandleInfoNV = 1000057000,

    /// Provided by [`VK_NV_EXTERNAL_MEMORY_WIN32`]
    ExportMemoryWin32HandleInfoNV = 1000057001,

    /// Provided by [`VK_NV_WIN32_KEYED_MUTEX`]
    Win32KeyedMutexAcquireReleaseInfoNV = 1000058000,

    /// Provided by [`VK_EXT_VALIDATION_FLAGS`]
    ValidationFlagsEXT = 1000061000,

    /// Provided by [`VkNnvi_SURFACE`]
    ViSurfaceCreateInfoNn = 1000062000,

    /// Provided by [`VK_EXT_ASTC_DECODE_MODE`]
    ImageViewAstcDecodeModeEXT = 1000067000,

    /// Provided by [`VK_EXT_ASTC_DECODE_MODE`]
    PhysicalDeviceAstcDecodeFeaturesEXT = 1000067001,

    /// Provided by [`VK_EXT_PIPELINE_ROBUSTNESS`]
    PipelineRobustnessCreateInfoEXT = 1000068000,

    /// Provided by [`VK_EXT_PIPELINE_ROBUSTNESS`]
    PhysicalDevicePipelineRobustnessFeaturesEXT = 1000068001,

    /// Provided by [`VK_EXT_PIPELINE_ROBUSTNESS`]
    PhysicalDevicePipelineRobustnessPropertiesEXT = 1000068002,

    /// Provided by [`VK_KHR_EXTERNAL_MEMORY_WIN32`]
    ImportMemoryWin32HandleInfoKHR = 1000073000,

    /// Provided by [`VK_KHR_EXTERNAL_MEMORY_WIN32`]
    ExportMemoryWin32HandleInfoKHR = 1000073001,

    /// Provided by [`VK_KHR_EXTERNAL_MEMORY_WIN32`]
    MemoryWin32HandlePropertiesKHR = 1000073002,

    /// Provided by [`VK_KHR_EXTERNAL_MEMORY_WIN32`]
    MemoryGetWin32HandleInfoKHR = 1000073003,

    /// Provided by [`VK_KHR_EXTERNAL_MEMORY_FD`]
    ImportMemoryFdInfoKHR = 1000074000,

    /// Provided by [`VK_KHR_EXTERNAL_MEMORY_FD`]
    MemoryFdPropertiesKHR = 1000074001,

    /// Provided by [`VK_KHR_EXTERNAL_MEMORY_FD`]
    MemoryGetFdInfoKHR = 1000074002,

    /// Provided by [`VK_KHR_WIN32_KEYED_MUTEX`]
    Win32KeyedMutexAcquireReleaseInfoKHR = 1000075000,

    /// Provided by [`VK_KHR_EXTERNAL_SEMAPHORE_WIN32`]
    ImportSemaphoreWin32HandleInfoKHR = 1000078000,

    /// Provided by [`VK_KHR_EXTERNAL_SEMAPHORE_WIN32`]
    ExportSemaphoreWin32HandleInfoKHR = 1000078001,

    /// Provided by [`VK_KHR_EXTERNAL_SEMAPHORE_WIN32`]
    D3D12FenceSubmitInfoKHR = 1000078002,

    /// Provided by [`VK_KHR_EXTERNAL_SEMAPHORE_WIN32`]
    SemaphoreGetWin32HandleInfoKHR = 1000078003,

    /// Provided by [`VK_KHR_EXTERNAL_SEMAPHORE_FD`]
    ImportSemaphoreFdInfoKHR = 1000079000,

    /// Provided by [`VK_KHR_EXTERNAL_SEMAPHORE_FD`]
    SemaphoreGetFdInfoKHR = 1000079001,

    /// Provided by [`VK_KHR_PUSH_DESCRIPTOR`]
    PhysicalDevicePushDescriptorPropertiesKHR = 1000080000,

    /// Provided by [`VK_EXT_CONDITIONAL_RENDERING`]
    CommandBufferInheritanceConditionalRenderingInfoEXT = 1000081000,

    /// Provided by [`VK_EXT_CONDITIONAL_RENDERING`]
    PhysicalDeviceConditionalRenderingFeaturesEXT = 1000081001,

    /// Provided by [`VK_EXT_CONDITIONAL_RENDERING`]
    ConditionalRenderingBeginInfoEXT = 1000081002,

    /// Provided by [`VK_KHR_INCREMENTAL_PRESENT`]
    PresentRegionsKHR = 1000084000,

    /// Provided by [`VK_NV_CLIP_SPACE_W_SCALING`]
    PipelineViewportWScalingStateCreateInfoNV = 1000087000,

    /// Provided by [`VK_EXT_DISPLAY_SURFACE_COUNTER`]
    SurfaceCapabilities2EXT = 1000090000,

    /// Provided by [`VK_EXT_DISPLAY_CONTROL`]
    DisplayPowerInfoEXT = 1000091000,

    /// Provided by [`VK_EXT_DISPLAY_CONTROL`]
    DeviceEventInfoEXT = 1000091001,

    /// Provided by [`VK_EXT_DISPLAY_CONTROL`]
    DisplayEventInfoEXT = 1000091002,

    /// Provided by [`VK_EXT_DISPLAY_CONTROL`]
    SwapchainCounterCreateInfoEXT = 1000091003,

    /// Provided by [`VkGoogledisplay_TIMING`]
    PresentTimesInfoGoogle = 1000092000,

    /// Provided by [`VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES`]
    PhysicalDeviceMultiviewPerViewAttributesPropertiesNVx = 1000097000,

    /// Provided by [`VK_NV_VIEWPORT_SWIZZLE`]
    PipelineViewportSwizzleStateCreateInfoNV = 1000098000,

    /// Provided by [`VK_EXT_DISCARD_RECTANGLES`]
    PhysicalDeviceDiscardRectanglePropertiesEXT = 1000099000,

    /// Provided by [`VK_EXT_DISCARD_RECTANGLES`]
    PipelineDiscardRectangleStateCreateInfoEXT = 1000099001,

    /// Provided by [`VK_EXT_CONSERVATIVE_RASTERIZATION`]
    PhysicalDeviceConservativeRasterizationPropertiesEXT = 1000101000,

    /// Provided by [`VK_EXT_CONSERVATIVE_RASTERIZATION`]
    PipelineRasterizationConservativeStateCreateInfoEXT = 1000101001,

    /// Provided by [`VK_EXT_DEPTH_CLIP_ENABLE`]
    PhysicalDeviceDepthClipEnableFeaturesEXT = 1000102000,

    /// Provided by [`VK_EXT_DEPTH_CLIP_ENABLE`]
    PipelineRasterizationDepthClipStateCreateInfoEXT = 1000102001,

    /// Provided by [`VK_EXT_HDR_METADATA`]
    HdrMetadataEXT = 1000105000,

    /// Provided by [`VkImgrelaxed_LINE_RASTERIZATION`]
    PhysicalDeviceRelaxedLineRasterizationFeaturesImg = 1000110000,

    /// Provided by [`VK_KHR_SHARED_PRESENTABLE_IMAGE`]
    SharedPresentSurfaceCapabilitiesKHR = 1000111000,

    /// Provided by [`VK_KHR_EXTERNAL_FENCE_WIN32`]
    ImportFenceWin32HandleInfoKHR = 1000114000,

    /// Provided by [`VK_KHR_EXTERNAL_FENCE_WIN32`]
    ExportFenceWin32HandleInfoKHR = 1000114001,

    /// Provided by [`VK_KHR_EXTERNAL_FENCE_WIN32`]
    FenceGetWin32HandleInfoKHR = 1000114002,

    /// Provided by [`VK_KHR_EXTERNAL_FENCE_FD`]
    ImportFenceFdInfoKHR = 1000115000,

    /// Provided by [`VK_KHR_EXTERNAL_FENCE_FD`]
    FenceGetFdInfoKHR = 1000115001,

    /// Provided by [`VK_KHR_PERFORMANCE_QUERY`]
    PhysicalDevicePerformanceQueryFeaturesKHR = 1000116000,

    /// Provided by [`VK_KHR_PERFORMANCE_QUERY`]
    PhysicalDevicePerformanceQueryPropertiesKHR = 1000116001,

    /// Provided by [`VK_KHR_PERFORMANCE_QUERY`]
    QueryPoolPerformanceCreateInfoKHR = 1000116002,

    /// Provided by [`VK_KHR_PERFORMANCE_QUERY`]
    PerformanceQuerySubmitInfoKHR = 1000116003,

    /// Provided by [`VK_KHR_PERFORMANCE_QUERY`]
    AcquireProfilingLockInfoKHR = 1000116004,

    /// Provided by [`VK_KHR_PERFORMANCE_QUERY`]
    PerformanceCounterKHR = 1000116005,

    /// Provided by [`VK_KHR_PERFORMANCE_QUERY`]
    PerformanceCounterDescriptionKHR = 1000116006,

    /// Provided by [`VK_KHR_GET_SURFACE_CAPABILITIES2`]
    PhysicalDeviceSurfaceInfo2KHR = 1000119000,

    /// Provided by [`VK_KHR_GET_SURFACE_CAPABILITIES2`]
    SurfaceCapabilities2KHR = 1000119001,

    /// Provided by [`VK_KHR_GET_SURFACE_CAPABILITIES2`]
    SurfaceFormat2KHR = 1000119002,

    /// Provided by [`VK_KHR_GET_DISPLAY_PROPERTIES2`]
    DisplayProperties2KHR = 1000121000,

    /// Provided by [`VK_KHR_GET_DISPLAY_PROPERTIES2`]
    DisplayPlaneProperties2KHR = 1000121001,

    /// Provided by [`VK_KHR_GET_DISPLAY_PROPERTIES2`]
    DisplayModeProperties2KHR = 1000121002,

    /// Provided by [`VK_KHR_GET_DISPLAY_PROPERTIES2`]
    DisplayPlaneInfo2KHR = 1000121003,

    /// Provided by [`VK_KHR_GET_DISPLAY_PROPERTIES2`]
    DisplayPlaneCapabilities2KHR = 1000121004,

    /// Provided by [`VK_MVK_IOS_SURFACE`]
    IosSurfaceCreateInfoMVK = 1000122000,

    /// Provided by [`VK_MVK_MACOS_SURFACE`]
    MacosSurfaceCreateInfoMVK = 1000123000,

    /// Provided by [`VK_EXT_DEBUG_UTILS`]
    DebugUtilsObjectNameInfoEXT = 1000128000,

    /// Provided by [`VK_EXT_DEBUG_UTILS`]
    DebugUtilsObjectTagInfoEXT = 1000128001,

    /// Provided by [`VK_EXT_DEBUG_UTILS`]
    DebugUtilsLabelEXT = 1000128002,

    /// Provided by [`VK_EXT_DEBUG_UTILS`]
    DebugUtilsMessengerCallbackDataEXT = 1000128003,

    /// Provided by [`VK_EXT_DEBUG_UTILS`]
    DebugUtilsMessengerCreateInfoEXT = 1000128004,

    /// Provided by [`VkAndroidexternal_MEMORY_ANDROID_HARDWARE_BUFFER`]
    AndroidHardwareBufferUsageAndroid = 1000129000,

    /// Provided by [`VkAndroidexternal_MEMORY_ANDROID_HARDWARE_BUFFER`]
    AndroidHardwareBufferPropertiesAndroid = 1000129001,

    /// Provided by [`VkAndroidexternal_MEMORY_ANDROID_HARDWARE_BUFFER`]
    AndroidHardwareBufferFormatPropertiesAndroid = 1000129002,

    /// Provided by [`VkAndroidexternal_MEMORY_ANDROID_HARDWARE_BUFFER`]
    ImportAndroidHardwareBufferInfoAndroid = 1000129003,

    /// Provided by [`VkAndroidexternal_MEMORY_ANDROID_HARDWARE_BUFFER`]
    MemoryGetAndroidHardwareBufferInfoAndroid = 1000129004,

    /// Provided by [`VkAndroidexternal_MEMORY_ANDROID_HARDWARE_BUFFER`]
    EXTernalFormatAndroid = 1000129005,

    /// Provided by [`VkAndroidexternal_MEMORY_ANDROID_HARDWARE_BUFFER`] with [`VK_KHR_FORMAT_FEATURE_FLAGS2 or VK_VERSION_1_3`]
    AndroidHardwareBufferFormatProperties2Android = 1000129006,

    /// Provided by [`VkAMDxshader_ENQUEUE`]
    PhysicalDeviceShaderEnqueueFeaturesAMDx = 1000134000,

    /// Provided by [`VkAMDxshader_ENQUEUE`]
    PhysicalDeviceShaderEnqueuePropertiesAMDx = 1000134001,

    /// Provided by [`VkAMDxshader_ENQUEUE`]
    ExecutionGraphPipelineScratchSizeAMDx = 1000134002,

    /// Provided by [`VkAMDxshader_ENQUEUE`]
    ExecutionGraphPipelineCreateInfoAMDx = 1000134003,

    /// Provided by [`VkAMDxshader_ENQUEUE`]
    PipelineShaderStageNodeCreateInfoAMDx = 1000134004,

    /// Provided by [`VK_EXT_SAMPLE_LOCATIONS`]
    SampleLocationsInfoEXT = 1000143000,

    /// Provided by [`VK_EXT_SAMPLE_LOCATIONS`]
    RenderPassSampleLocationsBeginInfoEXT = 1000143001,

    /// Provided by [`VK_EXT_SAMPLE_LOCATIONS`]
    PipelineSampleLocationsStateCreateInfoEXT = 1000143002,

    /// Provided by [`VK_EXT_SAMPLE_LOCATIONS`]
    PhysicalDeviceSampleLocationsPropertiesEXT = 1000143003,

    /// Provided by [`VK_EXT_SAMPLE_LOCATIONS`]
    MultisamplePropertiesEXT = 1000143004,

    /// Provided by [`VK_EXT_BLEND_OPERATION_ADVANCED`]
    PhysicalDeviceBlendOperationAdvancedFeaturesEXT = 1000148000,

    /// Provided by [`VK_EXT_BLEND_OPERATION_ADVANCED`]
    PhysicalDeviceBlendOperationAdvancedPropertiesEXT = 1000148001,

    /// Provided by [`VK_EXT_BLEND_OPERATION_ADVANCED`]
    PipelineColorBlendAdvancedStateCreateInfoEXT = 1000148002,

    /// Provided by [`VK_NV_FRAGMENT_COVERAGE_TO_COLOR`]
    PipelineCoverageToColorStateCreateInfoNV = 1000149000,

    /// Provided by [`VK_KHR_ACCELERATION_STRUCTURE`]
    WriteDescriptorSetAccelerationStructureKHR = 1000150007,

    /// Provided by [`VK_KHR_ACCELERATION_STRUCTURE`]
    AccelerationStructureBuildGeometryInfoKHR = 1000150000,

    /// Provided by [`VK_KHR_ACCELERATION_STRUCTURE`]
    AccelerationStructureDeviceAddressInfoKHR = 1000150002,

    /// Provided by [`VK_KHR_ACCELERATION_STRUCTURE`]
    AccelerationStructureGeometryAabbsDataKHR = 1000150003,

    /// Provided by [`VK_KHR_ACCELERATION_STRUCTURE`]
    AccelerationStructureGeometryInstancesDataKHR = 1000150004,

    /// Provided by [`VK_KHR_ACCELERATION_STRUCTURE`]
    AccelerationStructureGeometryTrianglesDataKHR = 1000150005,

    /// Provided by [`VK_KHR_ACCELERATION_STRUCTURE`]
    AccelerationStructureGeometryKHR = 1000150006,

    /// Provided by [`VK_KHR_ACCELERATION_STRUCTURE`]
    AccelerationStructureVersionInfoKHR = 1000150009,

    /// Provided by [`VK_KHR_ACCELERATION_STRUCTURE`]
    CopyAccelerationStructureInfoKHR = 1000150010,

    /// Provided by [`VK_KHR_ACCELERATION_STRUCTURE`]
    CopyAccelerationStructureToMemoryInfoKHR = 1000150011,

    /// Provided by [`VK_KHR_ACCELERATION_STRUCTURE`]
    CopyMemoryToAccelerationStructureInfoKHR = 1000150012,

    /// Provided by [`VK_KHR_ACCELERATION_STRUCTURE`]
    PhysicalDeviceAccelerationStructureFeaturesKHR = 1000150013,

    /// Provided by [`VK_KHR_ACCELERATION_STRUCTURE`]
    PhysicalDeviceAccelerationStructurePropertiesKHR = 1000150014,

    /// Provided by [`VK_KHR_ACCELERATION_STRUCTURE`]
    AccelerationStructureCreateInfoKHR = 1000150017,

    /// Provided by [`VK_KHR_ACCELERATION_STRUCTURE`]
    AccelerationStructureBuildSizesInfoKHR = 1000150020,

    /// Provided by [`VK_KHR_RAY_TRACING_PIPELINE`]
    PhysicalDeviceRayTracingPipelineFeaturesKHR = 1000347000,

    /// Provided by [`VK_KHR_RAY_TRACING_PIPELINE`]
    PhysicalDeviceRayTracingPipelinePropertiesKHR = 1000347001,

    /// Provided by [`VK_KHR_RAY_TRACING_PIPELINE`]
    RayTracingPipelineCreateInfoKHR = 1000150015,

    /// Provided by [`VK_KHR_RAY_TRACING_PIPELINE`]
    RayTracingShaderGroupCreateInfoKHR = 1000150016,

    /// Provided by [`VK_KHR_RAY_TRACING_PIPELINE`]
    RayTracingPipelineInterfaceCreateInfoKHR = 1000150018,

    /// Provided by [`VK_KHR_RAY_QUERY`]
    PhysicalDeviceRayQueryFeaturesKHR = 1000348013,

    /// Provided by [`VK_NV_FRAMEBUFFER_MIXED_SAMPLES`]
    PipelineCoverageModulationStateCreateInfoNV = 1000152000,

    /// Provided by [`VK_NV_SHADER_SM_BUILTINS`]
    PhysicalDeviceShaderSmBuiltinsFeaturesNV = 1000154000,

    /// Provided by [`VK_NV_SHADER_SM_BUILTINS`]
    PhysicalDeviceShaderSmBuiltinsPropertiesNV = 1000154001,

    /// Provided by [`VK_EXT_IMAGE_DRM_FORMAT_MODIFIER`]
    DrmFormatModifierPropertiesListEXT = 1000158000,

    /// Provided by [`VK_EXT_IMAGE_DRM_FORMAT_MODIFIER`]
    PhysicalDeviceImageDrmFormatModifierInfoEXT = 1000158002,

    /// Provided by [`VK_EXT_IMAGE_DRM_FORMAT_MODIFIER`]
    ImageDrmFormatModifierListCreateInfoEXT = 1000158003,

    /// Provided by [`VK_EXT_IMAGE_DRM_FORMAT_MODIFIER`]
    ImageDrmFormatModifierExplicitCreateInfoEXT = 1000158004,

    /// Provided by [`VK_EXT_IMAGE_DRM_FORMAT_MODIFIER`]
    ImageDrmFormatModifierPropertiesEXT = 1000158005,

    /// Provided by [`VK_EXT_IMAGE_DRM_FORMAT_MODIFIER WITH VK_KHR_FORMAT_FEATURE_FLAGS2 OR VK_VERSION_1_3`]
    DrmFormatModifierPropertiesList2EXT = 1000158006,

    /// Provided by [`VK_EXT_VALIDATION_CACHE`]
    ValidationCacheCreateInfoEXT = 1000160000,

    /// Provided by [`VK_EXT_VALIDATION_CACHE`]
    ShaderModuleValidationCacheCreateInfoEXT = 1000160001,

    /// Provided by [`VK_KHR_PORTABILITY_SUBSET`]
    PhysicalDevicePortabilitySubsetFeaturesKHR = 1000163000,

    /// Provided by [`VK_KHR_PORTABILITY_SUBSET`]
    PhysicalDevicePortabilitySubsetPropertiesKHR = 1000163001,

    /// Provided by [`VK_NV_SHADING_RATE_IMAGE`]
    PipelineViewportShadingRateImageStateCreateInfoNV = 1000164000,

    /// Provided by [`VK_NV_SHADING_RATE_IMAGE`]
    PhysicalDeviceShadingRateImageFeaturesNV = 1000164001,

    /// Provided by [`VK_NV_SHADING_RATE_IMAGE`]
    PhysicalDeviceShadingRateImagePropertiesNV = 1000164002,

    /// Provided by [`VK_NV_SHADING_RATE_IMAGE`]
    PipelineViewportCoarseSampleOrderStateCreateInfoNV = 1000164005,

    /// Provided by [`VK_NV_RAY_TRACING`]
    RayTracingPipelineCreateInfoNV = 1000165000,

    /// Provided by [`VK_NV_RAY_TRACING`]
    AccelerationStructureCreateInfoNV = 1000165001,

    /// Provided by [`VK_NV_RAY_TRACING`]
    GeometryNV = 1000165003,

    /// Provided by [`VK_NV_RAY_TRACING`]
    GeometryTrianglesNV = 1000165004,

    /// Provided by [`VK_NV_RAY_TRACING`]
    GeometryAabbNV = 1000165005,

    /// Provided by [`VK_NV_RAY_TRACING`]
    BindAccelerationStructureMemoryInfoNV = 1000165006,

    /// Provided by [`VK_NV_RAY_TRACING`]
    WriteDescriptorSetAccelerationStructureNV = 1000165007,

    /// Provided by [`VK_NV_RAY_TRACING`]
    AccelerationStructureMemoryRequirementsInfoNV = 1000165008,

    /// Provided by [`VK_NV_RAY_TRACING`]
    PhysicalDeviceRayTracingPropertiesNV = 1000165009,

    /// Provided by [`VK_NV_RAY_TRACING`]
    RayTracingShaderGroupCreateInfoNV = 1000165011,

    /// Provided by [`VK_NV_RAY_TRACING`]
    AccelerationStructureInfoNV = 1000165012,

    /// Provided by [`VK_NV_REPRESENTATIVE_FRAGMENT_TEST`]
    PhysicalDeviceRepresentativeFragmentTestFeaturesNV = 1000166000,

    /// Provided by [`VK_NV_REPRESENTATIVE_FRAGMENT_TEST`]
    PipelineRepresentativeFragmentTestStateCreateInfoNV = 1000166001,

    /// Provided by [`VK_EXT_FILTER_CUBIC`]
    PhysicalDeviceImageViewImageFormatInfoEXT = 1000170000,

    /// Provided by [`VK_EXT_FILTER_CUBIC`]
    FilterCubicImageViewImageFormatPropertiesEXT = 1000170001,

    /// Provided by [`VK_EXT_EXTERNAL_MEMORY_HOST`]
    ImportMemoryHostPointerInfoEXT = 1000178000,

    /// Provided by [`VK_EXT_EXTERNAL_MEMORY_HOST`]
    MemoryHostPointerPropertiesEXT = 1000178001,

    /// Provided by [`VK_EXT_EXTERNAL_MEMORY_HOST`]
    PhysicalDeviceEXTernalMemoryHostPropertiesEXT = 1000178002,

    /// Provided by [`VK_KHR_SHADER_CLOCK`]
    PhysicalDeviceShaderClockFeaturesKHR = 1000181000,

    /// Provided by [`VkAMDpipeline_COMPILER_CONTROL`]
    PipelineCompilerControlCreateInfoAMD = 1000183000,

    /// Provided by [`VkAMDshader_CORE_PROPERTIES`]
    PhysicalDeviceShaderCorePropertiesAMD = 1000185000,

    /// Provided by [`VK_KHR_VIDEO_DECODE_H265`]
    VideoDecodeH265CapabilitiesKHR = 1000187000,

    /// Provided by [`VK_KHR_VIDEO_DECODE_H265`]
    VideoDecodeH265SessionParametersCreateInfoKHR = 1000187001,

    /// Provided by [`VK_KHR_VIDEO_DECODE_H265`]
    VideoDecodeH265SessionParametersAddInfoKHR = 1000187002,

    /// Provided by [`VK_KHR_VIDEO_DECODE_H265`]
    VideoDecodeH265ProfileInfoKHR = 1000187003,

    /// Provided by [`VK_KHR_VIDEO_DECODE_H265`]
    VideoDecodeH265PictureInfoKHR = 1000187004,

    /// Provided by [`VK_KHR_VIDEO_DECODE_H265`]
    VideoDecodeH265DpbSlotInfoKHR = 1000187005,

    /// Provided by [`VK_KHR_GLOBAL_PRIORITY`]
    DeviceQueueGlobalPriorityCreateInfoKHR = 1000174000,

    /// Provided by [`VK_KHR_GLOBAL_PRIORITY`]
    PhysicalDeviceGlobalPriorityQueryFeaturesKHR = 1000388000,

    /// Provided by [`VK_KHR_GLOBAL_PRIORITY`]
    QueueFamilyGlobalPriorityPropertiesKHR = 1000388001,

    /// Provided by [`VkAMDmemory_OVERALLOCATION_BEHAVIOR`]
    DeviceMemoryOverallocationCreateInfoAMD = 1000189000,

    /// Provided by [`VK_EXT_VERTEX_ATTRIBUTE_DIVISOR`]
    PhysicalDeviceVertexAttributeDivisorPropertiesEXT = 1000190000,

    /// Provided by [`VkGgpframe_TOKEN`]
    PresentFrameTokenGgp = 1000191000,

    /// Provided by [`VK_NV_COMPUTE_SHADER_DERIVATIVES`]
    PhysicalDeviceComputeShaderDerivativesFeaturesNV = 1000201000,

    /// Provided by [`VK_NV_MESH_SHADER`]
    PhysicalDeviceMeshShaderFeaturesNV = 1000202000,

    /// Provided by [`VK_NV_MESH_SHADER`]
    PhysicalDeviceMeshShaderPropertiesNV = 1000202001,

    /// Provided by [`VK_NV_SHADER_IMAGE_FOOTPRINT`]
    PhysicalDeviceShaderImageFootprintFeaturesNV = 1000204000,

    /// Provided by [`VK_NV_SCISSOR_EXCLUSIVE`]
    PipelineViewportExclusiveScissorStateCreateInfoNV = 1000205000,

    /// Provided by [`VK_NV_SCISSOR_EXCLUSIVE`]
    PhysicalDeviceExclusiveScissorFeaturesNV = 1000205002,

    /// Provided by [`VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS`]
    CheckpointDataNV = 1000206000,

    /// Provided by [`VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS`]
    QueueFamilyCheckpointPropertiesNV = 1000206001,

    /// Provided by [`VkIntelshader_INTEGER_FUNCTIONS2`]
    PhysicalDeviceShaderIntegerFunctions2FeaturesIntel = 1000209000,

    /// Provided by [`VkIntelperformance_QUERY`]
    QueryPoolPerformanceQueryCreateInfoIntel = 1000210000,

    /// Provided by [`VkIntelperformance_QUERY`]
    InitializePerformanceApiInfoIntel = 1000210001,

    /// Provided by [`VkIntelperformance_QUERY`]
    PerformanceMarkerInfoIntel = 1000210002,

    /// Provided by [`VkIntelperformance_QUERY`]
    PerformanceStreamMarkerInfoIntel = 1000210003,

    /// Provided by [`VkIntelperformance_QUERY`]
    PerformanceOverrideInfoIntel = 1000210004,

    /// Provided by [`VkIntelperformance_QUERY`]
    PerformanceConfigurationAcquireInfoIntel = 1000210005,

    /// Provided by [`VK_EXT_PCI_BUS_INFO`]
    PhysicalDevicePciBusInfoPropertiesEXT = 1000212000,

    /// Provided by [`VkAMDdisplay_NATIVE_HDR`]
    DisplayNativeHdrSurfaceCapabilitiesAMD = 1000213000,

    /// Provided by [`VkAMDdisplay_NATIVE_HDR`]
    SwapchainDisplayNativeHdrCreateInfoAMD = 1000213001,

    /// Provided by [`VkFuchsiaimagepipe_SURFACE`]
    ImagepipeSurfaceCreateInfoFuchsia = 1000214000,

    /// Provided by [`VK_EXT_METAL_SURFACE`]
    MetalSurfaceCreateInfoEXT = 1000217000,

    /// Provided by [`VK_EXT_FRAGMENT_DENSITY_MAP`]
    PhysicalDeviceFragmentDensityMapFeaturesEXT = 1000218000,

    /// Provided by [`VK_EXT_FRAGMENT_DENSITY_MAP`]
    PhysicalDeviceFragmentDensityMapPropertiesEXT = 1000218001,

    /// Provided by [`VK_EXT_FRAGMENT_DENSITY_MAP`]
    RenderPassFragmentDensityMapCreateInfoEXT = 1000218002,

    /// Provided by [`VK_KHR_FRAGMENT_SHADING_RATE`]
    FragmentShadingRateAttachmentInfoKHR = 1000226000,

    /// Provided by [`VK_KHR_FRAGMENT_SHADING_RATE`]
    PipelineFragmentShadingRateStateCreateInfoKHR = 1000226001,

    /// Provided by [`VK_KHR_FRAGMENT_SHADING_RATE`]
    PhysicalDeviceFragmentShadingRatePropertiesKHR = 1000226002,

    /// Provided by [`VK_KHR_FRAGMENT_SHADING_RATE`]
    PhysicalDeviceFragmentShadingRateFeaturesKHR = 1000226003,

    /// Provided by [`VK_KHR_FRAGMENT_SHADING_RATE`]
    PhysicalDeviceFragmentShadingRateKHR = 1000226004,

    /// Provided by [`VK_AMD_SHADER_CORE_PROPERTIES2`]
    PhysicalDeviceShaderCoreProperties2AMD = 1000227000,

    /// Provided by [`VK_AMD_DEVICE_COHERENT_MEMORY`]
    PhysicalDeviceCoherentMemoryFeaturesAMD = 1000229000,

    /// Provided by [`VK_KHR_DYNAMIC_RENDERING_LOCAL_READ`]
    PhysicalDeviceDynamicRenderingLocalReadFeaturesKHR = 1000232000,

    /// Provided by [`VK_KHR_DYNAMIC_RENDERING_LOCAL_READ`]
    RenderingAttachmentLocationInfoKHR = 1000232001,

    /// Provided by [`VK_KHR_DYNAMIC_RENDERING_LOCAL_READ`]
    RenderingInputAttachmentIndexInfoKHR = 1000232002,

    /// Provided by [`VK_EXT_SHADER_IMAGE_ATOMIC_INT64`]
    PhysicalDeviceShaderImageAtomicInt64FeaturesEXT = 1000234000,

    /// Provided by [`VK_KHR_SHADER_QUAD_CONTROL`]
    PhysicalDeviceShaderQuadControlFeaturesKHR = 1000235000,

    /// Provided by [`VK_EXT_MEMORY_BUDGET`]
    PhysicalDeviceMemoryBudgetPropertiesEXT = 1000237000,

    /// Provided by [`VK_EXT_MEMORY_PRIORITY`]
    PhysicalDeviceMemoryPriorityFeaturesEXT = 1000238000,

    /// Provided by [`VK_EXT_MEMORY_PRIORITY`]
    MemoryPriorityAllocateInfoEXT = 1000238001,

    /// Provided by [`VK_KHR_SURFACE_PROTECTED_CAPABILITIES`]
    SurfaceProtectedCapabilitiesKHR = 1000239000,

    /// Provided by [`VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING`]
    PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV = 1000240000,

    /// Provided by [`VK_EXT_BUFFER_DEVICE_ADDRESS`]
    PhysicalDeviceBufferDeviceAddressFeaturesEXT = 1000244000,

    /// Provided by [`VK_EXT_BUFFER_DEVICE_ADDRESS`]
    BufferDeviceAddressCreateInfoEXT = 1000244002,

    /// Provided by [`VK_EXT_VALIDATION_FEATURES`]
    ValidationFeaturesEXT = 1000247000,

    /// Provided by [`VK_KHR_PRESENT_WAIT`]
    PhysicalDevicePresentWaitFeaturesKHR = 1000248000,

    /// Provided by [`VK_NV_COOPERATIVE_MATRIX`]
    PhysicalDeviceCooperativeMatrixFeaturesNV = 1000249000,

    /// Provided by [`VK_NV_COOPERATIVE_MATRIX`]
    CooperativeMatrixPropertiesNV = 1000249001,

    /// Provided by [`VK_NV_COOPERATIVE_MATRIX`]
    PhysicalDeviceCooperativeMatrixPropertiesNV = 1000249002,

    /// Provided by [`VK_NV_COVERAGE_REDUCTION_MODE`]
    PhysicalDeviceCoverageReductionModeFeaturesNV = 1000250000,

    /// Provided by [`VK_NV_COVERAGE_REDUCTION_MODE`]
    PipelineCoverageReductionStateCreateInfoNV = 1000250001,

    /// Provided by [`VK_NV_COVERAGE_REDUCTION_MODE`]
    FramebufferMixedSamplesCombinationNV = 1000250002,

    /// Provided by [`VK_EXT_FRAGMENT_SHADER_INTERLOCK`]
    PhysicalDeviceFragmentShaderInterlockFeaturesEXT = 1000251000,

    /// Provided by [`VK_EXT_YCBCR_IMAGE_ARRAYS`]
    PhysicalDeviceYcbcrImageArraysFeaturesEXT = 1000252000,

    /// Provided by [`VK_EXT_PROVOKING_VERTEX`]
    PhysicalDeviceProvokingVertexFeaturesEXT = 1000254000,

    /// Provided by [`VK_EXT_PROVOKING_VERTEX`]
    PipelineRasterizationProvokingVertexStateCreateInfoEXT = 1000254001,

    /// Provided by [`VK_EXT_PROVOKING_VERTEX`]
    PhysicalDeviceProvokingVertexPropertiesEXT = 1000254002,

    /// Provided by [`VK_EXT_FULL_SCREEN_EXCLUSIVE`]
    SurfaceFullScreenExclusiveInfoEXT = 1000255000,

    /// Provided by [`VK_EXT_FULL_SCREEN_EXCLUSIVE`]
    SurfaceCapabilitiesFullScreenExclusiveEXT = 1000255002,

    /// Provided by [`VK_KHR_WIN32_SURFACE`] with [`VK_EXT_FULL_SCREEN_EXCLUSIVE`]
    SurfaceFullScreenExclusiveWin32InfoEXT = 1000255001,

    /// Provided by [`VK_EXT_HEADLESS_SURFACE`]
    HeadlessSurfaceCreateInfoEXT = 1000256000,

    /// Provided by [`VK_EXT_SHADER_ATOMIC_FLOAT`]
    PhysicalDeviceShaderAtomicFloatFeaturesEXT = 1000260000,

    /// Provided by [`VK_EXT_EXTENDED_DYNAMIC_STATE`]
    PhysicalDeviceEXTendedDynamicStateFeaturesEXT = 1000267000,

    /// Provided by [`VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES`]
    PhysicalDevicePipelineExecutablePropertiesFeaturesKHR = 1000269000,

    /// Provided by [`VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES`]
    PipelineInfoKHR = 1000269001,

    /// Provided by [`VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES`]
    PipelineExecutablePropertiesKHR = 1000269002,

    /// Provided by [`VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES`]
    PipelineExecutableInfoKHR = 1000269003,

    /// Provided by [`VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES`]
    PipelineExecutableStatisticKHR = 1000269004,

    /// Provided by [`VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES`]
    PipelineExecutableInternalRepresentationKHR = 1000269005,

    /// Provided by [`VK_EXT_HOST_IMAGE_COPY`]
    PhysicalDeviceHostImageCopyFeaturesEXT = 1000270000,

    /// Provided by [`VK_EXT_HOST_IMAGE_COPY`]
    PhysicalDeviceHostImageCopyPropertiesEXT = 1000270001,

    /// Provided by [`VK_EXT_HOST_IMAGE_COPY`]
    MemoryToImageCopyEXT = 1000270002,

    /// Provided by [`VK_EXT_HOST_IMAGE_COPY`]
    ImageToMemoryCopyEXT = 1000270003,

    /// Provided by [`VK_EXT_HOST_IMAGE_COPY`]
    CopyImageToMemoryInfoEXT = 1000270004,

    /// Provided by [`VK_EXT_HOST_IMAGE_COPY`]
    CopyMemoryToImageInfoEXT = 1000270005,

    /// Provided by [`VK_EXT_HOST_IMAGE_COPY`]
    HostImageLayoutTransitionInfoEXT = 1000270006,

    /// Provided by [`VK_EXT_HOST_IMAGE_COPY`]
    CopyImageToImageInfoEXT = 1000270007,

    /// Provided by [`VK_EXT_HOST_IMAGE_COPY`]
    SubresourceHostMemcpySizeEXT = 1000270008,

    /// Provided by [`VK_EXT_HOST_IMAGE_COPY`]
    HostImageCopyDevicePerformanceQueryEXT = 1000270009,

    /// Provided by [`VK_KHR_MAP_MEMORY2`]
    MemoryMapInfoKHR = 1000271000,

    /// Provided by [`VK_KHR_MAP_MEMORY2`]
    MemoryUnmapInfoKHR = 1000271001,

    /// Provided by [`VK_EXT_MAP_MEMORY_PLACED`]
    PhysicalDeviceMapMemoryPlacedFeaturesEXT = 1000272000,

    /// Provided by [`VK_EXT_MAP_MEMORY_PLACED`]
    PhysicalDeviceMapMemoryPlacedPropertiesEXT = 1000272001,

    /// Provided by [`VK_EXT_MAP_MEMORY_PLACED`]
    MemoryMapPlacedInfoEXT = 1000272002,

    /// Provided by [`VK_EXT_SHADER_ATOMIC_FLOAT2`]
    PhysicalDeviceShaderAtomicFloat2FeaturesEXT = 1000273000,

    /// Provided by [`VK_EXT_SURFACE_MAINTENANCE1`]
    SurfacePresentModeEXT = 1000274000,

    /// Provided by [`VK_EXT_SURFACE_MAINTENANCE1`]
    SurfacePresentScalingCapabilitiesEXT = 1000274001,

    /// Provided by [`VK_EXT_SURFACE_MAINTENANCE1`]
    SurfacePresentModeCompatibilityEXT = 1000274002,

    /// Provided by [`VK_EXT_SWAPCHAIN_MAINTENANCE1`]
    PhysicalDeviceSwapchainMaintenance1FeaturesEXT = 1000275000,

    /// Provided by [`VK_EXT_SWAPCHAIN_MAINTENANCE1`]
    SwapchainPresentFenceInfoEXT = 1000275001,

    /// Provided by [`VK_EXT_SWAPCHAIN_MAINTENANCE1`]
    SwapchainPresentModesCreateInfoEXT = 1000275002,

    /// Provided by [`VK_EXT_SWAPCHAIN_MAINTENANCE1`]
    SwapchainPresentModeInfoEXT = 1000275003,

    /// Provided by [`VK_EXT_SWAPCHAIN_MAINTENANCE1`]
    SwapchainPresentScalingCreateInfoEXT = 1000275004,

    /// Provided by [`VK_EXT_SWAPCHAIN_MAINTENANCE1`]
    ReleaseSwapchainImagesInfoEXT = 1000275005,

    /// Provided by [`VK_NV_DEVICE_GENERATED_COMMANDS`]
    PhysicalDeviceDeviceGeneratedCommandsPropertiesNV = 1000277000,

    /// Provided by [`VK_NV_DEVICE_GENERATED_COMMANDS`]
    GraphicsShaderGroupCreateInfoNV = 1000277001,

    /// Provided by [`VK_NV_DEVICE_GENERATED_COMMANDS`]
    GraphicsPipelineShaderGroupsCreateInfoNV = 1000277002,

    /// Provided by [`VK_NV_DEVICE_GENERATED_COMMANDS`]
    IndirectCommandsLayoutTokenNV = 1000277003,

    /// Provided by [`VK_NV_DEVICE_GENERATED_COMMANDS`]
    IndirectCommandsLayoutCreateInfoNV = 1000277004,

    /// Provided by [`VK_NV_DEVICE_GENERATED_COMMANDS`]
    GeneratedCommandsInfoNV = 1000277005,

    /// Provided by [`VK_NV_DEVICE_GENERATED_COMMANDS`]
    GeneratedCommandsMemoryRequirementsInfoNV = 1000277006,

    /// Provided by [`VK_NV_DEVICE_GENERATED_COMMANDS`]
    PhysicalDeviceDeviceGeneratedCommandsFeaturesNV = 1000277007,

    /// Provided by [`VK_NV_INHERITED_VIEWPORT_SCISSOR`]
    PhysicalDeviceInheritedViewportScissorFeaturesNV = 1000278000,

    /// Provided by [`VK_NV_INHERITED_VIEWPORT_SCISSOR`]
    CommandBufferInheritanceViewportScissorInfoNV = 1000278001,

    /// Provided by [`VK_EXT_TEXEL_BUFFER_ALIGNMENT`]
    PhysicalDeviceTexelBufferAlignmentFeaturesEXT = 1000281000,

    /// Provided by [`VkQcomrender_PASS_TRANSFORM`]
    CommandBufferInheritanceRenderPassTransformInfoQcom = 1000282000,

    /// Provided by [`VkQcomrender_PASS_TRANSFORM`]
    RenderPassTransformBeginInfoQcom = 1000282001,

    /// Provided by [`VK_EXT_DEPTH_BIAS_CONTROL`]
    PhysicalDeviceDepthBiasControlFeaturesEXT = 1000283000,

    /// Provided by [`VK_EXT_DEPTH_BIAS_CONTROL`]
    DepthBiasInfoEXT = 1000283001,

    /// Provided by [`VK_EXT_DEPTH_BIAS_CONTROL`]
    DepthBiasRepresentationInfoEXT = 1000283002,

    /// Provided by [`VK_EXT_DEVICE_MEMORY_REPORT`]
    PhysicalDeviceDeviceMemoryReportFeaturesEXT = 1000284000,

    /// Provided by [`VK_EXT_DEVICE_MEMORY_REPORT`]
    DeviceDeviceMemoryReportCreateInfoEXT = 1000284001,

    /// Provided by [`VK_EXT_DEVICE_MEMORY_REPORT`]
    DeviceMemoryReportCallbackDataEXT = 1000284002,

    /// Provided by [`VK_EXT_ROBUSTNESS2`]
    PhysicalDeviceRobustness2FeaturesEXT = 1000286000,

    /// Provided by [`VK_EXT_ROBUSTNESS2`]
    PhysicalDeviceRobustness2PropertiesEXT = 1000286001,

    /// Provided by [`VK_EXT_CUSTOM_BORDER_COLOR`]
    SamplerCustomBorderColorCreateInfoEXT = 1000287000,

    /// Provided by [`VK_EXT_CUSTOM_BORDER_COLOR`]
    PhysicalDeviceCustomBorderColorPropertiesEXT = 1000287001,

    /// Provided by [`VK_EXT_CUSTOM_BORDER_COLOR`]
    PhysicalDeviceCustomBorderColorFeaturesEXT = 1000287002,

    /// Provided by [`VK_KHR_PIPELINE_LIBRARY`]
    PipelineLibraryCreateInfoKHR = 1000290000,

    /// Provided by [`VK_NV_PRESENT_BARRIER`]
    PhysicalDevicePresentBarrierFeaturesNV = 1000292000,

    /// Provided by [`VK_NV_PRESENT_BARRIER`]
    SurfaceCapabilitiesPresentBarrierNV = 1000292001,

    /// Provided by [`VK_NV_PRESENT_BARRIER`]
    SwapchainPresentBarrierCreateInfoNV = 1000292002,

    /// Provided by [`VK_KHR_PRESENT_ID`]
    PresentIdKHR = 1000294000,

    /// Provided by [`VK_KHR_PRESENT_ID`]
    PhysicalDevicePresentIdFeaturesKHR = 1000294001,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_QUEUE`]
    VideoEncodeInfoKHR = 1000299000,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_QUEUE`]
    VideoEncodeRateControlInfoKHR = 1000299001,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_QUEUE`]
    VideoEncodeRateControlLayerInfoKHR = 1000299002,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_QUEUE`]
    VideoEncodeCapabilitiesKHR = 1000299003,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_QUEUE`]
    VideoEncodeUsageInfoKHR = 1000299004,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_QUEUE`]
    QueryPoolVideoEncodeFeedbackCreateInfoKHR = 1000299005,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_QUEUE`]
    PhysicalDeviceVideoEncodeQualityLevelInfoKHR = 1000299006,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_QUEUE`]
    VideoEncodeQualityLevelPropertiesKHR = 1000299007,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_QUEUE`]
    VideoEncodeQualityLevelInfoKHR = 1000299008,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_QUEUE`]
    VideoEncodeSessionParametersGetInfoKHR = 1000299009,

    /// Provided by [`VK_KHR_VIDEO_ENCODE_QUEUE`]
    VideoEncodeSessionParametersFeedbackInfoKHR = 1000299010,

    /// Provided by [`VK_NV_DEVICE_DIAGNOSTICS_CONFIG`]
    PhysicalDeviceDiagnosticsConfigFeaturesNV = 1000300000,

    /// Provided by [`VK_NV_DEVICE_DIAGNOSTICS_CONFIG`]
    DeviceDiagnosticsConfigCreateInfoNV = 1000300001,

    /// Provided by [`VK_NV_CUDA_KERNEL_LAUNCH`]
    CudaModuleCreateInfoNV = 1000307000,

    /// Provided by [`VK_NV_CUDA_KERNEL_LAUNCH`]
    CudaFunctionCreateInfoNV = 1000307001,

    /// Provided by [`VK_NV_CUDA_KERNEL_LAUNCH`]
    CudaLaunchInfoNV = 1000307002,

    /// Provided by [`VK_NV_CUDA_KERNEL_LAUNCH`]
    PhysicalDeviceCudaKernelLaunchFeaturesNV = 1000307003,

    /// Provided by [`VK_NV_CUDA_KERNEL_LAUNCH`]
    PhysicalDeviceCudaKernelLaunchPropertiesNV = 1000307004,

    /// Provided by [`VK_NV_LOW_LATENCY`]
    QueryLowLatencySupportNV = 1000310000,

    /// Provided by [`VK_EXT_METAL_OBJECTS`]
    ExportMetalObjectCreateInfoEXT = 1000311000,

    /// Provided by [`VK_EXT_METAL_OBJECTS`]
    ExportMetalObjectsInfoEXT = 1000311001,

    /// Provided by [`VK_EXT_METAL_OBJECTS`]
    ExportMetalDeviceInfoEXT = 1000311002,

    /// Provided by [`VK_EXT_METAL_OBJECTS`]
    ExportMetalCommandQueueInfoEXT = 1000311003,

    /// Provided by [`VK_EXT_METAL_OBJECTS`]
    ExportMetalBufferInfoEXT = 1000311004,

    /// Provided by [`VK_EXT_METAL_OBJECTS`]
    ImportMetalBufferInfoEXT = 1000311005,

    /// Provided by [`VK_EXT_METAL_OBJECTS`]
    ExportMetalTextureInfoEXT = 1000311006,

    /// Provided by [`VK_EXT_METAL_OBJECTS`]
    ImportMetalTextureInfoEXT = 1000311007,

    /// Provided by [`VK_EXT_METAL_OBJECTS`]
    ExportMetalIoSurfaceInfoEXT = 1000311008,

    /// Provided by [`VK_EXT_METAL_OBJECTS`]
    ImportMetalIoSurfaceInfoEXT = 1000311009,

    /// Provided by [`VK_EXT_METAL_OBJECTS`]
    ExportMetalSharedEventInfoEXT = 1000311010,

    /// Provided by [`VK_EXT_METAL_OBJECTS`]
    ImportMetalSharedEventInfoEXT = 1000311011,

    /// Provided by [`VK_KHR_SYNCHRONIZATION2`] with [`VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS`]
    QueueFamilyCheckpointProperties2NV = 1000314008,

    /// Provided by [`VK_KHR_SYNCHRONIZATION2`] with [`VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS`]
    CheckpointData2NV = 1000314009,

    /// Provided by [`VK_EXT_DESCRIPTOR_BUFFER`]
    PhysicalDeviceDescriptorBufferPropertiesEXT = 1000316000,

    /// Provided by [`VK_EXT_DESCRIPTOR_BUFFER`]
    PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT = 1000316001,

    /// Provided by [`VK_EXT_DESCRIPTOR_BUFFER`]
    PhysicalDeviceDescriptorBufferFeaturesEXT = 1000316002,

    /// Provided by [`VK_EXT_DESCRIPTOR_BUFFER`]
    DescriptorAddressInfoEXT = 1000316003,

    /// Provided by [`VK_EXT_DESCRIPTOR_BUFFER`]
    DescriptorGetInfoEXT = 1000316004,

    /// Provided by [`VK_EXT_DESCRIPTOR_BUFFER`]
    BufferCaptureDescriptorDataInfoEXT = 1000316005,

    /// Provided by [`VK_EXT_DESCRIPTOR_BUFFER`]
    ImageCaptureDescriptorDataInfoEXT = 1000316006,

    /// Provided by [`VK_EXT_DESCRIPTOR_BUFFER`]
    ImageViewCaptureDescriptorDataInfoEXT = 1000316007,

    /// Provided by [`VK_EXT_DESCRIPTOR_BUFFER`]
    SamplerCaptureDescriptorDataInfoEXT = 1000316008,

    /// Provided by [`VK_EXT_DESCRIPTOR_BUFFER`]
    OpaqueCaptureDescriptorDataCreateInfoEXT = 1000316010,

    /// Provided by [`VK_EXT_DESCRIPTOR_BUFFER`]
    DescriptorBufferBindingInfoEXT = 1000316011,

    /// Provided by [`VK_EXT_DESCRIPTOR_BUFFER`]
    DescriptorBufferBindingPushDescriptorBufferHandleEXT = 1000316012,

    /// Provided by [`VK_EXT_DESCRIPTOR_BUFFER WITH VK_KHR_ACCELERATION_STRUCTURE OR VKNVRAY_TRACING`]
    AccelerationStructureCaptureDescriptorDataInfoEXT = 1000316009,

    /// Provided by [`VK_EXT_GRAPHICS_PIPELINE_LIBRARY`]
    PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT = 1000320000,

    /// Provided by [`VK_EXT_GRAPHICS_PIPELINE_LIBRARY`]
    PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT = 1000320001,

    /// Provided by [`VK_EXT_GRAPHICS_PIPELINE_LIBRARY`]
    GraphicsPipelineLibraryCreateInfoEXT = 1000320002,

    /// Provided by [`VkAMDshader_EARLY_AND_LATE_FRAGMENT_TESTS`]
    PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD = 1000321000,

    /// Provided by [`VK_KHR_FRAGMENT_SHADER_BARYCENTRIC`]
    PhysicalDeviceFragmentShaderBarycentricFeaturesKHR = 1000203000,

    /// Provided by [`VK_KHR_FRAGMENT_SHADER_BARYCENTRIC`]
    PhysicalDeviceFragmentShaderBarycentricPropertiesKHR = 1000322000,

    /// Provided by [`VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW`]
    PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR = 1000323000,

    /// Provided by [`VK_NV_FRAGMENT_SHADING_RATE_ENUMS`]
    PhysicalDeviceFragmentShadingRateEnumsPropertiesNV = 1000326000,

    /// Provided by [`VK_NV_FRAGMENT_SHADING_RATE_ENUMS`]
    PhysicalDeviceFragmentShadingRateEnumsFeaturesNV = 1000326001,

    /// Provided by [`VK_NV_FRAGMENT_SHADING_RATE_ENUMS`]
    PipelineFragmentShadingRateEnumStateCreateInfoNV = 1000326002,

    /// Provided by [`VK_NV_RAY_TRACING_MOTION_BLUR`]
    AccelerationStructureGeometryMotionTrianglesDataNV = 1000327000,

    /// Provided by [`VK_NV_RAY_TRACING_MOTION_BLUR`]
    PhysicalDeviceRayTracingMotionBlurFeaturesNV = 1000327001,

    /// Provided by [`VK_NV_RAY_TRACING_MOTION_BLUR`]
    AccelerationStructureMotionInfoNV = 1000327002,

    /// Provided by [`VK_EXT_MESH_SHADER`]
    PhysicalDeviceMeshShaderFeaturesEXT = 1000328000,

    /// Provided by [`VK_EXT_MESH_SHADER`]
    PhysicalDeviceMeshShaderPropertiesEXT = 1000328001,

    /// Provided by [`VK_EXT_YCBCR_2PLANE_444_FORMATS`]
    PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT = 1000330000,

    /// Provided by [`VK_EXT_FRAGMENT_DENSITY_MAP2`]
    PhysicalDeviceFragmentDensityMap2FeaturesEXT = 1000332000,

    /// Provided by [`VK_EXT_FRAGMENT_DENSITY_MAP2`]
    PhysicalDeviceFragmentDensityMap2PropertiesEXT = 1000332001,

    /// Provided by [`VkQcomrotated_COPY_COMMANDS`]
    CopyCommandTransformInfoQcom = 1000333000,

    /// Provided by [`VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT`]
    PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR = 1000336000,

    /// Provided by [`VK_EXT_IMAGE_COMPRESSION_CONTROL`]
    PhysicalDeviceImageCompressionControlFeaturesEXT = 1000338000,

    /// Provided by [`VK_EXT_IMAGE_COMPRESSION_CONTROL`]
    ImageCompressionControlEXT = 1000338001,

    /// Provided by [`VK_EXT_IMAGE_COMPRESSION_CONTROL`]
    ImageCompressionPropertiesEXT = 1000338004,

    /// Provided by [`VK_EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT`]
    PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT = 1000339000,

    /// Provided by [`VK_EXT_4444_FORMATS`]
    PhysicalDevice4444FormatsFeaturesEXT = 1000340000,

    /// Provided by [`VK_EXT_DEVICE_FAULT`]
    PhysicalDeviceFaultFeaturesEXT = 1000341000,

    /// Provided by [`VK_EXT_DEVICE_FAULT`]
    DeviceFaultCountsEXT = 1000341001,

    /// Provided by [`VK_EXT_DEVICE_FAULT`]
    DeviceFaultInfoEXT = 1000341002,

    /// Provided by [`VK_EXT_RGBA10X6_FORMATS`]
    PhysicalDeviceRgba10X6FormatsFeaturesEXT = 1000344000,

    /// Provided by [`VK_EXT_DIRECTFB_SURFACE`]
    DirectfbSurfaceCreateInfoEXT = 1000346000,

    /// Provided by [`VK_EXT_VERTEX_INPUT_DYNAMIC_STATE`]
    PhysicalDeviceVertexInputDynamicStateFeaturesEXT = 1000352000,

    /// Provided by [`VK_EXT_SHADER_OBJECT, VKEXTVERTEX_INPUT_DYNAMIC_STATE`]
    VertexInputBindingDescription2EXT = 1000352001,

    /// Provided by [`VK_EXT_SHADER_OBJECT, VKEXTVERTEX_INPUT_DYNAMIC_STATE`]
    VertexInputAttributeDescription2EXT = 1000352002,

    /// Provided by [`VK_EXT_PHYSICAL_DEVICE_DRM`]
    PhysicalDeviceDrmPropertiesEXT = 1000353000,

    /// Provided by [`VK_EXT_DEVICE_ADDRESS_BINDING_REPORT`]
    PhysicalDeviceAddressBindingReportFeaturesEXT = 1000354000,

    /// Provided by [`VK_EXT_DEVICE_ADDRESS_BINDING_REPORT`]
    DeviceAddressBindingCallbackDataEXT = 1000354001,

    /// Provided by [`VK_EXT_DEPTH_CLIP_CONTROL`]
    PhysicalDeviceDepthClipControlFeaturesEXT = 1000355000,

    /// Provided by [`VK_EXT_DEPTH_CLIP_CONTROL`]
    PipelineViewportDepthClipControlCreateInfoEXT = 1000355001,

    /// Provided by [`VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART`]
    PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT = 1000356000,

    /// Provided by [`VkFuchsiaexternal_MEMORY`]
    ImportMemoryZirconHandleInfoFuchsia = 1000364000,

    /// Provided by [`VkFuchsiaexternal_MEMORY`]
    MemoryZirconHandlePropertiesFuchsia = 1000364001,

    /// Provided by [`VkFuchsiaexternal_MEMORY`]
    MemoryGetZirconHandleInfoFuchsia = 1000364002,

    /// Provided by [`VkFuchsiaexternal_SEMAPHORE`]
    ImportSemaphoreZirconHandleInfoFuchsia = 1000365000,

    /// Provided by [`VkFuchsiaexternal_SEMAPHORE`]
    SemaphoreGetZirconHandleInfoFuchsia = 1000365001,

    /// Provided by [`VkFuchsiabuffer_COLLECTION`]
    BufferCollectionCreateInfoFuchsia = 1000366000,

    /// Provided by [`VkFuchsiabuffer_COLLECTION`]
    ImportMemoryBufferCollectionFuchsia = 1000366001,

    /// Provided by [`VkFuchsiabuffer_COLLECTION`]
    BufferCollectionImageCreateInfoFuchsia = 1000366002,

    /// Provided by [`VkFuchsiabuffer_COLLECTION`]
    BufferCollectionPropertiesFuchsia = 1000366003,

    /// Provided by [`VkFuchsiabuffer_COLLECTION`]
    BufferConstraintsInfoFuchsia = 1000366004,

    /// Provided by [`VkFuchsiabuffer_COLLECTION`]
    BufferCollectionBufferCreateInfoFuchsia = 1000366005,

    /// Provided by [`VkFuchsiabuffer_COLLECTION`]
    ImageConstraintsInfoFuchsia = 1000366006,

    /// Provided by [`VkFuchsiabuffer_COLLECTION`]
    ImageFormatConstraintsInfoFuchsia = 1000366007,

    /// Provided by [`VkFuchsiabuffer_COLLECTION`]
    SysmemColorSpaceFuchsia = 1000366008,

    /// Provided by [`VkFuchsiabuffer_COLLECTION`]
    BufferCollectionConstraintsInfoFuchsia = 1000366009,

    /// Provided by [`VkHuaweisubpass_SHADING`]
    SubpassShadingPipelineCreateInfoHuawei = 1000369000,

    /// Provided by [`VkHuaweisubpass_SHADING`]
    PhysicalDeviceSubpassShadingFeaturesHuawei = 1000369001,

    /// Provided by [`VkHuaweisubpass_SHADING`]
    PhysicalDeviceSubpassShadingPropertiesHuawei = 1000369002,

    /// Provided by [`VkHuaweiinvocation_MASK`]
    PhysicalDeviceInvocationMaskFeaturesHuawei = 1000370000,

    /// Provided by [`VK_NV_EXTERNAL_MEMORY_RDMA`]
    MemoryGetRemoteAddressInfoNV = 1000371000,

    /// Provided by [`VK_NV_EXTERNAL_MEMORY_RDMA`]
    PhysicalDeviceEXTernalMemoryRdmaFeaturesNV = 1000371001,

    /// Provided by [`VK_EXT_PIPELINE_PROPERTIES`]
    PipelinePropertiesIdentifierEXT = 1000372000,

    /// Provided by [`VK_EXT_PIPELINE_PROPERTIES`]
    PhysicalDevicePipelinePropertiesFeaturesEXT = 1000372001,

    /// Provided by [`VK_EXT_FRAME_BOUNDARY`]
    PhysicalDeviceFrameBoundaryFeaturesEXT = 1000375000,

    /// Provided by [`VK_EXT_FRAME_BOUNDARY`]
    FrameBoundaryEXT = 1000375001,

    /// Provided by [`VK_EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED`]
    PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT = 1000376000,

    /// Provided by [`VK_EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED`]
    SubpassResolvePerformanceQueryEXT = 1000376001,

    /// Provided by [`VK_EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED`]
    MultisampledRenderToSingleSampledInfoEXT = 1000376002,

    /// Provided by [`VK_EXT_EXTENDED_DYNAMIC_STATE2`]
    PhysicalDeviceEXTendedDynamicState2FeaturesEXT = 1000377000,

    /// Provided by [`VkQnxscreen_SURFACE`]
    ScreenSurfaceCreateInfoQnx = 1000378000,

    /// Provided by [`VK_EXT_COLOR_WRITE_ENABLE`]
    PhysicalDeviceColorWriteEnableFeaturesEXT = 1000381000,

    /// Provided by [`VK_EXT_COLOR_WRITE_ENABLE`]
    PipelineColorWriteCreateInfoEXT = 1000381001,

    /// Provided by [`VK_EXT_PRIMITIVES_GENERATED_QUERY`]
    PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT = 1000382000,

    /// Provided by [`VK_KHR_RAY_TRACING_MAINTENANCE1`]
    PhysicalDeviceRayTracingMaintenance1FeaturesKHR = 1000386000,

    /// Provided by [`VK_EXT_IMAGE_VIEW_MIN_LOD`]
    PhysicalDeviceImageViewMinLodFeaturesEXT = 1000391000,

    /// Provided by [`VK_EXT_IMAGE_VIEW_MIN_LOD`]
    ImageViewMinLodCreateInfoEXT = 1000391001,

    /// Provided by [`VK_EXT_MULTI_DRAW`]
    PhysicalDeviceMultiDrawFeaturesEXT = 1000392000,

    /// Provided by [`VK_EXT_MULTI_DRAW`]
    PhysicalDeviceMultiDrawPropertiesEXT = 1000392001,

    /// Provided by [`VK_EXT_IMAGE_2D_VIEW_OF_3D`]
    PhysicalDeviceImage2DViewOf3DFeaturesEXT = 1000393000,

    /// Provided by [`VK_EXT_SHADER_TILE_IMAGE`]
    PhysicalDeviceShaderTileImageFeaturesEXT = 1000395000,

    /// Provided by [`VK_EXT_SHADER_TILE_IMAGE`]
    PhysicalDeviceShaderTileImagePropertiesEXT = 1000395001,

    /// Provided by [`VK_EXT_OPACITY_MICROMAP`]
    MicromapBuildInfoEXT = 1000396000,

    /// Provided by [`VK_EXT_OPACITY_MICROMAP`]
    MicromapVersionInfoEXT = 1000396001,

    /// Provided by [`VK_EXT_OPACITY_MICROMAP`]
    CopyMicromapInfoEXT = 1000396002,

    /// Provided by [`VK_EXT_OPACITY_MICROMAP`]
    CopyMicromapToMemoryInfoEXT = 1000396003,

    /// Provided by [`VK_EXT_OPACITY_MICROMAP`]
    CopyMemoryToMicromapInfoEXT = 1000396004,

    /// Provided by [`VK_EXT_OPACITY_MICROMAP`]
    PhysicalDeviceOpacityMicromapFeaturesEXT = 1000396005,

    /// Provided by [`VK_EXT_OPACITY_MICROMAP`]
    PhysicalDeviceOpacityMicromapPropertiesEXT = 1000396006,

    /// Provided by [`VK_EXT_OPACITY_MICROMAP`]
    MicromapCreateInfoEXT = 1000396007,

    /// Provided by [`VK_EXT_OPACITY_MICROMAP`]
    MicromapBuildSizesInfoEXT = 1000396008,

    /// Provided by [`VK_EXT_OPACITY_MICROMAP`]
    AccelerationStructureTrianglesOpacityMicromapEXT = 1000396009,

    /// Provided by [`VK_NV_DISPLACEMENT_MICROMAP`]
    PhysicalDeviceDisplacementMicromapFeaturesNV = 1000397000,

    /// Provided by [`VK_NV_DISPLACEMENT_MICROMAP`]
    PhysicalDeviceDisplacementMicromapPropertiesNV = 1000397001,

    /// Provided by [`VK_NV_DISPLACEMENT_MICROMAP`]
    AccelerationStructureTrianglesDisplacementMicromapNV = 1000397002,

    /// Provided by [`VkHuaweicluster_CULLING_SHADER`]
    PhysicalDeviceClusterCullingShaderFeaturesHuawei = 1000404000,

    /// Provided by [`VkHuaweicluster_CULLING_SHADER`]
    PhysicalDeviceClusterCullingShaderPropertiesHuawei = 1000404001,

    /// Provided by [`VkHuaweicluster_CULLING_SHADER`]
    PhysicalDeviceClusterCullingShaderVrsFeaturesHuawei = 1000404002,

    /// Provided by [`VK_EXT_BORDER_COLOR_SWIZZLE`]
    PhysicalDeviceBorderColorSwizzleFeaturesEXT = 1000411000,

    /// Provided by [`VK_EXT_BORDER_COLOR_SWIZZLE`]
    SamplerBorderColorComponentMappingCreateInfoEXT = 1000411001,

    /// Provided by [`VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY`]
    PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT = 1000412000,

    /// Provided by [`VkArmshader_CORE_PROPERTIES`]
    PhysicalDeviceShaderCorePropertiesArm = 1000415000,

    /// Provided by [`VK_KHR_SHADER_SUBGROUP_ROTATE`]
    PhysicalDeviceShaderSubgroupRotateFeaturesKHR = 1000416000,

    /// Provided by [`VkArmscheduling_CONTROLS`]
    DeviceQueueShaderCoreControlCreateInfoArm = 1000417000,

    /// Provided by [`VkArmscheduling_CONTROLS`]
    PhysicalDeviceSchedulingControlsFeaturesArm = 1000417001,

    /// Provided by [`VkArmscheduling_CONTROLS`]
    PhysicalDeviceSchedulingControlsPropertiesArm = 1000417002,

    /// Provided by [`VK_EXT_IMAGE_SLICED_VIEW_OF_3D`]
    PhysicalDeviceImageSlicedViewOf3DFeaturesEXT = 1000418000,

    /// Provided by [`VK_EXT_IMAGE_SLICED_VIEW_OF_3D`]
    ImageViewSlicedCreateInfoEXT = 1000418001,

    /// Provided by [`VkValvedescriptor_SET_HOST_MAPPING`]
    PhysicalDeviceDescriptorSetHostMappingFeaturesValve = 1000420000,

    /// Provided by [`VkValvedescriptor_SET_HOST_MAPPING`]
    DescriptorSetBindingReferenceValve = 1000420001,

    /// Provided by [`VkValvedescriptor_SET_HOST_MAPPING`]
    DescriptorSetLayoutHostMappingInfoValve = 1000420002,

    /// Provided by [`VK_EXT_DEPTH_CLAMP_ZERO_ONE`]
    PhysicalDeviceDepthClampZeroOneFeaturesEXT = 1000421000,

    /// Provided by [`VK_EXT_NON_SEAMLESS_CUBE_MAP`]
    PhysicalDeviceNonSeamlessCubeMapFeaturesEXT = 1000422000,

    /// Provided by [`VkArmrender_PASS_STRIPED`]
    PhysicalDeviceRenderPassStripedFeaturesArm = 1000424000,

    /// Provided by [`VkArmrender_PASS_STRIPED`]
    PhysicalDeviceRenderPassStripedPropertiesArm = 1000424001,

    /// Provided by [`VkArmrender_PASS_STRIPED`]
    RenderPassStripeBeginInfoArm = 1000424002,

    /// Provided by [`VkArmrender_PASS_STRIPED`]
    RenderPassStripeInfoArm = 1000424003,

    /// Provided by [`VkArmrender_PASS_STRIPED`]
    RenderPassStripeSubmitInfoArm = 1000424004,

    /// Provided by [`VkQcomfragment_DENSITY_MAP_OFFSET`]
    PhysicalDeviceFragmentDensityMapOffsetFeaturesQcom = 1000425000,

    /// Provided by [`VkQcomfragment_DENSITY_MAP_OFFSET`]
    PhysicalDeviceFragmentDensityMapOffsetPropertiesQcom = 1000425001,

    /// Provided by [`VkQcomfragment_DENSITY_MAP_OFFSET`]
    SubpassFragmentDensityMapOffsetEndInfoQcom = 1000425002,

    /// Provided by [`VK_NV_COPY_MEMORY_INDIRECT`]
    PhysicalDeviceCopyMemoryIndirectFeaturesNV = 1000426000,

    /// Provided by [`VK_NV_COPY_MEMORY_INDIRECT`]
    PhysicalDeviceCopyMemoryIndirectPropertiesNV = 1000426001,

    /// Provided by [`VK_NV_MEMORY_DECOMPRESSION`]
    PhysicalDeviceMemoryDecompressionFeaturesNV = 1000427000,

    /// Provided by [`VK_NV_MEMORY_DECOMPRESSION`]
    PhysicalDeviceMemoryDecompressionPropertiesNV = 1000427001,

    /// Provided by [`VK_NV_DEVICE_GENERATED_COMMANDS_COMPUTE`]
    PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV = 1000428000,

    /// Provided by [`VK_NV_DEVICE_GENERATED_COMMANDS_COMPUTE`]
    ComputePipelineIndirectBufferInfoNV = 1000428001,

    /// Provided by [`VK_NV_DEVICE_GENERATED_COMMANDS_COMPUTE`]
    PipelineIndirectDeviceAddressInfoNV = 1000428002,

    /// Provided by [`VK_NV_LINEAR_COLOR_ATTACHMENT`]
    PhysicalDeviceLinearColorAttachmentFeaturesNV = 1000430000,

    /// Provided by [`VK_KHR_SHADER_MAXIMAL_RECONVERGENCE`]
    PhysicalDeviceShaderMaximalReconvergenceFeaturesKHR = 1000434000,

    /// Provided by [`VK_EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN`]
    PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT = 1000437000,

    /// Provided by [`VkQcomimage_PROCESSING`]
    PhysicalDeviceImageProcessingFeaturesQcom = 1000440000,

    /// Provided by [`VkQcomimage_PROCESSING`]
    PhysicalDeviceImageProcessingPropertiesQcom = 1000440001,

    /// Provided by [`VkQcomimage_PROCESSING`]
    ImageViewSampleWeightCreateInfoQcom = 1000440002,

    /// Provided by [`VK_EXT_NESTED_COMMAND_BUFFER`]
    PhysicalDeviceNestedCommandBufferFeaturesEXT = 1000451000,

    /// Provided by [`VK_EXT_NESTED_COMMAND_BUFFER`]
    PhysicalDeviceNestedCommandBufferPropertiesEXT = 1000451001,

    /// Provided by [`VK_EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED`]
    EXTernalMemoryAcquireUnmodifiedEXT = 1000453000,

    /// Provided by [`VK_EXT_EXTENDED_DYNAMIC_STATE3`]
    PhysicalDeviceEXTendedDynamicState3FeaturesEXT = 1000455000,

    /// Provided by [`VK_EXT_EXTENDED_DYNAMIC_STATE3`]
    PhysicalDeviceEXTendedDynamicState3PropertiesEXT = 1000455001,

    /// Provided by [`VK_EXT_SUBPASS_MERGE_FEEDBACK`]
    PhysicalDeviceSubpassMergeFeedbackFeaturesEXT = 1000458000,

    /// Provided by [`VK_EXT_SUBPASS_MERGE_FEEDBACK`]
    RenderPassCreationControlEXT = 1000458001,

    /// Provided by [`VK_EXT_SUBPASS_MERGE_FEEDBACK`]
    RenderPassCreationFeedbackCreateInfoEXT = 1000458002,

    /// Provided by [`VK_EXT_SUBPASS_MERGE_FEEDBACK`]
    RenderPassSubpassFeedbackCreateInfoEXT = 1000458003,

    /// Provided by [`VkLunargdirect_DRIVER_LOADING`]
    DirectDriverLoadingInfoLunarg = 1000459000,

    /// Provided by [`VkLunargdirect_DRIVER_LOADING`]
    DirectDriverLoadingListLunarg = 1000459001,

    /// Provided by [`VK_EXT_SHADER_MODULE_IDENTIFIER`]
    PhysicalDeviceShaderModuleIdentifierFeaturesEXT = 1000462000,

    /// Provided by [`VK_EXT_SHADER_MODULE_IDENTIFIER`]
    PhysicalDeviceShaderModuleIdentifierPropertiesEXT = 1000462001,

    /// Provided by [`VK_EXT_SHADER_MODULE_IDENTIFIER`]
    PipelineShaderStageModuleIdentifierCreateInfoEXT = 1000462002,

    /// Provided by [`VK_EXT_SHADER_MODULE_IDENTIFIER`]
    ShaderModuleIdentifierEXT = 1000462003,

    /// Provided by [`VK_EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS`]
    PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT = 1000342000,

    /// Provided by [`VK_NV_OPTICAL_FLOW`]
    PhysicalDeviceOpticalFlowFeaturesNV = 1000464000,

    /// Provided by [`VK_NV_OPTICAL_FLOW`]
    PhysicalDeviceOpticalFlowPropertiesNV = 1000464001,

    /// Provided by [`VK_NV_OPTICAL_FLOW`]
    OpticalFlowImageFormatInfoNV = 1000464002,

    /// Provided by [`VK_NV_OPTICAL_FLOW`]
    OpticalFlowImageFormatPropertiesNV = 1000464003,

    /// Provided by [`VK_NV_OPTICAL_FLOW`]
    OpticalFlowSessionCreateInfoNV = 1000464004,

    /// Provided by [`VK_NV_OPTICAL_FLOW`]
    OpticalFlowExecuteInfoNV = 1000464005,

    /// Provided by [`VK_NV_OPTICAL_FLOW`]
    OpticalFlowSessionCreatePrivateDataInfoNV = 1000464010,

    /// Provided by [`VK_EXT_LEGACY_DITHERING`]
    PhysicalDeviceLegacyDitheringFeaturesEXT = 1000465000,

    /// Provided by [`VK_EXT_PIPELINE_PROTECTED_ACCESS`]
    PhysicalDevicePipelineProtectedAccessFeaturesEXT = 1000466000,

    /// Provided by [`VkAndroidexternal_FORMAT_RESOLVE`]
    PhysicalDeviceEXTernalFormatResolveFeaturesAndroid = 1000468000,

    /// Provided by [`VkAndroidexternal_FORMAT_RESOLVE`]
    PhysicalDeviceEXTernalFormatResolvePropertiesAndroid = 1000468001,

    /// Provided by [`VkAndroidexternal_FORMAT_RESOLVE`]
    AndroidHardwareBufferFormatResolvePropertiesAndroid = 1000468002,

    /// Provided by [`VK_KHR_MAINTENANCE5`]
    PhysicalDeviceMaintenance5FeaturesKHR = 1000470000,

    /// Provided by [`VK_KHR_MAINTENANCE5`]
    PhysicalDeviceMaintenance5PropertiesKHR = 1000470001,

    /// Provided by [`VK_KHR_MAINTENANCE5`]
    RenderingAreaInfoKHR = 1000470003,

    /// Provided by [`VK_KHR_MAINTENANCE5`]
    DeviceImageSubresourceInfoKHR = 1000470004,

    /// Provided by [`VK_KHR_MAINTENANCE5`]
    SubresourceLayout2KHR = 1000338002,

    /// Provided by [`VK_KHR_MAINTENANCE5`]
    ImageSubresource2KHR = 1000338003,

    /// Provided by [`VK_KHR_MAINTENANCE5`]
    PipelineCreateFlags2CreateInfoKHR = 1000470005,

    /// Provided by [`VK_KHR_MAINTENANCE5`]
    BufferUsageFlags2CreateInfoKHR = 1000470006,

    /// Provided by [`VK_KHR_RAY_TRACING_POSITION_FETCH`]
    PhysicalDeviceRayTracingPositionFetchFeaturesKHR = 1000481000,

    /// Provided by [`VK_EXT_SHADER_OBJECT`]
    PhysicalDeviceShaderObjectFeaturesEXT = 1000482000,

    /// Provided by [`VK_EXT_SHADER_OBJECT`]
    PhysicalDeviceShaderObjectPropertiesEXT = 1000482001,

    /// Provided by [`VK_EXT_SHADER_OBJECT`]
    ShaderCreateInfoEXT = 1000482002,

    /// Provided by [`VkQcomtile_PROPERTIES`]
    PhysicalDeviceTilePropertiesFeaturesQcom = 1000484000,

    /// Provided by [`VkQcomtile_PROPERTIES`]
    TilePropertiesQcom = 1000484001,

    /// Provided by [`VkSecamigo_PROFILING`]
    PhysicalDeviceAmigoProfilingFeaturesSec = 1000485000,

    /// Provided by [`VkSecamigo_PROFILING`]
    AmigoProfilingSubmitInfoSec = 1000485001,

    /// Provided by [`VkQcommultiview_PER_VIEW_VIEWPORTS`]
    PhysicalDeviceMultiviewPerViewViewportsFeaturesQcom = 1000488000,

    /// Provided by [`VK_NV_RAY_TRACING_INVOCATION_REORDER`]
    PhysicalDeviceRayTracingInvocationReorderFeaturesNV = 1000490000,

    /// Provided by [`VK_NV_RAY_TRACING_INVOCATION_REORDER`]
    PhysicalDeviceRayTracingInvocationReorderPropertiesNV = 1000490001,

    /// Provided by [`VK_NV_EXTENDED_SPARSE_ADDRESS_SPACE`]
    PhysicalDeviceEXTendedSparseAddressSpaceFeaturesNV = 1000492000,

    /// Provided by [`VK_NV_EXTENDED_SPARSE_ADDRESS_SPACE`]
    PhysicalDeviceEXTendedSparseAddressSpacePropertiesNV = 1000492001,

    /// Provided by [`VK_EXT_MUTABLE_DESCRIPTOR_TYPE`]
    PhysicalDeviceMutableDescriptorTypeFeaturesEXT = 1000351000,

    /// Provided by [`VK_EXT_MUTABLE_DESCRIPTOR_TYPE`]
    MutableDescriptorTypeCreateInfoEXT = 1000351002,

    /// Provided by [`VK_EXT_LEGACY_VERTEX_ATTRIBUTES`]
    PhysicalDeviceLegacyVertexAttributesFeaturesEXT = 1000495000,

    /// Provided by [`VK_EXT_LEGACY_VERTEX_ATTRIBUTES`]
    PhysicalDeviceLegacyVertexAttributesPropertiesEXT = 1000495001,

    /// Provided by [`VK_EXT_LAYER_SETTINGS`]
    LayerSettingsCreateInfoEXT = 1000496000,

    /// Provided by [`VkArmshader_CORE_BUILTINS`]
    PhysicalDeviceShaderCoreBuiltinsFeaturesArm = 1000497000,

    /// Provided by [`VkArmshader_CORE_BUILTINS`]
    PhysicalDeviceShaderCoreBuiltinsPropertiesArm = 1000497001,

    /// Provided by [`VK_EXT_PIPELINE_LIBRARY_GROUP_HANDLES`]
    PhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT = 1000498000,

    /// Provided by [`VK_EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS`]
    PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT = 1000499000,

    /// Provided by [`VK_NV_LOW_LATENCY2`]
    LatencySleepModeInfoNV = 1000505000,

    /// Provided by [`VK_NV_LOW_LATENCY2`]
    LatencySleepInfoNV = 1000505001,

    /// Provided by [`VK_NV_LOW_LATENCY2`]
    SetLatencyMarkerInfoNV = 1000505002,

    /// Provided by [`VK_NV_LOW_LATENCY2`]
    GetLatencyMarkerInfoNV = 1000505003,

    /// Provided by [`VK_NV_LOW_LATENCY2`]
    LatencyTimingsFrameReportNV = 1000505004,

    /// Provided by [`VK_NV_LOW_LATENCY2`]
    LatencySubmissionPresentIdNV = 1000505005,

    /// Provided by [`VK_NV_LOW_LATENCY2`]
    OutOfBandQueueTypeInfoNV = 1000505006,

    /// Provided by [`VK_NV_LOW_LATENCY2`]
    SwapchainLatencyCreateInfoNV = 1000505007,

    /// Provided by [`VK_NV_LOW_LATENCY2`]
    LatencySurfaceCapabilitiesNV = 1000505008,

    /// Provided by [`VK_KHR_COOPERATIVE_MATRIX`]
    PhysicalDeviceCooperativeMatrixFeaturesKHR = 1000506000,

    /// Provided by [`VK_KHR_COOPERATIVE_MATRIX`]
    CooperativeMatrixPropertiesKHR = 1000506001,

    /// Provided by [`VK_KHR_COOPERATIVE_MATRIX`]
    PhysicalDeviceCooperativeMatrixPropertiesKHR = 1000506002,

    /// Provided by [`VkQcommultiview_PER_VIEW_RENDER_AREAS`]
    PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQcom = 1000510000,

    /// Provided by [`VkQcommultiview_PER_VIEW_RENDER_AREAS`]
    MultiviewPerViewRenderAreasRenderPassBeginInfoQcom = 1000510001,

    /// Provided by [`VK_KHR_VIDEO_DECODE_AV1`]
    VideoDecodeAv1CapabilitiesKHR = 1000512000,

    /// Provided by [`VK_KHR_VIDEO_DECODE_AV1`]
    VideoDecodeAv1PictureInfoKHR = 1000512001,

    /// Provided by [`VK_KHR_VIDEO_DECODE_AV1`]
    VideoDecodeAv1ProfileInfoKHR = 1000512003,

    /// Provided by [`VK_KHR_VIDEO_DECODE_AV1`]
    VideoDecodeAv1SessionParametersCreateInfoKHR = 1000512004,

    /// Provided by [`VK_KHR_VIDEO_DECODE_AV1`]
    VideoDecodeAv1DpbSlotInfoKHR = 1000512005,

    /// Provided by [`VK_KHR_VIDEO_MAINTENANCE1`]
    PhysicalDeviceVideoMaintenance1FeaturesKHR = 1000515000,

    /// Provided by [`VK_KHR_VIDEO_MAINTENANCE1`]
    VideoInlineQueryInfoKHR = 1000515001,

    /// Provided by [`VK_NV_PER_STAGE_DESCRIPTOR_SET`]
    PhysicalDevicePerStageDescriptorSetFeaturesNV = 1000516000,

    /// Provided by [`VkQcomimage_PROCESSING2`]
    PhysicalDeviceImageProcessing2FeaturesQcom = 1000518000,

    /// Provided by [`VkQcomimage_PROCESSING2`]
    PhysicalDeviceImageProcessing2PropertiesQcom = 1000518001,

    /// Provided by [`VkQcomimage_PROCESSING2`]
    SamplerBlockMatchWindowCreateInfoQcom = 1000518002,

    /// Provided by [`VkQcomfilter_CUBIC_WEIGHTS`]
    SamplerCubicWeightsCreateInfoQcom = 1000519000,

    /// Provided by [`VkQcomfilter_CUBIC_WEIGHTS`]
    PhysicalDeviceCubicWeightsFeaturesQcom = 1000519001,

    /// Provided by [`VkQcomfilter_CUBIC_WEIGHTS`]
    BlitImageCubicWeightsInfoQcom = 1000519002,

    /// Provided by [`VkQcomycbcr_DEGAMMA`]
    PhysicalDeviceYcbcrDegammaFeaturesQcom = 1000520000,

    /// Provided by [`VkQcomycbcr_DEGAMMA`]
    SamplerYcbcrConversionYcbcrDegammaCreateInfoQcom = 1000520001,

    /// Provided by [`VkQcomfilter_CUBIC_CLAMP`]
    PhysicalDeviceCubicClampFeaturesQcom = 1000521000,

    /// Provided by [`VK_EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE`]
    PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT = 1000524000,

    /// Provided by [`VK_KHR_VERTEX_ATTRIBUTE_DIVISOR`]
    PhysicalDeviceVertexAttributeDivisorPropertiesKHR = 1000525000,

    /// Provided by [`VK_KHR_VERTEX_ATTRIBUTE_DIVISOR`]
    PipelineVertexInputDivisorStateCreateInfoKHR = 1000190001,

    /// Provided by [`VK_KHR_VERTEX_ATTRIBUTE_DIVISOR`]
    PhysicalDeviceVertexAttributeDivisorFeaturesKHR = 1000190002,

    /// Provided by [`VK_KHR_SHADER_FLOAT_CONTROLS2`]
    PhysicalDeviceShaderFloatControls2FeaturesKHR = 1000528000,

    /// Provided by [`VkQnxexternal_MEMORY_SCREEN_BUFFER`]
    ScreenBufferPropertiesQnx = 1000529000,

    /// Provided by [`VkQnxexternal_MEMORY_SCREEN_BUFFER`]
    ScreenBufferFormatPropertiesQnx = 1000529001,

    /// Provided by [`VkQnxexternal_MEMORY_SCREEN_BUFFER`]
    ImportScreenBufferInfoQnx = 1000529002,

    /// Provided by [`VkQnxexternal_MEMORY_SCREEN_BUFFER`]
    EXTernalFormatQnx = 1000529003,

    /// Provided by [`VkQnxexternal_MEMORY_SCREEN_BUFFER`]
    PhysicalDeviceEXTernalMemoryScreenBufferFeaturesQnx = 1000529004,

    /// Provided by [`VkMsftlayered_DRIVER`]
    PhysicalDeviceLayeredDriverPropertiesMsft = 1000530000,

    /// Provided by [`VK_KHR_INDEX_TYPE_UINT8`]
    PhysicalDeviceIndexTypeUint8FeaturesKHR = 1000265000,

    /// Provided by [`VK_KHR_LINE_RASTERIZATION`]
    PhysicalDeviceLineRasterizationFeaturesKHR = 1000259000,

    /// Provided by [`VK_KHR_LINE_RASTERIZATION`]
    PipelineRasterizationLineStateCreateInfoKHR = 1000259001,

    /// Provided by [`VK_KHR_LINE_RASTERIZATION`]
    PhysicalDeviceLineRasterizationPropertiesKHR = 1000259002,

    /// Provided by [`VK_KHR_CALIBRATED_TIMESTAMPS`]
    CalibratedTimestampInfoKHR = 1000184000,

    /// Provided by [`VK_KHR_SHADER_EXPECT_ASSUME`]
    PhysicalDeviceShaderExpectAssumeFeaturesKHR = 1000544000,

    /// Provided by [`VK_KHR_MAINTENANCE6`]
    PhysicalDeviceMaintenance6FeaturesKHR = 1000545000,

    /// Provided by [`VK_KHR_MAINTENANCE6`]
    PhysicalDeviceMaintenance6PropertiesKHR = 1000545001,

    /// Provided by [`VK_KHR_MAINTENANCE6`]
    BindMemoryStatusKHR = 1000545002,

    /// Provided by [`VK_KHR_MAINTENANCE6`]
    BindDescriptorSetsInfoKHR = 1000545003,

    /// Provided by [`VK_KHR_MAINTENANCE6`]
    PushConstantsInfoKHR = 1000545004,

    /// Provided by [`VK_KHR_MAINTENANCE6`] with [`VK_KHR_PUSH_DESCRIPTOR`]
    PushDescriptorSetInfoKHR = 1000545005,

    /// Provided by [`VK_KHR_MAINTENANCE6`] with [`VK_KHR_PUSH_DESCRIPTOR`]
    PushDescriptorSetWithTemplateInfoKHR = 1000545006,

    /// Provided by [`VK_KHR_MAINTENANCE6`] with [`VK_EXT_DESCRIPTOR_BUFFER`]
    SetDescriptorBufferOffsetsInfoEXT = 1000545007,

    /// Provided by [`VK_KHR_MAINTENANCE6`] with [`VK_EXT_DESCRIPTOR_BUFFER`]
    BindDescriptorBufferEmbeddedSamplersInfoEXT = 1000545008,

    /// Provided by [`VK_NV_DESCRIPTOR_POOL_OVERALLOCATION`]
    PhysicalDeviceDescriptorPoolOverallocationFeaturesNV = 1000546000,

    /// Provided by [`VK_NV_RAW_ACCESS_CHAINS`]
    PhysicalDeviceRawAccessChainsFeaturesNV = 1000555000,

    /// Provided by [`VK_NV_SHADER_ATOMIC_FLOAT16_VECTOR`]
    PhysicalDeviceShaderAtomicFloat16VectorFeaturesNV = 1000563000,

    /// Provided by [`VK_EXT_SHADER_REPLICATED_COMPOSITES`]
    PhysicalDeviceShaderReplicatedCompositesFeaturesEXT = 1000564000,

    /// Provided by [`VK_NV_RAY_TRACING_VALIDATION`]
    PhysicalDeviceRayTracingValidationFeaturesNV = 1000568000,

    /// Provided by [`VK_MESA_IMAGE_ALIGNMENT_CONTROL`]
    PhysicalDeviceImageAlignmentControlFeaturesMESA = 1000575000,

    /// Provided by [`VK_MESA_IMAGE_ALIGNMENT_CONTROL`]
    PhysicalDeviceImageAlignmentControlPropertiesMESA = 1000575001,

    /// Provided by [`VK_MESA_IMAGE_ALIGNMENT_CONTROL`]
    ImageAlignmentControlCreateInfoMESA = 1000575002,
}
