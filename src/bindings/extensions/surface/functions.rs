use super::VkSurfaceKHR;
use crate::{
    bindings::{VkBool32, VkInstance, VkPhysicalDevice},
    VkPresentModeKHR, VkResult, VkSurfaceCapabilitiesKHR, VkSurfaceFormatKHR,
};
use std::{ffi::c_void, ptr::NonNull};

pub type VkGetPhysicalDeviceSurfaceFormatsKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    p_surface_format_count: &mut u32,
    p_surface_formats: Option<NonNull<VkSurfaceFormatKHR>>,
) -> VkResult;

pub type VkGetPhysicalDeviceSurfaceCapabilitiesKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    p_surface_capabilities: &mut VkSurfaceCapabilitiesKHR,
) -> VkResult;

pub type VkGetPhysicalDeviceSurfacePresentModesKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    p_present_mode_count: &mut u32,
    p_present_modes: Option<NonNull<VkPresentModeKHR>>,
) -> VkResult;

pub type VkGetPhysicalDeviceSurfaceSupportKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    surface: VkSurfaceKHR,
    p_supported: &mut VkBool32,
) -> VkResult;

pub type VkDestroySurfaceKHR = extern "system" fn(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    p_allocator: Option<NonNull<c_void>>,
);
