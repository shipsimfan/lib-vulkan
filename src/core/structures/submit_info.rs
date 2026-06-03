use crate::{VkCommandBuffer, VkPipelineStageFlags, VkSemaphore, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_TRUE, VK_VERSION_1_0, VkCommandBufferLevel, VkCommandPoolCreateInfo, VkDevice,
    VkPipelineStageFlag,
};

/// Structure specifying a queue submit operation
///
/// # Description
/// The order that command buffers appear in `command_buffers` is used to determine submission
/// order, and thus all the implicit ordering guarantees that respect it. Other than these implicit
/// ordering guarantees and any explicit synchronization primitives, these command buffers may
/// overlap or otherwise execute out of order.
///
/// The second synchronization scope of each semaphore wait operation defined by this structure is
/// limited to operations in stages indicated by the corresponding element of
/// `wait_dst_stage_mask`.
///
/// # Valid Usage (Implicit)
///  - Each of the elements of `command_buffers`, the elements of `signal_semaphores`, and the
///    elements of `wait_semaphores` that are valid handles of non-ignored parameters must have
///    been created, allocated, or retrieved from the same [`VkDevice`]
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkSubmitInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::SubmitInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If the `next` chain of this structure includes a [`VkTimelineSemaphoreSubmitInfo`]
    ///    structure and any element of `wait_semaphores` was created with a [`VkSemaphoreType`] of
    ///    [`VkSemaphoreType::Timeline`], then its `wait_semaphore_value_count` member must equal
    ///    `wait_semaphore_count`
    ///  - If the `next` chain of this structure includes a [`VkTimelineSemaphoreSubmitInfo`]
    ///    structure and any element of `signal_semaphores` was created with a [`VkSemaphoreType`]
    ///    of [`VkSemaphoreType::Timeline`], then its `signal_semaphore_value_count` member must
    ///    equal `signal_semaphore_count`
    ///  - If the `next` chain of this structure does not include a [`VkProtectedSubmitInfo`]
    ///    structure with `protected_submit` set to [`VK_TRUE`], then each element of the
    ///    `command_buffers` array must be an unprotected command buffer
    ///  - If the `next` chain of this structure includes a [`VkProtectedSubmitInfo`] structure
    ///    with `protected_submit` set to [`VK_TRUE`], then each element of the `command_buffers`
    ///    array must be a protected command buffer
    ///  - If the `next` chain of this structure includes a [`VkFrameBoundaryTensorsArm`] structure
    ///    then it must also include a [`VkFrameBoundaryExt`] structure
    ///  - If at least one [`VkCommandBufferSubmitInfo`] structure in `command_buffer_infos`
    ///    references a `command_buffer` allocated from a pool that was created with a
    ///    [`VkDataGraphProcessingEngineCreateInfoArm`] structure in the `next` chain of
    ///    [`VkCommandPoolCreateInfo`] that included a foreign data graph processing engine in its
    ///    `processing_engines` member, then `wait_semaphore_infos` and `signal_semaphore_infos`
    ///    must only reference semaphore objects that were created from external handle types
    ///    reported as supported in a
    ///    [`VkQueueFamilyDataGraphProcessingEnginePropertiesArm::foreign_semaphore_handle_types`]
    ///    structure via [`VkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesArm`]
    ///    with a `queue_family_index` matching the one the command pool was created for, for all
    ///    the foreign data graph processing engines that were part of the
    ///    [`VkDataGraphProcessingEngineCreateInfoArm`] used to create the command pool
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of [`VkAmigoProfilingSubmitInfoSec`],
    ///    [`VkD3D12FenceSubmitInfoKhr`], [`VkDeviceGroupSubmitInfo`], [`VkFrameBoundaryExt`],
    ///    [`VkFrameBoundaryTensorsArm`], [`VkLatencySubmissionPresentIdNv`],
    ///    [`VkPerformanceQuerySubmitInfoKhr`], [`VkProtectedSubmitInfo`],
    ///    [`VkThrottleHintSubmitInfoSec`], [`VkTimelineSemaphoreSubmitInfo`],
    ///    [`VkWin32KeyedMutexAcquireReleaseInfoKhr`], or [`VkWin32KeyedMutexAcquireReleaseInfoNv`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `wait_semaphore_count` is the number of semaphores upon which to wait before executing the
    /// command buffers for the batch.
    pub wait_semaphore_count: u32,

    /// `wait_semaphores` is a pointer to an array of [`VkSemaphore`] handles upon which to wait
    /// before the command buffers for this batch begin execution. If semaphores to wait on are
    /// provided, they define a semaphore wait operation.
    ///
    /// # Valid Usage
    ///  - If any element of `wait_semaphores` or `signal_semaphores` was created with a
    ///    [`VkSemaphoreType`] of [`VkSemaphoreType::Timeline`], then the `next` chain must include
    ///    a [`VkTimelineSemaphoreSubmitInfo`] structure
    ///
    /// # Valid Usage (Implicit)
    ///  - If `wait_semaphore_count` is not 0, `wait_semaphores` must be a valid pointer to an
    ///    array of `wait_semaphore_count` valid [`VkSemaphore`] handles
    pub wait_semaphores: *const VkSemaphore,

    /// `wait_dst_stage_mask` is a pointer to an array of pipeline stages at which each
    /// corresponding semaphore wait will occur.
    ///
    /// # Valid Usage
    ///  - If the `geometry_shader` feature is not enabled, `wait_dst_stage_mask` must not contain
    ///    [`VkPipelineStageFlag::GeometryShader`]
    ///  - If the `tessellation_shader` feature is not enabled, `wait_dst_stage_mask` must not
    ///    contain [`VkPipelineStageFlag::TessellationControlShader`] or
    ///    [`VkPipelineStageFlag::TessellationEvaluationShader`]
    ///  - If the `conditional_rendering` feature is not enabled, `wait_dst_stage_mask` must not
    ///    contain [`VkPipelineStageFlag::ConditionalRenderingExt`]
    ///  - If the `fragment_density_map` feature is not enabled, `wait_dst_stage_mask` must not
    ///    contain [`VkPipelineStageFlag::FragmentDensityProcessExt`]
    ///  - If the `transform_feedback` feature is not enabled, `wait_dst_stage_mask` must not
    ///    contain [`VkPipelineStageFlag::TransformFeedbackExt`]
    ///  - If the `mesh_shader` feature is not enabled, `wait_dst_stage_mask` must not contain
    ///    [`VkPipelineStageFlag::MeshShaderExt`]
    ///  - If the `task_shader` feature is not enabled, `wait_dst_stage_mask` must not contain
    ///    [`VkPipelineStageFlag::TaskShaderExt`]
    ///  - If neither of the `shading_rate_image` or the `attachment_fragment_shading_rate`
    ///    features are enabled, `wait_dst_stage_mask` must not contain
    ///    [`VkPipelineStageFlag::FragmentShadingRateAttachmentKhr`]
    ///  - If the `synchronization2` feature is not enabled, `wait_dst_stage_mask` must not be 0
    ///  - If neither the [`nv_ray_tracing`] extension or the `ray_tracing_pipeline` feature are
    ///    enabled, `wait_dst_stage_mask` must not contain
    ///    [`VkPipelineStageFlag::RayTracingShaderKhr`]
    ///  - If the `acceleration_structure` feature is not enabled, `wait_dst_stage_mask` must not
    ///    contain [`VkPipelineStageFlag::AccelerationStructureBuildKhr`]
    ///  - Each element of `wait_dst_stage_mask` must not include [`VkPipelineStageFlag::Host`]
    ///
    /// # Valid Usage (Implicit)
    ///  - If `wait_semaphore_count` is not 0, `wait_dst_stage_mask` must be a valid pointer to an
    ///    array of `wait_semaphore_count` valid combinations of [`VkPipelineStageFlag`] values
    pub wait_dst_stage_mask: *const VkPipelineStageFlags,

    /// `command_buffer_count` is the number of command buffers to execute in the batch.
    pub command_buffer_count: u32,

    /// `command_buffers` is a pointer to an array of [`VkCommandBuffer`] handles to execute in the
    /// batch.
    ///
    /// # Valid Usage
    ///  - Each element of `command_buffers` must not have been allocated with
    ///    [`VkCommandBufferLevel::Secondary`]
    ///  - If `command_buffers` contains any resumed render pass instances, they must be suspended
    ///    by a render pass instance earlier in submission order within `command_buffers`
    ///  - If `command_buffers` contains any suspended render pass instances, they must be resumed
    ///    by a render pass instance later in submission order within `command_buffers`
    ///  - If `command_buffers` contains any suspended render pass instances, there must be no
    ///    action or synchronization commands executed in a primary or secondary command buffer
    ///    between that render pass instance and the render pass instance that resumes it
    ///  - If `command_buffers` contains any suspended render pass instances, there must be no
    ///    render pass instances between that render pass instance and the render pass instance
    ///    that resumes it
    ///  - If the `variable_sample_locations` limit is not supported, and any element of
    ///    `command_buffers` contains any suspended render pass instances, where a graphics
    ///    pipeline has been bound, any pipelines bound in the render pass instance that resumes
    ///    it, or any subsequent render pass instances that resume from that one and so on, must
    ///    use the same sample locations
    ///
    /// # Valid Usage (Implicit)
    ///  - If `command_buffer_count` is not 0, `command_buffers` must be a valid pointer to an
    ///    array of `command_buffer_count` valid [`VkCommandBuffer`] handles
    pub command_buffers: *const VkCommandBuffer,

    /// `signal_semaphore_count` is the number of semaphores to be signaled once the commands
    /// specified in `command_buffers` have completed execution.
    pub signal_semaphore_count: u32,

    /// `signal_semaphores` is a pointer to an array of [`VkSemaphore`] handles which will be
    /// signaled when the command buffers for this batch have completed execution. If semaphores to
    /// be signaled are provided, they define a semaphore signal operation.
    ///
    /// # Valid Usage
    ///  - For each element of `signal_semaphores` created with a [`VkSemaphoreType`] of
    ///    [`VkSemaphoreType::Timeline`] the corresponding element of
    ///    [`VkTimelineSemaphoreSubmitInfo::signal_semaphore_values`] must have a value greater
    ///    than the current value of the semaphore when the semaphore signal operation is executed
    ///  - For each element of `wait_semaphores` created with a [`VkSemaphoreType`] of
    ///    [`VkSemaphoreType::Timeline`] the corresponding element of
    ///    [`VkTimelineSemaphoreSubmitInfo::wait_semaphore_values`] must have a value which does
    ///    not differ from the current value of the semaphore or the value of any outstanding
    ///    semaphore wait or signal operation on that semaphore by more than
    ///    `max_timeline_semaphore_value_difference`
    ///  - For each element of `signal_semaphores` created with a [`VkSemaphoreType`] of
    ///    [`VkSemaphoreType::Timeline`] the corresponding element of
    ///    [`VkTimelineSemaphoreSubmitInfo::p_signal_semaphore_values`] must have a value which
    ///    does not differ from the current value of the semaphore or the value of any outstanding
    ///    semaphore wait or signal operation on that semaphore by more than
    ///    `max_timeline_semaphore_value_difference`
    ///
    /// # Valid Usage (Implicit)
    ///  - If `signal_semaphore_count` is not 0, `signal_semaphores` must be a valid pointer to an
    ///    array of `signal_semaphore_count` valid [`VkSemaphore`] handles
    pub signal_semaphores: *const VkSemaphore,
}

impl const Default for VkSubmitInfo {
    fn default() -> Self {
        VkSubmitInfo {
            r#type: VkStructureType::SubmitInfo,
            next: null(),
            wait_semaphore_count: 0,
            wait_semaphores: null(),
            wait_dst_stage_mask: null(),
            command_buffer_count: 0,
            command_buffers: null(),
            signal_semaphore_count: 0,
            signal_semaphores: null(),
        }
    }
}

impl NextChain for VkSubmitInfo {
    fn structure_type(&self) -> VkStructureType {
        self.r#type
    }

    fn next(&self) -> *const c_void {
        self.next
    }
    
    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: Option<&dyn NextChain>) {
        self.next = next.map_or(null(), |n| n.as_ptr());
    }
}
