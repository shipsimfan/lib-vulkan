use crate::{VkCommandBuffer, VkCopyBufferInfo2};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_3, VkCommandPool, VkQueueFlag};

/// Copy data between buffer regions
///
/// # Parameters
///  - `command_buffer` is the command buffer into which the command will be recorded.
///  - `copy_buffer_info` is a pointer to a [`VkCopyBufferInfo2`] structure describing the copy
///    parameters.
///
/// # Description
/// Each source region specified by `copy_buffer_info.regions` is copied from the source buffer to
/// the destination region of the destination buffer. If any of the specified regions in
/// `copy_buffer_info.src_buffer` overlaps in memory with any of the specified regions in
/// `copy_buffer_info.dst_buffer`, values read from those overlapping regions are undefined.
///
/// # Valid Usage
///  - If `command_buffer` is an unprotected command buffer and `protected_no_fault` is not
///    supported, `src_buffer` must not be a protected buffer
///  - If `command_buffer` is an unprotected command buffer and `protected_no_fault` is not
///    supported, `dst_buffer` must not be a protected buffer
///  - If `command_buffer` is a protected command buffer and `protected_no_fault` is not supported,
///    `dst_buffer` must not be an unprotected buffer
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `copy_buffer_info` must be a valid pointer to a valid [`VkCopyBufferInfo2`] structure
///  - `command_buffer` must be in the recording state
///  - The [`VkCommandPool`] that `command_buffer` was allocated from must support
///    [`VkQueueFlag::Compute`], [`VkQueueFlag::Graphics`], or [`VkQueueFlag::Transfer`] operations
///  - This command must only be called outside of a render pass instance
///  - This command must not be called between suspended render pass instances
///  - This command must only be called outside of a video coding scope
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// Provided by [`VK_VERSION_1_3`]
pub type VkCmdCopyBuffer2 = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    copy_buffer_info: *const VkCopyBufferInfo2,
);

/// The name of [`VkCmdCopyBuffer2`]
pub const VK_CMD_COPY_BUFFER2: &CStr = c"vkCmdCopyBuffer2";
