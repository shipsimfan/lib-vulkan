use crate::{VkBuffer, VkBufferCopy, VkCommandBuffer};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VkBufferUsageFlag, VkCommandPool, VkDevice, VkDeviceMemory, VkQueueFlag,
};

/// Copy data between buffer regions
///
/// # Parameters
///  - `command_buffer` is the command buffer into which the command will be recorded.
///  - `src_buffer` is the source buffer.
///  - `dst_buffer` is the destination buffer.
///  - `region_count` is the number of regions to copy.
///  - `regions` is a pointer to an array of [`VkBufferCopy`] structures specifying the regions to
///    copy.
///
/// # Description
/// Each source region specified by `regions` is copied from the source buffer to the destination
/// region of the destination buffer. If any of the specified regions in `src_buffer` overlaps in
/// memory with any of the specified regions in `dst_buffer`, values read from those overlapping
/// regions are undefined.
///
/// # Valid Usage
///  - If `command_buffer` is an unprotected command buffer and `protected_no_fault` is not
///    supported, `src_buffer` must not be a protected buffer
///  - If `command_buffer` is an unprotected command buffer and `protected_no_fault` is not
///    supported, `dst_buffer` must not be a protected buffer
///  - If `command_buffer` is a protected command buffer and `protected_no_fault` is not supported,
///    `dst_buffer` must not be an unprotected buffer
///  - The `src_offset` member of each element of `regions` must be less than the size of
///    `src_buffer`
///  - The `dst_offset` member of each element of `regions` must be less than the size of
///    `dst_buffer`
///  - The `size` member of each element of `regions` must be less than or equal to the size of
///    `src_buffer` minus `src_offset`
///  - The `size` member of each element of `regions` must be less than or equal to the size of
///    `dst_buffer` minus `dst_offset`
///  - The union of the source regions, and the union of the destination regions, specified by the
///    elements of `regions`, must not overlap in memory
///  - `src_buffer` must have been created with the [`VkBufferUsageFlag::TransferSrc`] usage flag
///    set
///  - If `src_buffer` is non-sparse then it must be bound completely and contiguously to a single
///    [`VkDeviceMemory`] object
///  - `dst_buffer` must have been created with the [`VkBufferUsageFlag::TransferDst`] usage flag
///    set
///  - If `dst_buffer` is non-sparse then it must be bound completely and contiguously to a single
///    [`VkDeviceMemory`] object
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `src_buffer` must be a valid [`VkBuffer`] handle
///  - `dst_buffer` must be a valid [`VkBuffer`] handle
///  - `regions` must be a valid pointer to an array of `region_count` valid [`VkBufferCopy`]
///    structures
///  - `command_buffer` must be in the recording state
///  - The [`VkCommandPool`] that `command_buffer` was allocated from must support
///    [`VkQueueFlag::Compute`], [`VkQueueFlag::Graphics`], or [`VkQueueFlag::Transfer`] operations
///  - This command must only be called outside of a render pass instance
///  - This command must not be called between suspended render pass instances
///  - This command must only be called outside of a video coding scope
///  - `region_count` must be greater than 0
///  - Each of `command_buffer`, `dst_buffer`, and `src_buffer` must have been created, allocated,
///    or retrieved from the same [`VkDevice`]
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCmdCopyBuffer = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    src_buffer: VkBuffer,
    dst_buffer: VkBuffer,
    region_count: u32,
    regions: *const VkBufferCopy,
);

/// The name of [`VkCmdCopyBuffer`]
pub const VK_CMD_COPY_BUFFER: &CStr = c"vkCmdCopyBuffer";
