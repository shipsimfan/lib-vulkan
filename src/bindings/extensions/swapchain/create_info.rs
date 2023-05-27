use crate::{
    VkBool32, VkColorSpaceKHR, VkCompositeAlphaFlagBitsKHR, VkExtent2D, VkFormat,
    VkImageUsageFlags, VkPresentModeKHR, VkSharingMode, VkStructureType, VkSurfaceKHR,
    VkSurfaceTransformFlagBitsKHR, VkSwapchainCreateFlagsKHR, VkSwapchainKHR,
};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkSwapchainCreateInfoKHR {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkSwapchainCreateFlagsKHR,
    pub(crate) surface: VkSurfaceKHR,
    pub(crate) min_image_count: u32,
    pub(crate) image_format: VkFormat,
    pub(crate) image_color_space: VkColorSpaceKHR,
    pub(crate) image_extent: VkExtent2D,
    pub(crate) image_array_layers: u32,
    pub(crate) image_usage: VkImageUsageFlags,
    pub(crate) image_sharing_mode: VkSharingMode,
    pub(crate) queue_family_index_count: u32,
    pub(crate) p_queue_family_indices: *const u32,
    pub(crate) pre_transform: VkSurfaceTransformFlagBitsKHR,
    pub(crate) composite_alpha: VkCompositeAlphaFlagBitsKHR,
    pub(crate) present_mode: VkPresentModeKHR,
    pub(crate) clipped: VkBool32,
    pub(crate) old_swapchain: Option<VkSwapchainKHR>,
}
