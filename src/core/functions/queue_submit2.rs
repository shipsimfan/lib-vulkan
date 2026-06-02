use crate::{VkFence, VkQueue, VkResult, VkSubmitInfo2};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VK_VERSION_1_3, VkCommandBufferUsageFlag, VkCommandPool, VkDevice,
    VkDeviceQueueCreateFlag, VkEvent, VkSubmitFlag,
};

/// Submits command buffers to a queue
///
/// # Parameters
///  - `queue` is the queue that the command buffers will be submitted to.
///  - `submit_count` is the number of elements in the `submits` array.
///  - `submits` is a pointer to an array of [`VkSubmitInfo2`] structures, each specifying a
///    command buffer submission batch. Command buffers and semaphores specified in this array may
///    be accessed at any point until the queue operations they define complete execution on the
///    device.
///  - `fence` is an optional handle to a fence to be signaled once all submitted command buffers
///    have completed execution. If `fence` is not [`VK_NULL_HANDLE`], it defines a fence signal
///    operation. If it is not [`VK_NULL_HANDLE`], fence may be accessed at any point until this
///    command completes on the device.
///
/// # Description
/// [`VkQueueSubmit2`] is a queue submission command, with each batch defined by an element of
/// `submits`.
///
/// The first synchronization scope of each semaphore signal operation defined by this command
/// includes every command in the same batch that the signal operation is defined in, and all
/// commands that occur earlier in submission order. The scope is limited by the `stage_mask`
/// member of the [`VkSemaphoreSubmitInfo`] used to define each such operation.
///
/// The second synchronization scope of each semaphore wait operation defined by this command
/// includes every command in the same batch that the wait operation is defined in, and all
/// commands that occur later in submission order. The scope is limited by the `stage_mask` member
/// of the [`VkSemaphoreSubmitInfo`] used to define each such operation.
///
/// If any command buffer submitted to this queue is in the executable state, it is moved to the
/// pending state. Once execution of all submissions of a command buffer complete, it moves from
/// the pending state, back to the executable state. If a command buffer was recorded with the
/// [`VkCommandBufferUsageFlag::OneTimeSubmit`] flag, it instead moves back to the invalid state.
///
/// If [`VkQueueSubmit2`] fails, it may return [`VkResult::VkErrorOutOfHostMemory`] or
/// [`VkResult::VkErrorOutOfDeviceMemory`]. If it does, the implementation must ensure that the
/// state and contents of any resources or synchronization primitives referenced by the submitted
/// command buffers and any semaphores referenced by `submits` is unaffected by the call or its
/// failure. If [`VkQueueSubmit2`] fails in such a way that the implementation is unable to make
/// that guarantee, the implementation must return [`VkResult::VkErrorDeviceLost`].
///
/// # Valid Usage
///  - If `fence` is not [`VK_NULL_HANDLE`], `fence` must be unsignaled
///  - If `fence` is not [`VK_NULL_HANDLE`], `fence` must not be associated with any other queue
///    command that has not yet completed execution on that queue
///  - The `synchronization2` feature must be enabled
///  - If a command recorded into the `command_buffer` member of any element of the
///    `command_buffer_infos` member of any element of `submits` referenced a [`VkEvent`], that
///    event must not be referenced by a command that has been submitted to another queue and is
///    still in the pending state
///  - The `semaphore` member of any binary semaphore element of the `signal_semaphore_infos`
///    member of any element of `submits` must be unsignaled when the semaphore signal operation it
///    defines is executed on the device
///  - The `stage_mask` member of any element of the `signal_semaphore_infos` member of any element
///    of `submits` must only include pipeline stages that are supported by the queue family which
///    queue belongs to
///  - The `stage_mask` member of any element of the `wait_semaphore_infos` member of any element
///    of `submits` must only include pipeline stages that are supported by the queue family which
///    queue belongs to
///  - When a semaphore wait operation for a binary semaphore is executed, as defined by the
///    semaphore member of any element of the `wait_semaphore_infos` member of any element of
///    `submits`, there must be no other queues waiting on the same semaphore
///  - The `semaphore` member of any element of the `wait_semaphore_infos` member of any element of
///    `submits` that was created with a [`VkSemaphoreType`] of [`VkSemaphoreType::Binary`] must
///    reference a semaphore signal operation that has been submitted for execution and any
///    semaphore signal operations on which it depends must have also been submitted for execution
///  - The `command_buffer` member of any element of the `command_buffer_infos` member of any
///    element of `submits` must be in the pending or executable state
///  - If a command recorded into the `command_buffer` member of any element of the
///    `command_buffer_infos` member of any element of `submits` was not recorded with the
///    [`VkCommandBufferUsageFlag::SimultaneousUse`], it must not be in the pending state
///  - Any secondary command buffers recorded into the `command_buffer` member of any element of
///    the `command_buffer_infos` member of any element of `submits` must be in the pending or
///    executable state
///  - If any secondary command buffers recorded into the `command_buffer` member of any element of
///    the `command_buffer_infos` member of any element of `submits` was not recorded with the
///    [`VkCommandBufferUsageFlag::SimultaneousUse`], it must not be in the pending state
///  - The `command_buffer` member of any element of the `command_buffer_infos` member of any
///    element of `submits` must have been allocated from a [`VkCommandPool`] that was created for
///    the same queue family queue belongs to
///  - If a command recorded into the `command_buffer` member of any element of the
///    `command_buffer_infos` member of any element of `submits` includes a Queue Family Ownership
///    Transfer Acquire Operation, there must exist a previously submitted Queue Family Ownership
///    Transfer Release Operation on a queue in the queue family identified by the acquire
///    operation, with parameters matching the acquire operation as defined in the definition of
///    such acquire operations, and which happens before the acquire operation
///  - If a command recorded into the `command_buffer` member of any element of the
///    `command_buffer_infos` member of any element of `submits` includes a Queue Family Ownership
///    Transfer Acquire Operation, the affected resource must not be modified in any way between
///    the last matching release operation and the acquire operation
///  - If a command recorded into the `command_buffer` member of any element of the
///    `command_buffer_infos` member of any element of `submits` was a [`VkCmdBeginQuery`] whose
///    `query_pool` was created with a `query_type` of [`VkQueryType::PerformanceQueryKhr`], the
///    profiling lock must have been held continuously on the [`VkDevice`] that queue was retrieved
///    from, throughout recording of those command buffers
///  - If `queue` was not created with [`VkDeviceQueueCreateFlag::Protected`], the flags member of
///    any element of `submits` must not include [`VkSubmitFlag::ProtectedKhr`]
///
/// # Valid Usage (Implicit)
///  - `queue` must be a valid [`VkQueue`] handle
///  - If `submit_count` is not 0, `submits` must be a valid pointer to an array of `submit_count`
///    valid [`VkSubmitInfo2`] structures
///  - If `fence` is not [`VK_NULL_HANDLE`], `fence` must be a valid [`VkFence`] handle
///  - Both of `fence`, and `queue` that are valid handles of non-ignored parameters must have been
///    created, allocated, or retrieved from the same [`VkDevice`]
///
/// # Host Synchronization
///  - Host access to `queue` must be externally synchronized if it was not created with
///    [`VkDeviceQueueCreateFlag::InternallySynchronizedKhr`]
///  - Host access to `fence` must be externally synchronized
///
/// Provided by [`VK_VERSION_1_3`]
pub type VkQueueSubmit2 = unsafe extern "system" fn(
    queue: VkQueue,
    submit_count: u32,
    submits: *const VkSubmitInfo2,
    fence: VkFence,
) -> VkResult;

/// The name of [`VkQueueSubmit2`]
pub const VK_QUEUE_SUBMIT2: &CStr = c"vkQueueSubmit2";
