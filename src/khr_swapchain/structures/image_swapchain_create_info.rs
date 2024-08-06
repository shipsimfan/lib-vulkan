use crate::{VkStructureType, VkSwapchainKHR};
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_swapchain, VK_NULL_HANDLE, VK_VERSION_1_1};

/// Specify that an image will be bound to swapchain memory
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageSwapchainCreateInfoKHR {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `swapchain` is [`VK_NULL_HANDLE`] or a handle of a swapchain that the image will be bound
    /// to.
    pub swapchain: VkSwapchainKHR,
}

impl Default for VkImageSwapchainCreateInfoKHR {
    fn default() -> Self {
        VkImageSwapchainCreateInfoKHR {
            r#type: VkStructureType::ImageSwapchainCreateInfoKHR,
            next: null(),
            swapchain: null_mut(),
        }
    }
}
