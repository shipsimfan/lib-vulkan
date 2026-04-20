use crate::{VkCommandBuffer, VkCommandBufferBeginInfo, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_FALSE, VK_VERSION_1_0, VkCommandBufferInheritanceInfo, VkCommandBufferUsageFlag,
    VkCommandPool, VkCommandPoolCreateFlag, VkQueryControlFlag,
};

/// Start recording a command buffer
///
/// # Parameters
///  - `command_buffer` is the handle of the command buffer which is to be put in the recording
///    state.
///  - `begin_info` is a pointer to a [`VkCommandBufferBeginInfo`] structure defining additional
///    information about how the command buffer begins recording.
///
/// # Valid Usage
///  - `command_buffer` must not be in the recording or pending state
///  - If `command_buffer` was allocated from a [`VkCommandPool`] which did not have the
///    [`VkCommandPoolCreateFlag::ResetCommandBufferBit`] flag set, `command_buffer` must be in the
///    initial state
///  - If `command_buffer` is a secondary command buffer, the `inheritance_info` member of
///    `begin_info` must be a valid [`VkCommandBufferInheritanceInfo`] structure
///  - If `command_buffer` is a secondary command buffer and either the `occlusion_query_enable`
///    member of the inheritance_info member of `begin_info` is [`VK_FALSE`], or the
///    `occlusion_query_precise` feature is not enabled, then
///    `begin_info.inheritance_info.query_flags` must not contain
///    [`VkQueryControlFlag::PreciseBit`]
///  - If `command_buffer` is a primary command buffer, then `begin_info.flags` must not set both
///    the [`VkCommandBufferUsageFlag::OneTimeSubmitBit`] and the
///    [`VkCommandBufferUsageFlag::SimultaneousUseBit`] flags
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `begin_info` must be a valid pointer to a valid [`VkCommandBufferBeginInfo`] structure
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkBeginCommandBuffer = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    begin_info: *const VkCommandBufferBeginInfo,
) -> VkResult;

/// The name of [`VkBeginCommandBuffer`]
pub const VK_BEGIN_COMMAND_BUFFER: &CStr = c"vkBeginCommandBuffer";
