use crate::{VkResult, VkSemaphore, VkStructureType, khr_swapchain::VkSwapchainKhr};
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_TRUE, VkDevice, khr_swapchain};

/// Structure describing parameters of a queue presentation
///
/// Before an application can present an image, the image’s layout must be transitioned to the
/// [`VkImageLayout::PresentSrcKhr`] layout, or for a shared presentable image the
/// [`VkImageLayout::SharedPresentKhr`] layout.
///
/// Provided by [`khr_swapchain`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPresentInfoKhr {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PresentInfoKhr`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If a [`VkPresentIdKhr`] structure is included in the `next` chain, and the `present_id`
    ///    feature is not enabled, each `present_id`s entry in that structure must be [`null`]
    ///  - If the [`swapchain_maintenance1`] feature is not enabled, then the `next` chain must not
    ///    include a [`VkSwapchainPresentFenceInfoKhr`] structure
    ///  - If the `next` chain of this structure includes a [`VkFrameBoundaryTensorsArms`]
    ///    structure then it must also include a [`VkFrameBoundaryExt`] structure
    ///  - If a [`VkPresentId2Khr`] structure is included in the `next` chain, and the
    ///    `present_id2` feature is not enabled, each `present_id`s entry in that structure must be
    ///    zero
    ///  - If a [`VkPresentId2Khr`] structure is included and contains non-zero `present_id`s,
    ///    `present_d2_supported` must be [`VK_TRUE`] in the [`VkSurfaceCapabilitiesPresentId2Khr`]
    ///    structure returned by [`VkGetPhysicalDeviceSurfaceCapabilities2Khr`] for the surface
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of [`VkDeviceGroupPresentInfoKhr`],
    ///    [`VkDisplayPresentInfoKhr`], [`VkFrameBoundaryExt`], [`VkFrameBoundaryTensorsArm`],
    ///    [`VkPresentFrameTokenGgp`], [`VkPresentId2Khr`], [`VkPresentIdKhr`],
    ///    [`VkPresentRegionsKhr`], [`VkPresentTimesInfoGoogle`], [`VkPresentTimingsInfoExt`],
    ///    [`VkSetPresentConfigNv`], [`VkSwapchainPresentFenceInfoKhr`], or
    ///    [`VkSwapchainPresentModeInfoKhr`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `wait_semaphore_count` is the number of semaphores to wait for before issuing the present
    /// request. The number may be zero.
    pub wait_semaphore_count: u32,

    /// `wait_semaphores` is [`null`] or a pointer to an array of [`VkSemaphore`] objects with
    /// `wait_semaphore_count` entries, and specifies the semaphores to wait for before issuing the
    /// present request.
    ///
    /// # Valid Usage (Implicit)
    ///  - If `wait_semaphore_count` is not 0, `wait_semaphores` must be a valid pointer to an
    ///    array of `wait_semaphore_count` valid [`VkSemaphore`] handles
    ///  - Both of the elements of `swapchains`, and the elements of `wait_semaphores` that are
    ///    valid handles of non-ignored parameters must have been created, allocated, or retrieved
    ///    from the same [`VkDevice`]
    ///
    /// # Host Synchronization
    ///  - Host access to each member of `wait_semaphores` must be externally synchronized
    pub wait_semaphores: *const VkSemaphore,

    /// `swapchain_count` is the number of swapchains being presented to by this command.
    ///
    /// # Valid Usage (Implicit)
    ///  - `swapchain_count` must be greater than 0
    pub swapchain_count: u32,

    /// `swapchains` is a pointer to an array of [`VkSwapchainKhr`] objects with `swapchain_count`
    /// entries.
    ///
    /// # Valid Usage
    ///  - Elements of `swapchains` must be unique
    ///  - If any element of the `swapchains` array has been created with
    ///    [`VkSwapchainPresentModesCreateInfoKhr`], all of the elements of this array must be
    ///    created with [`VkSwapchainPresentModesCreateInfoKhr`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `swapchains` must be a valid pointer to an array of `swapchain_count` valid
    ///    [`VkSwapchainKhr`] handles
    ///  - Both of the elements of `swapchains`, and the elements of `wait_semaphores` that are
    ///    valid handles of non-ignored parameters must have been created, allocated, or retrieved
    ///    from the same [`VkDevice`]
    ///
    /// # Host Synchronization
    ///  - Host access to each member of `swapchains` must be externally synchronized
    pub swapchains: *const VkSwapchainKhr,

    /// `image_indices` is a pointer to an array of indices into the array of each swapchain’s
    /// presentable images, with `swapchain_count` entries. Each entry in this array identifies the
    /// image to present on the corresponding entry in the `swapchains` array.
    ///
    /// # Valid Usage
    ///  - Each element of `image_indices` must be the index of a presentable image acquired from
    ///    the swapchain specified by the corresponding element of the `swapchains` array, and the
    ///    presented image subresource must be in the [`VkImageLayout::PresentSrcKhr`] or
    ///    [`VkImageLayout::ShadedPresentKhr`] layout at the time the operation is executed on a
    ///    [`VkDevice`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `image_indices` must be a valid pointer to an array of `swapchain_count` [`u32`] values
    pub image_indices: *const u32,

    /// `results` is a pointer to an array of [`VkResult`] typed elements with `swapchain_count`
    /// entries. Applications that do not need per-swapchain results can use [`null_mut`] for
    /// `results`. If not [`null_mut`], each entry in `results` will be set to the [`VkResult`] for
    /// presenting the swapchain corresponding to the same index in `swapchains`.
    ///
    /// # Valid Usage (Implicit)
    ///  - If `results` is not [`null_mut`], `results` must be a valid pointer to an array of
    ///    `swapchain_count` [`VkResult`] values
    pub results: *mut VkResult,
}

impl Default for VkPresentInfoKhr {
    fn default() -> Self {
        VkPresentInfoKhr {
            r#type: VkStructureType::PresentInfoKhr,
            next: null(),
            wait_semaphore_count: 0,
            wait_semaphores: null(),
            swapchain_count: 0,
            swapchains: null(),
            image_indices: null(),
            results: null_mut(),
        }
    }
}
