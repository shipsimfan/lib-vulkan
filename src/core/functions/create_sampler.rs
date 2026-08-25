use crate::{VkAllocationCallbacks, VkDevice, VkResult, VkSampler, VkSamplerCreateInfo};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkPhysicalDeviceLimits, VkQueueFlag};
#[allow(unused_imports)]
use std::ptr::null;

/// Create a new sampler object
///
/// # Parameters
///  - `device` is the logical device that creates the sampler.
///  - `create_info` is a pointer to a [`VkSamplerCreateInfo`] structure specifying the state of
///    the sampler object.
///  - `allocator` controls host memory allocation.
///  - `sampler` is a pointer to a [`VkSampler`] handle in which the resulting sampler object is
///    returned.
///
/// # Valid Usage
///  - `device` must support at least one queue family with one of the [`VkQueueFlag::Compute`] or
///    [`VkQueueFlag::Graphics`] capabilities
///  - There must be less than [`VkPhysicalDeviceLimits::max_sampler_allocation_count`]
///    [`VkSampler`] objects currently created on the device
///  - If there are any pipelines or shaders with embedded samplers currently created on the
///    device, there must be less than
///    `(max_sampler_allocation_count - (min_sampler_heap_reserved_range_with_embedded / sampler_descriptor_size))`
///    [`VkSampler`] objects currently created on the device
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `create_info` must be a valid pointer to a valid [`VkSamplerCreateInfo`] structure
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - `sampler` must be a valid pointer to a [`VkSampler`] handle
///
/// # Return Codes
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
/// Provided by [`VK_VERSION_1_0`]
pub type VkCreateSampler = unsafe extern "system" fn(
    device: VkDevice,
    create_info: *const VkSamplerCreateInfo,
    allocator: *const VkAllocationCallbacks,
    sampler: *mut VkSampler,
) -> VkResult;

/// The name of [`VkCreateSampler`]
pub const VK_CREATE_SAMPLER: &CStr = c"vkCreateSampler";
