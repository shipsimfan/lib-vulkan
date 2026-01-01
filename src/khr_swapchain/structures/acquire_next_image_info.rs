use crate::{VkFence, VkSemaphore, VkStructureType, khr_swapchain::VkSwapchainKhr};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VK_VERSION_1_1, VkDevice,
    khr_swapchain::{self, VkAcquireNextImageKhr},
};

/// Structure specifying parameters of the acquire
///
/// # Description
/// If [`VkAcquireNextImageKhr`] is used, the device mask is considered to include all physical
/// devices in the logical device.
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkAcquireNextImageInfoKhr {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::AcquireNextImageInfoKhr`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`]
    pub next: *const c_void,

    /// `swapchain` is a non-retired swapchain from which an image is acquired.
    ///
    /// # Valid Usage
    ///  - `swapchain` must not be in the retired state
    ///
    /// # Valid Usage (Implicit)
    ///  - `swapchain` must be a valid [`VkSwapchainKhr`] handle
    ///  - Each of `fence`, `semaphore`, and `swapchain` that are valid handles of non-ignored
    ///    parameters must have been created, allocated, or retrieved from the same [`VkDevice`]
    ///
    /// # Host Synchronization
    ///  - Host access to `swapchain` must be externally synchronized
    pub swapchain: VkSwapchainKhr,

    /// `timeout` specifies how long the function waits, in nanoseconds, if no image is available.
    pub timeout: u64,

    /// `semaphore` is [`VK_NULL_HANDLE`] or a semaphore to signal.
    ///
    /// # Valid Usage
    ///  - If `semaphore` is not [`VK_NULL_HANDLE`], it must be unsignaled
    ///  - If `semaphore` is not [`VK_NULL_HANDLE`], it must not have any uncompleted signal or
    ///    wait operations pending
    ///  - `semaphore` and `fence` must not both be equal to [`VK_NULL_HANDLE`]
    ///  - `semaphore` must have a [`VkSemaphoreType`] of [`VkSemaphoreType::Binary`]
    ///
    /// # Valid Usage (Implicit)
    ///  - If `semaphore` is not [`VK_NULL_HANDLE`], `semaphore` must be a valid [`VkSemaphore`]
    ///    handle
    ///  - Each of `fence`, `semaphore`, and `swapchain` that are valid handles of non-ignored
    ///    parameters must have been created, allocated, or retrieved from the same [`VkDevice`]
    ///
    /// # Host Synchronization
    ///  - Host access to `semaphore` must be externally synchronized
    pub semaphore: VkSemaphore,

    /// `fence` is [`VK_NULL_HANDLE`] or a fence to signal.
    ///
    /// # Valid Usage
    ///  - If `fence` is not [`VK_NULL_HANDLE`], `fence` must be unsignaled
    ///  - If `fence` is not [`VK_NULL_HANDLE`], `fence` must not be associated with any other
    ///    queue command that has not yet completed execution on that queue
    ///  - `semaphore` and `fence` must not both be equal to [`VK_NULL_HANDLE`]
    ///
    /// # Valid Usage (Implicit)
    ///  - If `fence` is not [`VK_NULL_HANDLE`], `fence` must be a valid [`VkFence`] handle
    ///  - Each of `fence`, `semaphore`, and `swapchain` that are valid handles of non-ignored
    ///    parameters must have been created, allocated, or retrieved from the same [`VkDevice`]
    ///
    /// # Host Synchronization
    ///  - Host access to `fence` must be externally synchronized
    pub fence: VkFence,

    /// `device_mask` is a mask of physical devices for which the swapchain image will be ready to
    /// use when the semaphore or fence is signaled.
    ///
    /// # Valid Usage
    ///  - `device_mask` must be a valid device mask
    ///  - `device_mask` must not be zero
    pub device_mask: u32,
}

impl const Default for VkAcquireNextImageInfoKhr {
    fn default() -> Self {
        VkAcquireNextImageInfoKhr {
            r#type: VkStructureType::AcquireNextImageInfoKhr,
            next: null(),
            swapchain: VkSwapchainKhr::null(),
            timeout: 0,
            semaphore: VkSemaphore::null(),
            fence: VkFence::null(),
            device_mask: 0,
        }
    }
}
