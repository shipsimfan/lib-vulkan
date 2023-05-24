use super::VkDeviceGroupPresentModeFlagsKHR;
use crate::bindings::{VkStructureType, VK_MAX_DEVICE_GROUP_SIZE};
use std::{ffi::c_void, ptr::NonNull};

#[repr(C)]
pub struct VkDeviceGroupPresentCapabilitiesKHR {
    s_type: VkStructureType,
    p_next: Option<NonNull<c_void>>,
    present_mask: [u32; VK_MAX_DEVICE_GROUP_SIZE],
    modes: VkDeviceGroupPresentModeFlagsKHR,
}
