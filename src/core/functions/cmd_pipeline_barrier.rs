use crate::{
    VkBufferMemoryBarrier, VkCommandBuffer, VkDependencyFlags, VkImageMemoryBarrier,
    VkMemoryBarrier, VkPipelineStageFlags,
};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_0, VkCommandPool, VkCommandPoolCreateInfo, VkDependencyFlag, VkImageLayout,
    VkPipelineStageFlag, VkQueueFlag, VkRenderPass,
};

/// Insert a memory dependency
///
/// # Parameters
///  - `command_buffer` is the command buffer into which the command is recorded.
///  - `src_stage_mask` is a bitmask of [`VkPipelineStageFlag`]s specifying the source stages.
///  - `dst_stage_mask` is a bitmask of [`VkPipelineStageFlag`]s specifying the destination
///    stages.
///  - `dependency_flags` is a bitmask of [`VkDependencyFlag`]s specifying how execution and memory
///    dependencies are formed.
///  - `memory_barrier_count` is the length of the `memory_barriers` array.
///  - `memory_barriers` is a pointer to an array of [`VkMemoryBarrier`] structures.
///  - `buffer_memory_barrier_count` is the length of the `buffer_memory_barrier` array.
///  - `buffer_memory_barrier` is a pointer to an array of [`VkBufferMemoryBarrier`] structures.
///  - `image_memory_barrier_count` is the length of the `image_memory_barriers` array.
///  - `image_memory_barriers` is a pointer to an array of [`VkImageMemoryBarrier`] structures.
///
/// # Description
/// [`VkCmdPipelineBarrier`] operates almost identically to [`VkCmdPipelineBarrier2`], except that
/// the scopes and barriers are defined as direct parameters rather than being defined by a
/// [`VkDependencyInfo`].
///
/// When [`VkCmdPipelineBarrier`] is submitted to a queue, it defines a memory dependency between
/// commands that were submitted to the same queue before it, and those submitted to the same queue
/// after it.
///
/// If [`VkCmdPipelineBarrier`] was recorded outside a render pass instance, the first
/// synchronization scope includes all commands that occur earlier in submission order. If
/// [`VkCmdPipelineBarrier`] was recorded inside a render pass instance, the first synchronization
/// scope includes only commands that occur earlier in submission order within the same subpass. In
/// either case, the first synchronization scope is limited to operations on the pipeline stages
/// determined by the source stage mask specified by `src_stage_mask`.
///
/// If [`VkCmdPipelineBarrier`] was recorded outside a render pass instance, the second
/// synchronization scope includes all commands that occur later in submission order. If
/// [`VkCmdPipelineBarrier`] was recorded inside a render pass instance, the second synchronization
/// scope includes only commands that occur later in submission order within the same subpass. In
/// either case, the second synchronization scope is limited to operations on the pipeline stages
/// determined by the destination stage mask specified by `dst_stage_mask`.
///
/// The first access scope is limited to accesses in the pipeline stages determined by the source
/// stage mask specified by `src_stage_mask`. Within that, the first access scope only includes the
/// first access scopes defined by elements of the `memory_barriers`, `buffer_memory_barrier` and
/// `image_memory_barriers` arrays, which each define a set of memory barriers. If no memory
/// barriers are specified, then the first access scope includes no accesses.
///
/// The second access scope is limited to accesses in the pipeline stages determined by the
/// destination stage mask specified by `dst_stage_mask`. Within that, the second access scope only
/// includes the second access scopes defined by elements of the `memory_barriers`,
/// `buffer_memory_barrier` and `image_memory_barriers` arrays, which each define a set of memory
/// barriers. If no memory barriers are specified, then the second access scope includes no
/// accesses.
///
/// If `dependency_flags` includes [`VkDependencyFlag::ByRegion`], then any dependency between
/// framebuffer-space pipeline stages is framebuffer-local - otherwise it is framebuffer-global.
///
/// # Valid Usage
///  - If the `geometry_shader` feature is not enabled, `src_stage_mask` must not contain
///    [`VkPipelineStageFlag::GeometryShader`]
///  - If the `tessellation_shader` feature is not enabled, `src_stage_mask` must not contain
///    [`VkPipelineStageFlag::TessellationControlShader`] or
///    [`VkPipelineStageFlag::TessellationEvaluationShader`]
///  - If the conditionalRendering feature is not enabled, `src_stage_mask` must not contain
///    [`VkPipelineStageFlag::ConditionalRenderingExt`]
///  - If the `fragment_density_map` feature is not enabled, `src_stage_mask` must not contain
///    [`VkPipelineStageFlag::FragmentDensityProcessExt`]
///  - If the `transform_feedback` feature is not enabled, `src_stage_mask` must not contain
///    [`VkPipelineStageFlag::TransformFeedbackExt`]
///  - If the `mesh_shader` feature is not enabled, `src_stage_mask` must not contain
///    [`VkPipelineStageFlag::MeshShaderExt`]
///  - If the `task_shader` feature is not enabled, `src_stage_mask` must not contain
///    [`VkPipelineStageFlag::TaskShaderExt`]
///  - If neither of the `shading_rate_image` or the `attachment_fragment_shading_rate` features
///    are enabled, `src_stage_mask` must not contain
///    [`VkPipelineStageFlag::FragmentShadingRateAttachmentKhr`]
///  - If the `synchronization2` feature is not enabled, `src_stage_mask` must not be 0
///  - If neither the `nv_ray_tracing` extension or the `ray_tracing_pipeline` feature are enabled,
///    `src_stage_mask` must not contain [`VkPipelineStageFlag::RayTracingShaderKhr`]
///  - If the `acceleration_structure` feature is not enabled, `src_stage_mask` must not contain
///    [`VkPipelineStageFlag::AccelerationStructureBuildKhr`]
///  - If the `ray_query` feature is not enabled and a memory barrier `src_access_mask` includes
///    [`VkAccessFlag::AccelerationStructureReadKhr`], `src_stage_mask` must not include any of
///    the `VkPipelineStageFlag::*Shader`] stages except
///    [`VkPipelineStageFlag::RayTracingShaderKhr`]
///  - The `src_access_mask` member of each element of `memory_barriers` must only include access
///    flags that are supported by one or more of the pipeline stages in `src_stage_mask`, as
///    specified in the table of supported access types
///  - The `dst_access_mask` member of each element of `memory_barriers` must only include access
///    flags that are supported by one or more of the pipeline stages in `dst_stage_mask`, as
///    specified in the table of supported access types
///  - The `dst_access_mask` member of each element of `memory_barriers` must only include access
///    flags that are supported by one or more of the pipeline stages in `dst_stage_mask`, as
///    specified in the table of supported access types
///  - For each element of `buffer_memory_barrier`, if its `src_queue_family_index` and
///    `dst_queue_family_index` members are equal, or if its `dst_queue_family_index` is the queue
///    family index that was used to create the command pool that `command_buffer` was allocated
///    from, then its `dst_access_mask` member must only contain access flags that are supported by
///    one or more of the pipeline stages in `dst_stage_mask`, as specified in the table of
///    supported access types
///  - For each element of `image_memory_barriers`, if its `src_queue_family_index` and
///    `dst_queue_family_index` members are equal, or if its `src_queue_family_index` is the queue
///    family index that was used to create the command pool that `command_buffer` was allocated
///    from, then its `src_access_mask` member must only contain access flags that are supported by
///    one or more of the pipeline stages in `src_stage_mask`, as specified in the table of
///    supported access types
///  - For each element of `image_memory_barriers`, if its `src_queue_family_index` and
///    `dst_queue_family_index` members are equal, or if its `dst_queue_family_index` is the queue
///    family index that was used to create the command pool that `command_buffer` was allocated
///    from, then its `dst_access_mask` member must only contain access flags that are supported by
///    one or more of the pipeline stages in `dst_stage_mask`, as specified in the table of
///    supported access types
///  - If [`VkCmdPipelineBarrier`] is called within a render pass instance using a [`VkRenderPass`]
///    object, and the image member of any image memory barrier is a color resolve attachment, the
///    corresponding color attachment must be [`VK_ATTACHMENT_UNUSED`]
///  - If [`VkCmdPipelineBarrier`] is called within a render pass instance using a [`VkRenderPass`]
///    object, and the image member of any image memory barrier is a color resolve attachment, it
///    must have been created with a non-zero [`VkExternalFormatAndroid::external_format`] value
///  - If [`VkCmdPipelineBarrier`] is called within a render pass instance, the `old_layout` and
///    `new_layout` members of any image memory barrier included in this command must be equal
///  - If [`VkCmdPipelineBarrier`] is called within a render pass instance, the
///    `src_queue_family_index` and `dst_queue_family_index` members of any memory barrier included
///    in this command must be equal
///  - If [`VkCmdPipelineBarrier`] is called within a render pass instance using a [`VkRenderPass`]
///    object, the render pass must have been created with at least one subpass dependency that
///    expresses a dependency from the current subpass to itself, does not include
///    [`VkDependencyFlag::ByRegion`] if this command does not, does not include
///    [`VkDependencyFlag::ViewLocal`] if this command does not, and has synchronization scopes
///    and access scopes that are all supersets of the scopes defined in this command
///  - If [`VkCmdPipelineBarrier`] is called within a render pass instance using a [`VkRenderPass`]
///    object, it must not include any buffer memory barriers
///  - If [`VkCmdPipelineBarrier`] is called within a render pass instance using a [`VkRenderPass`]
///    object, the image member of any image memory barrier included in this command must be an
///    attachment used in the current subpass both as an input attachment, and as either a color,
///    color resolve, or depth/stencil attachment
///  - If [`VkCmdPipelineBarrier`] is called within a render pass instance, and the source stage
///    masks of any memory barriers include framebuffer-space stages, destination stage masks of
///    all memory barriers must only include framebuffer-space stages
///  - If [`VkCmdPipelineBarrier`] is called within a render pass instance, and the source stage
///    masks of any memory barriers include framebuffer-space stages, then `dependency_flags` must
///    include [`VkDependencyFlag::ByRegion`]
///  - If [`VkCmdPipelineBarrier`] is called within a render pass instance, the source and
///    destination stage masks of any memory barriers must only include graphics pipeline stages
///  - If [`VkCmdPipelineBarrier`] is called outside of a render pass instance, the dependency
///    flags must not include [`VkDependencyFlag::ViewLocal`]
///  - If [`VkCmdPipelineBarrier`] is called inside a render pass instance, and there is more than
///    one view in the current subpass, dependency flags must include
///    [`VkDependencyFlag::ViewLocal`]
///  - If none of the `shader_tile_image_color_read_access`,
///    `shader_tile_image_stencil_read_access`, or `shader_tile_image_depth_read_access` features
///    are enabled, and the `dynamic_rendering_local_read` feature is not enabled,
///    [`VkCmdPipelineBarrier`] must not be called within a render pass instance started with
///    [`VkCmdBeginRendering`]
///  - If the `dynamic_rendering_local_read` feature is not enabled, and [`VkCmdPipelineBarrier`]
///    is called within a render pass instance started with [`VkCmdBeginRendering`], there must be
///    no buffer or image memory barriers specified by this command
///  - If the `dynamic_rendering_local_read` feature is not enabled, and [`VkCmdPipelineBarrier`]
///    is called within a render pass instance started with [`VkCmdBeginRendering`], memory
///    barriers specified by this command must only include
///    [`VkAccessFlag2::ColorAttachmentRead`], [`VkAccessFlag2::COLOR_ATTACHMENT_WRITE_BIT`],
///    [`VkAccessFlag2::DepthStencilAttachmentRead`], or
///    [`VkAccessFlag2::DepthStencilAttachmentWrite`] in their access masks
///  - If [`VkCmdPipelineBarrier`] is called within a render pass instance started with
///    [`VkCmdBeginRendering`], [`VkImageLayout::RenderingLocalRead`] is used as an
///    attachment in the current render pass instance, it must be in the
///    [`VkImageLayout::RenderingLocalRead`] or [`VkImageLayout::General`] layout
///  - If [`VkCmdPipelineBarrier`] is called within a render pass instance started with
///    [`VkCmdBeginRendering`], this command must only specify framebuffer-space stages in
///    `src_stage_mask` and `dst_stage_mask`
///  - If called within a render pass instance using a [`VkRenderPass`] object, the `old_layout`
///    member of any image memory barrier included in this command must be equal to the layout that
///    the corresponding attachment uses during the subpass
///  - If called within a render pass instance started with [`VkCmdBeginRendering`], the
///    `old_layout` member of any image memory barrier included in this command must be equal to
///    the layout that the corresponding attachment uses during the render pass instance
///  - Any pipeline stage included in `src_stage_mask` must be supported by the capabilities of the
///    queue family specified by the `queue_family_index` member of the [`VkCommandPoolCreateInfo`]
///    structure that was used to create the [`VkCommandPool`] that `command_buffer` was allocated
///    from, as specified in the table of supported pipeline stages
///  - Any pipeline stage included in `dst_stage_mask` must be supported by the capabilities of the
///    queue family specified by the `queue_family_index` member of the [`VkCommandPoolCreateInfo`]
///    structure that was used to create the [`VkCommandPool`] that `command_buffer` was allocated
///    from, as specified in the table of supported pipeline stages
///  - If either `src_stage_mask` or `dst_stage_mask` includes [`VkPipelineStageFlag::Host`],
///    for each element of `image_memory_barriers`, `src_queue_family_index` and
///    `dst_queue_family_index` must be equal
///  - If either `src_stage_mask` or `dst_stage_mask` includes [`VkPipelineStageFlag::Host`],
///    for each element of `buffer_memory_barrier`, `src_queue_family_index` and
///    `dst_queue_family_index` must be equal
///  - If a buffer or image memory barrier specifies a queue family ownership transfer operation,
///    either the `src_queue_family_index` or `dst_queue_family_index` member and the queue family
///    index that was used to create the command pool that `command_buffer` was allocated from must
///    be equal
///  - If the maintenance8 feature is not enabled, `dependency_flags` must not include
///    [`VkDependencyFlag::QueueFamilyOwnershipTransferUseAllStagesKhr`]
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `src_stage_mask` must be a valid combination of [`VkPipelineStageFlag`]s values
///  - `dst_stage_mask` must be a valid combination of [`VkPipelineStageFlag`]s values
///  - `dependency_flags` must be a valid combination of [`VkDependencyFlag`]s values
///  - If `memory_barrier_count` is not 0, `memory_barriers` must be a valid pointer to an array of
///    `memory_barrier_count` valid [`VkMemoryBarrier`] structures
///  - If `buffer_memory_barrier_count` is not 0, `buffer_memory_barrier` must be a valid pointer
///    to an array of `buffer_memory_barrier_count` valid [`VkBufferMemoryBarrier`] structures
///  - If `image_memory_barrier_count` is not 0, `image_memory_barriers` must be a valid pointer to
///    an array of `image_memory_barrier_count` valid [`VkImageMemoryBarrier`] structures
///  - `command_buffer` must be in the recording state
///  - The [`VkCommandPool`] that `command_buffer` was allocated from must support
///    [`VkQueueFlag::Compute`], [`VkQueueFlag::Graphics`], [`VkQueueFlag::Transfer`],
///    [`VkQueueFlag::VideoDecodeKhr`], or [`VkQueueFlag::VideoEncodeKhr`] operations
///  - This command must not be called between suspended render pass instances
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCmdPipelineBarrier = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    src_stage_mask: VkPipelineStageFlags,
    dst_stage_mask: VkPipelineStageFlags,
    dependency_flags: VkDependencyFlags,
    memory_barrier_count: u32,
    memory_barriers: *const VkMemoryBarrier,
    buffer_memory_barrier_count: u32,
    buffer_memory_barriers: *const VkBufferMemoryBarrier,
    image_memory_barrier_count: u32,
    image_memory_barriers: *const VkImageMemoryBarrier,
);

/// The name of [`VkCmdPipelineBarrier`]
pub const VK_CMD_PIPELINE_BARRIER: &CStr = c"[`VkCmdPipelineBarrier`]";
