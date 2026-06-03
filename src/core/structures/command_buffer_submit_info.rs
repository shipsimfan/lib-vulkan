use crate::{VkCommandBuffer, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_3, VkCommandBufferLevel, VkRenderingFlag};

/// Structure specifying a command buffer submission
///
/// Provided by [`VK_VERSION_1_3`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkCommandBufferSubmitInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::CommandBufferSubmitInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If a [`VkRenderPassStripeSubmitInfoArm`] is included in the `next` chain, the value of
    ///    [`VkRenderPassStripeSubmitInfoArm::stripe_semaphore_info_count`] must be equal to the
    ///    sum of the [`VkRenderPassStripeBeginInfoArm::stripe_info_count`] parameters provided to
    ///    render pass instances recorded in `command_buffer` that did not specify the
    ///    [`VkRenderingFlag::Resuming`] flag
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`] or a pointer to a valid instance of
    ///    [`VkRenderPassStripeSubmitInfoArm`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `command_buffer` is a [`VkCommandBuffer`] to be submitted for execution.
    ///
    /// # Valid Usage
    ///  - `command_buffer` must not have been allocated with [`VkCommandBufferLevel::Secondary`]
    ///  - If any render pass instance in `command_buffer` was recorded with a
    ///    [`VkRenderPassStripeBeginInfoArm`] structure in its `next` chain and did not specify the
    ///    [`VkRenderingFlag::Resuming`] flag, a [`VkRenderPassStripeSubmitInfoArm`] must be
    ///    included in the `next` chain
    ///
    /// # Valid Usage (Implicit)
    ///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
    pub command_buffer: VkCommandBuffer,

    /// `device_mask` is a bitmask indicating which devices in a device group execute the command
    /// buffer. A `device_mask` of 0 is equivalent to setting all bits corresponding to valid
    /// devices in the group to 1.
    ///
    /// # Valid Usage
    ///  - If `device_mask` is not 0, it must be a valid device mask
    pub device_mask: u32,
}

impl const Default for VkCommandBufferSubmitInfo {
    fn default() -> Self {
        VkCommandBufferSubmitInfo {
            r#type: VkStructureType::CommandBufferSubmitInfo,
            next: null(),
            command_buffer: VkCommandBuffer::null(),
            device_mask: 0,
        }
    }
}

impl NextChain for VkCommandBufferSubmitInfo {
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
