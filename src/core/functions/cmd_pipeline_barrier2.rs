use crate::{VkCommandBuffer, VkDependencyInfo};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_VERSION_1_3, VkAccessFlag2, VkCmdBeginRendering, VkCommandPool, VkDependencyFlag,
    VkImageLayout, VkQueueFlag, VkRenderPass,
};

/// Insert a memory dependency
///
/// # Parameters
///  - `command_buffer` is the command buffer into which the command is recorded.
///  - `dependency_info` is a pointer to a [`VkDependencyInfo`] structure defining the scopes of
///    this operation.
///
/// # Description
/// When [`VkCmdPipelineBarrier2`] is submitted to a queue, it defines memory dependencies between
/// commands that were submitted to the same queue before it, and those submitted to the same queue
/// after it.
///
/// The first synchronization scope and access scope of each memory dependency defined by
/// `dependency_info` are applied to operations that occurred earlier in submission order.
///
/// The second synchronization scope and access scope of each memory dependency defined by
/// `dependency_info` are applied to operations that occurred later in submission order.
///
/// If [`VkCmdPipelineBarrier2`] is recorded within a render pass instance, the synchronization
/// scopes are limited to a subset of operations within the same subpass or render pass instance.
///
/// # Valid Usage
///  - If [`VkCmdPipelineBarrier2`] is called within a render pass instance using a
///    [`VkRenderPass`] object, the render pass must have been created with at least one subpass
///    dependency that expresses a dependency from the current subpass to itself, does not include
///    [`VkDependencyFlag::ByRegion`] if this command does not, does not include
///    [`VkDependencyFlag::ViewLocal`] if this command does not, and has synchronization scopes and
///    access scopes that are all supersets of the scopes defined in this command
///  - If [`VkCmdPipelineBarrier2`] is called within a render pass instance using a
///    [`VkRenderPass`] object, it must not include any buffer memory barriers
///  - If [`VkCmdPipelineBarrier2`] is called within a render pass instance using a
///    [`VkRenderPass`] object, the image member of any image memory barrier included in this
///    command must be an attachment used in the current subpass both as an input attachment, and
///    as either a color, color resolve, or depth/stencil attachment
///  - If [`VkCmdPipelineBarrier2`] is called within a render pass instance, and the source stage
///    masks of any memory barriers include framebuffer-space stages, destination stage masks of
///    all memory barriers must only include framebuffer-space stages
///  - If [`VkCmdPipelineBarrier2`] is called within a render pass instance, and the source stage
///    masks of any memory barriers include framebuffer-space stages, then `dependency_flags` must
///    include [`VkDependencyFlag::ByRegion`]
///  - If [`VkCmdPipelineBarrier2`] is called within a render pass instance, the source and
///    destination stage masks of any memory barriers must only include graphics pipeline stages
///  - If [`VkCmdPipelineBarrier2`] is called outside of a render pass instance, the dependency
///    flags must not include [`VkDependencyFlag::ViewLocal`]
///  - If [`VkCmdPipelineBarrier2`] is called inside a render pass instance, and there is more than
///    one view in the current subpass, dependency flags must include
///    [`VkDependencyFlag::ViewLocal`]
///  - If none of the `shader_tile_image_color_read_access`,
///    `shader_tile_image_stencil_read_access`, or `shader_tile_image_depth_read_access` features
///    are enabled, and the `dynamic_rendering_local_read` feature is not enabled,
///    [`VkCmdPipelineBarrier2`] must not be called within a render pass instance started with
///    [`VkCmdBeginRendering`]
///  - If the `dynamic_rendering_local_read` feature is not enabled, and [`VkCmdPipelineBarrier2`]
///    is called within a render pass instance started with [`VkCmdBeginRendering`], there must be
///    no buffer or image memory barriers specified by this command
///  - If the `dynamic_rendering_local_read` feature is not enabled, and [`VkCmdPipelineBarrier2`]
///    is called within a render pass instance started with [`VkCmdBeginRendering`], memory
///    barriers specified by this command must only include [`VkAccessFlag2::ColorAttachmentRead`],
///    [`VkAccessFlag2::ColorAttachmentWrite`], [`VkAccessFlag2::DepthStencilAttachmentRead`], or
///    [`VkAccessFlag2::DepthStencilAttachmentWrite`] in their access masks
///  - If [`VkCmdPipelineBarrier2`] is called within a render pass instance started with
///    [`VkCmdBeginRendering`], and the image member of any image memory barrier is used as an
///    attachment in the current render pass instance, it must be in the
///    [`VkImageLayout::RenderingLocalRead`] or [`VkImageLayout::General`] layout
///  - If [`VkCmdPipelineBarrier2`] is called within a render pass instance started with
///    [`VkCmdBeginRendering`], this command must only specify framebuffer-space stages in
///    `src_stage_mask` and `dst_stage_mask`
///  - If [`VkCmdPipelineBarrier2`] is called within a render pass instance using a
///    [`VkRenderPass`] object, and the image member of any image memory barrier is a color resolve
///    attachment, the corresponding color attachment must be [`VK_ATTACHMENT_UNUSED`]
///  - If [`VkCmdPipelineBarrier2`] is called within a render pass instance using a
///    [`VkRenderPass`] object, and the image member of any image memory barrier is a color resolve
///    attachment, it must have been created with a non-zero
///    [`VkExternalFormatAndroid::external_format`] value
///  - If [`VkCmdPipelineBarrier2`] is called within a render pass instance, the `old_layout` and
///    `new_layout` members of any image memory barrier included in this command must be equal
///  - If [`VkCmdPipelineBarrier2`] is called within a render pass instance, the
///    `src_queue_family_index` and `dst_queue_family_index` members of any memory barrier included
///    in this command must be equal
///  - The `synchronization2` feature must be enabled
///  - The `src_stage_mask` member of any element of the `memory_barriers` member of
///    `dependency_info` must only include pipeline stages valid for the queue family that was used
///    to create the command pool that `command_buffer` was allocated from
///  - The `dst_stage_mask` member of any element of the `memory_barriers` member of
///    `dependency_info` must only include pipeline stages valid for the queue family that was used
///    to create the command pool that `command_buffer` was allocated from
///  - If a buffer or image memory barrier does not specify an acquire operation, or if it does but
///    `dependency_info.dependency_flags` includes
///    [`VkDependencyFlag::QueueFamilyOwnershipTransferUseAllStagesKhr`], the respective
///    `src_stage_mask` member of the element of the `buffer_memory_barriers` or
///    `image_memory_barriers` members of `dependency_info` must only include pipeline stages valid
///    for the queue family that was used to create the command pool that `command_buffer` was
///    allocated from
///  - If a buffer or image memory barrier does not specify an release operation, or if it does but
///    `dependency_info.dependency_flags` includes
///    [`VkDependencyFlag::QueueFamilyOwnershipTransferUseAllStagesKhr`], the respective
///    `dst_stage_mask` member of the element of the `buffer_memory_barriers` or
///    `image_memory_barriers` members of `dependency_info` must only include pipeline stages valid
///    for the queue family that was used to create the command pool that `command_buffer` was
///    allocated from
///  - If a buffer or image memory barrier specifies a queue family ownership transfer operation,
///    either the `src_queue_family_index` or `dst_queue_family_index` member of the element of the
///    `buffer_memory_barriers` or `image_memory_barriers` members of `dependency_info` and the
///    queue family index that was used to create the command pool that `command_buffer` was
///    allocated from must be equal
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `dependency_info` must be a valid pointer to a valid [`VkDependencyInfo`] structure
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
/// Provided by [`VK_VERSION_1_3`]
pub type VkCmdPipelineBarrier2 = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    dependency_info: *const VkDependencyInfo,
);

/// The name of [`VkCmdPipelineBarrier2`]
pub const VK_CMD_PIPELINE_BARRIER2: &CStr = c"vkCmdPipelineBarrier2";
