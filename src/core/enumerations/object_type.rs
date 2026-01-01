// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VK_VERSION_1_1, VK_VERSION_1_3, ext_debug_utils, khr_surface, khr_swapchain,
};

/// Specify an enumeration to track object handle types
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkObjectType {
    /// Unknown/Undefined Handle
    Unknown = 0,

    /// [`VkInstance`](crate::VkInstance)
    Instance = 1,

    /// [`VkPhysicalDevice`](crate::VkPhysicalDevice)
    PhysicalDevice = 2,

    /// [`VkDevice`](crate::VkDevice)
    Device = 3,

    /// [`VkQueue`](crate::VkQueue)
    Queue = 4,

    /// [`VkSemaphore`](crate::VkSemaphore)
    Semaphore = 5,

    /// [`VkCommandBuffer`](crate::VkCommandBuffer)
    CommandBuffer = 6,

    /// [`VkFence`](crate::VkFence)
    Fence = 7,

    /// [`VkDeviceMemory`](crate::VkDeviceMemory)
    DeviceMemory = 8,

    /// [`VkBuffer`](crate::VkBuffer)
    Buffer = 9,

    /// [`VkImage`](crate::VkImage)
    Image = 10,

    /// [`VkEvent`](crate::VkEvent)
    Event = 11,

    /// [`VkQueryPool`](crate::VkQueryPool)
    QueryPool = 12,

    /// [`VkBufferView`](crate::VkBufferView)
    BufferView = 13,

    /// [`VkImageView`](crate::VkImageView)
    ImageView = 14,

    /// [`VkShaderModule`](crate::VkShaderModule)
    ShaderModule = 15,

    /// [`VkPipelineCache`](crate::VkPipelineCache)
    PipelineCache = 16,

    /// [`VkPipelineLayout`](crate::VkPipelineLayout)
    PipelineLayout = 17,

    /// [`VkRenderPass`](crate::VkRenderPass)
    RenderPass = 18,

    /// [`VkPipeline`](crate::VkPipeline)
    Pipeline = 19,

    /// [`VkDescriptorSetLayout`](crate::VkDescriptorSetLayout)
    DescriptorSetLayout = 20,

    /// [`VkSampler`](crate::VkSampler)
    Sampler = 21,

    /// [`VkDescriptorPool`](crate::VkDescriptorPool)
    DescriptorPool = 22,

    /// [`VkDescriptorSet`](crate::VkDescriptorSet)
    DescriptorSet = 23,

    /// [`VkFramebuffer`](crate::VkFramebuffer)
    Framebuffer = 24,

    /// [`VkCommandPool`](crate::VkCommandPool)
    CommandPool = 25,

    /// [`VkSamplerYCBCRConversion`](crate::VkSamplerYCBCRConversion)
    ///
    /// Provided by [`VK_VERSION_1_1`]
    SamplerYCBCRConversion = 1000156000,

    /// [`VkDescriptorUpdateTemplate`](crate::VkDescriptorUpdateTemplate)
    ///
    /// Provided by [`VK_VERSION_1_1`]
    DescriptorUpdateTemplate = 1000085000,

    /// [`VkPrivateDataSlot`](crate::VkPrivateDataSlot)
    ///
    /// Provided by [`VK_VERSION_1_3`]
    PrivateDataSlot = 1000295000,

    /// [`VkSurfaceKhr`](crate::VkSurfaceKhr)
    ///
    /// Provided by [`khr_surface`]
    SurfaceKhr = 1000000000,

    /// [`VkSwapchainKhr`](crate::VkSwapchainKhr)
    ///
    /// Provided by [`khr_swapchain`]
    SwapchainKhr = 1000001000,

    /// [`VkDisplayKhr`](crate::VkDisplayKhr)
    ///
    /// Provided by [`khr_display`]
    DisplayKhr = 1000002000,

    /// [`VkDisplayModeKhr`](crate::VkDisplayModeKhr)
    ///
    /// Provided by [`khr_display`]
    DisplayModeKhr = 1000002001,

    /// [`VkDebugReportCallbackExt`](crate::VkDebugReportCallbackExt)
    ///
    /// Provided by [`ext_debug_report`]
    DebugReportCallbackExt = 1000011000,

    /// [`VkVideoSessionKhr`](crate::VkVideoSessionKhr)
    ///
    /// Provided by [`khr_video_queue`]
    VideoSessionKhr = 1000023000,

    /// [`VkVideoSessionParametersKhr`](crate::VkVideoSessionParametersKhr)
    ///
    /// Provided by [`khr_video_queue`]
    VideoSessionParametersKhr = 1000023001,

    /// [`VkCuModuleNvX`](crate::VkCuModuleNvX)
    ///
    /// Provided by [`nvx_binary_import`]
    CuModuleNvX = 1000029000,

    /// [`VkCuFunctionNvX`](crate::VkCuFunctionNvX)
    ///
    /// Provided by [`nvx_binary_import`]
    CuFunctionNvX = 1000029001,

    /// [`VkDebugUtilsMessengerExt`](crate::VkDebugUtilsMessengerExt)
    ///
    /// Provided by [`ext_debug_utils`]
    DebugUtilsMessengerExt = 1000128000,

    /// [`VkAccelerationStructureKhr`](crate::VkAccelerationStructureKhr)
    ///
    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureKhr = 1000150000,

    /// [`VkValidationCacheExt`](crate::VkValidationCacheExt)
    ///
    /// Provided by [`ext_validation_cache`]
    ValidationCacheExt = 1000160000,

    /// [`VkAccelerationStructureNv`](crate::VkAccelerationStructureNv)
    ///
    /// Provided by [`nv_ray_tracing`]
    AccelerationStructureNv = 1000165000,

    /// [`VkPerformanceConfigurationIntel`](crate::VkPerformanceConfigurationIntel)
    ///
    /// Provided by [`intel_performance_query`]
    PerformanceConfigurationIntel = 1000210000,

    /// [`VkDeferredOperationKhr`](crate::VkDeferredOperationKhr)
    ///
    /// Provided by [`khr_deferred_host_operations`]
    DeferredOperationKhr = 1000268000,

    /// [`VkIndirectCommandsLayoutNv`](crate::VkIndirectCommandsLayoutNv)
    ///
    /// Provided by [`nv_device_generated_commands`]
    IndirectCommandsLayoutNv = 1000277000,

    /// [`VkCudaModuleNv`](crate::VkCudaModuleNv)
    ///
    /// Provided by [`nv_cuda_kernel_launch`]
    CudaModuleNv = 1000307000,

    /// [`VkCudaFunctionNv`](crate::VkCudaFunctionNv)
    ///
    /// Provided by [`nv_cuda_kernel_launch`]
    CudaFunctionNv = 1000307001,

    /// [`VkBufferCollectionFuchsia`](crate::VkBufferCollectionFuchsia)
    ///
    /// Provided by [`fuchsia_buffer_collection`]
    BufferCollectionFuchsia = 1000366000,

    /// [`VkMicromapExt`](crate::VkMicromapExt)
    ///
    /// Provided by [`ext_opacity_micromap`]
    MicromapExt = 1000396000,

    /// [`VkOpticalFlowSessionNv`](crate::VkOpticalFlowSessionNv)
    ///
    /// Provided by [`nv_optical_flow`]
    OpticalFlowSessionNv = 1000464000,

    /// [`VkShaderExt`](crate::VkShaderExt)
    ///
    /// Provided by [`ext_shader_object`]
    ShaderExt = 1000482000,
}
