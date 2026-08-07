use crate::{VkCopyDescriptorSet, VkDevice, VkWriteDescriptorSet};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkDescriptorType};

/// Update the contents of a descriptor set object
///
/// # Parameters
///  - `device` is the logical device that updates the descriptor sets.
///  - `descriptor_write_count` is the number of elements in the `descriptor_writes` array.
///  - `descriptor_writes` is a pointer to an array of [`VkWriteDescriptorSet`] structures
///    describing the descriptor sets to write to.
///  - `descriptor_copy_count` is the number of elements in the `descriptor_copies` array.
///  - `descriptor_copies` is a pointer to an array of [`VkCopyDescriptorSet`] structures
///    describing the descriptor sets to copy between.
///
/// # Description
/// The operations described by `descriptor_writes` are performed first, followed by the operations
/// described by `descriptor_copies`. Within each array, the operations are performed in the order
/// they appear in the array.
///
/// Each element in the `descriptor_writes` array describes an operation updating the descriptor
/// set using descriptors for resources specified in the structure.
///
/// Each element in the `descriptor_copies` array is a [`VkCopyDescriptorSet`] structure describing
/// an operation copying descriptors between sets.
///
/// If the `dst_set` member of any element of `descriptor_writes` or `descriptor_copies` is bound,
/// accessed, or modified by any command that was recorded to a command buffer which is currently
/// in the recording or executable state, and any of the descriptor bindings that are updated were
/// not created with the [`VkDescriptorBinding::UpdateAfterBind`] or
/// [`VkDescriptorBinding::UpdateUnusedWhilePending`] bits set, that command buffer becomes
/// invalid.
///
/// Copying a descriptor from a descriptor set does not constitute a use of the referenced resource
/// or view, as it is the reference itself that is copied. Applications can copy a descriptor
/// referencing a destroyed resource, and it can copy an undefined descriptor. The destination
/// descriptor becomes undefined in both cases.
///
/// # Valid Usage
///  - For each element `i` where `descriptor_writes[i].descriptor_type` is
///    [`VkDescriptorType::UniformTexelBuffer`] or [`VkDescriptorType::StorageTexelBuffer`],
///    elements of the `texel_buffer_view` member of `descriptor_writes[i]` must have been created
///    on `device`
///  - For each element `i` where `descriptor_writes[i].descriptor_type` is
///    [`VkDescriptorType::UniformBuffer`], [`VkDescriptorType::StorageBuffer`],
///    [`VkDescriptorType::UniformBufferDynamic`], or [`VkDescriptorType::StorageBufferDynamic`],
///    the `buffer` member of any element of the `buffer_info` member of `descriptor_writes[i]`
///    must have been created on `device`
///  - For each element `i` where `descriptor_writes[i].descriptor_type` is
///    [`VkDescriptorType::Sampler`] or [`VkDescriptorType::CombinedImageSampler`], and `dst_set`
///    was not allocated with a layout that included immutable samplers for `dst_binding` with
///    `descriptor_type`, the sampler member of any element of the `image_info` member of
///    `descriptor_writes[i]` must have been created on `device`
///  - For each element `i` where `descriptor_writes[i].descriptor_type` is
///    [`VkDescriptorType::SampledImage`], [`VkDescriptorType::StorageImage`],
///    [`VkDescriptorType::InputAttachment`], [`VkDescriptorType::SampleWeightImageQcom`],
///    [`VkDescriptorType::BlockMatchImageQcom`], or [`VkDescriptorType::CombinedImageSampler`] the
///    `image_view` member of any element of `descriptor_writes[i]` must have been created on
///    `device`
///  - For each element `i` where `descriptor_writes[i].descriptor_type` is
///    [`VkDescriptorType::AccelerationStructureKhr`], elements of the `acceleration_structures`
///    member of a [`VkWriteDescriptorSetAccelerationStructureKhr`] structure in the `next` chain
///    of `descriptor_writes[i]` must have been created on `device`
///  - For each element `i` where `descriptor_writes[i].descriptor_type` is
///    [`VkDescriptorType::AccelerationStructureNv`], elements of the `acceleration_structures`
///    member of a [`VkWriteDescriptorSetAccelerationStructureNv`] structure in the `next` chain of
///    `descriptor_writes[i]` must have been created on `device`
///  - For each element `i` where `descriptor_writes[i].descriptor_type` is
///    [`VkDescriptorType::TensorArm`], elements of the `tensor_views` member of a
///    [`VkWriteDescriptorSetTensorArm`] structure in the `next` chain of `descriptor_writes[i]`
///    must have been created on `device`
///  - For each element `i` where `descriptor_writes[i].descriptor_type` is
///    [`VkDescriptorType::Sampler`], [`VkDescriptorType::CombinedImageSampler`],
///    [`VkDescriptorType::SampledImage`], [`VkDescriptorType::StorageImage`],
///    [`VkDescriptorType::SampleWeightImageQcom`], [`VkDescriptorType::BlockMatchImageQcom`], or
///    [`VkDescriptorType::InputAttachment`], `descriptor_writes[i].image_info` must be a valid
///    pointer to an array of `descriptor_writes[i].descriptor_count` valid
///    [`VkDescriptorImageInfo`] structures
///  - The `dst_set` member of each element of `descriptor_writes` or `descriptor_copies` for
///    bindings which were created without the [`VkDescriptorBinding::UpdateAfterBind`] or
///    [`VkDescriptorBinding::UpdateUnusedWhilePending`] bits set must not be used by any command
///    that was recorded to a command buffer which is in the pending state
///  - Host access to `descriptor_writes[i].dst_set` and `descriptor_copies[i].dst_set` must be
///    externally synchronized unless explicitly denoted otherwise for specific flags
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - If `descriptor_write_count` is not 0, `descriptor_writes` must be a valid pointer to an
///    array of `descriptor_write_count` valid [`VkWriteDescriptorSet`] structures
///  - If `descriptor_copy_count` is not 0, `descriptor_copies` must be a valid pointer to an array
///    of `descriptor_copy_count` valid [`VkCopyDescriptorSet`] structures
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkUpdateDescriptorSets = unsafe extern "system" fn(
    device: VkDevice,
    descriptor_write_count: u32,
    descriptor_writes: *const VkWriteDescriptorSet,
    descriptor_copy_count: u32,
    descriptor_copies: *const VkCopyDescriptorSet,
);

/// The name of [`VkUpdateDescriptorSets`]
pub const VK_UPDATE_DESCRIPTOR_SETS: &CStr = c"vkUpdateDescriptorSets";
