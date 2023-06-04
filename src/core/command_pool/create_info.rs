use crate::{VkCommandPoolCreateFlags, VkCommandPoolCreateInfo, VkStructureType};
use std::ptr::null;

pub struct CommandPoolCreateInfo {
    pub queue_family_index: u32,
}

impl CommandPoolCreateInfo {
    pub(super) fn into_binding(&self) -> VkCommandPoolCreateInfo {
        VkCommandPoolCreateInfo {
            s_type: VkStructureType::CommandPoolCreateInfo,
            p_next: null(),
            flags: VkCommandPoolCreateFlags::default(),
            queue_family_index: self.queue_family_index,
        }
    }
}
