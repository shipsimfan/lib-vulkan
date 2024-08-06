use crate::{VkFence, VkSemaphore, VkStructureType, VkSwapchainKHR};
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_swapchain, VkAcquireNextImageKHR, VK_NULL_HANDLE, VK_VERSION_1_1};

/// Structure specifying parameters of the acquire
///
/// If [`VkAcquireNextImageKHR`] is used, the device mask is considered to include all physical
/// devices in the logical device.
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkAcquireNextImageInfoKHR {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `swapchain` is a non-retired swapchain from which an image is acquired.
    pub swapchain: VkSwapchainKHR,

    /// `timeout` specifies how long the function waits, in nanoseconds, if no image is available.
    pub timeout: u64,

    /// `semaphore` is [`VK_NULL_HANDLE`] or a semaphore to signal.
    pub semaphore: VkSemaphore,

    /// `fence` is [`VK_NULL_HANDLE`] or a fence to signal.
    pub fence: VkFence,

    /// `device_mask` is a mask of physical devices for which the swapchain image will be ready to
    /// use when the semaphore or fence is signaled.
    pub device_mask: u32,
}

impl Default for VkAcquireNextImageInfoKHR {
    fn default() -> Self {
        VkAcquireNextImageInfoKHR {
            r#type: VkStructureType::AcquireNextImageInfoKHR,
            next: null(),
            swapchain: null_mut(),
            timeout: 0,
            semaphore: null_mut(),
            fence: null_mut(),
            device_mask: 0,
        }
    }
}
