use crate::{VkDeviceGroupPresentModeFlagsKHR, VkStructureType};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_swapchain, VkDeviceGroupPresentModeFlagBitsKHR, VK_VERSION_1_1};

/// Structure specifying parameters of a newly created swapchain object
///
/// If this structure is not present, modes is considered to be
/// [`VkDeviceGroupPresentModeFlagBitsKHR::LocalBitKhr`].
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceGroupSwapchainCreateInfoKHR {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `modes` is a bitfield of modes that the swapchain can be used with.
    pub modes: VkDeviceGroupPresentModeFlagsKHR,
}

impl Default for VkDeviceGroupSwapchainCreateInfoKHR {
    fn default() -> Self {
        VkDeviceGroupSwapchainCreateInfoKHR {
            r#type: VkStructureType::DeviceGroupSwapchainCreateInfoKHR,
            next: null(),
            modes: 0,
        }
    }
}
