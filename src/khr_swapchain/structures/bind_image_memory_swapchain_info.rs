use crate::{VkStructureType, VkSwapchainKHR};
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_swapchain, VK_NULL_HANDLE, VK_VERSION_1_1};

/// Structure specifying swapchain image memory to bind to
///
/// If `swapchain` is not [`null`], the `swapchain` and `image_index` are used to determine the
/// memory that the image is bound to, instead of `memory` and `memory_offset`.
///
/// Memory can be bound to a swapchain and use the `device_indices` or
/// `split_instance_bind_regions` members of [`VkBindImageMemoryDeviceGroupInfo`].
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkBindImageMemorySwapchainInfoKHR {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `swapchain` is [`VK_NULL_HANDLE`] or a swapchain handle.
    pub swapchain: VkSwapchainKHR,

    /// `image_index` is an image index within swapchain.
    pub image_index: u32,
}

impl Default for VkBindImageMemorySwapchainInfoKHR {
    fn default() -> Self {
        VkBindImageMemorySwapchainInfoKHR {
            r#type: VkStructureType::BindImageMemorySwapchainInfoKHR,
            next: null(),
            swapchain: null_mut(),
            image_index: 0,
        }
    }
}
