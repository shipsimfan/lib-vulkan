use crate::{VkAllocationCallbacks, VkDevice, VkImageView, VkImageViewCreateInfo, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkQueueFlag};
#[allow(unused_imports)]
use std::ptr::null;

/// Create an image view from an existing image
///
/// # Parameters
///  - `device` is the logical device that creates the image view.
///  - `create_info` is a pointer to a [`VkImageViewCreateInfo`] structure containing parameters to
///    be used to create the image view.
///  - `allocator` controls host memory allocation.
///  - `view` is a pointer to a [`VkImageView`] handle in which the resulting image view object is
///    returned.
///
/// # Valid Usage
///  - `device` must support at least one queue family with one of the
///    [`VkQueueFlag::VideoEncodeBitKhr`], [`VkQueueFlag::VideoDecodeBitKhr`],
///    [`VkQueueFlag::ComputeBit`], or [`VkQueueFlag::GraphicsBit`] capabilities
///  - [`VkImageViewCreateInfo::image`] must have been created from device
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `create_info` must be a valid pointer to a valid [`VkImageViewCreateInfo`] structure
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - `view` must be a valid pointer to a [`VkImageView`] handle
///  - The device must have been created with at least 1 queue
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
pub type VkCreateImageView = extern "system" fn(
    device: VkDevice,
    create_info: *const VkImageViewCreateInfo,
    allocator: *const VkAllocationCallbacks,
    view: *mut VkImageView,
) -> VkResult;

/// The name of [`VkCreateImageView`]
pub const VK_CREATE_IMAGE_VIEW: &CStr = c"vkCreateImageView";
