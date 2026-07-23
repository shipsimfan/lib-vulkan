use crate::{VkCommandBuffer, VkPipelineLayout, VkShaderStageFlags};
use std::ffi::{CStr, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VkCommandPool, VkDevice, VkPhysicalDeviceLimits, VkPushConstantRange,
    VkQueueFlag, VkShaderStageFlag,
};
#[allow(unused_imports)]
use std::ptr::null;

/// Update the values of push constants
///
/// # Parameters
///  - `command_buffer` is the command buffer in which the push constant update will be recorded.
///  - `layout` is the pipeline layout used to program the push constant updates.
///  - `stage_flags` is a bitmask of [`VkShaderStageFlag`]s specifying the shader stages that will
///    use the push constants in the updated range.
///  - `offset` is the start offset of the push constant range to update, in units of bytes.
///  - `size` is the size of the push constant range to update, in units of bytes.
///  - `values` is a pointer to an array of size bytes containing the new push constant values.
///
/// # Description
/// When a command buffer begins recording, all push constant values are undefined. Reads of
/// undefined push constant values by the executing shader return undefined values.
///
/// Push constant values can be updated incrementally, causing shader stages in `stage_flags` to
/// read the new data from `values` for push constants modified by this command, while still
/// reading the previous data for push constants not modified by this command. When a bound
/// pipeline command is issued, the bound pipeline’s layout must be compatible with the layouts
/// used to set the values of all push constants in the pipeline layout’s push constant ranges, as
/// described in Pipeline Layout Compatibility. Binding a pipeline with a layout that is not
/// compatible with the push constant layout does not disturb the push constant values.
///
/// # Valid Usage
///  - If `command_buffer` is a secondary command buffer, it must have begun with
///    [`VkCommandBufferInheritanceDescriptorHeapInfoExt::sampler_heap_bind_info`] equal to
///    [`null`]
///  - If `command_buffer` is a secondary command buffer, it must have begun with
///    [`VkCommandBufferInheritanceDescriptorHeapInfoExt::resource_heap_bind_info`] equal to
///    [`null`]
///  - For each byte in the range specified by `offset` and `size` and for each shader stage in
///    `stage_flags`, there must be a push constant range in `layout` that includes that byte and
///    that stage
///  - For each byte in the range specified by `offset` and `size` and for each push constant range
///    that overlaps that byte, `stage_flags` must include all stages in that push constant range’s
///    [`VkPushConstantRange::stage_flags`]
///  - `offset` must be a multiple of 4
///  - `size` must be a multiple of 4
///  - `offset` must be less than [`VkPhysicalDeviceLimits::max_push_constants_size`]
///  - `size` must be less than or equal to [`VkPhysicalDeviceLimits::max_push_constants_size`]
///    minus `offset`
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `layout` must be a valid [`VkPipelineLayout`] handle
///  - `stage_flags` must be a valid combination of [`VkShaderStageFlag`]s values
///  - `stage_flags` must not be 0
///  - `values` must be a valid pointer to an array of `size` bytes
///  - `command_buffer` must be in the recording state
///  - The [`VkCommandPool`] that `command_buffer` was allocated from must support
///    [`VkQueueFlag::Compute`], or [`VkQueueFlag::Graphics`] operations
///  - This command must only be called outside of a video coding scope
///  - `size` must be greater than 0
///  - Both of `command_buffer`, and `layout` must have been created, allocated, or retrieved from
///    the same [`VkDevice`]
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCmdPushConstants = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    layout: VkPipelineLayout,
    stage_flags: VkShaderStageFlags,
    offset: u32,
    size: u32,
    values: *const c_void,
);

/// The name of [`VkCmdPushConstants`]
pub const VK_CMD_PUSH_CONSTANTS: &CStr = c"vkCmdPushConstants";
