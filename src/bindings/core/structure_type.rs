#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum VkStructureType {
    ApplicationInfo = 0,
    InstanceCreateInfo = 1,
    DeviceQueueCreateInfo = 2,
    DeviceCreateInfo = 3,
    ImageViewCreateInfo = 15,
    ShaderModuleCreateInfo = 16,
    PipelineShaderStageCreateInfo = 18,
    PipelineVertexINputStateCreateInfo = 19,
    PipelineDynamicStateCreateInfo = 27,
    SwapchainCreateInfo = 1000001000,
    #[cfg(target_os = "windows")]
    Win32SurfaceCreateInfo = 1000009000,
}
