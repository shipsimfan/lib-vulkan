use crate::{VkStructureType, khr_swapchain::VkSwapchainKhr};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_1, khr_swapchain};

/// Specify that an image will be bound to swapchain memory
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkImageSwapchainCreateInfoKhr {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::ImageSwapchainCreateInfoKhr`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `swapchain` is [`VK_NULL_HANDLE`] or a handle of a swapchain that the image will be bound
    /// to.
    ///
    /// # Valid Usage
    ///  - If `swapchain` is not [`VK_NULL_HANDLE`], the fields of [`VkImageCreateInfo`] must match
    ///    the implied image creation parameters of the swapchain
    ///
    /// # Valid Usage (Implicit)
    ///  - If `swapchain` is not [`VK_NULL_HANDLE`], `swapchain` must be a valid [`VkSwapchainKhr`]
    ///    handle
    pub swapchain: VkSwapchainKhr,
}

impl Default for VkImageSwapchainCreateInfoKhr {
    fn default() -> Self {
        VkImageSwapchainCreateInfoKhr {
            r#type: VkStructureType::ImageSwapchainCreateInfoKhr,
            next: null(),
            swapchain: VkSwapchainKhr::null(),
        }
    }
}
