use crate::{VkDescriptorPool, VkDescriptorSet, VkDevice, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_0, VkDescriptorPoolCreateFlag};

/// Free one or more descriptor sets
///
/// # Parameters
///  - `device` is the logical device that owns the descriptor pool.
///  - `descriptor_pool` is the descriptor pool from which the descriptor sets were allocated.
///  - `descriptor_set_count` is the number of elements in the `descriptor_sets` array.
///  - `descriptor_sets` is a pointer to an array of handles to [`VkDescriptorSet`] objects.
///
/// # Description
/// After calling [`VkFreeDescriptorSets`], all descriptor sets in `descriptor_sets` are invalid.
///
/// # Valid Usage
///  - All submitted commands that refer to any element of `descriptor_sets` must have completed
///    execution
///  - `descriptor_sets` must be a valid pointer to an array of `descriptor_set_count`
///    [`VkDescriptorSet`] handles, each element of which must either be a valid handle or
///    [`VK_NULL_HANDLE`]
///  - `descriptor_pool` must have been created with the
///    [`VkDescriptorPoolCreateFlag::FreeDescriptorSet`] flag
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `descriptor_pool` must be a valid [`VkDescriptorPool`] handle
///  - `descriptor_set_count` must be greater than 0
///  - `descriptor_pool` must have been created, allocated, or retrieved from `device`
///  - Each element of `descriptor_sets` that is a valid handle must have been created, allocated,
///    or retrieved from `descriptor_pool`
///
/// # Host Synchronization
///  - Host access to `descriptor_pool` must be externally synchronized
///  - Host access to each member of `descriptor_sets` must be externally synchronized
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkFreeDescriptorSets = unsafe extern "system" fn(
    device: VkDevice,
    descriptor_pool: VkDescriptorPool,
    descriptor_set_count: u32,
    descriptor_sets: *const VkDescriptorSet,
) -> VkResult;

/// The name of [`VkFreeDescriptorSets`]
pub const VK_FREE_DESCRIPTOR_SETS: &CStr = c"vkFreeDescriptorSets";
