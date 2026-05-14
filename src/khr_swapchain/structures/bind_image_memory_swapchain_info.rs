use crate::{VkStructureType, khr_swapchain::VkSwapchainKhr};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VK_VERSION_1_1,
    khr_swapchain::{
        self, VkAcquireNextImage2Khr, VkAcquireNextImageKhr, VkSwapchainCreateFlagKhr,
    },
};

/// Structure specifying swapchain image memory to bind to
///
/// # Description
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
pub struct VkBindImageMemorySwapchainInfoKhr {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::BindImageMemorySwapchainInfoKhr`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `swapchain` is [`VK_NULL_HANDLE`] or a swapchain handle.
    ///
    /// # Valid Usage (Implicit)
    ///  - `swapchain` must be a valid [`VkSwapchainKhr`] handle
    ///
    /// # Host Synchronization
    ///  - Host access to `swapchain` must be externally synchronized
    pub swapchain: VkSwapchainKhr,

    /// `image_index` is an image index within `swapchain`.
    ///
    /// # Valid Usage
    ///  - `image_index` must be less than the number of images in `swapchain`
    ///  - If the `swapchain` has been created with
    ///    [`VkSwapchainCreateFlagKhr::DeferredMemoryAllocationKhr`], `image_index` must be one
    ///    that has previously been returned by [`VkAcquireNextImageKhr`] or
    ///    [`VkAcquireNextImage2Khr`]
    pub image_index: u32,
}

impl const Default for VkBindImageMemorySwapchainInfoKhr {
    fn default() -> Self {
        VkBindImageMemorySwapchainInfoKhr {
            r#type: VkStructureType::BindImageMemorySwapchainInfoKhr,
            next: null(),
            swapchain: VkSwapchainKhr::null(),
            image_index: 0,
        }
    }
}
