// rustdoc imports
#[allow(unused_imports)]
use crate::{ext_debug_utils, khr_surface, VK_VERSION_1_0, VK_VERSION_1_1, VK_VERSION_1_3};

/// Specify an enumeration to track object handle types
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkObjectType {
    Unknown = 0,
    Instance = 1,
    PhysicalDevice = 2,
    Device = 3,
    Queue = 4,
    Semaphore = 5,
    CommandBuffer = 6,
    Fence = 7,
    DeviceMemory = 8,
    Buffer = 9,
    Image = 10,
    Event = 11,
    QueryPool = 12,
    BufferView = 13,
    ImageView = 14,
    ShaderModule = 15,
    PipelineCache = 16,
    PipelineLayout = 17,
    RenderPass = 18,
    Pipeline = 19,
    DescriptorSetLayout = 20,
    Sampler = 21,
    DescriptorPool = 22,
    DescriptorSet = 23,
    Framebuffer = 24,
    CommandPool = 25,

    /// Provided by [`VK_VERSION_1_1`]
    SamplerYCBCRConversion = 1000156000,

    /// Provided by [`VK_VERSION_1_1`]
    DescriptorUpdateTemplate = 1000085000,

    /// Provided by [`VK_VERSION_1_3`]
    PrivateDataSlot = 1000295000,

    /// Provided by [`khr_surface`]
    SurfaceKHR = 1000000000,

    /// Provided by [`khr_swapchain`]
    SwapchainKHR = 1000001000,

    /// Provided by [`khr_display`]
    DisplayKHR = 1000002000,

    /// Provided by [`khr_display`]
    DisplayModeKHR = 1000002001,

    /// Provided by [`ext_debug_report`]
    DebugReportCallbackEXT = 1000011000,

    /// Provided by [`khr_video_queue`]
    VideoSessionKHR = 1000023000,

    /// Provided by [`khr_video_queue`]
    VideoSessionParametersKHR = 1000023001,

    /// Provided by [`nvx_binary_import`]
    CuModuleNVX = 1000029000,

    /// Provided by [`nvx_binary_import`]
    CuFunctionNVX = 1000029001,

    /// Provided by [`ext_debug_utils`]
    DebugUtilsMessengerEXT = 1000128000,

    /// Provided by [`khr_acceleration_structure`]
    AccelerationStructureKHR = 1000150000,

    /// Provided by [`ext_validation_cache`]
    ValidationCacheEXT = 1000160000,

    /// Provided by [`nv_ray_tracing`]
    AccelerationStructureNV = 1000165000,

    /// Provided by [`intel_performance_query`]
    PerformanceConfigurationIntel = 1000210000,

    /// Provided by [`khr_deferred_host_operations`]
    DeferredOperationKHR = 1000268000,

    /// Provided by [`nv_device_generated_commands`]
    IndirectCommandsLayoutNV = 1000277000,

    /// Provided by [`nv_cuda_kernel_launch`]
    CudaModuleNV = 1000307000,

    /// Provided by [`nv_cuda_kernel_launch`]
    CudaFunctionNV = 1000307001,

    /// Provided by [`fuchsia_buffer_collection`]
    BufferCollectionFuchsia = 1000366000,

    /// Provided by [`ext_opacity_micromap`]
    MicromapEXT = 1000396000,

    /// Provided by [`nv_optical_flow`]
    OpticalFlowSessionNV = 1000464000,

    /// Provided by [`ext_shader_object`]
    ShaderEXT = 1000482000,
}
