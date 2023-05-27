mod surface;
mod swapchain;
#[cfg(target_os = "windows")]
mod win32_surface;

pub use surface::{
    VkColorSpaceKHR, VkCompositeAlphaFlagBitsKHR, VkCompositeAlphaFlagsKHR, VkPresentModeKHR,
    VkSurfaceCapabilitiesKHR, VkSurfaceFormatKHR, VkSurfaceTransformFlagBitsKHR,
    VkSurfaceTransformFlagsKHR,
};

pub(crate) use surface::*;
pub(crate) use swapchain::*;
#[cfg(target_os = "windows")]
pub(crate) use win32_surface::*;
