use crate::{VkCommandBuffer, VkDescriptorSet, VkPipelineBindPoint, VkPipelineLayout};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VK_VERSION_1_0, VK_WHOLE_SIZE, VkCommandPool, VkCommandPoolCreateInfo,
    VkDescriptorPool, VkDescriptorPoolCreateFlag, VkDescriptorPoolCreateInfo,
    VkDescriptorSetLayout, VkDescriptorSetLayoutCreateFlag, VkDescriptorSetLayoutCreateInfo,
    VkDescriptorType, VkDevice, VkPhysicalDeviceLimits, VkPipelineLayoutCreateInfo, VkQueueFlag,
};
#[allow(unused_imports)]
use std::ptr::null;

/// Binds descriptor sets to a command buffer
///
/// # Parameters
///  - `command_buffer` is the command buffer that the descriptor sets will be bound to.
///  - `pipeline_bind_point` is a [`VkPipelineBindPoint`] indicating the type of the pipeline that
///    will use the descriptors. There is a separate set of bind points for each pipeline type, so
///    binding one does not disturb the others.
///  - `layout` is a [`VkPipelineLayout`] object used to program the bindings.
///  - `first_set` is the set number of the first descriptor set to be bound.
///  - `descriptor_set_count` is the number of elements in the `descriptor_sets` array.
///  - `descriptor_sets` is a pointer to an array of handles to [`VkDescriptorSet`] objects
///    describing the descriptor sets to bind to.
///  - `dynamic_offset_count` is the number of dynamic offsets in the `dynamic_offsets` array.
///  - `dynamic_offsets` is a pointer to an array of [`u32`] values specifying dynamic offsets.
///
/// # Description
/// [`VkCmdBindDescriptorSets`] binds descriptor sets
/// `descriptor_sets[0..descriptor_set_count - 1]` to set numbers
/// `[first_set..first_set + descriptor_set_count - 1]` for subsequent bound pipeline commands set
/// by `pipeline_bind_point`. Any bindings that were previously applied via these sets, or calls to
/// [`VkCmdSetDescriptorBufferOffsetsExt`] or [`VkCmdBindDescriptorBufferEmbeddedSamplersExt`], are
/// no longer valid.
///
/// Once bound, a descriptor set affects rendering of subsequent commands that interact with the
/// given pipeline type in the command buffer until either a different set is bound to the same set
/// number, or the set is disturbed as described in Pipeline Layout Compatibility.
///
/// A compatible descriptor set must be bound for all set numbers that any shaders in a pipeline
/// access, at the time that a drawing or dispatching command is recorded to execute using that
/// pipeline. However, if none of the shaders in a pipeline statically use any bindings with a
/// particular set number, then no descriptor set need be bound for that set number, even if the
/// pipeline layout includes a non-trivial descriptor set layout for that set number.
///
/// When consuming a descriptor, a descriptor is considered valid if the descriptor is not
/// undefined as described by descriptor set allocation. If the `null_descriptor` feature is
/// enabled, a null descriptor is also considered valid. A descriptor that was disturbed by
/// Pipeline Layout Compatibility, or was never bound by [`VkCmdBindDescriptorSets`] is not
/// considered valid. For any given descriptor, [`VkDescriptorBindingFlag`] and
/// [`VkDescriptorSetLayoutCreateFlag`] determine if validity is defined in terms of the descriptor
/// being statically accessed, or dynamically accessed. If the descriptor is determined to be
/// accessed by the appropriate definition, the consuming descriptor type in the pipeline must
/// match the [`VkDescriptorType`] in [`VkDescriptorSetLayoutCreateInfo`] for the descriptor to be
/// considered valid. If a descriptor is a mutable descriptor, the consuming descriptor type in the
/// pipeline must match the active descriptor type for the descriptor to be considered valid.
///
/// If any of the sets being bound include dynamic uniform or storage buffers, then
/// `dynamic_offsets` includes one element for each array element in each dynamic descriptor type
/// binding in each set. Values are taken from `dynamic_offsets` in an order such that all entries
/// for set `N` come before set `N + 1`; within a set, entries are ordered by the binding numbers
/// in the descriptor set layouts; and within a binding array, elements are in order.
/// `dynamic_offset_count` must equal the total number of dynamic descriptors in the sets being
/// bound.
///
/// The effective offset used for dynamic uniform and storage buffer bindings is the sum of the
/// relative offset taken from `dynamic_offsets`, and the base address of the buffer plus base
/// offset in the descriptor set. The range of the dynamic uniform and storage buffer bindings is
/// the buffer range as specified in the descriptor set.
///
/// Each of the `descriptor_sets` must be compatible with the pipeline layout specified by
/// `layout`. The layout used to program the bindings must also be compatible with the pipeline
/// used in subsequent bound pipeline commands with that pipeline type, as defined in the Pipeline
/// Layout Compatibility section.
///
/// The descriptor set contents bound by a call to [`VkCmdBindDescriptorSets`] may be consumed at
/// the following times:
///  - For descriptor bindings created with the [`VkDescriptorBindingFlag::UpdateAfterBind`] bit
///    set, the contents may be consumed when the command buffer is submitted to a queue, or during
///    shader execution of the resulting draws and dispatches, or any time in between. Otherwise,
///  - during host execution of the command, or during shader execution of the resulting draws and
///    dispatches, or any time in between.
///
/// Thus, the contents of a descriptor set binding must not be altered (overwritten by an update
/// command, or freed) between the first point in time that it may be consumed, and when the
/// command completes executing on the queue.
///
/// The contents of `dynamic_offsets` are consumed immediately during execution of
/// [`VkCmdBindDescriptorSets`]. Once all pending uses have completed, it is legal to update and
/// reuse a descriptor set.
///
/// # Valid Usage
///  - If `command_buffer` is a secondary command buffer, it must have begun with
///    [`VkCommandBufferInheritanceDescriptorHeapInfoExt::sampler_heap_bind_info`] equal to
///    [`null`]
///  - If `command_buffer` is a secondary command buffer, it must have begun with
///    [`VkCommandBufferInheritanceDescriptorHeapInfoExt::resource_heap_bind_info`] equal to
///    [`null`]
///  - Each element of `descriptor_sets` that is not [`VK_NULL_HANDLE`] must have been allocated
///    with a [`VkDescriptorSetLayout`] that matches (is the same as, or identically defined as)
///    the [`VkDescriptorSetLayout`] at set `n` in `layout`, where `n` is the sum of `first_set`
///    and the index into `descriptor_sets`
///  - `dynamic_offset_count` must be equal to the total number of dynamic descriptors in
///    `descriptor_sets`
///  - The sum of `first_set` and `descriptor_set_count` must be less than or equal to
///    [`VkPipelineLayoutCreateInfo::set_layout_count`] provided when `layout` was created
///  - Each element of `dynamic_offsets` which corresponds to a descriptor binding with type
///    [`VkDescriptorType::UniformBufferDynamic`] must be a multiple of
///    [`VkPhysicalDeviceLimits::min_uniform_buffer_offset_alignment`]
///  - Each element of `dynamic_offsets` which corresponds to a descriptor binding with type
///    [`VkDescriptorType::StorageBufferDynamic`] must be a multiple of
///    [`VkPhysicalDeviceLimits::min_storage_buffer_offset_alignment`]
///  - For each dynamic uniform or storage buffer binding in `descriptor_sets`, the sum of the
///    effective offset and the range of the binding must be less than or equal to the size of the
///    buffer
///  - For each dynamic uniform or storage buffer binding in `descriptor_sets`, if the range was
///    set with [`VK_WHOLE_SIZE`] then `dynamic_offsets` which corresponds to the descriptor
///    binding must be 0
///  - Each element of `descriptor_sets` must not have been allocated from a [`VkDescriptorPool`]
///    with the [`VkDescriptorPoolCreateFlag::HostOnlyExt`] flag set
///  - If the `graphics_pipeline_library` feature is not enabled, each element of `descriptor_sets`
///    must be a valid [`VkDescriptorSet`]
///  - Each element of `descriptor_sets` must have been allocated with a [`VkDescriptorSetLayout`]
///    which was not created with [`VkDescriptorSetLayoutCreateFlag::DescriptorBufferExt`]
///  - If any element of `descriptor_sets` was allocated from a descriptor pool created with a
///    [`VkDescriptorPoolCreateInfo`] structure that had a
///    [`VkDataGraphProcessingEngineCreateInfoArm`] structure specifying foreign data processing
///    engines in its `next` chain, then the command pool from which `command_buffer` was allocated
///    must have been created with a [`VkCommandPoolCreateInfo`] structure that had a
///    [`VkDataGraphProcessingEngineCreateInfoArm`] structure in its `next` chain specifying a
///    superset of all the foreign data processing engines specified when creating the descriptor
///    pools from which the elements of `descriptor_sets` were allocated
///  - If none of the elements of `descriptor_sets` were allocated from a descriptor pool created
///    with a [`VkDescriptorPoolCreateInfo`] structure that had a
///    [`VkDataGraphProcessingEngineCreateInfoArm`] structure specifying foreign data processing
///    engines in its `next` chain, then the command pool from which `command_buffer` was allocated
///    must not have been created with a [`VkCommandPoolCreateInfo`] structure that had a
///    [`VkDataGraphProcessingEngineCreateInfoArm`] structure in its `next` chain
///  - `pipeline_bind_point` must be supported by the `command_buffer`’s parent [`VkCommandPool`]’s
///    queue family
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `pipeline_bind_point` must be a valid [`VkPipelineBindPoint`] value
///  - `layout` must be a valid [`VkPipelineLayout`] handle
///  - `descriptor_sets` must be a valid pointer to an array of `descriptor_set_count` valid or
///    [`VK_NULL_HANDLE`] [`VkDescriptorSet`] handles
///  - If `dynamic_offset_count` is not 0, `dynamic_offsets` must be a valid pointer to an array of
///    `dynamic_offset_count` [`u32`] values
///  - `command_buffer` must be in the recording state
///  - The [`VkCommandPool`] that `command_buffer` was allocated from must support
///    [`VkQueueFlag::Compute`], [`VkQueueFlag::DataGraphArm`], or [`VkQueueFlag::Graphics`]
///    operations
///  - This command must only be called outside of a video coding scope
///  - `descriptor_set_count` must be greater than 0
///  - Each of `command_buffer`, layout, and the elements of `descriptor_sets` that are valid
///    handles of non-ignored parameters must have been created, allocated, or retrieved from the
///    same [`VkDevice`]
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCmdBindDescriptorSets = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    pipeline_bind_point: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    first_set: u32,
    descriptor_set_count: u32,
    descriptor_sets: *const VkDescriptorSet,
    dynamic_offset_count: u32,
    dynamic_offsets: *const u32,
);

/// The name of [`VkCmdBindDescriptorSets`]
pub const VK_CMD_BIND_DESCRIPTOR_SETS: &CStr = c"vkCmdBindDescriptorSets";
