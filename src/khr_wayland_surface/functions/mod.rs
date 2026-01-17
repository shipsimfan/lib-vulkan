mod create_wayland_surface;
mod get_physical_device_wayland_presentation_support;

pub use create_wayland_surface::{VK_CREATE_WAYLAND_SURFACE_KHR, VkCreateWaylandSurfaceKhr};
pub use get_physical_device_wayland_presentation_support::{
    VK_GET_PHYSICAL_DEVICE_WAYLAND_PRESENTATION_SUPPORT_KHR,
    VkGetPhysicalDeviceWaylandPresentationSupportKhr,
};
