use crate::{VkCommandBuffer, VkRenderingInfo};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VK_VERSION_1_3, VkCommandBufferBeginInfo, VkCommandBufferUsageFlag,
    VkCommandPool, VkQueueFlag, VkRenderingFlag, VkResolveModeFlag,
};
#[allow(unused_imports)]
use std::ptr::null;

/// Begin a dynamic render pass instance
///
/// # Parameters
///  - `command_buffer` is the command buffer in which to record the command.
///  - `rendering_info` is a pointer to a [`VkRenderingInfo`] structure specifying details of the
///    render pass instance to begin.
///
/// # Description
/// After beginning a render pass instance, the command buffer is ready to record draw commands.
///
/// If `rendering_info.flags` includes [`VkRenderingFlag::Resuming`] then this render pass is
/// resumed from a render pass instance that has been suspended earlier in submission order.
///
/// If there is an instance of [`VkTileMemorySizeInfoQcom`] included in the `next` chain of
/// [`VkRenderingInfo`], the structure is ignored.
///
/// # Valid Usage
///  - The `dynamic_rendering` feature must be enabled
///  - If `command_buffer` is a secondary command buffer, and the `nested_command_buffer` feature
///    is not enabled, `rendering_info.flags` must not include
///    [`VkRenderingFlag::ContentsSecondaryCommandBuffers`]
///  - If `command_buffer` is a secondary command buffer,
///    [`VkCommandBufferUsageFlag::RenderPassContinue`] must not have been set in
///    [`VkCommandBufferBeginInfo::flags`] when `command_buffer` began
///  - If `rendering_info.depth_attachment` is not [`null`] and
///    `rendering_info.depth_attachment.image_view` is not [`VK_NULL_HANDLE`], when
///    `rendering_info.depth_attachment.image_view` is accessed it must be in the layout specified
///    by `rendering_info.depth_attachment.image_layout`
///  - If `rendering_info.depth_attachment` is not [`null`],
///    `rendering_info.depth_attachment.image_view` is not [`VK_NULL_HANDLE`],
///    `rendering_info.depth_attachment.resolve_mode` is not [`VkResolveModeFlag::None`], and
///    `rendering_info.depth_attachment.resolve_image_view` is not [`VK_NULL_HANDLE`],
///    `rendering_info.depth_attachment.resolve_image_view` must be in the layout specified by
///    `rendering_info.depth_attachment.resolve_image_layout`
///  - If `rendering_info.stencil_attachment` is not [`null`] and
///    `rendering_info.stencil_attachment.image_view` is not [`VK_NULL_HANDLE`], when
///    `rendering_info.stencil_attachment.image_view` is accessed it must be in the layout
///    specified by `rendering_info.stencil_attachment.image_layout`
///  - If `rendering_info.stencil_attachment` is not [`null`],
///    `rendering_info.stencil_attachment.image_view` is not [`VK_NULL_HANDLE`],
///    `rendering_info.stencil_attachment.resolve_mode` is not [`VkResolveModeFlag::None`], and
///    `rendering_info.stencil_attachment.resolve_image_view` is not [`VK_NULL_HANDLE`],
///    `rendering_info.stencil_attachment.resolve_image_view` must be in the layout specified by
///    `rendering_info.stencil_attachment.resolve_image_layout`
///  - For each element of `rendering_info.color_attachments`, if `image_view` is not
///    [`VK_NULL_HANDLE`], when that image view is accessed it must be in the layout specified by
///    the image_layout member of that same element of `rendering_info.color_attachments`
///  - For each element of `rendering_info.color_attachments`, if either resolve_mode is
///    [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`], or `image_view` is not
///    [`VK_NULL_HANDLE`] and `resolve_mode` is not [`VkResolveModeFlag::None`], and
///    `resolve_image_view` is not [`VK_NULL_HANDLE`], `resolve_image_view` must be in the layout
///    specified by `resolve_image_layout`
///  - If [`VkTileShadingRenderPassFlagQcom::EnableQcom`] is included in
///    [`VkRenderPassTileShadingCreateInfoQcom::flags`], `command_buffer` must not have been
///    recorded with [`VkCommandBufferUsageFlag::SimultaneousUse`]
///  - [`VkRenderPassTileShadingCreateInfoQcom::flags`] must not include
///    [`VkTileShadingRenderPassFlagQcom::PerTileExecutionQcom`]
///  - If `rendering_info.flags` contains
///    [`VkRenderingFlag::LocalReadConcurrentAccessControlKhr`], `maintenance10` must be enabled
///  - If `rendering_info.flags` does not contain
///    [`VkRenderingFlag::LocalReadConcurrentAccessControlKhr`], attachments must not specify
///    [`VkRenderingAttachmentFlagKhr::InputAttachmentFeedbackKhr`]
///  - If [`VkRenderingFragmentDensityMapAttachmentInfoExt::image_view`] is not equal to
///    [`VK_NULL_HANDLE`], when `image_view` is accessed it must be in the layout specified by
///    [`VkRenderingFragmentDensityMapAttachmentInfoExt::image_layout`]
///  - If [`VkRenderingFragmentShadingRateAttachmentInfoKhr::image_view`] is not equal to
///    [`VK_NULL_HANDLE`], when `image_view` is accessed it must be in the layout specified by
///    [`VkRenderingFragmentShadingRateAttachmentInfoKhr::image_layout`]
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `rendering_info` must be a valid pointer to a valid [`VkRenderingInfo`] structure
///  - `command_buffer` must be in the recording state
///  - The [`VkCommandPool`] that `command_buffer` was allocated from must support
///    [`VkQueueFlag::Graphics`] operations
///  - This command must only be called outside of a render pass instance
///  - This command must not be called between suspended render pass instances
///  - This command must only be called outside of a video coding scope
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// Provided by [`VK_VERSION_1_3`]
pub type VkCmdBeginRendering = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    rendering_info: *const VkRenderingInfo,
);

/// The name of [`VkCmdBeginRendering`]
pub const VK_CMD_BEGIN_RENDERING: &CStr = c"vkCmdBeginRendering";
