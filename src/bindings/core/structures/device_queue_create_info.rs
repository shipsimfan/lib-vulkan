use crate::{bindings::VkStructureType, VkDeviceQueueCreateFlags};
use std::{ffi::c_void, marker::PhantomData, ptr::NonNull};

#[repr(C)]
pub struct VkDeviceQueueCreateInfo<'a> {
    s_type: VkStructureType,
    p_next: Option<NonNull<c_void>>,
    flags: VkDeviceQueueCreateFlags,
    queue_family_index: u32,
    queue_count: u32,
    p_queue_priorities: NonNull<f32>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> VkDeviceQueueCreateInfo<'a> {
    pub fn new(
        flags: VkDeviceQueueCreateFlags,
        queue_family_index: u32,
        queue_priorities: &'a [f32],
    ) -> Self {
        assert!(queue_priorities.len() > 0);
        for priority in queue_priorities {
            assert!(*priority >= 0. && *priority <= 1.);
        }

        VkDeviceQueueCreateInfo {
            s_type: VkStructureType::DeviceQueueCreateInfo,
            p_next: None,
            flags,
            queue_family_index,
            queue_count: queue_priorities.len() as u32,
            p_queue_priorities: unsafe { NonNull::new_unchecked(queue_priorities.as_ptr() as _) },
            phantom: PhantomData,
        }
    }

    pub fn flags(&self) -> VkDeviceQueueCreateFlags {
        self.flags
    }

    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }

    pub fn queue_priorities(&self) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(self.p_queue_priorities.as_ptr(), self.queue_count as usize)
        }
    }
}
