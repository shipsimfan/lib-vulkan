use crate::{VkCommandBufferLevel, VkCommandPool, VkStructureType, util::NextChain};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;
#[allow(unused_imports)]
use std::ptr::null;

/// Structure specifying the allocation parameters for command buffer object
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkCommandBufferAllocateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::CommandBufferAllocateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`]
    pub next: *const c_void,

    /// `command_pool` is the command pool from which the command buffers are allocated.
    ///
    /// # Valid Usage (Implicit)
    ///  - `command_pool` must be a valid [`VkCommandPool`] handle
    ///
    /// # Host Synchronization
    ///  - Host access to `command_pool` must be externally synchronized
    pub command_pool: VkCommandPool,

    /// `level` is a [`VkCommandBufferLevel`] value specifying the command buffer level.
    ///
    /// # Valid Usage (Implicit)
    ///  - `level` must be a valid [`VkCommandBufferLevel`] value
    pub level: VkCommandBufferLevel,

    /// `command_buffer_count` is the number of command buffers to allocate from the pool.
    pub command_buffer_count: u32,
}

const impl Default for VkCommandBufferAllocateInfo {
    fn default() -> Self {
        VkCommandBufferAllocateInfo {
            r#type: VkStructureType::CommandBufferAllocateInfo,
            next: null(),
            command_pool: VkCommandPool::null(),
            level: VkCommandBufferLevel::Primary,
            command_buffer_count: 0,
        }
    }
}

impl NextChain for VkCommandBufferAllocateInfo {
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
