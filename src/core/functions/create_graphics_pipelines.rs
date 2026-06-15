use crate::{
    VkAllocationCallbacks, VkDevice, VkGraphicsPipelineCreateInfo, VkPipeline, VkPipelineCache,
    VkResult,
};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VK_VERSION_1_0, VkPipelineCreateFlag, VkPipelineShaderStageCreateInfo,
    VkQueueFlag, VkSampler,
};
#[allow(unused_imports)]
use std::ptr::null;

/// Create graphics pipelines
///
/// # Parameters
///  - `device` is the logical device that creates the graphics pipelines.
///  - `pipeline_cache` is either [`VK_NULL_HANDLE`], indicating that pipeline caching is disabled,
///    or to enable caching, the handle of a valid [`VkPipelineCache`] object. The implementation
///    must not access this object outside of the duration of this command.
///  - `create_info_count` is the length of the `create_infos` and `pipelines` arrays.
///  - `create_infos` is a pointer to an array of [`VkGraphicsPipelineCreateInfo`] structures.
///  - `allocator` controls host memory allocation.
///  - `pipelines` is a pointer to an array of [`VkPipeline`] handles in which the resulting
///    graphics pipeline objects are returned.
///
/// # Description
/// The [`VkGraphicsPipelineCreateInfo`] structure includes an array of
/// [`VkPipelineShaderStageCreateInfo`] structures for each of the desired active shader stages, as
/// well as creation information for all relevant fixed-function stages, and a pipeline layout.
///
/// Pipelines are created and returned as described for Multiple Pipeline Creation.
///
/// # Valid Usage
///  - `device` must support at least one queue family with the [`VkQueueFlag::Graphics`]
///    capability
///  - If the `flags` member of any element of `create_infos` contains the
///    [`VkPipelineCreateFlag::Derivative`] flag, and the `base_pipeline_index` member of that same
///    element is not -1, `base_pipeline_index` must be less than the index into `create_infos`
///    that corresponds to that element
///  - If the `flags` member of any element of `create_infos` contains the
///    [`VkPipelineCreateFlag::Derivative`] flag, the base pipeline must have been created with the
///    [`VkPipelineCreateFlag::AllowDerivatives`] flag set
///  - If `pipeline_cache` was created with [`VkPipelineCacheCreateFlag::ExternallySynchronized`],
///    host access to `pipeline_cache` must be externally synchronized
///  - If [`VkPipelineBinaryInfoKhr::binary_count`] is not 0 for any element of `create_infos`,
///    `pipeline_cache` must be [`VK_NULL_HANDLE`]
///  - If a [`VkPipelineCreateFlags2CreateInfoKhr`] structure with the
///    [`VkPipelineCreateFlag2::CaptureDataKhr`] flag set is included in the `next` chain of any
///    element of `create_infos`, `pipeline_cache` must be [`VK_NULL_HANDLE`]
///  - If [`VkPipelineBinaryInfoKhr::binary_count`] is not 0 for any element of `create_infos`,
///    [`VkPipelineCreationFeedbackFlag::ApplicationPipelineCacheHit`] must not be set in the flags
///    of that element
///  - If [`VkPipelineBinaryInfoKhr::binary_count`] is not 0 for any element of `create_infos`,
///    [`VkPipelineCreationFeedbackFlag::BasePipelineAcceleration`] must not be set in the flags of
///    that element
///  - If [`VkPipelineBinaryInfoKhr::binary_count`] is not 0 for any element of `create_infos`,
///    [`VkPipelineCreateFlag::FailOnPipelineCompileRequiredExt`] must not be set in the flags of
///    that element
///  - If any element of `create_infos` sets [`VkPipelineCreateFlag2::DescriptorHeapExt`] and
///    includes embedded sampler mappings, there must be less than
///    `(max_sampler_allocation_count - (min_sampler_heap_reserved_range_with_embedded / sampler_descriptor_size))`
///    [`VkSampler`] objects currently created on the device
///  - If any element of `create_infos` sets [`VkPipelineCreateFlag2::DescriptorHeapExt`] and
///    includes embedded sampler mappings, this command must not cause the total number of unique
///    embedded samplers in pipelines and shaders on this device to exceed
///    `max_descriptor_heap_embedded_samplers`
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - If `pipeline_cache` is not [`VK_NULL_HANDLE`], `pipeline_cache` must be a valid
///    [`VkPipelineCache`] handle
///  - `create_infos` must be a valid pointer to an array of `create_info_count` valid
///    [`VkGraphicsPipelineCreateInfo`] structures
///  - If `allocator` is not [`null`], `allocator` must be a valid pointer to a valid
///    [`VkAllocationCallbacks`] structure
///  - `pipelines` must be a valid pointer to an array of `create_info_count` [`VkPipeline`]
///    handles
///  - `create_info_count` must be greater than 0
///  - If `pipeline_cache` is a valid handle, it must have been created, allocated, or retrieved
///    from device
///
/// # Return Codes
/// On success, this command returns:
///  - [`VkResult::VkPipelineCompileRequired`]
///  - [`VkResult::VkSuccess`]
///
/// On failure, this command returns:
///  - [`VkResult::VkErrorInvalidShaderNv`]
///  - [`VkResult::VkErrorOutOfHostMemory`]
///  - [`VkResult::VkErrorOutOfDeviceMemory`]
///  - [`VkResult::VkErrorUnknown`]
///  - [`VkResult::VkErrorValidationFailedExt`]
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCreateGraphicsPipelines = unsafe extern "system" fn(
    device: VkDevice,
    pipeline_cache: VkPipelineCache,
    create_info_count: u32,
    create_infos: *const VkGraphicsPipelineCreateInfo,
    allocator: *const VkAllocationCallbacks,
    pipelines: *mut VkPipeline,
) -> VkResult;

/// The name of [`VkCmdEndRendering`]
pub const VK_CREATE_GRAPHICS_PIPELINES: &CStr = c"vkCreateGraphicsPipelines";
