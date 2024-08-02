// rustdoc imports
#[allow(unused_imports)]
use crate::{
    ext_debug_utils, khr_surface, khr_swapchain, VK_VERSION_1_0, VK_VERSION_1_1, VK_VERSION_1_2,
    VK_VERSION_1_3,
};

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

    /// Provided by [`khr_swapchain`]
    SwapchainCreateInfoKHR = 1000001000,

    /// Provided by [`khr_swapchain`]
    PresentInfoKHR = 1000001001,

    /// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with [`khr_surface`]
    DeviceGroupPresentCapabilitiesKHR = 1000060007,

    /// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with [`khr_swapchain`]
    ImageSwapchainCreateInfoKHR = 1000060008,

    /// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with [`khr_swapchain`]
    BindImageMemorySwapchainInfoKHR = 1000060009,

    /// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with [`khr_swapchain`]
    AcquireNextImageInfoKHR = 1000060010,

    /// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with [`khr_swapchain`]
    DeviceGroupPresentInfoKHR = 1000060011,

    /// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with [`khr_swapchain`]
    DeviceGroupSwapchainCreateInfoKHR = 1000060012,

    /// Provided by [`khr_display`]
    DisplayModeCreateInfoKHR = 1000002000,

    /// Provided by [`khr_display`]
    DisplaySurfaceCreateInfoKHR = 1000002001,

    /// Provided by [`khr_display_swapchain`]
    DisplayPresentInfoKHR = 1000003000,

    /// Provided by [`khr_xlib_surface`]
    XlibSurfaceCreateInfoKHR = 1000004000,

    /// Provided by [`khr_xcb_surface`]
    XcbSurfaceCreateInfoKHR = 1000005000,

    /// Provided by [`khr_wayland_surface`]
    WaylandSurfaceCreateInfoKHR = 1000006000,

    /// Provided by [`khr_android_surface`]
    AndroidSurfaceCreateInfoKHR = 1000008000,

    /// Provided by [`khr_win32_surface`]
    Win32SurfaceCreateInfoKHR = 1000009000,

    /// Provided by [`ext_debug_report`]
    DebugReportCallbackCreateInfoEXT = 1000011000,

    /// Provided by [`amd_raserization_order`]
    PipelineRasterizationStateRasterizationOrderAMD = 1000018000,

    /// Provided by [`ext_debug_marker`]
    DebugMarkerObjectNameInfoEXT = 1000022000,

    /// Provided by [`ext_debug_marker`]
    DebugMarkerObjectTagInfoEXT = 1000022001,

    /// Provided by [`ext_debug_marker`]
    DebugMarkerMarkerInfoEXT = 1000022002,

    /// Provided by [`khr_video_queue`]
    VideoProfileInfoKHR = 1000023000,

    /// Provided by [`khr_video_queue`]
    VideoCapabilitiesKHR = 1000023001,

    /// Provided by [`khr_video_queue`]
    VideoPictureResourceInfoKHR = 1000023002,

    /// Provided by [`khr_video_queue`]
    VideoSessionMemoryRequirementsKHR = 1000023003,

    /// Provided by [`khr_video_queue`]
    BindVideoSessionMemoryInfoKHR = 1000023004,

    /// Provided by [`khr_video_queue`]
    VideoSessionCreateInfoKHR = 1000023005,

    /// Provided by [`khr_video_queue`]
    VideoSessionParametersCreateInfoKHR = 1000023006,

    /// Provided by [`khr_video_queue`]
    VideoSessionParametersUpdateInfoKHR = 1000023007,

    /// Provided by [`khr_video_queue`]
    VideoBeginCodingInfoKHR = 1000023008,

    /// Provided by [`khr_video_queue`]
    VideoEndCodingInfoKHR = 1000023009,

    /// Provided by [`khr_video_queue`]
    VideoCodingControlInfoKHR = 1000023010,

    /// Provided by [`khr_video_queue`]
    VideoReferenceSlotInfoKHR = 1000023011,

    /// Provided by [`khr_video_queue`]
    QueueFamilyVideoPropertiesKHR = 1000023012,

    /// Provided by [`khr_video_queue`]
    VideoProfileListInfoKHR = 1000023013,

    /// Provided by [`khr_video_queue`]
    PhysicalDeviceVideoFormatInfoKHR = 1000023014,

    /// Provided by [`khr_video_queue`]
    VideoFormatPropertiesKHR = 1000023015,

    /// Provided by [`khr_video_queue`]
    QueueFamilyQueryResultStatusPropertiesKHR = 1000023016,

    /// Provided by [`khr_video_decode_queue`]
    VideoDecodeInfoKHR = 1000024000,

    /// Provided by [`khr_video_decode_queue`]
    VideoDecodeCapabilitiesKHR = 1000024001,

    /// Provided by [`khr_video_decode_queue`]
    VideoDecodeUsageInfoKHR = 1000024002,

    /// Provided by [`nv_dedicated_allocation`]
    DedicatedAllocationImageCreateInfoNV = 1000026000,

    /// Provided by [`nv_dedicated_allocation`]
    DedicatedAllocationBufferCreateInfoNV = 1000026001,

    /// Provided by [`nv_dedicated_allocation`]
    DedicatedAllocationMemoryAllocateInfoNV = 1000026002,

    /// Provided by [`ext_transform_feedback`]
    PhysicalDeviceTransformFeedbackFeaturesEXT = 1000028000,

    /// Provided by [`ext_transform_feedback`]
    PhysicalDeviceTransformFeedbackPropertiesEXT = 1000028001,

    /// Provided by [`ext_transform_feedback`]
    PipelineRasterizationStateStreamCreateInfoEXT = 1000028002,

    /// Provided by [`nvx_binary_import`]
    CuModuleCreateInfoNVx = 1000029000,

    /// Provided by [`nvx_binary_import`]
    CuFunctionCreateInfoNVx = 1000029001,

    /// Provided by [`nvx_binary_import`]
    CuLaunchInfoNVx = 1000029002,

    /// Provided by [`nvx_image_view_handle`]
    ImageViewHandleInfoNVx = 1000030000,

    /// Provided by [`nvx_image_view_handle`]
    ImageViewAddressPropertiesNVx = 1000030001,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264CapabilitiesKHR = 1000038000,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264SessionParametersCreateInfoKHR = 1000038001,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264SessionParametersAddInfoKHR = 1000038002,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264PictureInfoKHR = 1000038003,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264DpbSlotInfoKHR = 1000038004,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264NaluSliceInfoKHR = 1000038005,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264GopRemainingFrameInfoKHR = 1000038006,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264ProfileInfoKHR = 1000038007,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264RateControlInfoKHR = 1000038008,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264RateControlLayerInfoKHR = 1000038009,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264SessionCreateInfoKHR = 1000038010,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264QualityLevelPropertiesKHR = 1000038011,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264SessionParametersGetInfoKHR = 1000038012,

    /// Provided by [`khr_video_encode_h264`]
    VideoEncodeH264SessionParametersFeedbackInfoKHR = 1000038013,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265CapabilitiesKHR = 1000039000,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265SessionParametersCreateInfoKHR = 1000039001,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265SessionParametersAddInfoKHR = 1000039002,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265PictureInfoKHR = 1000039003,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265DpbSlotInfoKHR = 1000039004,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265NaluSliceSegmentInfoKHR = 1000039005,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265GopRemainingFrameInfoKHR = 1000039006,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265ProfileInfoKHR = 1000039007,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265RateControlInfoKHR = 1000039009,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265RateControlLayerInfoKHR = 1000039010,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265SessionCreateInfoKHR = 1000039011,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265QualityLevelPropertiesKHR = 1000039012,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265SessionParametersGetInfoKHR = 1000039013,

    /// Provided by [`khr_video_encode_h265`]
    VideoEncodeH265SessionParametersFeedbackInfoKHR = 1000039014,

    /// Provided by [`khr_video_decode_h264`]
    VideoDecodeH264CapabilitiesKHR = 1000040000,

    /// Provided by [`khr_video_decode_h264`]
    VideoDecodeH264PictureInfoKHR = 1000040001,

    /// Provided by [`khr_video_decode_h264`]
    VideoDecodeH264ProfileInfoKHR = 1000040003,

    /// Provided by [`khr_video_decode_h264`]
    VideoDecodeH264SessionParametersCreateInfoKHR = 1000040004,

    /// Provided by [`khr_video_decode_h264`]
    VideoDecodeH264SessionParametersAddInfoKHR = 1000040005,

    /// Provided by [`khr_video_decode_h264`]
    VideoDecodeH264DpbSlotInfoKHR = 1000040006,

    /// Provided by [`amd_texture_gather_bias_lod`]
    TextureLodGatherFormatPropertiesAMD = 1000041000,

    /// Provided by [`khr_dynamic_rendering`] with [`khr_fragment_shading_rate`]
    RenderingFragmentShadingRateAttachmentInfoKHR = 1000044006,

    /// Provided by [`khr_dynamic_rendering`] with [`ext_fragment_density_map`]
    RenderingFragmentDensityMapAttachmentInfoEXT = 1000044007,

    /// Provided by [`khr_dynamic_rendering`] with [`amd_mixed_attachment_samples`]
    AttachmentSampleCountInfoAMD = 1000044008,

    /// Provided by [`khr_dynamic_rendering`] with [`nvx_multiview_per_view_attributes`]
    MultiviewPerViewAttributesInfoNVx = 1000044009,

    /// Provided by [`gpgstream_descriptor_surface`]
    StreamDescriptorSurfaceCreateInfoGgp = 1000049000,

    /// Provided by [`nv_corner_sampled_image`]
    PhysicalDeviceCornerSampledImageFeaturesNV = 1000050000,

    /// Provided by [`nv_external_memory`]
    EXTernalMemoryImageCreateInfoNV = 1000056000,

    /// Provided by [`nv_external_memory`]
    ExportMemoryAllocateInfoNV = 1000056001,

    /// Provided by [`nv_external_memory_win32`]
    ImportMemoryWin32HandleInfoNV = 1000057000,

    /// Provided by [`nv_external_memory_win32`]
    ExportMemoryWin32HandleInfoNV = 1000057001,

    /// Provided by [`nv_win32_keyed_mutex`]
    Win32KeyedMutexAcquireReleaseInfoNV = 1000058000,

    /// Provided by [`ext_validation_flags`]
    ValidationFlagsEXT = 1000061000,

    /// Provided by [`nnvi_surface`]
    ViSurfaceCreateInfoNn = 1000062000,

    /// Provided by [`ext_astc_decode_mode`]
    ImageViewAstcDecodeModeEXT = 1000067000,

    /// Provided by [`ext_astc_decode_mode`]
    PhysicalDeviceAstcDecodeFeaturesEXT = 1000067001,

    /// Provided by [`ext_pipeline_robustness`]
    PipelineRobustnessCreateInfoEXT = 1000068000,

    /// Provided by [`ext_pipeline_robustness`]
    PhysicalDevicePipelineRobustnessFeaturesEXT = 1000068001,

    /// Provided by [`ext_pipeline_robustness`]
    PhysicalDevicePipelineRobustnessPropertiesEXT = 1000068002,

    /// Provided by [`khr_external_memory_win32`]
    ImportMemoryWin32HandleInfoKHR = 1000073000,

    /// Provided by [`khr_external_memory_win32`]
    ExportMemoryWin32HandleInfoKHR = 1000073001,

    /// Provided by [`khr_external_memory_win32`]
    MemoryWin32HandlePropertiesKHR = 1000073002,

    /// Provided by [`khr_external_memory_win32`]
    MemoryGetWin32HandleInfoKHR = 1000073003,

    /// Provided by [`khr_external_memory_fd`]
    ImportMemoryFdInfoKHR = 1000074000,

    /// Provided by [`khr_external_memory_fd`]
    MemoryFdPropertiesKHR = 1000074001,

    /// Provided by [`khr_external_memory_fd`]
    MemoryGetFdInfoKHR = 1000074002,

    /// Provided by [`khr_win32_keyed_mutex`]
    Win32KeyedMutexAcquireReleaseInfoKHR = 1000075000,

    /// Provided by [`khr_external_semaphore_win32`]
    ImportSemaphoreWin32HandleInfoKHR = 1000078000,

    /// Provided by [`khr_external_semaphore_win32`]
    ExportSemaphoreWin32HandleInfoKHR = 1000078001,

    /// Provided by [`khr_external_semaphore_win32`]
    D3D12FenceSubmitInfoKHR = 1000078002,

    /// Provided by [`khr_external_semaphore_win32`]
    SemaphoreGetWin32HandleInfoKHR = 1000078003,

    /// Provided by [`khr_external_semaphore_fd`]
    ImportSemaphoreFdInfoKHR = 1000079000,

    /// Provided by [`khr_external_semaphore_fd`]
    SemaphoreGetFdInfoKHR = 1000079001,

    /// Provided by [`khr_push_descriptor`]
    PhysicalDevicePushDescriptorPropertiesKHR = 1000080000,

    /// Provided by [`ext_conditional_rendering`]
    CommandBufferInheritanceConditionalRenderingInfoEXT = 1000081000,

    /// Provided by [`ext_conditional_rendering`]
    PhysicalDeviceConditionalRenderingFeaturesEXT = 1000081001,

    /// Provided by [`ext_conditional_rendering`]
    ConditionalRenderingBeginInfoEXT = 1000081002,

    /// Provided by [`khr_incremental_present`]
    PresentRegionsKHR = 1000084000,

    /// Provided by [`nv_clip_space_w_scaling`]
    PipelineViewportWScalingStateCreateInfoNV = 1000087000,

    /// Provided by [`ext_display_surface_counter`]
    SurfaceCapabilities2EXT = 1000090000,

    /// Provided by [`ext_display_control`]
    DisplayPowerInfoEXT = 1000091000,

    /// Provided by [`ext_display_control`]
    DeviceEventInfoEXT = 1000091001,

    /// Provided by [`ext_display_control`]
    DisplayEventInfoEXT = 1000091002,

    /// Provided by [`ext_display_control`]
    SwapchainCounterCreateInfoEXT = 1000091003,

    /// Provided by [`googledisplay_timing`]
    PresentTimesInfoGoogle = 1000092000,

    /// Provided by [`nvx_multiview_per_view_attributes`]
    PhysicalDeviceMultiviewPerViewAttributesPropertiesNVx = 1000097000,

    /// Provided by [`nv_viewport_swizzle`]
    PipelineViewportSwizzleStateCreateInfoNV = 1000098000,

    /// Provided by [`ext_discard_rectangles`]
    PhysicalDeviceDiscardRectanglePropertiesEXT = 1000099000,

    /// Provided by [`ext_discard_rectangles`]
    PipelineDiscardRectangleStateCreateInfoEXT = 1000099001,

    /// Provided by [`ext_conservative_rasterization`]
    PhysicalDeviceConservativeRasterizationPropertiesEXT = 1000101000,

    /// Provided by [`ext_conservative_rasterization`]
    PipelineRasterizationConservativeStateCreateInfoEXT = 1000101001,

    /// Provided by [`ext_depth_clip_enable`]
    PhysicalDeviceDepthClipEnableFeaturesEXT = 1000102000,

    /// Provided by [`ext_depth_clip_enable`]
    PipelineRasterizationDepthClipStateCreateInfoEXT = 1000102001,

    /// Provided by [`ext_hdr_metadata`]
    HdrMetadataEXT = 1000105000,

    /// Provided by [`imgrelaxed_line_rasterization`]
    PhysicalDeviceRelaxedLineRasterizationFeaturesImg = 1000110000,

    /// Provided by [`khr_shared_presentable_image`]
    SharedPresentSurfaceCapabilitiesKHR = 1000111000,

    /// Provided by [`khr_external_fence_win32`]
    ImportFenceWin32HandleInfoKHR = 1000114000,

    /// Provided by [`khr_external_fence_win32`]
    ExportFenceWin32HandleInfoKHR = 1000114001,

    /// Provided by [`khr_external_fence_win32`]
    FenceGetWin32HandleInfoKHR = 1000114002,

    /// Provided by [`khr_external_fence_fd`]
    ImportFenceFdInfoKHR = 1000115000,

    /// Provided by [`khr_external_fence_fd`]
    FenceGetFdInfoKHR = 1000115001,

    /// Provided by [`khr_performance_query`]
    PhysicalDevicePerformanceQueryFeaturesKHR = 1000116000,

    /// Provided by [`khr_performance_query`]
    PhysicalDevicePerformanceQueryPropertiesKHR = 1000116001,

    /// Provided by [`khr_performance_query`]
    QueryPoolPerformanceCreateInfoKHR = 1000116002,

    /// Provided by [`khr_performance_query`]
    PerformanceQuerySubmitInfoKHR = 1000116003,

    /// Provided by [`khr_performance_query`]
    AcquireProfilingLockInfoKHR = 1000116004,

    /// Provided by [`khr_performance_query`]
    PerformanceCounterKHR = 1000116005,

    /// Provided by [`khr_performance_query`]
    PerformanceCounterDescriptionKHR = 1000116006,

    /// Provided by [`khr_get_surface_capabilities2`]
    PhysicalDeviceSurfaceInfo2KHR = 1000119000,

    /// Provided by [`khr_get_surface_capabilities2`]
    SurfaceCapabilities2KHR = 1000119001,

    /// Provided by [`khr_get_surface_capabilities2`]
    SurfaceFormat2KHR = 1000119002,

    /// Provided by [`khr_get_display_properties2`]
    DisplayProperties2KHR = 1000121000,

    /// Provided by [`khr_get_display_properties2`]
    DisplayPlaneProperties2KHR = 1000121001,

    /// Provided by [`khr_get_display_properties2`]
    DisplayModeProperties2KHR = 1000121002,

    /// Provided by [`khr_get_display_properties2`]
    DisplayPlaneInfo2KHR = 1000121003,

    /// Provided by [`khr_get_display_properties2`]
    DisplayPlaneCapabilities2KHR = 1000121004,

    /// Provided by [`mvk_ios_surface`]
    IosSurfaceCreateInfoMVK = 1000122000,

    /// Provided by [`mvk_macos_surface`]
    MacosSurfaceCreateInfoMVK = 1000123000,

    /// Provided by [`ext_debug_utils`]
    DebugUtilsObjectNameInfoEXT = 1000128000,

    /// Provided by [`ext_debug_utils`]
    DebugUtilsObjectTagInfoEXT = 1000128001,

    /// Provided by [`ext_debug_utils`]
    DebugUtilsLabelEXT = 1000128002,

    /// Provided by [`ext_debug_utils`]
    DebugUtilsMessengerCallbackDataEXT = 1000128003,

    /// Provided by [`ext_debug_utils`]
    DebugUtilsMessengerCreateInfoEXT = 1000128004,

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
    EXTernalFormatAndroid = 1000129005,

    /// Provided by [`android_external_memory_android_hardware_buffer`] with [`khr_format_feature_flags2`] or [`VK_VERSION_1_3`]
    AndroidHardwareBufferFormatProperties2Android = 1000129006,

    /// Provided by [`amd_xshader_enqueue`]
    PhysicalDeviceShaderEnqueueFeaturesAMDx = 1000134000,

    /// Provided by [`amd_xshader_enqueue`]
    PhysicalDeviceShaderEnqueuePropertiesAMDx = 1000134001,

    /// Provided by [`amd_xshader_enqueue`]
    ExecutionGraphPipelineScratchSizeAMDx = 1000134002,

    /// Provided by [`amd_xshader_enqueue`]
    ExecutionGraphPipelineCreateInfoAMDx = 1000134003,

    /// Provided by [`amd_xshader_enqueue`]
    PipelineShaderStageNodeCreateInfoAMDx = 1000134004,

    /// Provided by [`ext_sample_locations`]
    SampleLocationsInfoEXT = 1000143000,

    /// Provided by [`ext_sample_locations`]
    RenderPassSampleLocationsBeginInfoEXT = 1000143001,

    /// Provided by [`ext_sample_locations`]
    PipelineSampleLocationsStateCreateInfoEXT = 1000143002,

    /// Provided by [`ext_sample_locations`]
    PhysicalDeviceSampleLocationsPropertiesEXT = 1000143003,

    /// Provided by [`ext_sample_locations`]
    MultisamplePropertiesEXT = 1000143004,

    /// Provided by [`ext_blend_operation_advanced`]
    PhysicalDeviceBlendOperationAdvancedFeaturesEXT = 1000148000,

    /// Provided by [`ext_blend_operation_advanced`]
    PhysicalDeviceBlendOperationAdvancedPropertiesEXT = 1000148001,

    /// Provided by [`ext_blend_operation_advanced`]
    PipelineColorBlendAdvancedStateCreateInfoEXT = 1000148002,

    /// Provided by [`nv_fragment_coverage_to_color`]
    PipelineCoverageToColorStateCreateInfoNV = 1000149000,

    /// Provided by [`khr_acceleration_structure`]
    WriteDescriptorSetAccelerationStructureKHR = 1000150007,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureBuildGeometryInfoKHR = 1000150000,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureDeviceAddressInfoKHR = 1000150002,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureGeometryAabbsDataKHR = 1000150003,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureGeometryInstancesDataKHR = 1000150004,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureGeometryTrianglesDataKHR = 1000150005,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureGeometryKHR = 1000150006,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureVersionInfoKHR = 1000150009,

    /// Provided by [`khr_acceleration_structure`]
    CopyAccelerationStructureInfoKHR = 1000150010,

    /// Provided by [`khr_acceleration_structure`]
    CopyAccelerationStructureToMemoryInfoKHR = 1000150011,

    /// Provided by [`khr_acceleration_structure`]
    CopyMemoryToAccelerationStructureInfoKHR = 1000150012,

    /// Provided by [`khr_acceleration_structure`]
    PhysicalDeviceAccelerationStructureFeaturesKHR = 1000150013,

    /// Provided by [`khr_acceleration_structure`]
    PhysicalDeviceAccelerationStructurePropertiesKHR = 1000150014,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureCreateInfoKHR = 1000150017,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureBuildSizesInfoKHR = 1000150020,

    /// Provided by [`khr_ray_tracing_pipeline`]
    PhysicalDeviceRayTracingPipelineFeaturesKHR = 1000347000,

    /// Provided by [`khr_ray_tracing_pipeline`]
    PhysicalDeviceRayTracingPipelinePropertiesKHR = 1000347001,

    /// Provided by [`khr_ray_tracing_pipeline`]
    RayTracingPipelineCreateInfoKHR = 1000150015,

    /// Provided by [`khr_ray_tracing_pipeline`]
    RayTracingShaderGroupCreateInfoKHR = 1000150016,

    /// Provided by [`khr_ray_tracing_pipeline`]
    RayTracingPipelineInterfaceCreateInfoKHR = 1000150018,

    /// Provided by [`khr_ray_query`]
    PhysicalDeviceRayQueryFeaturesKHR = 1000348013,

    /// Provided by [`nv_framebuffer_mixed_samples`]
    PipelineCoverageModulationStateCreateInfoNV = 1000152000,

    /// Provided by [`nv_shader_sm_builtins`]
    PhysicalDeviceShaderSmBuiltinsFeaturesNV = 1000154000,

    /// Provided by [`nv_shader_sm_builtins`]
    PhysicalDeviceShaderSmBuiltinsPropertiesNV = 1000154001,

    /// Provided by [`ext_image_drm_format_modifier`]
    DrmFormatModifierPropertiesListEXT = 1000158000,

    /// Provided by [`ext_image_drm_format_modifier`]
    PhysicalDeviceImageDrmFormatModifierInfoEXT = 1000158002,

    /// Provided by [`ext_image_drm_format_modifier`]
    ImageDrmFormatModifierListCreateInfoEXT = 1000158003,

    /// Provided by [`ext_image_drm_format_modifier`]
    ImageDrmFormatModifierExplicitCreateInfoEXT = 1000158004,

    /// Provided by [`ext_image_drm_format_modifier`]
    ImageDrmFormatModifierPropertiesEXT = 1000158005,

    /// Provided by [`ext_image_drm_format_modifier`] with [`khr_format_feature_flags2`] or [`VK_VERSION_1_3`]
    DrmFormatModifierPropertiesList2EXT = 1000158006,

    /// Provided by [`ext_validation_cache`]
    ValidationCacheCreateInfoEXT = 1000160000,

    /// Provided by [`ext_validation_cache`]
    ShaderModuleValidationCacheCreateInfoEXT = 1000160001,

    /// Provided by [`khr_portability_subset`]
    PhysicalDevicePortabilitySubsetFeaturesKHR = 1000163000,

    /// Provided by [`khr_portability_subset`]
    PhysicalDevicePortabilitySubsetPropertiesKHR = 1000163001,

    /// Provided by [`nv_shading_rate_image`]
    PipelineViewportShadingRateImageStateCreateInfoNV = 1000164000,

    /// Provided by [`nv_shading_rate_image`]
    PhysicalDeviceShadingRateImageFeaturesNV = 1000164001,

    /// Provided by [`nv_shading_rate_image`]
    PhysicalDeviceShadingRateImagePropertiesNV = 1000164002,

    /// Provided by [`nv_shading_rate_image`]
    PipelineViewportCoarseSampleOrderStateCreateInfoNV = 1000164005,

    /// Provided by [`nv_ray_tracing`]
    RayTracingPipelineCreateInfoNV = 1000165000,

    /// Provided by [`nv_ray_tracing`]
    AccelerationStructureCreateInfoNV = 1000165001,

    /// Provided by [`nv_ray_tracing`]
    GeometryNV = 1000165003,

    /// Provided by [`nv_ray_tracing`]
    GeometryTrianglesNV = 1000165004,

    /// Provided by [`nv_ray_tracing`]
    GeometryAabbNV = 1000165005,

    /// Provided by [`nv_ray_tracing`]
    BindAccelerationStructureMemoryInfoNV = 1000165006,

    /// Provided by [`nv_ray_tracing`]
    WriteDescriptorSetAccelerationStructureNV = 1000165007,

    /// Provided by [`nv_ray_tracing`]
    AccelerationStructureMemoryRequirementsInfoNV = 1000165008,

    /// Provided by [`nv_ray_tracing`]
    PhysicalDeviceRayTracingPropertiesNV = 1000165009,

    /// Provided by [`nv_ray_tracing`]
    RayTracingShaderGroupCreateInfoNV = 1000165011,

    /// Provided by [`nv_ray_tracing`]
    AccelerationStructureInfoNV = 1000165012,

    /// Provided by [`nv_representative_fragment_test`]
    PhysicalDeviceRepresentativeFragmentTestFeaturesNV = 1000166000,

    /// Provided by [`nv_representative_fragment_test`]
    PipelineRepresentativeFragmentTestStateCreateInfoNV = 1000166001,

    /// Provided by [`ext_filter_cubic`]
    PhysicalDeviceImageViewImageFormatInfoEXT = 1000170000,

    /// Provided by [`ext_filter_cubic`]
    FilterCubicImageViewImageFormatPropertiesEXT = 1000170001,

    /// Provided by [`ext_external_memory_host`]
    ImportMemoryHostPointerInfoEXT = 1000178000,

    /// Provided by [`ext_external_memory_host`]
    MemoryHostPointerPropertiesEXT = 1000178001,

    /// Provided by [`ext_external_memory_host`]
    PhysicalDeviceEXTernalMemoryHostPropertiesEXT = 1000178002,

    /// Provided by [`khr_shader_clock`]
    PhysicalDeviceShaderClockFeaturesKHR = 1000181000,

    /// Provided by [`amd_pipeline_compiler_control`]
    PipelineCompilerControlCreateInfoAMD = 1000183000,

    /// Provided by [`amd_shader_core_properties`]
    PhysicalDeviceShaderCorePropertiesAMD = 1000185000,

    /// Provided by [`khr_video_decode_h265`]
    VideoDecodeH265CapabilitiesKHR = 1000187000,

    /// Provided by [`khr_video_decode_h265`]
    VideoDecodeH265SessionParametersCreateInfoKHR = 1000187001,

    /// Provided by [`khr_video_decode_h265`]
    VideoDecodeH265SessionParametersAddInfoKHR = 1000187002,

    /// Provided by [`khr_video_decode_h265`]
    VideoDecodeH265ProfileInfoKHR = 1000187003,

    /// Provided by [`khr_video_decode_h265`]
    VideoDecodeH265PictureInfoKHR = 1000187004,

    /// Provided by [`khr_video_decode_h265`]
    VideoDecodeH265DpbSlotInfoKHR = 1000187005,

    /// Provided by [`khr_global_priority`]
    DeviceQueueGlobalPriorityCreateInfoKHR = 1000174000,

    /// Provided by [`khr_global_priority`]
    PhysicalDeviceGlobalPriorityQueryFeaturesKHR = 1000388000,

    /// Provided by [`khr_global_priority`]
    QueueFamilyGlobalPriorityPropertiesKHR = 1000388001,

    /// Provided by [`amd_memory_overallocation_behavior`]
    DeviceMemoryOverallocationCreateInfoAMD = 1000189000,

    /// Provided by [`ext_vertex_attribute_divisor`]
    PhysicalDeviceVertexAttributeDivisorPropertiesEXT = 1000190000,

    /// Provided by [`gpgframe_token`]
    PresentFrameTokenGgp = 1000191000,

    /// Provided by [`nv_compute_shader_derivatives`]
    PhysicalDeviceComputeShaderDerivativesFeaturesNV = 1000201000,

    /// Provided by [`nv_mesh_shader`]
    PhysicalDeviceMeshShaderFeaturesNV = 1000202000,

    /// Provided by [`nv_mesh_shader`]
    PhysicalDeviceMeshShaderPropertiesNV = 1000202001,

    /// Provided by [`nv_shader_image_footprint`]
    PhysicalDeviceShaderImageFootprintFeaturesNV = 1000204000,

    /// Provided by [`nv_scissor_exclusive`]
    PipelineViewportExclusiveScissorStateCreateInfoNV = 1000205000,

    /// Provided by [`nv_scissor_exclusive`]
    PhysicalDeviceExclusiveScissorFeaturesNV = 1000205002,

    /// Provided by [`nv_device_diagnostic_checkpoints`]
    CheckpointDataNV = 1000206000,

    /// Provided by [`nv_device_diagnostic_checkpoints`]
    QueueFamilyCheckpointPropertiesNV = 1000206001,

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
    PhysicalDevicePciBusInfoPropertiesEXT = 1000212000,

    /// Provided by [`amd_display_native_hdr`]
    DisplayNativeHdrSurfaceCapabilitiesAMD = 1000213000,

    /// Provided by [`amd_display_native_hdr`]
    SwapchainDisplayNativeHdrCreateInfoAMD = 1000213001,

    /// Provided by [`fuchsia_imagepipe_surface`]
    ImagepipeSurfaceCreateInfoFuchsia = 1000214000,

    /// Provided by [`ext_metal_surface`]
    MetalSurfaceCreateInfoEXT = 1000217000,

    /// Provided by [`ext_fragment_density_map`]
    PhysicalDeviceFragmentDensityMapFeaturesEXT = 1000218000,

    /// Provided by [`ext_fragment_density_map`]
    PhysicalDeviceFragmentDensityMapPropertiesEXT = 1000218001,

    /// Provided by [`ext_fragment_density_map`]
    RenderPassFragmentDensityMapCreateInfoEXT = 1000218002,

    /// Provided by [`khr_fragment_shading_rate`]
    FragmentShadingRateAttachmentInfoKHR = 1000226000,

    /// Provided by [`khr_fragment_shading_rate`]
    PipelineFragmentShadingRateStateCreateInfoKHR = 1000226001,

    /// Provided by [`khr_fragment_shading_rate`]
    PhysicalDeviceFragmentShadingRatePropertiesKHR = 1000226002,

    /// Provided by [`khr_fragment_shading_rate`]
    PhysicalDeviceFragmentShadingRateFeaturesKHR = 1000226003,

    /// Provided by [`khr_fragment_shading_rate`]
    PhysicalDeviceFragmentShadingRateKHR = 1000226004,

    /// Provided by [`amd_shader_core_properties2`]
    PhysicalDeviceShaderCoreProperties2AMD = 1000227000,

    /// Provided by [`amd_device_coherent_memory`]
    PhysicalDeviceCoherentMemoryFeaturesAMD = 1000229000,

    /// Provided by [`khr_dynamic_rendering_local_read`]
    PhysicalDeviceDynamicRenderingLocalReadFeaturesKHR = 1000232000,

    /// Provided by [`khr_dynamic_rendering_local_read`]
    RenderingAttachmentLocationInfoKHR = 1000232001,

    /// Provided by [`khr_dynamic_rendering_local_read`]
    RenderingInputAttachmentIndexInfoKHR = 1000232002,

    /// Provided by [`ext_shader_image_atomic_int64`]
    PhysicalDeviceShaderImageAtomicInt64FeaturesEXT = 1000234000,

    /// Provided by [`khr_shader_quad_control`]
    PhysicalDeviceShaderQuadControlFeaturesKHR = 1000235000,

    /// Provided by [`ext_memory_budget`]
    PhysicalDeviceMemoryBudgetPropertiesEXT = 1000237000,

    /// Provided by [`ext_memory_priority`]
    PhysicalDeviceMemoryPriorityFeaturesEXT = 1000238000,

    /// Provided by [`ext_memory_priority`]
    MemoryPriorityAllocateInfoEXT = 1000238001,

    /// Provided by [`khr_surface_protected_capabilities`]
    SurfaceProtectedCapabilitiesKHR = 1000239000,

    /// Provided by [`nv_dedicated_allocation_image_aliasing`]
    PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV = 1000240000,

    /// Provided by [`ext_buffer_device_address`]
    PhysicalDeviceBufferDeviceAddressFeaturesEXT = 1000244000,

    /// Provided by [`ext_buffer_device_address`]
    BufferDeviceAddressCreateInfoEXT = 1000244002,

    /// Provided by [`ext_validation_features`]
    ValidationFeaturesEXT = 1000247000,

    /// Provided by [`khr_present_wait`]
    PhysicalDevicePresentWaitFeaturesKHR = 1000248000,

    /// Provided by [`nv_cooperative_matrix`]
    PhysicalDeviceCooperativeMatrixFeaturesNV = 1000249000,

    /// Provided by [`nv_cooperative_matrix`]
    CooperativeMatrixPropertiesNV = 1000249001,

    /// Provided by [`nv_cooperative_matrix`]
    PhysicalDeviceCooperativeMatrixPropertiesNV = 1000249002,

    /// Provided by [`nv_coverage_reduction_mode`]
    PhysicalDeviceCoverageReductionModeFeaturesNV = 1000250000,

    /// Provided by [`nv_coverage_reduction_mode`]
    PipelineCoverageReductionStateCreateInfoNV = 1000250001,

    /// Provided by [`nv_coverage_reduction_mode`]
    FramebufferMixedSamplesCombinationNV = 1000250002,

    /// Provided by [`ext_fragment_shader_interlock`]
    PhysicalDeviceFragmentShaderInterlockFeaturesEXT = 1000251000,

    /// Provided by [`ext_ycbcr_image_arrays`]
    PhysicalDeviceYcbcrImageArraysFeaturesEXT = 1000252000,

    /// Provided by [`ext_provoking_vertex`]
    PhysicalDeviceProvokingVertexFeaturesEXT = 1000254000,

    /// Provided by [`ext_provoking_vertex`]
    PipelineRasterizationProvokingVertexStateCreateInfoEXT = 1000254001,

    /// Provided by [`ext_provoking_vertex`]
    PhysicalDeviceProvokingVertexPropertiesEXT = 1000254002,

    /// Provided by [`ext_full_screen_exclusive`]
    SurfaceFullScreenExclusiveInfoEXT = 1000255000,

    /// Provided by [`ext_full_screen_exclusive`]
    SurfaceCapabilitiesFullScreenExclusiveEXT = 1000255002,

    /// Provided by [`khr_win32_surface`] with [`ext_full_screen_exclusive`]
    SurfaceFullScreenExclusiveWin32InfoEXT = 1000255001,

    /// Provided by [`ext_headless_surface`]
    HeadlessSurfaceCreateInfoEXT = 1000256000,

    /// Provided by [`ext_shader_atomic_float`]
    PhysicalDeviceShaderAtomicFloatFeaturesEXT = 1000260000,

    /// Provided by [`ext_extended_dynamic_state`]
    PhysicalDeviceEXTendedDynamicStateFeaturesEXT = 1000267000,

    /// Provided by [`khr_pipeline_executable_properties`]
    PhysicalDevicePipelineExecutablePropertiesFeaturesKHR = 1000269000,

    /// Provided by [`khr_pipeline_executable_properties`]
    PipelineInfoKHR = 1000269001,

    /// Provided by [`khr_pipeline_executable_properties`]
    PipelineExecutablePropertiesKHR = 1000269002,

    /// Provided by [`khr_pipeline_executable_properties`]
    PipelineExecutableInfoKHR = 1000269003,

    /// Provided by [`khr_pipeline_executable_properties`]
    PipelineExecutableStatisticKHR = 1000269004,

    /// Provided by [`khr_pipeline_executable_properties`]
    PipelineExecutableInternalRepresentationKHR = 1000269005,

    /// Provided by [`ext_host_image_copy`]
    PhysicalDeviceHostImageCopyFeaturesEXT = 1000270000,

    /// Provided by [`ext_host_image_copy`]
    PhysicalDeviceHostImageCopyPropertiesEXT = 1000270001,

    /// Provided by [`ext_host_image_copy`]
    MemoryToImageCopyEXT = 1000270002,

    /// Provided by [`ext_host_image_copy`]
    ImageToMemoryCopyEXT = 1000270003,

    /// Provided by [`ext_host_image_copy`]
    CopyImageToMemoryInfoEXT = 1000270004,

    /// Provided by [`ext_host_image_copy`]
    CopyMemoryToImageInfoEXT = 1000270005,

    /// Provided by [`ext_host_image_copy`]
    HostImageLayoutTransitionInfoEXT = 1000270006,

    /// Provided by [`ext_host_image_copy`]
    CopyImageToImageInfoEXT = 1000270007,

    /// Provided by [`ext_host_image_copy`]
    SubresourceHostMemcpySizeEXT = 1000270008,

    /// Provided by [`ext_host_image_copy`]
    HostImageCopyDevicePerformanceQueryEXT = 1000270009,

    /// Provided by [`khr_map_memory2`]
    MemoryMapInfoKHR = 1000271000,

    /// Provided by [`khr_map_memory2`]
    MemoryUnmapInfoKHR = 1000271001,

    /// Provided by [`ext_map_memory_placed`]
    PhysicalDeviceMapMemoryPlacedFeaturesEXT = 1000272000,

    /// Provided by [`ext_map_memory_placed`]
    PhysicalDeviceMapMemoryPlacedPropertiesEXT = 1000272001,

    /// Provided by [`ext_map_memory_placed`]
    MemoryMapPlacedInfoEXT = 1000272002,

    /// Provided by [`ext_shader_atomic_float2`]
    PhysicalDeviceShaderAtomicFloat2FeaturesEXT = 1000273000,

    /// Provided by [`ext_surface_maintenance1`]
    SurfacePresentModeEXT = 1000274000,

    /// Provided by [`ext_surface_maintenance1`]
    SurfacePresentScalingCapabilitiesEXT = 1000274001,

    /// Provided by [`ext_surface_maintenance1`]
    SurfacePresentModeCompatibilityEXT = 1000274002,

    /// Provided by [`ext_swapchain_maintenance1`]
    PhysicalDeviceSwapchainMaintenance1FeaturesEXT = 1000275000,

    /// Provided by [`ext_swapchain_maintenance1`]
    SwapchainPresentFenceInfoEXT = 1000275001,

    /// Provided by [`ext_swapchain_maintenance1`]
    SwapchainPresentModesCreateInfoEXT = 1000275002,

    /// Provided by [`ext_swapchain_maintenance1`]
    SwapchainPresentModeInfoEXT = 1000275003,

    /// Provided by [`ext_swapchain_maintenance1`]
    SwapchainPresentScalingCreateInfoEXT = 1000275004,

    /// Provided by [`ext_swapchain_maintenance1`]
    ReleaseSwapchainImagesInfoEXT = 1000275005,

    /// Provided by [`nv_device_generated_commands`]
    PhysicalDeviceDeviceGeneratedCommandsPropertiesNV = 1000277000,

    /// Provided by [`nv_device_generated_commands`]
    GraphicsShaderGroupCreateInfoNV = 1000277001,

    /// Provided by [`nv_device_generated_commands`]
    GraphicsPipelineShaderGroupsCreateInfoNV = 1000277002,

    /// Provided by [`nv_device_generated_commands`]
    IndirectCommandsLayoutTokenNV = 1000277003,

    /// Provided by [`nv_device_generated_commands`]
    IndirectCommandsLayoutCreateInfoNV = 1000277004,

    /// Provided by [`nv_device_generated_commands`]
    GeneratedCommandsInfoNV = 1000277005,

    /// Provided by [`nv_device_generated_commands`]
    GeneratedCommandsMemoryRequirementsInfoNV = 1000277006,

    /// Provided by [`nv_device_generated_commands`]
    PhysicalDeviceDeviceGeneratedCommandsFeaturesNV = 1000277007,

    /// Provided by [`nv_inherited_viewport_scissor`]
    PhysicalDeviceInheritedViewportScissorFeaturesNV = 1000278000,

    /// Provided by [`nv_inherited_viewport_scissor`]
    CommandBufferInheritanceViewportScissorInfoNV = 1000278001,

    /// Provided by [`ext_texel_buffer_alignment`]
    PhysicalDeviceTexelBufferAlignmentFeaturesEXT = 1000281000,

    /// Provided by [`qcom_render_pass_transform`]
    CommandBufferInheritanceRenderPassTransformInfoQcom = 1000282000,

    /// Provided by [`qcom_render_pass_transform`]
    RenderPassTransformBeginInfoQcom = 1000282001,

    /// Provided by [`ext_depth_bias_control`]
    PhysicalDeviceDepthBiasControlFeaturesEXT = 1000283000,

    /// Provided by [`ext_depth_bias_control`]
    DepthBiasInfoEXT = 1000283001,

    /// Provided by [`ext_depth_bias_control`]
    DepthBiasRepresentationInfoEXT = 1000283002,

    /// Provided by [`ext_device_memory_report`]
    PhysicalDeviceDeviceMemoryReportFeaturesEXT = 1000284000,

    /// Provided by [`ext_device_memory_report`]
    DeviceDeviceMemoryReportCreateInfoEXT = 1000284001,

    /// Provided by [`ext_device_memory_report`]
    DeviceMemoryReportCallbackDataEXT = 1000284002,

    /// Provided by [`ext_robustness2`]
    PhysicalDeviceRobustness2FeaturesEXT = 1000286000,

    /// Provided by [`ext_robustness2`]
    PhysicalDeviceRobustness2PropertiesEXT = 1000286001,

    /// Provided by [`ext_custom_border_color`]
    SamplerCustomBorderColorCreateInfoEXT = 1000287000,

    /// Provided by [`ext_custom_border_color`]
    PhysicalDeviceCustomBorderColorPropertiesEXT = 1000287001,

    /// Provided by [`ext_custom_border_color`]
    PhysicalDeviceCustomBorderColorFeaturesEXT = 1000287002,

    /// Provided by [`khr_pipeline_library`]
    PipelineLibraryCreateInfoKHR = 1000290000,

    /// Provided by [`nv_present_barrier`]
    PhysicalDevicePresentBarrierFeaturesNV = 1000292000,

    /// Provided by [`nv_present_barrier`]
    SurfaceCapabilitiesPresentBarrierNV = 1000292001,

    /// Provided by [`nv_present_barrier`]
    SwapchainPresentBarrierCreateInfoNV = 1000292002,

    /// Provided by [`khr_present_id`]
    PresentIdKHR = 1000294000,

    /// Provided by [`khr_present_id`]
    PhysicalDevicePresentIdFeaturesKHR = 1000294001,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeInfoKHR = 1000299000,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeRateControlInfoKHR = 1000299001,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeRateControlLayerInfoKHR = 1000299002,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeCapabilitiesKHR = 1000299003,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeUsageInfoKHR = 1000299004,

    /// Provided by [`khr_video_encode_queue`]
    QueryPoolVideoEncodeFeedbackCreateInfoKHR = 1000299005,

    /// Provided by [`khr_video_encode_queue`]
    PhysicalDeviceVideoEncodeQualityLevelInfoKHR = 1000299006,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeQualityLevelPropertiesKHR = 1000299007,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeQualityLevelInfoKHR = 1000299008,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeSessionParametersGetInfoKHR = 1000299009,

    /// Provided by [`khr_video_encode_queue`]
    VideoEncodeSessionParametersFeedbackInfoKHR = 1000299010,

    /// Provided by [`nv_device_diagnostics_config`]
    PhysicalDeviceDiagnosticsConfigFeaturesNV = 1000300000,

    /// Provided by [`nv_device_diagnostics_config`]
    DeviceDiagnosticsConfigCreateInfoNV = 1000300001,

    /// Provided by [`nv_cuda_kernel_launch`]
    CudaModuleCreateInfoNV = 1000307000,

    /// Provided by [`nv_cuda_kernel_launch`]
    CudaFunctionCreateInfoNV = 1000307001,

    /// Provided by [`nv_cuda_kernel_launch`]
    CudaLaunchInfoNV = 1000307002,

    /// Provided by [`nv_cuda_kernel_launch`]
    PhysicalDeviceCudaKernelLaunchFeaturesNV = 1000307003,

    /// Provided by [`nv_cuda_kernel_launch`]
    PhysicalDeviceCudaKernelLaunchPropertiesNV = 1000307004,

    /// Provided by [`nv_low_latency`]
    QueryLowLatencySupportNV = 1000310000,

    /// Provided by [`ext_metal_objects`]
    ExportMetalObjectCreateInfoEXT = 1000311000,

    /// Provided by [`ext_metal_objects`]
    ExportMetalObjectsInfoEXT = 1000311001,

    /// Provided by [`ext_metal_objects`]
    ExportMetalDeviceInfoEXT = 1000311002,

    /// Provided by [`ext_metal_objects`]
    ExportMetalCommandQueueInfoEXT = 1000311003,

    /// Provided by [`ext_metal_objects`]
    ExportMetalBufferInfoEXT = 1000311004,

    /// Provided by [`ext_metal_objects`]
    ImportMetalBufferInfoEXT = 1000311005,

    /// Provided by [`ext_metal_objects`]
    ExportMetalTextureInfoEXT = 1000311006,

    /// Provided by [`ext_metal_objects`]
    ImportMetalTextureInfoEXT = 1000311007,

    /// Provided by [`ext_metal_objects`]
    ExportMetalIoSurfaceInfoEXT = 1000311008,

    /// Provided by [`ext_metal_objects`]
    ImportMetalIoSurfaceInfoEXT = 1000311009,

    /// Provided by [`ext_metal_objects`]
    ExportMetalSharedEventInfoEXT = 1000311010,

    /// Provided by [`ext_metal_objects`]
    ImportMetalSharedEventInfoEXT = 1000311011,

    /// Provided by [`khr_synchronization2`] with [`nv_device_diagnostic_checkpoints`]
    QueueFamilyCheckpointProperties2NV = 1000314008,

    /// Provided by [`khr_synchronization2`] with [`nv_device_diagnostic_checkpoints`]
    CheckpointData2NV = 1000314009,

    /// Provided by [`ext_descriptor_buffer`]
    PhysicalDeviceDescriptorBufferPropertiesEXT = 1000316000,

    /// Provided by [`ext_descriptor_buffer`]
    PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT = 1000316001,

    /// Provided by [`ext_descriptor_buffer`]
    PhysicalDeviceDescriptorBufferFeaturesEXT = 1000316002,

    /// Provided by [`ext_descriptor_buffer`]
    DescriptorAddressInfoEXT = 1000316003,

    /// Provided by [`ext_descriptor_buffer`]
    DescriptorGetInfoEXT = 1000316004,

    /// Provided by [`ext_descriptor_buffer`]
    BufferCaptureDescriptorDataInfoEXT = 1000316005,

    /// Provided by [`ext_descriptor_buffer`]
    ImageCaptureDescriptorDataInfoEXT = 1000316006,

    /// Provided by [`ext_descriptor_buffer`]
    ImageViewCaptureDescriptorDataInfoEXT = 1000316007,

    /// Provided by [`ext_descriptor_buffer`]
    SamplerCaptureDescriptorDataInfoEXT = 1000316008,

    /// Provided by [`ext_descriptor_buffer`]
    OpaqueCaptureDescriptorDataCreateInfoEXT = 1000316010,

    /// Provided by [`ext_descriptor_buffer`]
    DescriptorBufferBindingInfoEXT = 1000316011,

    /// Provided by [`ext_descriptor_buffer`]
    DescriptorBufferBindingPushDescriptorBufferHandleEXT = 1000316012,

    /// Provided by [`ext_descriptor_buffer`] with [`khr_acceleration_structure`] or [`nv_ray_tracing`]
    AccelerationStructureCaptureDescriptorDataInfoEXT = 1000316009,

    /// Provided by [`ext_graphics_pipeline_library`]
    PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT = 1000320000,

    /// Provided by [`ext_graphics_pipeline_library`]
    PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT = 1000320001,

    /// Provided by [`ext_graphics_pipeline_library`]
    GraphicsPipelineLibraryCreateInfoEXT = 1000320002,

    /// Provided by [`amd_shader_early_and_late_fragment_tests`]
    PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD = 1000321000,

    /// Provided by [`khr_fragment_shader_barycentric`]
    PhysicalDeviceFragmentShaderBarycentricFeaturesKHR = 1000203000,

    /// Provided by [`khr_fragment_shader_barycentric`]
    PhysicalDeviceFragmentShaderBarycentricPropertiesKHR = 1000322000,

    /// Provided by [`khr_shader_subgroup_uniform_control_flow`]
    PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR = 1000323000,

    /// Provided by [`nv_fragment_shading_rate_enums`]
    PhysicalDeviceFragmentShadingRateEnumsPropertiesNV = 1000326000,

    /// Provided by [`nv_fragment_shading_rate_enums`]
    PhysicalDeviceFragmentShadingRateEnumsFeaturesNV = 1000326001,

    /// Provided by [`nv_fragment_shading_rate_enums`]
    PipelineFragmentShadingRateEnumStateCreateInfoNV = 1000326002,

    /// Provided by [`nv_ray_tracing_motion_blur`]
    AccelerationStructureGeometryMotionTrianglesDataNV = 1000327000,

    /// Provided by [`nv_ray_tracing_motion_blur`]
    PhysicalDeviceRayTracingMotionBlurFeaturesNV = 1000327001,

    /// Provided by [`nv_ray_tracing_motion_blur`]
    AccelerationStructureMotionInfoNV = 1000327002,

    /// Provided by [`ext_mesh_shader`]
    PhysicalDeviceMeshShaderFeaturesEXT = 1000328000,

    /// Provided by [`ext_mesh_shader`]
    PhysicalDeviceMeshShaderPropertiesEXT = 1000328001,

    /// Provided by [`ext_ycbcr_2plane_444_formats`]
    PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT = 1000330000,

    /// Provided by [`ext_fragment_density_map2`]
    PhysicalDeviceFragmentDensityMap2FeaturesEXT = 1000332000,

    /// Provided by [`ext_fragment_density_map2`]
    PhysicalDeviceFragmentDensityMap2PropertiesEXT = 1000332001,

    /// Provided by [`qcom_rotated_copy_commands`]
    CopyCommandTransformInfoQcom = 1000333000,

    /// Provided by [`khr_workgroup_memory_explicit_layout`]
    PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR = 1000336000,

    /// Provided by [`ext_image_compression_control`]
    PhysicalDeviceImageCompressionControlFeaturesEXT = 1000338000,

    /// Provided by [`ext_image_compression_control`]
    ImageCompressionControlEXT = 1000338001,

    /// Provided by [`ext_image_compression_control`]
    ImageCompressionPropertiesEXT = 1000338004,

    /// Provided by [`ext_attachment_feedback_loop_layout`]
    PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT = 1000339000,

    /// Provided by [`ext_4444_formats`]
    PhysicalDevice4444FormatsFeaturesEXT = 1000340000,

    /// Provided by [`ext_device_fault`]
    PhysicalDeviceFaultFeaturesEXT = 1000341000,

    /// Provided by [`ext_device_fault`]
    DeviceFaultCountsEXT = 1000341001,

    /// Provided by [`ext_device_fault`]
    DeviceFaultInfoEXT = 1000341002,

    /// Provided by [`ext_rgba10x6_formats`]
    PhysicalDeviceRgba10X6FormatsFeaturesEXT = 1000344000,

    /// Provided by [`ext_directfb_surface`]
    DirectfbSurfaceCreateInfoEXT = 1000346000,

    /// Provided by [`ext_vertex_input_dynamic_state`]
    PhysicalDeviceVertexInputDynamicStateFeaturesEXT = 1000352000,

    /// Provided by [`ext_shader_object`], [`ext_vertex_input_dynamic_state`]
    VertexInputBindingDescription2EXT = 1000352001,

    /// Provided by [`ext_shader_object`], [`ext_vertex_input_dynamic_state`]
    VertexInputAttributeDescription2EXT = 1000352002,

    /// Provided by [`ext_physical_device_drm`]
    PhysicalDeviceDrmPropertiesEXT = 1000353000,

    /// Provided by [`ext_device_address_binding_report`]
    PhysicalDeviceAddressBindingReportFeaturesEXT = 1000354000,

    /// Provided by [`ext_device_address_binding_report`]
    DeviceAddressBindingCallbackDataEXT = 1000354001,

    /// Provided by [`ext_depth_clip_control`]
    PhysicalDeviceDepthClipControlFeaturesEXT = 1000355000,

    /// Provided by [`ext_depth_clip_control`]
    PipelineViewportDepthClipControlCreateInfoEXT = 1000355001,

    /// Provided by [`ext_primitive_topology_list_restart`]
    PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT = 1000356000,

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
    MemoryGetRemoteAddressInfoNV = 1000371000,

    /// Provided by [`nv_external_memory_rdma`]
    PhysicalDeviceEXTernalMemoryRdmaFeaturesNV = 1000371001,

    /// Provided by [`ext_pipeline_properties`]
    PipelinePropertiesIdentifierEXT = 1000372000,

    /// Provided by [`ext_pipeline_properties`]
    PhysicalDevicePipelinePropertiesFeaturesEXT = 1000372001,

    /// Provided by [`ext_frame_boundary`]
    PhysicalDeviceFrameBoundaryFeaturesEXT = 1000375000,

    /// Provided by [`ext_frame_boundary`]
    FrameBoundaryEXT = 1000375001,

    /// Provided by [`ext_multisampled_render_to_single_sampled`]
    PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT = 1000376000,

    /// Provided by [`ext_multisampled_render_to_single_sampled`]
    SubpassResolvePerformanceQueryEXT = 1000376001,

    /// Provided by [`ext_multisampled_render_to_single_sampled`]
    MultisampledRenderToSingleSampledInfoEXT = 1000376002,

    /// Provided by [`ext_extended_dynamic_state2`]
    PhysicalDeviceEXTendedDynamicState2FeaturesEXT = 1000377000,

    /// Provided by [`qnx_screen_surface`]
    ScreenSurfaceCreateInfoQnx = 1000378000,

    /// Provided by [`ext_color_write_enable`]
    PhysicalDeviceColorWriteEnableFeaturesEXT = 1000381000,

    /// Provided by [`ext_color_write_enable`]
    PipelineColorWriteCreateInfoEXT = 1000381001,

    /// Provided by [`ext_primitives_generated_query`]
    PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT = 1000382000,

    /// Provided by [`khr_ray_tracing_maintenance1`]
    PhysicalDeviceRayTracingMaintenance1FeaturesKHR = 1000386000,

    /// Provided by [`ext_image_view_min_lod`]
    PhysicalDeviceImageViewMinLodFeaturesEXT = 1000391000,

    /// Provided by [`ext_image_view_min_lod`]
    ImageViewMinLodCreateInfoEXT = 1000391001,

    /// Provided by [`ext_multi_draw`]
    PhysicalDeviceMultiDrawFeaturesEXT = 1000392000,

    /// Provided by [`ext_multi_draw`]
    PhysicalDeviceMultiDrawPropertiesEXT = 1000392001,

    /// Provided by [`ext_image_2d_view_of_3d`]
    PhysicalDeviceImage2DViewOf3DFeaturesEXT = 1000393000,

    /// Provided by [`ext_shader_tile_image`]
    PhysicalDeviceShaderTileImageFeaturesEXT = 1000395000,

    /// Provided by [`ext_shader_tile_image`]
    PhysicalDeviceShaderTileImagePropertiesEXT = 1000395001,

    /// Provided by [`ext_opacity_micromap`]
    MicromapBuildInfoEXT = 1000396000,

    /// Provided by [`ext_opacity_micromap`]
    MicromapVersionInfoEXT = 1000396001,

    /// Provided by [`ext_opacity_micromap`]
    CopyMicromapInfoEXT = 1000396002,

    /// Provided by [`ext_opacity_micromap`]
    CopyMicromapToMemoryInfoEXT = 1000396003,

    /// Provided by [`ext_opacity_micromap`]
    CopyMemoryToMicromapInfoEXT = 1000396004,

    /// Provided by [`ext_opacity_micromap`]
    PhysicalDeviceOpacityMicromapFeaturesEXT = 1000396005,

    /// Provided by [`ext_opacity_micromap`]
    PhysicalDeviceOpacityMicromapPropertiesEXT = 1000396006,

    /// Provided by [`ext_opacity_micromap`]
    MicromapCreateInfoEXT = 1000396007,

    /// Provided by [`ext_opacity_micromap`]
    MicromapBuildSizesInfoEXT = 1000396008,

    /// Provided by [`ext_opacity_micromap`]
    AccelerationStructureTrianglesOpacityMicromapEXT = 1000396009,

    /// Provided by [`nv_displacement_micromap`]
    PhysicalDeviceDisplacementMicromapFeaturesNV = 1000397000,

    /// Provided by [`nv_displacement_micromap`]
    PhysicalDeviceDisplacementMicromapPropertiesNV = 1000397001,

    /// Provided by [`nv_displacement_micromap`]
    AccelerationStructureTrianglesDisplacementMicromapNV = 1000397002,

    /// Provided by [`huawei_cluster_culling_shader`]
    PhysicalDeviceClusterCullingShaderFeaturesHuawei = 1000404000,

    /// Provided by [`huawei_cluster_culling_shader`]
    PhysicalDeviceClusterCullingShaderPropertiesHuawei = 1000404001,

    /// Provided by [`huawei_cluster_culling_shader`]
    PhysicalDeviceClusterCullingShaderVrsFeaturesHuawei = 1000404002,

    /// Provided by [`ext_border_color_swizzle`]
    PhysicalDeviceBorderColorSwizzleFeaturesEXT = 1000411000,

    /// Provided by [`ext_border_color_swizzle`]
    SamplerBorderColorComponentMappingCreateInfoEXT = 1000411001,

    /// Provided by [`ext_pageable_device_local_memory`]
    PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT = 1000412000,

    /// Provided by [`arm_shader_core_properties`]
    PhysicalDeviceShaderCorePropertiesArm = 1000415000,

    /// Provided by [`khr_shader_subgroup_rotate`]
    PhysicalDeviceShaderSubgroupRotateFeaturesKHR = 1000416000,

    /// Provided by [`arm_scheduling_controls`]
    DeviceQueueShaderCoreControlCreateInfoArm = 1000417000,

    /// Provided by [`arm_scheduling_controls`]
    PhysicalDeviceSchedulingControlsFeaturesArm = 1000417001,

    /// Provided by [`arm_scheduling_controls`]
    PhysicalDeviceSchedulingControlsPropertiesArm = 1000417002,

    /// Provided by [`ext_image_sliced_view_of_3d`]
    PhysicalDeviceImageSlicedViewOf3DFeaturesEXT = 1000418000,

    /// Provided by [`ext_image_sliced_view_of_3d`]
    ImageViewSlicedCreateInfoEXT = 1000418001,

    /// Provided by [`valve_descriptor_set_host_mapping`]
    PhysicalDeviceDescriptorSetHostMappingFeaturesValve = 1000420000,

    /// Provided by [`valve_descriptor_set_host_mapping`]
    DescriptorSetBindingReferenceValve = 1000420001,

    /// Provided by [`valve_descriptor_set_host_mapping`]
    DescriptorSetLayoutHostMappingInfoValve = 1000420002,

    /// Provided by [`ext_depth_clamp_zero_one`]
    PhysicalDeviceDepthClampZeroOneFeaturesEXT = 1000421000,

    /// Provided by [`ext_non_seamless_cube_map`]
    PhysicalDeviceNonSeamlessCubeMapFeaturesEXT = 1000422000,

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
    PhysicalDeviceCopyMemoryIndirectFeaturesNV = 1000426000,

    /// Provided by [`nv_copy_memory_indirect`]
    PhysicalDeviceCopyMemoryIndirectPropertiesNV = 1000426001,

    /// Provided by [`nv_memory_decompression`]
    PhysicalDeviceMemoryDecompressionFeaturesNV = 1000427000,

    /// Provided by [`nv_memory_decompression`]
    PhysicalDeviceMemoryDecompressionPropertiesNV = 1000427001,

    /// Provided by [`nv_device_generated_commands_compute`]
    PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV = 1000428000,

    /// Provided by [`nv_device_generated_commands_compute`]
    ComputePipelineIndirectBufferInfoNV = 1000428001,

    /// Provided by [`nv_device_generated_commands_compute`]
    PipelineIndirectDeviceAddressInfoNV = 1000428002,

    /// Provided by [`nv_linear_color_attachment`]
    PhysicalDeviceLinearColorAttachmentFeaturesNV = 1000430000,

    /// Provided by [`khr_shader_maximal_reconvergence`]
    PhysicalDeviceShaderMaximalReconvergenceFeaturesKHR = 1000434000,

    /// Provided by [`ext_image_compression_control_swapchain`]
    PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT = 1000437000,

    /// Provided by [`qcom_image_processing`]
    PhysicalDeviceImageProcessingFeaturesQcom = 1000440000,

    /// Provided by [`qcom_image_processing`]
    PhysicalDeviceImageProcessingPropertiesQcom = 1000440001,

    /// Provided by [`qcom_image_processing`]
    ImageViewSampleWeightCreateInfoQcom = 1000440002,

    /// Provided by [`ext_nested_command_buffer`]
    PhysicalDeviceNestedCommandBufferFeaturesEXT = 1000451000,

    /// Provided by [`ext_nested_command_buffer`]
    PhysicalDeviceNestedCommandBufferPropertiesEXT = 1000451001,

    /// Provided by [`ext_external_memory_acquire_unmodified`]
    EXTernalMemoryAcquireUnmodifiedEXT = 1000453000,

    /// Provided by [`ext_extended_dynamic_state3`]
    PhysicalDeviceEXTendedDynamicState3FeaturesEXT = 1000455000,

    /// Provided by [`ext_extended_dynamic_state3`]
    PhysicalDeviceEXTendedDynamicState3PropertiesEXT = 1000455001,

    /// Provided by [`ext_subpass_merge_feedback`]
    PhysicalDeviceSubpassMergeFeedbackFeaturesEXT = 1000458000,

    /// Provided by [`ext_subpass_merge_feedback`]
    RenderPassCreationControlEXT = 1000458001,

    /// Provided by [`ext_subpass_merge_feedback`]
    RenderPassCreationFeedbackCreateInfoEXT = 1000458002,

    /// Provided by [`ext_subpass_merge_feedback`]
    RenderPassSubpassFeedbackCreateInfoEXT = 1000458003,

    /// Provided by [`lunarg_direct_driver_loading`]
    DirectDriverLoadingInfoLunarg = 1000459000,

    /// Provided by [`lunarg_direct_driver_loading`]
    DirectDriverLoadingListLunarg = 1000459001,

    /// Provided by [`ext_shader_module_identifier`]
    PhysicalDeviceShaderModuleIdentifierFeaturesEXT = 1000462000,

    /// Provided by [`ext_shader_module_identifier`]
    PhysicalDeviceShaderModuleIdentifierPropertiesEXT = 1000462001,

    /// Provided by [`ext_shader_module_identifier`]
    PipelineShaderStageModuleIdentifierCreateInfoEXT = 1000462002,

    /// Provided by [`ext_shader_module_identifier`]
    ShaderModuleIdentifierEXT = 1000462003,

    /// Provided by [`ext_rasterization_order_attachment_access`]
    PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT = 1000342000,

    /// Provided by [`nv_optical_flow`]
    PhysicalDeviceOpticalFlowFeaturesNV = 1000464000,

    /// Provided by [`nv_optical_flow`]
    PhysicalDeviceOpticalFlowPropertiesNV = 1000464001,

    /// Provided by [`nv_optical_flow`]
    OpticalFlowImageFormatInfoNV = 1000464002,

    /// Provided by [`nv_optical_flow`]
    OpticalFlowImageFormatPropertiesNV = 1000464003,

    /// Provided by [`nv_optical_flow`]
    OpticalFlowSessionCreateInfoNV = 1000464004,

    /// Provided by [`nv_optical_flow`]
    OpticalFlowExecuteInfoNV = 1000464005,

    /// Provided by [`nv_optical_flow`]
    OpticalFlowSessionCreatePrivateDataInfoNV = 1000464010,

    /// Provided by [`ext_legacy_dithering`]
    PhysicalDeviceLegacyDitheringFeaturesEXT = 1000465000,

    /// Provided by [`ext_pipeline_protected_access`]
    PhysicalDevicePipelineProtectedAccessFeaturesEXT = 1000466000,

    /// Provided by [`android_external_format_resolve`]
    PhysicalDeviceEXTernalFormatResolveFeaturesAndroid = 1000468000,

    /// Provided by [`android_external_format_resolve`]
    PhysicalDeviceEXTernalFormatResolvePropertiesAndroid = 1000468001,

    /// Provided by [`android_external_format_resolve`]
    AndroidHardwareBufferFormatResolvePropertiesAndroid = 1000468002,

    /// Provided by [`khr_maintenance5`]
    PhysicalDeviceMaintenance5FeaturesKHR = 1000470000,

    /// Provided by [`khr_maintenance5`]
    PhysicalDeviceMaintenance5PropertiesKHR = 1000470001,

    /// Provided by [`khr_maintenance5`]
    RenderingAreaInfoKHR = 1000470003,

    /// Provided by [`khr_maintenance5`]
    DeviceImageSubresourceInfoKHR = 1000470004,

    /// Provided by [`khr_maintenance5`]
    SubresourceLayout2KHR = 1000338002,

    /// Provided by [`khr_maintenance5`]
    ImageSubresource2KHR = 1000338003,

    /// Provided by [`khr_maintenance5`]
    PipelineCreateFlags2CreateInfoKHR = 1000470005,

    /// Provided by [`khr_maintenance5`]
    BufferUsageFlags2CreateInfoKHR = 1000470006,

    /// Provided by [`khr_ray_tracing_position_fetch`]
    PhysicalDeviceRayTracingPositionFetchFeaturesKHR = 1000481000,

    /// Provided by [`ext_shader_object`]
    PhysicalDeviceShaderObjectFeaturesEXT = 1000482000,

    /// Provided by [`ext_shader_object`]
    PhysicalDeviceShaderObjectPropertiesEXT = 1000482001,

    /// Provided by [`ext_shader_object`]
    ShaderCreateInfoEXT = 1000482002,

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
    PhysicalDeviceRayTracingInvocationReorderFeaturesNV = 1000490000,

    /// Provided by [`nv_ray_tracing_invocation_reorder`]
    PhysicalDeviceRayTracingInvocationReorderPropertiesNV = 1000490001,

    /// Provided by [`nv_extended_sparse_address_space`]
    PhysicalDeviceEXTendedSparseAddressSpaceFeaturesNV = 1000492000,

    /// Provided by [`nv_extended_sparse_address_space`]
    PhysicalDeviceEXTendedSparseAddressSpacePropertiesNV = 1000492001,

    /// Provided by [`ext_mutable_descriptor_type`]
    PhysicalDeviceMutableDescriptorTypeFeaturesEXT = 1000351000,

    /// Provided by [`ext_mutable_descriptor_type`]
    MutableDescriptorTypeCreateInfoEXT = 1000351002,

    /// Provided by [`ext_legacy_vertex_attributes`]
    PhysicalDeviceLegacyVertexAttributesFeaturesEXT = 1000495000,

    /// Provided by [`ext_legacy_vertex_attributes`]
    PhysicalDeviceLegacyVertexAttributesPropertiesEXT = 1000495001,

    /// Provided by [`ext_layer_settings`]
    LayerSettingsCreateInfoEXT = 1000496000,

    /// Provided by [`arm_shader_core_builtins`]
    PhysicalDeviceShaderCoreBuiltinsFeaturesArm = 1000497000,

    /// Provided by [`arm_shader_core_builtins`]
    PhysicalDeviceShaderCoreBuiltinsPropertiesArm = 1000497001,

    /// Provided by [`ext_pipeline_library_group_handles`]
    PhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT = 1000498000,

    /// Provided by [`ext_dynamic_rendering_unused_attachments`]
    PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT = 1000499000,

    /// Provided by [`nv_low_latency2`]
    LatencySleepModeInfoNV = 1000505000,

    /// Provided by [`nv_low_latency2`]
    LatencySleepInfoNV = 1000505001,

    /// Provided by [`nv_low_latency2`]
    SetLatencyMarkerInfoNV = 1000505002,

    /// Provided by [`nv_low_latency2`]
    GetLatencyMarkerInfoNV = 1000505003,

    /// Provided by [`nv_low_latency2`]
    LatencyTimingsFrameReportNV = 1000505004,

    /// Provided by [`nv_low_latency2`]
    LatencySubmissionPresentIdNV = 1000505005,

    /// Provided by [`nv_low_latency2`]
    OutOfBandQueueTypeInfoNV = 1000505006,

    /// Provided by [`nv_low_latency2`]
    SwapchainLatencyCreateInfoNV = 1000505007,

    /// Provided by [`nv_low_latency2`]
    LatencySurfaceCapabilitiesNV = 1000505008,

    /// Provided by [`khr_cooperative_matrix`]
    PhysicalDeviceCooperativeMatrixFeaturesKHR = 1000506000,

    /// Provided by [`khr_cooperative_matrix`]
    CooperativeMatrixPropertiesKHR = 1000506001,

    /// Provided by [`khr_cooperative_matrix`]
    PhysicalDeviceCooperativeMatrixPropertiesKHR = 1000506002,

    /// Provided by [`qcom_multiview_per_view_render_areas`]
    PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQcom = 1000510000,

    /// Provided by [`qcom_multiview_per_view_render_areas`]
    MultiviewPerViewRenderAreasRenderPassBeginInfoQcom = 1000510001,

    /// Provided by [`khr_video_decode_av1`]
    VideoDecodeAv1CapabilitiesKHR = 1000512000,

    /// Provided by [`khr_video_decode_av1`]
    VideoDecodeAv1PictureInfoKHR = 1000512001,

    /// Provided by [`khr_video_decode_av1`]
    VideoDecodeAv1ProfileInfoKHR = 1000512003,

    /// Provided by [`khr_video_decode_av1`]
    VideoDecodeAv1SessionParametersCreateInfoKHR = 1000512004,

    /// Provided by [`khr_video_decode_av1`]
    VideoDecodeAv1DpbSlotInfoKHR = 1000512005,

    /// Provided by [`khr_video_maintenance1`]
    PhysicalDeviceVideoMaintenance1FeaturesKHR = 1000515000,

    /// Provided by [`khr_video_maintenance1`]
    VideoInlineQueryInfoKHR = 1000515001,

    /// Provided by [`nv_per_stage_descriptor_set`]
    PhysicalDevicePerStageDescriptorSetFeaturesNV = 1000516000,

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
    PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT = 1000524000,

    /// Provided by [`khr_vertex_attribute_divisor`]
    PhysicalDeviceVertexAttributeDivisorPropertiesKHR = 1000525000,

    /// Provided by [`khr_vertex_attribute_divisor`]
    PipelineVertexInputDivisorStateCreateInfoKHR = 1000190001,

    /// Provided by [`khr_vertex_attribute_divisor`]
    PhysicalDeviceVertexAttributeDivisorFeaturesKHR = 1000190002,

    /// Provided by [`khr_shader_float_controls2`]
    PhysicalDeviceShaderFloatControls2FeaturesKHR = 1000528000,

    /// Provided by [`qnx_external_memory_screen_buffer`]
    ScreenBufferPropertiesQnx = 1000529000,

    /// Provided by [`qnx_external_memory_screen_buffer`]
    ScreenBufferFormatPropertiesQnx = 1000529001,

    /// Provided by [`qnx_external_memory_screen_buffer`]
    ImportScreenBufferInfoQnx = 1000529002,

    /// Provided by [`qnx_external_memory_screen_buffer`]
    EXTernalFormatQnx = 1000529003,

    /// Provided by [`qnx_external_memory_screen_buffer`]
    PhysicalDeviceEXTernalMemoryScreenBufferFeaturesQnx = 1000529004,

    /// Provided by [`msft_layered_driver`]
    PhysicalDeviceLayeredDriverPropertiesMsft = 1000530000,

    /// Provided by [`khr_index_type_uint8`]
    PhysicalDeviceIndexTypeUint8FeaturesKHR = 1000265000,

    /// Provided by [`khr_line_rasterization`]
    PhysicalDeviceLineRasterizationFeaturesKHR = 1000259000,

    /// Provided by [`khr_line_rasterization`]
    PipelineRasterizationLineStateCreateInfoKHR = 1000259001,

    /// Provided by [`khr_line_rasterization`]
    PhysicalDeviceLineRasterizationPropertiesKHR = 1000259002,

    /// Provided by [`khr_calibrated_timestamps`]
    CalibratedTimestampInfoKHR = 1000184000,

    /// Provided by [`khr_shader_expect_assume`]
    PhysicalDeviceShaderExpectAssumeFeaturesKHR = 1000544000,

    /// Provided by [`khr_maintenance6`]
    PhysicalDeviceMaintenance6FeaturesKHR = 1000545000,

    /// Provided by [`khr_maintenance6`]
    PhysicalDeviceMaintenance6PropertiesKHR = 1000545001,

    /// Provided by [`khr_maintenance6`]
    BindMemoryStatusKHR = 1000545002,

    /// Provided by [`khr_maintenance6`]
    BindDescriptorSetsInfoKHR = 1000545003,

    /// Provided by [`khr_maintenance6`]
    PushConstantsInfoKHR = 1000545004,

    /// Provided by [`khr_maintenance6`] with [`khr_push_descriptor`]
    PushDescriptorSetInfoKHR = 1000545005,

    /// Provided by [`khr_maintenance6`] with [`khr_push_descriptor`]
    PushDescriptorSetWithTemplateInfoKHR = 1000545006,

    /// Provided by [`khr_maintenance6`] with [`ext_descriptor_buffer`]
    SetDescriptorBufferOffsetsInfoEXT = 1000545007,

    /// Provided by [`khr_maintenance6`] with [`ext_descriptor_buffer`]
    BindDescriptorBufferEmbeddedSamplersInfoEXT = 1000545008,

    /// Provided by [`nv_descriptor_pool_overallocation`]
    PhysicalDeviceDescriptorPoolOverallocationFeaturesNV = 1000546000,

    /// Provided by [`nv_raw_access_chains`]
    PhysicalDeviceRawAccessChainsFeaturesNV = 1000555000,

    /// Provided by [`nv_shader_atomic_float16_vector`]
    PhysicalDeviceShaderAtomicFloat16VectorFeaturesNV = 1000563000,

    /// Provided by [`ext_shader_replicated_composites`]
    PhysicalDeviceShaderReplicatedCompositesFeaturesEXT = 1000564000,

    /// Provided by [`nv_ray_tracing_validation`]
    PhysicalDeviceRayTracingValidationFeaturesNV = 1000568000,

    /// Provided by [`mesa_image_alignment_control`]
    PhysicalDeviceImageAlignmentControlFeaturesMESA = 1000575000,

    /// Provided by [`mesa_image_alignment_control`]
    PhysicalDeviceImageAlignmentControlPropertiesMESA = 1000575001,

    /// Provided by [`mesa_image_alignment_control`]
    ImageAlignmentControlCreateInfoMESA = 1000575002,
}
