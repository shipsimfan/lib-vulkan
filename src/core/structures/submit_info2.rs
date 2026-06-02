use crate::{
    VkCommandBufferSubmitInfo, VkSemaphoreSubmitInfo, VkStructureType, VkSubmitFlags,
    util::NextChain,
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_3, VkSubmitFlag};

/// Structure specifying a queue submit operation
///
/// # Valid Usage
///  - If the same semaphore is used as the `semaphore` member of both an element of
///    `signal_semaphore_infos` and `wait_semaphore_infos`, and that semaphore is a timeline
///    semaphore, the `value` member of the `signal_semaphore_infos` element must be greater than
///    the `value` member of the `wait_semaphore_infos` element
///
/// Provided by [`VK_VERSION_1_3`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSubmitInfo2 {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::SubmitInfo2`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If the `next` chain of this structure includes a [`VkFrameBoundaryTensorsArm`] structure
    ///    then it must also include a [`VkFrameBoundaryExt`] structure
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of [`VkFrameBoundaryExt`],
    ///    [`VkFrameBoundaryTensorsArm`], [`VkLatencySubmissionPresentIdNv`],
    ///    [`VkPerformanceQuerySubmitInfoKhr`], [`VkWin32KeyedMutexAcquireReleaseInfoKhr`], or
    ///    [`VkWin32KeyedMutexAcquireReleaseInfoNv`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkSubmitFlags`].
    ///
    /// # Valid Usage
    ///  - If `flags` includes [`VkSubmitFlag::Protected`], all elements of `command_buffers` must
    ///    be protected command buffers
    ///  - If `flags` does not include [`VkSubmitFlag::Protected`], each element of
    ///    `command_buffers` must not be a protected command buffer
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of [`VkSubmitFlags`] values
    pub flags: VkSubmitFlags,

    /// `wait_semaphore_info_count` is the number of elements in `wait_semaphore_infos`.
    pub wait_semaphore_info_count: u32,

    /// `wait_semaphore_infos` is a pointer to an array of [`VkSemaphoreSubmitInfo`] structures
    /// defining semaphore wait operations.
    ///
    /// # Valid Usage (Implicit)
    ///  - If `wait_semaphore_info_count` is not 0, `wait_semaphore_infos` must be a valid pointer
    ///    to an array of `wait_semaphore_info_count` valid [`VkSemaphoreSubmitInfo`] structures
    pub wait_semaphore_infos: *const VkSemaphoreSubmitInfo,

    /// `command_buffer_info_count` is the number of elements in `command_buffer_infos` and the
    /// number of command buffers to execute in the batch.
    pub command_buffer_info_count: u32,

    /// `command_buffer_infos` is a pointer to an array of VkCommandBufferSubmitInfo structures
    /// describing command buffers to execute in the batch.
    ///
    /// # Valid Usage
    ///  - If any `command_buffer` member of an element of `command_buffer_infos` contains any
    ///    resumed render pass instances, they must be suspended by a render pass instance earlier
    ///    in submission order within `command_buffer_infos`
    ///  - If any `command_buffer` member of an element of `command_buffer_infos` contains any
    ///    suspended render pass instances, they must be resumed by a render pass instance later in
    ///    submission order within `command_buffer_infos`
    ///  - If any `command_buffer` member of an element of `command_buffer_infos` contains any
    ///    suspended render pass instances, there must be no action or synchronization commands
    ///    between that render pass instance and the render pass instance that resumes it
    ///  - If any `command_buffer` member of an element of `command_buffer_infos` contains any
    ///    suspended render pass instances, there must be no render pass instances between that
    ///    render pass instance and the render pass instance that resumes it
    ///  - If the `variable_sample_locations` limit is not supported, and any `command_buffer`
    ///    member of an element of `command_buffer_infos` contains any suspended render pass
    ///    instances, where a graphics pipeline has been bound, any pipelines bound in the render
    ///    pass instance that resumes it, or any subsequent render pass instances that resume from
    ///    that one and so on, must use the same sample locations
    ///  - If at least one [`VkCommandBufferSubmitInfo`] structure in `command_buffer_infos`
    ///    references a `command_buffer` allocated from a pool that was created with a
    ///    [`VkDataGraphProcessingEngineCreateInfoArm`] structure in the `next` chain of
    ///    [`VkCommandPoolCreateInfo`] that included a foreign data graph processing engine in its
    ///    `processing_engines` member, then `wait_semaphore_infos` and `signal_semaphore_infos`
    ///    must only reference semaphore objects that were created from external handle types
    ///    reported as supported in a
    ///    [`VkQueueFamilyDataGraphProcessingEnginePropertiesArm::foreign_semaphore_handle_types`]
    ///    structure via [`VkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesArm`]
    ///    with a `queue_family_index` matching the one the command pool was created for, for all
    ///    the foreign data graph processing engines that were part of the
    ///    [`VkDataGraphProcessingEngineCreateInfoArm`] used to create the command pool
    ///
    /// # Valid Usage (Implicit)
    ///  - If `command_buffer_info_count` is not 0, `command_buffer_infos` must be a valid pointer
    ///    to an array of `command_buffer_info_count` valid [`VkCommandBufferSubmitInfo`]
    ///    structures
    pub command_buffer_infos: *const VkCommandBufferSubmitInfo,

    /// `signal_semaphore_info_count` is the number of elements in `signal_semaphore_infos`.
    pub signal_semaphore_info_count: u32,

    /// `signal_semaphore_infos` is a pointer to an array of [`VkSemaphoreSubmitInfo`] describing
    /// semaphore signal operations.
    ///
    /// # Valid Usage
    ///  - If the `semaphore` member of any element of `signal_semaphore_infos` is a timeline
    ///    semaphore, the `value` member of that element must have a value greater than the current
    ///    value of the semaphore when the semaphore signal operation is executed
    ///  - If the `semaphore` member of any element of `signal_semaphore_infos` is a timeline
    ///    semaphore, the `value` member of that element must have a value which does not differ
    ///    from the current value of the semaphore or the value of any outstanding semaphore wait
    ///    or signal operation on that semaphore by more than
    ///    `max_timeline_semaphore_value_difference`
    ///  - If the `semaphore` member of any element of `signal_semaphore_infos` is a timeline
    ///    semaphore, the `value` member of that element must have a value which does not differ
    ///    from the current value of the semaphore or the value of any outstanding semaphore wait
    ///    or signal operation on that semaphore by more than
    ///    `max_timeline_semaphore_value_difference`
    ///  - If the `semaphore` member of any element of `wait_semaphore_infos` is a timeline
    ///    semaphore, the `value` member of that element must have a value which does not differ
    ///    from the current value of the semaphore or the value of any outstanding semaphore wait
    ///    or signal operation on that semaphore by more than
    ///    `max_timeline_semaphore_value_difference`
    ///
    /// # Valid Usage (Implicit)
    ///  - If `signal_semaphore_info_count` is not 0, `signal_semaphore_infos` must be a valid
    ///    pointer to an array of `signal_semaphore_info_count` valid [`VkSemaphoreSubmitInfo`]
    ///    structures
    pub signal_semaphore_infos: *const VkSemaphoreSubmitInfo,
}

impl const Default for VkSubmitInfo2 {
    fn default() -> Self {
        VkSubmitInfo2 {
            r#type: VkStructureType::SubmitInfo2,
            next: null(),
            flags: VkSubmitFlags::empty(),
            wait_semaphore_info_count: 0,
            wait_semaphore_infos: null(),
            command_buffer_info_count: 0,
            command_buffer_infos: null(),
            signal_semaphore_info_count: 0,
            signal_semaphore_infos: null(),
        }
    }
}

impl NextChain for VkSubmitInfo2 {
    fn next(&self) -> *const c_void {
        self.next
    }

    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: *const c_void) {
        self.next = next;
    }
}
