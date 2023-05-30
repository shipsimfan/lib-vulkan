#![feature(extern_types)]
#![feature(concat_idents)]

mod bindings;
mod core;
mod cstring_util;
mod extensions;
mod library;
mod loader;

pub use self::core::{
    ApplicationInfo, Device, DeviceCreateInfo, DeviceQueueCreateInfo, ExtensionProperties, Image,
    ImageView, ImageViewCreateInfo, Instance, InstanceCreateInfo, LayerProperties, PhysicalDevice,
    PhysicalDeviceFeatures, PhysicalDeviceLimits, PhysicalDeviceProperties,
    PhysicalDeviceSparseProperties, PipelineInputAssemblyStateCreateInfo,
    PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo,
    PipelineShaderStageCreateInfo, PipelineVertexInputStateCreateInfo, PipelineViewportState,
    PipelineViewportStateCreateInfo, Queue, ShaderModule,
};
pub use bindings::{
    VkColorSpaceKHR as ColorSpace, VkComponentMapping as ComponentMapping,
    VkComponentSwizzle as ComponentSwizzle, VkCompositeAlphaFlagBitsKHR as CompositeAlphaFlagBits,
    VkCompositeAlphaFlagsKHR as CompositeAlphaFlags, VkCullModeFlagBits as CullModeFlagBits,
    VkCullModeFlags as CullModeFlags, VkDynamicState as DynamicState, VkExtent2D as Extent2D,
    VkExtent3D as Extent3D, VkFormat as Format, VkFrontFace as FrontFace,
    VkImageAspectFlagBits as ImageAspectFlagBits, VkImageAspectFlags as ImageAspectFlags,
    VkImageSubresourceRange as ImageSubresourceRange, VkImageUsageFlagBits as ImageUsageFlagBits,
    VkImageUsageFlags as ImageUsageFlags, VkImageViewType as ImageViewType, VkInstance,
    VkOffset2D as Offset2D, VkPhysicalDeviceType as PhysicalDeviceType,
    VkPolygonMode as PolygonMode, VkPresentModeKHR as PresentMode,
    VkPrimitiveTopology as PrimitiveTopology, VkQueueFamilyProperties as QueueFamilyProperties,
    VkQueueFlagBits as QueueFlagBits, VkQueueFlags as QueueFlags, VkRect2D as Rect2D, VkResult,
    VkSampleCountFlagBits as SampleCountFlagBits, VkSampleCountFlags as SampleCountFlags,
    VkShaderStageFlagBits as ShaderStageFlagBits, VkShaderStageFlags as ShaderStageFlags,
    VkSharingMode as SharingMode, VkSpecializationMapEntry as SpecializationMapEntry,
    VkSurfaceCapabilitiesKHR as SurfaceCapabilities, VkSurfaceFormatKHR as SurfaceFormat,
    VkSurfaceTransformFlagBitsKHR as SurfaceTransformFlagBits,
    VkSurfaceTransformFlagsKHR as SurfaceTransformFlags, VkVersion as Version,
    VkVertexInputAttributeDescription as VertexInputAttributeDescription,
    VkVertexInputBindingDescription as VertexInputBindingDescription,
    VkVertexInputRate as VertexInputRate, VkViewport as Viewport, VK_API_VERSION_1_0,
    VK_API_VERSION_1_1, VK_API_VERSION_1_2, VK_API_VERSION_1_3, VK_HEADER_VERSION,
    VK_HEADER_VERSION_COMPLETE, VK_UUID_SIZE,
};
#[cfg(target_os = "windows")]
pub use extensions::Win32SurfaceCreateInfo;
pub use extensions::{Surface, Swapchain, SwapchainCreateInfo};
pub use library::Library;
pub use loader::{Loader, NativeLoader};

pub(crate) use self::core::*;
pub(crate) use bindings::*;
pub(crate) use cstring_util::*;
pub(crate) use extensions::*;

pub type Result<T> = std::result::Result<T, VkResult>;
