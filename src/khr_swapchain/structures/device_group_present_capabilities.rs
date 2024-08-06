use crate::{VkDeviceGroupPresentModeFlagsKHR, VkStructureType, VK_MAX_DEVICE_GROUP_SIZE};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    khr_swapchain, VkDeviceGroupPresentInfoKHR, VkDeviceGroupPresentModeFlagBitsKHR, VK_VERSION_1_1,
};

/// Present capabilities from other physical devices
///
/// modes always has [`VkDeviceGroupPresentModeFlagBitsKHR::LocalBitKhr`] set.
///
/// The present mode flags are also used when presenting an image, in
/// [`VkDeviceGroupPresentInfoKHR::mode`].
///
/// If a device group only includes a single physical device, then modes must equal
/// [`VkDeviceGroupPresentModeFlagBitsKHR::LocalBitKhr`].
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceGroupPresentCapabilitiesKHR {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `present_mask` is an array of [`VK_MAX_DEVICE_GROUP_SIZE`] [`u32`] masks, where the mask at
    /// element `i` is non-zero if physical device i has a presentation engine, and where bit `j`
    /// is set in element `i` if physical device `i` can present swapchain images from physical
    /// device `j`. If element `i` is non-zero, then bit `i` must be set.
    pub present_mask: [u32; VK_MAX_DEVICE_GROUP_SIZE],

    /// `modes` is a bitmask of [`VkDeviceGroupPresentModeFlagBitsKHR`] indicating which device
    /// group presentation modes are supported.
    pub modes: VkDeviceGroupPresentModeFlagsKHR,
}

impl Default for VkDeviceGroupPresentCapabilitiesKHR {
    fn default() -> Self {
        VkDeviceGroupPresentCapabilitiesKHR {
            r#type: VkStructureType::DeviceGroupPresentCapabilitiesKHR,
            next: null(),
            present_mask: [0; VK_MAX_DEVICE_GROUP_SIZE],
            modes: 0,
        }
    }
}
