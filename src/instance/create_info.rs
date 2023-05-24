use crate::VkStructureType;
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkInstanceCreateInfo {
    s_type: VkStructureType,
    p_next: *const c_void,
    flags: VkInstanceCreateFlags,
    p_application_info: VkApplicationInfo,
}

pub struct InstanceCreateInfo {}

impl InstanceCreateInfo {
    pub(super) fn into_vk(self) -> VkInstanceCreateInfo {
        VkInstanceCreateInfo {
            s_type: VkStructureType::InstanceCreateInfo,
        }
    }
}
