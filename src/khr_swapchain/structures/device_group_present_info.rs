use crate::{VkDeviceGroupPresentModeFlagBitsKHR, VkStructureType};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_swapchain, VkDeviceGroupPresentCapabilitiesKHR, VkPresentInfoKHR, VK_VERSION_1_1};

/// Mode and mask controlling which physical devices' images are presented
///
/// If mode is [`VkDeviceGroupPresentModeFlagBitsKHR::LocalBitKhr`], then each element of
/// `device_masks` selects which instance of the swapchain image is presented. Each element of
/// `device_masks` must have exactly one bit set, and the corresponding physical device must have a
/// presentation engine as reported by [`VkDeviceGroupPresentCapabilitiesKHR`].
///
/// If mode is [`VkDeviceGroupPresentModeFlagBitsKHR::RemoteBitKhr`], then each element of
/// `device_masks` selects which instance of the swapchain image is presented. Each element of
/// `device_masks` must have exactly one bit set, and some physical device in the logical device
/// must include that bit in its [`VkDeviceGroupPresentCapabilitiesKHR::present_mask`].
///
/// If mode is [`VkDeviceGroupPresentModeFlagBitsKHR::SumBitKhr`], then each element of
/// `device_masks` selects which instances of the swapchain image are component-wise summed and the
/// sum of those images is presented. If the sum in any component is outside the representable
/// range, the value of that component is undefined. Each element of `device_masks` must have a
/// value for which all set bits are set in one of the elements of
/// [`VkDeviceGroupPresentCapabilitiesKHR::present_mask`].
///
/// If mode is [`VkDeviceGroupPresentModeFlagBitsKHR::LocalMultiDeviceBitKhr`], then each element
/// of `device_masks` selects which instance(s) of the swapchain images are presented. For each bit
/// set in each element of `device_masks`, the corresponding physical device must have a
/// presentation engine as reported by [`VkDeviceGroupPresentCapabilitiesKHR`].
///
/// If [`VkDeviceGroupPresentInfoKHR`] is not provided or `swapchain_count` is zero then the masks
/// are considered to be 1. If [`VkDeviceGroupPresentInfoKHR`] is not provided, mode is considered
/// to be [`VkDeviceGroupPresentModeFlagBitsKHR::LocalBitKhr`].
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceGroupPresentInfoKHR {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `swapchain_count` is zero or the number of elements in `device_masks`.
    pub swapchain_count: u32,

    /// `device_masks` is a pointer to an array of device masks, one for each element of
    /// [`VkPresentInfoKHR::swapchains`].
    pub device_masks: *const u32,

    /// `mode` is a [`VkDeviceGroupPresentModeFlagBitsKHR`] value specifying the device group
    /// present mode that will be used for this present.
    pub mode: VkDeviceGroupPresentModeFlagBitsKHR,
}

impl Default for VkDeviceGroupPresentInfoKHR {
    fn default() -> Self {
        VkDeviceGroupPresentInfoKHR {
            r#type: VkStructureType::DeviceGroupPresentInfoKHR,
            next: null(),
            swapchain_count: 0,
            device_masks: null(),
            mode: VkDeviceGroupPresentModeFlagBitsKHR::LocalBitKhr,
        }
    }
}
