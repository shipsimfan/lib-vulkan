use crate::{VkStructureType, khr_swapchain::VkDeviceGroupPresentModeFlagsKhr, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_1,
    khr_swapchain::{self, VkDeviceGroupPresentModeFlagKhr},
};

/// Structure specifying parameters of a newly created swapchain object
///
/// # Description
/// If this structure is not present, modes is considered to be
/// [`VkDeviceGroupPresentModeFlagKhr::LocalKhr`].
///
/// Provided by [`VK_VERSION_1_1`] with [`khr_swapchain`], [`khr_device_group`] with
/// [`khr_swapchain`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDeviceGroupSwapchainCreateInfoKhr {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::DeviceGroupSwapchainCreateInfoKhr`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `modes` is a bitfield of modes that the swapchain can be used with.
    ///
    /// # Valid Usage (Implicit)
    ///  - `modes` must be a valid combination of [`VkDeviceGroupPresentModeFlagKhr`] values
    ///  - `modes` must not be 0
    pub modes: VkDeviceGroupPresentModeFlagsKhr,
}

impl const Default for VkDeviceGroupSwapchainCreateInfoKhr {
    fn default() -> Self {
        VkDeviceGroupSwapchainCreateInfoKhr {
            r#type: VkStructureType::DeviceGroupSwapchainCreateInfoKhr,
            next: null(),
            modes: VkDeviceGroupPresentModeFlagsKhr::empty(),
        }
    }
}

impl NextChain for VkDeviceGroupSwapchainCreateInfoKhr {
    fn structure_type(&self) -> VkStructureType {
        self.r#type
    }

    fn next(&self) -> *const c_void {
        self.next
    }
    
    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: Option<&dyn NextChain>) {
        self.next = next.map_or(null(), |n| n.as_ptr());
    }
}
