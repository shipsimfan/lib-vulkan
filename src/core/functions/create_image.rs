use crate::{VkAllocationCallbacks, VkDevice, VkImage, VkImageCreateInfo, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkBuffer, VkImageCreateFlag, VkPhysicalDeviceLimits, VkQueueFlag};
#[allow(unused_imports)]
use std::ptr::null;

/// Create a new image object
///
/// # Parameters
///  - `device` is the logical device that creates the image.
///  - `create_info` is a pointer to a [`VkImageCreateInfo`] structure containing parameters to be
///    used to create the image.
///  - `allocator` controls host memory allocation.
///  - `image` is a pointer to a [`VkImage`] handle in which the resulting image object is
///    returned.
///
/// # Valid Usage
///  - `device` must support at least one queue family with one of the
///    [`VkQueueFlag::VideoEncodeKhr`], [`VkQueueFlag::VideoDecodeKhr`],
///    [`VkQueueFlag::OpticalFlowNv`], [`VkQueueFlag::SparseBinding`], [`VkQueueFlag::Transfer`],
///    [`VkQueueFlag::Compute`], or [`VkQueueFlag::Graphics`] capabilities
///  - If the `flags` member of `create_info` includes [`VkImageCreateFlag::SparseBinding`], and
///    the `extended_sparse_address_space` feature is not enabled, creating this [`VkImage`] must
///    not cause the total required sparse memory for all currently valid sparse resources on the
///    device to exceed [`VkPhysicalDeviceLimits::sparse_address_space_size`]
///  - If the `flags` member of `create_info` includes [`VkImageCreateFlag::SparseBinding`], the0
///    `extended_sparse_address_space` feature is enabled, and the usage member of `create_info`
///    contains bits not in
///    [`VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNv::extended_sparse_image_usage_flags`],
///    creating this [`VkImage`] must not cause the total required sparse memory for all currently
///    valid sparse resources on the device, excluding [`VkBuffer`] created with usage member of
///    `create_info` containing bits in
///    [`VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNv::extended_sparse_buffer_usage_flags`]
///    and [`VkImage`] created with usage member of `create_info` containing bits in
///    [`VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNv::extended_sparse_image_usage_flags`],
///    to exceed [`VkPhysicalDeviceLimits::sparse_address_space_size`]
///  - If the `flags` member of `create_info` includes [`VkImageCreateFlag::SparseBinding`] and the
///    `extended_sparse_address_space` feature is enabled, creating this [`VkImage`] must not cause
///    the total required sparse memory for all currently valid sparse resources on the device to
///    exceed
///    [`VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNv::extended_sparse_address_space_size`]
///  - If a [`VkBufferCollectionImageCreateInfoFuchsia`] has been chained to `next`, `create_info`
///    must match the `sysmem` chosen [`VkImageCreateInfo`] excepting members
///    [`VkImageCreateInfo::extent`] and [`VkImageCreateInfo::usage`] in the match criteria
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `create_info` must be a valid pointer to a valid [`VkImageCreateInfo`] structure
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - `image` must be a valid pointer to a [`VkImage`] handle
///  - The device must have been created with at least 1 queue
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorCompressionExhaustedExt`]
///  - [`VkResult::VkErrorInvalidOpaqueCaptureAddress`]
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCreateImage = unsafe extern "system" fn(
    device: VkDevice,
    create_info: *const VkImageCreateInfo,
    allocator: *const VkAllocationCallbacks,
    view: *mut VkImage,
) -> VkResult;

/// The name of [`VkCreateImage`]
pub const VK_CREATE_IMAGE: &CStr = c"vkCreateImage";
