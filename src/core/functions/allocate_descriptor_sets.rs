use crate::{VkDescriptorSet, VkDescriptorSetAllocateInfo, VkDevice, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_NULL_HANDLE, VK_VERSION_1_0, VkDescriptorPoolCreateInfo, VkDescriptorType};

/// Allocate one or more descriptor sets
///
/// # Parameters
///  - `device` is the logical device that owns the descriptor pool.
///  - `allocate_info` is a pointer to a [`VkDescriptorSetAllocateInfo`] structure describing
///    parameters of the allocation.
///  - `descriptor_sets` is a pointer to an array of [`VkDescriptorSet`] handles in which the
///    resulting descriptor set objects are returned.
///
/// # Description
/// The allocated descriptor sets are returned in `descriptor_sets`.
///
/// When a descriptor set is allocated, the initial state is largely uninitialized and all
/// descriptors are undefined, with the exception that samplers with a non-null
/// `immutable_samplers` are initialized on allocation. Descriptors also become undefined if the
/// underlying resource or view object is destroyed. Descriptor sets containing undefined
/// descriptors can still be bound and used, subject to the following conditions:
///  - For descriptor set bindings created with the [`VkDescriptorBindingFlag::PartiallyBound`] bit
///    set, all descriptors in that binding that are dynamically used must have been populated
///    before the descriptor set is consumed.
///  - For descriptor set bindings created without the [`VkDescriptorBindingFlag::PartiallyBound`]
///    bit set, all descriptors in that binding that are statically used must have been populated
///    before the descriptor set is consumed.
///  - Descriptor bindings with descriptor type of [`VkDescriptorType::InlineUniformBlock`] can be
///    undefined when the descriptor set is consumed; though values in that block will be
///    undefined.
///  - Entries that are not used by a pipeline can have undefined descriptors.
///
/// If a call to [`VkAllocateDescriptorSets`] would cause the total number of descriptor sets
/// allocated from the pool to exceed the value of [`VkDescriptorPoolCreateInfo::max_sets`] used to
/// create `allocate_info.descriptor_pool`, then the allocation may fail due to lack of space in
/// the descriptor pool. Similarly, the allocation may fail due to lack of space if the call to
/// [`VkAllocateDescriptorSets`] would cause the number of any given descriptor type to exceed the
/// sum of all the `descriptor_count` members of each element of
/// [`VkDescriptorPoolCreateInfo::pool_sizes`] with a type equal to that type.
///
/// Additionally, the allocation may also fail if a call to [`VkAllocateDescriptorSets`] would
/// cause the total number of inline uniform block bindings allocated from the pool to exceed the
/// value of [`VkDescriptorPoolInlineUniformBlockCreateInfo::max_inline_uniform_block_bindings`]
/// used to create the descriptor pool.
///
/// If the allocation fails due to no more space in the descriptor pool, and not because of system
/// or device memory exhaustion, then [`VkResult::VkErrorOutOfPoolMemory`] must be returned.
///
/// [`VkAllocateDescriptorSets`] can be used to create multiple descriptor sets. If the creation of
/// any of those descriptor sets fails, then the implementation must destroy all successfully
/// created descriptor set objects from this command, set all entries of the `descriptor_sets`
/// array to [`VK_NULL_HANDLE`] and return the error.
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `allocate_info` must be a valid pointer to a valid [`VkDescriptorSetAllocateInfo`] structure
///  - `descriptor_sets` must be a valid pointer to an array of
///    `allocate_info.descriptor_set_count` [`VkDescriptorSet`] handles
///  - The device must have been created with at least 1 queue
///  - `allocate_info.descriptor_set_count` must be greater than 0
///
/// # Return Codes
///
/// On success, this command returns:
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorFragmentedPool`]
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorOutOfPoolMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkAllocateDescriptorSets = unsafe extern "system" fn(
    device: VkDevice,
    allocate_info: *const VkDescriptorSetAllocateInfo,
    descriptor_sets: *mut VkDescriptorSet,
) -> VkResult;

/// The name of [`VkAllocateDescriptorSets`]
pub const VK_ALLOCATE_DESCRIPTOR_SETS: &CStr = c"vkAllocateDescriptorSets";
