use crate::{VkStructureType, khr_swapchain::VkDeviceGroupPresentModeFlagKhr};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_1,
    khr_swapchain::{
        self, VkAcquireNextImageInfoKhr, VkDeviceGroupPresentCapabilitiesKhr,
        VkDeviceGroupSwapchainCreateInfoKhr, VkPresentInfoKhr,
    },
};

/// Mode and mask controlling which physical devices' images are presented
///
/// # Description
/// If mode is [`VkDeviceGroupPresentModeFlagKhr::LocalKhr`], then each element of
/// `device_masks` selects which instance of the swapchain image is presented. Each element of
/// `device_masks` must have exactly one bit set, and the corresponding physical device must have a
/// presentation engine as reported by [`VkDeviceGroupPresentCapabilitiesKhr`].
///
/// If mode is [`VkDeviceGroupPresentModeFlagKhr::RemoteKhr`], then each element of
/// `device_masks` selects which instance of the swapchain image is presented. Each element of
/// `device_masks` must have exactly one bit set, and some physical device in the logical device
/// must include that bit in its [`VkDeviceGroupPresentCapabilitiesKhr::present_mask`].
///
/// If mode is [`VkDeviceGroupPresentModeFlagKhr::SumKhr`], then each element of
/// `device_masks` selects which instances of the swapchain image are component-wise summed and the
/// sum of those images is presented. If the sum in any component is outside the representable
/// range, the value of that component is undefined. Each element of `device_masks` must have a
/// value for which all set bits are set in one of the elements of
/// [`VkDeviceGroupPresentCapabilitiesKhr::present_mask`].
///
/// If mode is [`VkDeviceGroupPresentModeFlagKhr::LocalMultiDeviceKhr`], then each element
/// of `device_masks` selects which instance(s) of the swapchain images are presented. For each bit
/// set in each element of `device_masks`, the corresponding physical device must have a
/// presentation engine as reported by [`VkDeviceGroupPresentCapabilitiesKhr`].
///
/// If [`VkDeviceGroupPresentInfoKhr`] is not provided or `swapchain_count` is zero then the masks
/// are considered to be 1. If [`VkDeviceGroupPresentInfoKhr`] is not provided, mode is considered
/// to be [`VkDeviceGroupPresentModeFlagKhr::LocalKhr`].
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceGroupPresentInfoKhr {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::DeviceGroupPresentInfoKhr`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `swapchain_count` is zero or the number of elements in `device_masks`.
    ///
    /// # Valid Usage
    ///  - `swapchain_count` must equal 0 or [`VkPresentInfoKhr::swapchain_count`]
    pub swapchain_count: u32,

    /// `device_masks` is a pointer to an array of device masks, one for each element of
    /// [`VkPresentInfoKhr::swapchains`].
    ///
    /// # Valid Usage
    ///  - If `mode` is [`VkDeviceGroupPresentModeFlagKhr::LocalKhr`], then each element of
    ///    `device_masks` must have exactly one bit set, and the corresponding element of
    ///    [`VkDeviceGroupPresentCapabilitiesKhr::present_mask`] must be non-zero
    ///  - If `mode` is [`VkDeviceGroupPresentModeFlagKhr::RemoteKhr`], then each element of
    ///    `device_masks` must have exactly one bit set, and some physical device in the logical
    ///    device must include that bit in its
    ///    [`VkDeviceGroupPresentCapabilitiesKhr::present_mask`]
    ///  - If `mode` is [`VkDeviceGroupPresentModeFlagKhr::SumKhr`], then each element of
    ///    `device_masks` must have a value for which all set bits are set in one of the elements
    ///    of [`VkDeviceGroupPresentCapabilitiesKhr::present_mask`]
    ///  - If `mode` is [`VkDeviceGroupPresentModeFlagKhr::LocalMultiDeviceKhr`], then for each
    ///    bit set in each element of `device_masks`, the corresponding element of
    ///    [`VkDeviceGroupPresentCapabilitiesKhr::present_mask`] must be non-zero
    ///  - The value of each element of `device_masks` must be equal to the device mask passed in
    ///    [`VkAcquireNextImageInfoKhr::device_mask`] when the image index was last acquired
    ///
    /// # Valid Usage (Implicit)
    ///  - If `swapchain_count` is not 0, `device_masks` must be a valid pointer to an array of
    ///    `swapchain_count` [`u32`] values
    pub device_masks: *const u32,

    /// `mode` is a [`VkDeviceGroupPresentModeFlagKhr`] value specifying the device group
    /// present mode that will be used for this present.
    ///
    /// # Valid Usage
    ///  - `mode` must have exactly one bit set, and that bit must have been included in
    ///    [`VkDeviceGroupSwapchainCreateInfoKhr::modes`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `mode` must be a valid [`VkDeviceGroupPresentModeFlagKhr`] value
    pub mode: VkDeviceGroupPresentModeFlagKhr,
}

impl const Default for VkDeviceGroupPresentInfoKhr {
    fn default() -> Self {
        VkDeviceGroupPresentInfoKhr {
            r#type: VkStructureType::DeviceGroupPresentInfoKhr,
            next: null(),
            swapchain_count: 0,
            device_masks: null(),
            mode: VkDeviceGroupPresentModeFlagKhr::LocalKhr,
        }
    }
}
