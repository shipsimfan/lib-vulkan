use crate::{VkFence, VkQueue, VkResult, VkSubmitInfo};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VK_TRUE, VK_VERSION_1_0, VkCommandBufferUsageFlag, VkCommandPool, VkDevice,
    VkDeviceQueueCreateFlag, VkEvent, VkSharingMode,
};

/// Submits a sequence of semaphores or command buffers to a queue
///
/// # Parameters
///  - `queue` is the queue that the command buffers will be submitted to.
///  - `submit_count` is the number of elements in the `submits` array.
///  - `submits` is a pointer to an array of [`VkSubmitInfo`] structures, each specifying a command
///    buffer submission batch. Command buffers and semaphores specified in this array may be
///    accessed at any point until the queue operations they define complete execution on the
///    device.
///  - `fence` is an optional handle to a fence to be signaled once all submitted command buffers
///    have completed execution. If `fence` is not [`VK_NULL_HANDLE`], it defines a fence signal
///    operation. If it is not [`VK_NULL_HANDLE`], `fence` may be accessed at any point until this
///    command completes on the device.
///
/// # Description
/// [`VkQueueSubmit`] is a queue submission command, with each batch defined by an element of
/// `submits`. Batches begin execution in the order they appear in `submits`, but may complete out
/// of order.
///
/// The order that batches appear in `submits` is used to determine submission order, and thus all
/// the implicit ordering guarantees that respect it. Other than these implicit ordering guarantees
/// and any explicit synchronization primitives, these batches may overlap or otherwise execute out
/// of order.
///
/// Fence operations submitted with [`VkQueueSubmit`] have additional ordering constraints compared
/// to other submission commands, with dependencies involving previous and subsequent queue
/// operations. Information about these additional constraints can be found in the fence sections
/// of the synchronization chapter.
///
/// The first synchronization scope of each semaphore signal operation defined by this command
/// includes every command in the same batch that the signal operation is defined in, and all
/// commands that occur earlier in submission order.
///
/// The second synchronization scope of each semaphore wait operation defined by this command
/// includes every command in the same batch that the wait operation is defined in, and all
/// commands that occur later in submission order. The scope is limited by the
/// `wait_dst_stage_mask` for each batch, as described in [`VkSubmitInfo`].
///
/// If any command buffer submitted to this queue is in the executable state, it is moved to the
/// pending state. Once execution of all submissions of a command buffer complete, it moves from
/// the pending state, back to the executable state. If a command buffer was recorded with the
/// [`VkCommandBufferUsageFlag::OneTimeSubmit`] flag, it instead moves to the invalid state.
///
/// If [`VkQueueSubmit`] fails, it may return [`VkResult::VkErrorOutOfHostMemory`] or
/// [`VkResult::VkErrorOutOfDeviceMemory`]. If it does, the implementation must ensure that the
/// state and contents of any resources or synchronization primitives referenced by the submitted
/// command buffers and any semaphores referenced by `submits` is unaffected by the call or its
/// failure. If [`VkQueueSubmit`] fails in such a way that the implementation is unable to make
/// that guarantee, the implementation must return [`VkResult::VkErrorDeviceLost`].
///
/// # Valid Usage
///  - If `fence` is not [`VK_NULL_HANDLE`], `fence` must be unsignaled
///  - If `fence` is not [`VK_NULL_HANDLE`], `fence` must not be associated with any other queue
///    command that has not yet completed execution on that queue
///  - Any calls to [`VkCmdSetEvent`], [`VkCmdResetEvent`] or [`VkCmdWaitEvents`] that have been
///    recorded into any of the command buffer elements of the `command_buffers` member of any
///    element of `submits`, must not reference any [`VkEvent`] that is referenced by any of those
///    commands in a command buffer that has been submitted to another queue and is still in the
///    pending state
///  - Any stage flag included in any element of the `wait_dst_stage_mask` member of any element of
///    `submits` must be a pipeline stage supported by one of the capabilities of queue, as
///    specified in the table of supported pipeline stages
///  - Each binary semaphore element of the `signal_semaphores` member of any element of `submits`
///    must be unsignaled when the semaphore signal operation it defines is executed on the device
///  - When a semaphore wait operation referring to a binary semaphore defined by any element of
///    the `wait_semaphores` member of any element of `submits` executes on queue, there must be no
///    other queues waiting on the same semaphore
///  - All elements of the `wait_semaphores` member of all elements of `submits` created with a
///    [`VkSemaphoreType`] of [`VkSemaphoreType::Binary`] must reference a semaphore signal
///    operation that has been submitted for execution and any semaphore signal operations on which
///    it depends must have also been submitted for execution
///  - Each element of the `command_buffers` member of each element of `submits` must be in the
///    pending or executable state
///  - If any element of the `command_buffers` member of any element of `submits` was not recorded
///    with the [`VkCommandBufferUsageFlag::SimultaneousUse`], it must not be in the pending state
///  - Any secondary command buffers recorded into any element of the `command_buffers` member of
///    any element of `submits` must be in the pending or executable state
///  - If any secondary command buffers recorded into any element of the `command_buffers` member
///    of any element of `submits` was not recorded with the
///    [`VkCommandBufferUsageFlag::SimultaneousUse`], it must not be in the pending state
///  - Each element of the `command_buffers` member of each element of `submits` must have been
///    allocated from a [`VkCommandPool`] that was created for the same queue family queue belongs
///    to
///  - If any element of `submits.command_buffers` includes a Queue Family Ownership Transfer
///    Acquire Operation, there must exist a previously submitted Queue Family Ownership Transfer
///    Release Operation on a queue in the queue family identified by the acquire operation, with
///    parameters matching the acquire operation as defined in the definition of such acquire
///    operations, and which happens-before the acquire operation
///  - If any element of `submits.command_buffers` includes a Queue Family Ownership Transfer
///    Acquire Operation, the affected resource must not be modified in any way between the last
///    matching release operation and the acquire operation
///  - If a command recorded into any element of `command_buffers` was a [`VkCmdBeginQuery`] whose
///    `query_pool` was created with a `query_type` of [`VkQueryType::PerformanceQueryKhr`], the
///    profiling lock must have been held continuously on the [`VkDevice`] that queue was retrieved
///    from, throughout recording of those command buffers
///  - Any resource created with [`VkSharingMode::Exclusive`] that is read by an operation
///    specified by `submits` must not be owned by any queue family other than the one which queue
///    belongs to, at the time it is executed
///  - Any resource created with [`VkSharingMode::Concurrent`] that is accessed by an operation
///    specified by `submits` must have included the queue family of queue at resource creation
///    time
///  - If queue was not created with [`VkDeviceQueueCreateFlag::Protected`], there must be no
///    element of `submits` that includes a [`VkProtectedSubmitInfo`] structure in its `next` chain
///    with protectedSubmit equal to [`VK_TRUE`]
///
/// # Valid Usage (Implicit)
///  - `queue` must be a valid [`VkQueue`] handle
///  - If `submit_count` is not 0, `submits` must be a valid pointer to an array of `submit_count`
///    valid [`VkSubmitInfo`] structures
///  - If `fence` is not [`VK_NULL_HANDLE`], `fence` must be a valid [`VkFence`] handle
///  - Both of `fence`, and `queue` that are valid handles of non-ignored parameters must have been
///    created, allocated, or retrieved from the same [`VkDevice`]
///
/// # Host Synchronization
///  - Host access to `queue` must be externally synchronized if it was not created with
///    [`VkDeviceQueueCreateFlag::InternallySynchronizedKhr`]
///  - Host access to `fence` must be externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkQueueSubmit = unsafe extern "system" fn(
    queue: VkQueue,
    submit_count: u32,
    submits: *const VkSubmitInfo,
    fence: VkFence,
) -> VkResult;

/// The name of [`VkQueueSubmit`]
pub const VK_QUEUE_SUBMIT: &CStr = c"vkQueueSubmit";
