use crate::{
    VkAllocationCallbacks, VkBool32, VkInstance, VkPhysicalDevice, VkPresentModeKHR, VkResult,
    VkSurfaceCapabilitiesKHR, VkSurfaceFormatKHR, VkSurfaceKHR,
};

pub(crate) type VkDestroySurfaceKHR = extern "system" fn(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    p_allocator: *const VkAllocationCallbacks,
);

pub(crate) type VkGetPhysicalDeviceSurfaceCapabilities = extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    p_surface_capabilities: *mut VkSurfaceCapabilitiesKHR,
) -> VkResult;

pub(crate) type VkGetPhysicalDeviceSurfaceFormatsKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut VkSurfaceFormatKHR,
) -> VkResult;

pub(crate) type VkGetPhysicalDeviceSurfacePresentModesKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut VkPresentModeKHR,
) -> VkResult;

pub(crate) type VkGetPhysicalDeviceSurfaceSupportKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    surface: VkSurfaceKHR,
    p_supported: *mut VkBool32,
) -> VkResult;
