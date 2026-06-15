use crate::{VkCommandBuffer, VkPipeline, VkPipelineBindPoint};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_FALSE, VK_NULL_HANDLE, VK_TRUE, VK_VERSION_1_0, VkCommandPool, VkCommandPoolCreateInfo,
    VkDevice, VkDynamicState, VkPipelineCreateFlag, VkQueueFlag,
};

/// Bind a pipeline object to a command buffer
///
/// # Parameters
///  - `command_buffer` is the command buffer that the pipeline will be bound to.
///  - `pipeline_bind_point` is a [`VkPipelineBindPoint`] value specifying to which bind point the
///    pipeline is bound. Binding one does not disturb the others.
///  - `pipeline` is the pipeline to be bound.
///
/// # Description
/// Once bound, a pipeline binding affects subsequent commands that interact with the given
/// pipeline type in the command buffer until a different pipeline of the same type is bound to the
/// bind point, or until the pipeline bind point is disturbed by binding a shader object. Commands
/// that do not interact with the given pipeline type must not be affected by the pipeline state.
///
/// # Valid Usage
///  - If `pipeline_bind_point` is [`VkPipelineBindPoint::Compute`], the [`VkCommandPool`] that
///    `command_buffer` was allocated from must support compute operations
///  - If `pipeline_bind_point` is [`VkPipelineBindPoint::Graphics`], the [`VkCommandPool`] that
///    `command_buffer` was allocated from must support graphics operations
///  - If `pipeline_bind_point` is [`VkPipelineBindPoint::Compute`], `pipeline` must be a compute
///    pipeline
///  - If `pipeline_bind_point` is [`VkPipelineBindPoint::Graphics`], `pipeline` must be a graphics
///    pipeline
///  - If the `variable_multisample_rate` feature is not supported, `pipeline` is a graphics
///    pipeline, the current subpass uses no attachments, and this is not the first call to this
///    function with a graphics pipeline after transitioning to the current subpass, then the
///    sample count specified by this pipeline must match that set in the previous pipeline
///  - If [`VkPhysicalDeviceSampleLocationsPropertiesExt::variable_sample_locations`] is
///    [`VK_FALSE`], and `pipeline` is a graphics pipeline created with a `render_pass` that is not
///    [`VK_NULL_HANDLE`] and with a [`VkPipelineSampleLocationsStateCreateInfoExt`] structure
///    having its `sample_locations_enable` member set to [`VK_TRUE`] but without
///    [`VkDynamicState::SampleLocationsExt`] enabled then the current render pass instance must
///    have been begun by specifying a [`VkRenderPassSampleLocationsBeginInfoExt`] structure whose
///    `post_subpass_sample_locations` member contains an element with a `subpass_index` matching
///    the current subpass index and the `sample_locations_info` member of that element must match
///    the `sample_locations_info` specified in [`VkPipelineSampleLocationsStateCreateInfoExt`]
///    when the pipeline was created
///  - This command must not be recorded when transform feedback is active
///  - If `pipeline_bind_point` is [`VkPipelineBindPoint::RayTracingKhr`], the [`VkCommandPool`]
///    that `command_buffer` was allocated from must support compute operations
///  - If `pipeline_bind_point` is [`VkPipelineBindPoint::RayTracingKhr`], `pipeline` must be a ray
///    tracing pipeline
///  - If `pipeline_bind_point` is [`VkPipelineBindPoint::RayTracingKhr`], `command_buffer` must
///    not be a protected command buffer
///  - If the `pipeline_protected_access` feature is enabled, and `command_buffer` is a protected
///    command buffer, `pipeline` must have been created without
///    [`VkPipelineCreateFlag::NoProtectedAccess`]
///  - If the `pipeline_protected_access` feature is enabled, and `command_buffer` is not a
///    protected command buffer, `pipeline` must have been created without
///    [`VkPipelineCreateFlag::ProtectedAccessOnly`]
///  - `pipeline` must not have been created with [`VkPipelineCreateFlag::LibraryKhr`] set
///  - If `command_buffer` is a secondary command buffer with
///    [`VkCommandBufferInheritanceViewportScissorInfoNv::viewport_scissor_2d`] enabled and
///    `pipeline_bind_point` is [`VkPipelineBindPoint::Graphics`], then the pipeline must have been
///    created with [`VkDynamicState::ViewportWithCount`] or [`VkDynamicState::Viewport`], and
///    [`VkDynamicState::ScissorWithCount`] or [`VkDynamicState::Scissor`] enabled
///  - If `command_buffer` is a secondary command buffer with
///    [`VkCommandBufferInheritanceViewportScissorInfoNV::viewport_scissor_2d`] enabled and
///    `pipeline_bind_point` is [`VkPipelineBindPoint::Graphics`] and `pipeline` was created with
///    [`VkPipelineDiscardRectangleStateCreateInfoExt`] structure and its `discard_rectangle_count`
///    member is not 0, or the pipeline was created with
///    [`VkDynamicState::DiscardRectangleEnableExt`] enabled, then the pipeline must have been
///    created with [`VkDynamicState::DiscardRectangleExt`] enabled
///  - If `pipeline_bind_point` is [`VkPipelineBindPoint::Graphics`] and the
///    `provoking_vertex_mode_per_pipeline` limit is [`VK_FALSE`], then pipeline’s
///    [`VkPipelineRasterizationProvokingVertexStateCreateInfoExt::provoking_vertex_mode`] must be
///    the same as that of any other pipelines previously bound to this bind point within the
///    current render pass instance, including any pipeline already bound when beginning the render
///    pass instance
///  - If `pipeline_bind_point` is [`VkPipelineBindPoint::SubpassShadingHuawei`], the
///    [`VkCommandPool`] that `command_buffer` was allocated from must support compute operations
///  - If `pipeline_bind_point` is [`VkPipelineBindPoint::SubpassShadingHuawei`], `pipeline` must
///    be a subpass shading pipeline
///  - If `pipeline_bind_point` is [`VkPipelineBindPoint::DataGraphArm`], the [`VkCommandPool`]
///    that `command_buffer` was allocated from must have been created for a queue family that
///    supports [`VkQueueFlag::DataGraphArm`]
///  - If `pipeline_bind_point` is [`VkPipelineBindPoint::DataGraphArm`], `pipeline` must be a data
///    graph pipeline
///  - If `pipeline` is a data graph pipeline and the [`VkDataGraphPipelineCreateInfoArm`]
///    structure used to create it had a [`VkDataGraphProcessingEngineCreateInfoArm`] structure in
///    its `next` chain that specified any foreign data processing engines, then the command pool
///    from which `command_buffer` was allocated must have been created with a
///    [`VkCommandPoolCreateInfo`] structure that had a
///    [`VkDataGraphProcessingEngineCreateInfoArm`] structure specifying a superset of the foreign
///    data graph processing engines specified at pipeline creation time in its `next` chain
///  - If `pipeline` is a data graph pipeline and the [`VkDataGraphPipelineCreateInfoArm`]
///    structure used to create it did not have a [`VkDataGraphProcessingEngineCreateInfoArm`]
///    structure in its `next` chain, then the command pool from which `command_buffer` was
///    allocated must not have been created with a [`VkCommandPoolCreateInfo`] that had a
///    [`VkDataGraphProcessingEngineCreateInfoArm`] structure in its `next` chain
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `pipeline_bind_point` must be a valid [`VkPipelineBindPoint`] value
///  - `pipeline` must be a valid [`VkPipeline`] handle
///  - `command_buffer` must be in the recording state
///  - The [`VkCommandPool`] that `command_buffer` was allocated from must support
///    [`VkQueueFlag::Compute`], [`VkQueueFlag::DataGraphArm`], or [`VkQueueFlag::Graphics`]
///    operations
///  - This command must only be called outside of a video coding scope
///  - Both of `command_buffer`, and `pipeline` must have been created, allocated, or retrieved
///    from the same [`VkDevice`]
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCmdBindPipeline = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    pipeline_bind_point: VkPipelineBindPoint,
    pipeline: VkPipeline,
);

/// The name of [`VkCmdBindPipeline`]
pub const VK_CMD_BIND_PIPELINE: &CStr = c"vkCmdBindPipeline";
