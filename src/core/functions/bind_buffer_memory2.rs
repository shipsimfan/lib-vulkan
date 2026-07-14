use crate::{VkBindBufferMemoryInfo, VkDevice, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_1;

/// Bind device memory to buffer objects
///
/// # Parameters
///  - `device` is the logical device that owns the buffers and memory.
///  - `bind_info_count` is the number of elements in `bind_infos`.
///  - `bind_infos` is a pointer to an array of `bind_info_count` [`VkBindBufferMemoryInfo`]
///    structures describing buffers and memory to bind.
///
/// # Description
/// On some implementations, it may be more efficient to batch memory bindings into a single
/// command.
///
/// If the `maintenance6` feature is enabled, this command must attempt to perform all of the
/// memory binding operations described by `bind_infos`, and must not early exit on the first
/// failure.
///
/// If any of the memory binding operations described by `bind_infos` fail, the [`VkResult`]
/// returned by this command must be the return value of any one of the memory binding operations
/// which did not return [`VkResult::VkSuccess`].
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `bind_infos` must be a valid pointer to an array of `bind_info_count` valid
///    [`VkBindBufferMemoryInfo`] structures
///  - `bind_info_count` must be greater than 0
///
/// # Return Codes
///
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorInvalidOpaqueCaptureAddress`]
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
/// Provided by [`VK_VERSION_1_1`]
pub type VkBindBufferMemory2 = unsafe extern "system" fn(
    device: VkDevice,
    bind_info_count: u32,
    bind_infos: *const VkBindBufferMemoryInfo,
) -> VkResult;

/// The name of [`VkBindBufferMemory2`]
pub const VK_BIND_BUFFER_MEMORY2: &CStr = c"vkBindBufferMemory2";
