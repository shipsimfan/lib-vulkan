use crate::{VkCommandBuffer, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkCommandPool};

/// Finish recording a command buffer
///
/// # Parameters
///  - `command_buffer` is the command buffer to complete recording.
///
/// # Description
/// The command buffer must have been in the recording state, and, if successful, is moved to the
/// executable state.
///
/// If there was an error during recording, the application will be notified by an unsuccessful
/// return code returned by [`VkEndCommandBuffer`], and the command buffer will be moved to the
/// invalid state.
///
/// In case the application recorded one or more video encode operations into the command buffer,
/// implementations may return the [`VkResult::VkErrorInvalidVideoStdParametersKhr`] error if any
/// of the specified Video Std parameters do not adhere to the syntactic or semantic requirements
/// of the used video compression standard, or if values derived from parameters according to the
/// rules defined by the used video compression standard do not adhere to the capabilities of the
/// video compression standard or the implementation.
///
/// # Valid Usage
///  - `command_buffer` must be in the recording state
///  - If `command_buffer` is a primary command buffer, there must not be an active render pass
///    instance
///  - All queries made active during the recording of `command_buffer` must have been made
///    inactive
///  - Conditional rendering must not be active
///  - There must be no video session object bound
///  - If `command_buffer` is a secondary command buffer, there must not be an outstanding
///    [`VkCmdBeginDebugUtilsLabelExt`] command recorded to `command_buffer` that has not
///    previously been ended by a call to [`VkCmdEndDebugUtilsLabelExt`]
///  - If `command_buffer` is a secondary command buffer, there must not be an outstanding
///    [`VkCmdDebugMarkerBeginExt`] command recorded to `command_buffer` that has not previously
///    been ended by a call to [`VkCmdDebugMarkerEndExt`]
///  - `command_buffer` must not have any shader instrumentation active
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
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
///  - [`VkResult::VkErrorInvalidVideoStdParametersKhr`]
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkEndCommandBuffer =
    unsafe extern "system" fn(command_buffer: VkCommandBuffer) -> VkResult;

/// The name of [`VkEndCommandBuffer`]
pub const VK_END_COMMAND_BUFFER: &CStr = c"vkEndCommandBuffer";
