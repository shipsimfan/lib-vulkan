use crate::{VkResult, VkSemaphore, VkStructureType, VkSwapchainKHR};
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_swapchain;

/// Structure describing parameters of a queue presentation
///
/// Before an application can present an image, the image’s layout **must** be transitioned to the
/// [`VkImageLayout::PresentSrcKHR`] layout, or for a shared presentable image the
/// [`VkImageLayout::SharedPresentKHR`] layout.
///
/// Provided by [`khr_swapchain`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPresentInfoKHR {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `wait_semaphore_count` is the number of semaphores to wait for before issuing the present
    /// request. The number may be zero.
    pub wait_semaphore_count: u32,

    /// `wait_semaphores` is [`null`] or a pointer to an array of [`VkSemaphore`] objects with
    /// `wait_semaphore_count` entries, and specifies the semaphores to wait for before issuing the
    /// present request.
    pub wait_semaphore: *const VkSemaphore,

    /// `swapchain_count` is the number of swapchains being presented to by this command.
    pub swapchain_count: u32,

    /// `swapchains` is a pointer to an array of [`VkSwapchainKHR`] objects with `swapchain_count`
    /// entries.
    pub swapchains: *const VkSwapchainKHR,

    /// `image_indices` is a pointer to an array of indices into the array of each swapchain’s
    /// presentable images, with `swapchain_count` entries. Each entry in this array identifies the
    /// image to present on the corresponding entry in the `swapchains` array.
    pub image_indices: *const u32,

    /// `results` is a pointer to an array of [`VkResult`] typed elements with `swapchain_count`
    /// entries. Applications that do not need per-swapchain results can use [`null_mut`] for
    /// `results`. If not [`null_mut`], each entry in `results` will be set to the [`VkResult`] for
    /// presenting the swapchain corresponding to the same index in `swapchains`.
    pub results: *mut VkResult,
}

impl Default for VkPresentInfoKHR {
    fn default() -> Self {
        VkPresentInfoKHR {
            r#type: VkStructureType::PresentInfoKHR,
            next: null(),
            wait_semaphore_count: 0,
            wait_semaphore: null(),
            swapchain_count: 0,
            swapchains: null(),
            image_indices: null(),
            results: null_mut(),
        }
    }
}
