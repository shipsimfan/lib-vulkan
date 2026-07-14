use crate::{VkAllocationCallbacks, VkBuffer, VkBufferCreateInfo, VkDevice, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VkBufferCreateFlag, VkBufferUsageFlag, VkImage, VkPhysicalDeviceLimits,
    VkQueueFlag,
};
#[allow(unused_imports)]
use std::ptr::null;

/// Create a new buffer object
///
/// # Parameters
///  - `device` is the logical device that creates the buffer object.
///  - `create_info` is a pointer to a [`VkBufferCreateInfo`] structure containing parameters
///    affecting creation of the buffer.
///  - `allocator` controls host memory allocation.
///  - `buffer` is a pointer to a [`VkBuffer`] handle in which the resulting buffer object is
///    returned.
///
/// # Description
/// Implementations may fail to create a buffer if the effective usage includes the
/// [`VkBufferUsageFlag::DescriptorHeapExt`] flag, and size is greater than the maximum of
/// `max_resource_heap_size` and `max_sampler_heap_size`. If this happens,
/// [`VkResult::VkErrorOutOfDeviceMemory`] will be returned.
///
/// # Valid Usage
///  - `device` must support at least one queue family with one of the
///    [`VkQueueFlag::VideoEncodeKhr`], [`VkQueueFlag::VideoDecodeKhr`],
///    [`VkQueueFlag::SparseBinding`], [`VkQueueFlag::Transfer`], [`VkQueueFlag::Compute`], or
///    [`VkQueueFlag::Graphics`] capabilities
///  - If the `flags` member of `create_info` includes [`VkBufferCreateFlag::SparseBinding`], and
///    the `extended_sparse_address_space` feature is not enabled, creating this [`VkBuffer`] must
///    not cause the total required sparse memory for all currently valid sparse resources on the
///    device to exceed [`VkPhysicalDeviceLimits::sparse_address_space_size`]
///  - If the `flags` member of `create_info` includes [`VkBufferCreateFlag::SparseBinding`], the
///    `extended_sparse_address_space` feature is enabled, and the usage member of `create_info`
///    contains bits not in
///    [`VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNv::extended_sparse_buffer_usage_flags`],
///    creating this [`VkBuffer`] must not cause the total required sparse memory for all currently
///    valid sparse resources on the device, excluding [`VkBuffer`] created with usage member of
///    `create_info` containing bits in
///    [`VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNv::extended_sparse_buffer_usage_flags`]
///    and [`VkImage`] created with usage member of `create_info` containing bits in
///    [`VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNv::extended_sparse_image_usage_flags`],
///    to exceed [`VkPhysicalDeviceLimits::sparse_address_space_size`]
///  - If the `flags` member of `create_info` includes [`VkBufferCreateFlag::SparseBinding`] and
///    the `extended_sparse_address_space` feature is enabled, creating this [`VkBuffer`] must not
///    cause the total required sparse memory for all currently valid sparse resources on the
///    device to exceed
///    [`VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNv::extended_sparse_address_space_size`]
///  - If using the [`VkBuffer`] for an import operation from a [`VkBufferCollectionFuchsia`] where
///    a [`VkBufferCollectionBufferCreateInfoFuchsia`] has been chained to `next`, `create_info`
///    must match the [`VkBufferConstraintsInfoFuchsia::create_info`] used when setting the
///    constraints on the buffer collection with [`VkSetBufferCollectionBufferConstraintsFuchsia`]
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `create_info` must be a valid pointer to a valid [`VkBufferCreateInfo`] structure
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - `buffer` must be a valid pointer to a [`VkBuffer`] handle
///  - The `device` must have been created with at least 1 queue
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
pub type VkCreateBuffer = unsafe extern "system" fn(
    device: VkDevice,
    create_info: *const VkBufferCreateInfo,
    allocator: *const VkAllocationCallbacks,
    buffer: *mut VkBuffer,
) -> VkResult;

/// The name of [`VkCreateBuffer`]
pub const VK_CREATE_BUFFER: &CStr = c"vkCreateBuffer";
