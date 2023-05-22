use crate::{bindings::VkStructureType, VkQueueGlobalPriorityKHR};
use std::{ffi::c_void, ptr::NonNull};

#[repr(C)]
pub struct VkDeviceQueueGlobalPriorityCreateInfoKHR {
    s_type: VkStructureType,
    p_next: Option<NonNull<c_void>>,
    global_priority: VkQueueGlobalPriorityKHR,
}

impl VkDeviceQueueGlobalPriorityCreateInfoKHR {
    pub fn new(global_priority: VkQueueGlobalPriorityKHR) -> Self {
        VkDeviceQueueGlobalPriorityCreateInfoKHR {
            s_type: VkStructureType::DeviceQueueGlobalPriorityCreateInfoKhr,
            p_next: None,
            global_priority,
        }
    }

    pub fn global_priority(&self) -> VkQueueGlobalPriorityKHR {
        self.global_priority
    }
}
