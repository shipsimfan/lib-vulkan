use crate::{
    VkStructureType,
    khr_swapchain::{VK_MAX_DEVICE_GROUP_SIZE, VkDeviceGroupPresentModeFlagsKhr},
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_1,
    khr_swapchain::{self, VkDeviceGroupPresentInfoKhr, VkDeviceGroupPresentModeFlagKhr},
};

/// Present capabilities from other physical devices
///
/// # Description
/// `modes` always has [`VkDeviceGroupPresentModeFlagKhr::LocalKhr`] set.
///
/// The present mode flags are also used when presenting an image, in
/// [`VkDeviceGroupPresentInfoKhr::mode`].
///
/// If a device group only includes a single physical device, then modes must equal
/// [`VkDeviceGroupPresentModeFlagKhr::LocalKhr`].
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceGroupPresentCapabilitiesKhr {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::DeviceGroupPresentCapabilitiesKhr`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`]
    pub next: *const c_void,

    /// `present_mask` is an array of [`VK_MAX_DEVICE_GROUP_SIZE`] [`u32`] masks, where the mask at
    /// element `i` is non-zero if physical device i has a presentation engine, and where bit `j`
    /// is set in element `i` if physical device `i` can present swapchain images from physical
    /// device `j`. If element `i` is non-zero, then bit `i` must be set.
    pub present_mask: [u32; VK_MAX_DEVICE_GROUP_SIZE],

    /// `modes` is a bitmask of [`VkDeviceGroupPresentModeFlagKhr`] indicating which device
    /// group presentation modes are supported.
    pub modes: VkDeviceGroupPresentModeFlagsKhr,
}

impl const Default for VkDeviceGroupPresentCapabilitiesKhr {
    fn default() -> Self {
        VkDeviceGroupPresentCapabilitiesKhr {
            r#type: VkStructureType::DeviceGroupPresentCapabilitiesKhr,
            next: null(),
            present_mask: [0; VK_MAX_DEVICE_GROUP_SIZE],
            modes: VkDeviceGroupPresentModeFlagsKhr::empty(),
        }
    }
}
