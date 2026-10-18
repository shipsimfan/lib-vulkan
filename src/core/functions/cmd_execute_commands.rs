use crate::VkCommandBuffer;
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_FALSE, VK_NULL_HANDLE, VK_TRUE, VK_VERSION_1_0, VkBeginCommandBuffer, VkCmdBeginRendering,
    VkCommandBufferBeginInfo, VkCommandBufferLevel, VkCommandBufferUsageFlag, VkCommandPool,
    VkDevice, VkFormat, VkFramebuffer, VkQueueFlag, VkRenderingInfo, VkSampleCountFlag,
};
#[allow(unused_imports)]
use std::ptr::null;

/// Execute a secondary command buffer from a primary command buffer
///
/// # Parameters
///  - `command_buffer` is a handle to a primary command buffer that the secondary command buffers
///    are executed in.
///  - `command_buffer_count` is the length of the `command_buffers` array.
///  - `command_buffers` is a pointer to an array of `command_buffer_count` secondary command
///    buffer handles, which are recorded to execute in the primary command buffer in the order
///    they are listed in the array.
///
/// # Description
/// If any element of `command_buffers` was not recorded with the
/// [`VkCommandBufferUsageFlag::SimultaneousUse`] flag, and it was recorded into any other primary
/// command buffer which is currently in the executable or recording state, that primary command
/// buffer becomes invalid.
///
/// If the `nested_command_buffer` feature is enabled it is valid usage for
/// [`VkCmdExecuteCommands`] to also be recorded to a secondary command buffer.
///
/// # Valid Usage
///  - Each element of `command_buffers` must have been allocated with a level of
///    [`VkCommandBufferLevel::Secondary`]
///  - Each element of `command_buffers` must be in the pending or executable state
///  - If any element of `command_buffers` was not recorded with the
///    [`VkCommandBufferUsageFlag::SimultaneousUse`] flag, it must not be in the pending state
///  - If any element of `command_buffers` was not recorded with the
///    [`VkCommandBufferUsageFlag::SimultaneousUse`] flag, it must not have already been recorded
///    to `command_buffer`
///  - If any element of `command_buffers` was not recorded with the
///    [`VkCommandBufferUsageFlag::SimultaneousUse`] flag, it must not appear more than once in
///    `command_buffers`
///  - Each element of `command_buffers` must have been allocated from a [`VkCommandPool`] that was
///    created for the same queue family as the [`VkCommandPool`] from which `command_buffer` was
///    allocated
///  - If this command is called within a render pass instance, each element of `command_buffers`
///    must have been recorded with the [`VkCommandBufferUsageFlag::RenderPassContinue`] flag
///  - If this command is called within a render pass instance, and any element of
///    `command_buffers` was recorded with [`[`VkCommandBufferInheritanceInfo`]::framebuffer`] not
///    equal to [`VK_NULL_HANDLE`], that [`VkFramebuffer`] must match the [`VkFramebuffer`] used in
///    the current render pass instance
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRenderPass`],
///    and [`VkCmdNextSubpass`] has not been called in the current render pass instance, the
///    contents parameter of [`VkCmdBeginRenderPass`] must have been
///    [`VkSubpassContents::SecondaryCommandBuffer`], or
///    [`VkSubpassContents::InlineAndSecondaryCommandBuffersExt`]
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRenderPass`],
///    and [`VkCmdNextSubpass`] has been called in the current render pass instance, the contents
///    parameter of the last call to [`VkCmdNextSubpass`] must have been
///    [`VkSubpassContents::SecondaryCommandBuffer`], or
///    [`VkSubpassContents::InlineAndSecondaryCommandBuffersKhr`]
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRenderPass`],
///    each element of `command_buffers` must have been recorded with
///    [`[`VkCommandBufferInheritanceInfo`]::subpass`] set to the index of the subpass which the given
///    command buffer will be executed in
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRenderPass`],
///    the render passes specified in the `begin_info.inheritance_info.render_pass` members of the
///    [`VkBeginCommandBuffer`] commands used to begin recording each element of `command_buffers`
///    must be compatible with the current render pass
///  - If this command is called within a render pass instance that included
///    [`VkRenderPassTransformBeginInfoQcom`] in the `next` chain of [`VkRenderPassBeginInfo`],
///    then each element of `command_buffers` must have been recorded with
///    [`VkCommandBufferInheritanceRenderPassTransformInfoQcom`] in the `next` chain of
///    [`VkCommandBufferBeginInfo`]
///  - If this command is called within a render pass instance that included
///    [`VkRenderPassTransformBeginInfoQcom`] in the `next` chain of [`VkRenderPassBeginInfo`],
///    then each element of `command_buffers` must have been recorded with
///    [`VkCommandBufferInheritanceRenderPassTransformInfoQcom::transform`] identical to
///    [`VkRenderPassTransformBeginInfoQcom::transform`]
///  - If this command is called within a render pass instance that included
///    [`VkRenderPassTransformBeginInfoQcom`] in the `next` chain of [`VkRenderPassBeginInfo`],
///    then each element of `command_buffers` must have been recorded with
///    [`VkCommandBufferInheritanceRenderPassTransformInfoQcom::render_area`] identical to
///    [`VkRenderPassBeginInfo::render_area`]
///  - If [`VkCmdExecuteCommands`] is not being called within a render pass instance, each element
///    of `command_buffers` must not have been recorded with the
///    [`VkCommandBufferUsageFlag::RenderPassContinue`]
///  - If the `inherited_queries` feature is not enabled, `command_buffer` must not have any
///    queries active
///  - If `command_buffer` has a [`VkQueryTpe::Occlusion`] query active, then each element of
///    `command_buffers` must have been recorded with
///    [`[`VkCommandBufferInheritanceInfo`]::occlusion_query_enable`] set to [`VK_TRUE`]
///  - If `command_buffer` has a [`VkQueryTpe::Occlusion`] query active, then each element of
///    `command_buffers` must have been recorded with
///    [`[`VkCommandBufferInheritanceInfo`]::query_flags`] having all bits set that are set for the
///    query
///  - If `command_buffer` has a [`VkQueryTpe::PipelineStatistics`] query active, then each element
///    of `command_buffers` must have been recorded with
///    [`[`VkCommandBufferInheritanceInfo`]::pipeline_statistics`] having all bits set that are set in
///    the [`VkQueryPool`] the query uses
///  - Each element of `command_buffers` must not begin any query types that are active in
///    `command_buffer`
///  - `command_buffer` must not have any queries other than [`VkQueryTpe::Occlusion`] and
///    [`VkQueryTpe::PipelineStatistics`] active
///  - If `command_buffer` is a protected command buffer and `protected_no_fault` is not supported,
///    each element of `command_buffers` must be a protected command buffer
///  - If `command_buffer` is an unprotected command buffer and `protected_no_fault` is not
///    supported, each element of `command_buffers` must be an unprotected command buffer
///  - This command must not be recorded when transform feedback is active
///  - If this command is called within a render pass instance and any recorded command in
///    `command_buffer` in the current subpass will write to an image subresource as an attachment,
///    commands recorded in elements of `command_buffers` must not read from the memory backing
///    that image subresource in any other way
///  - If this command is called within a render pass instance and any recorded command in
///    `command_buffer` in the current subpass will read from an image subresource used as an
///    attachment in any way other than as an attachment, commands recorded in elements of
///    `command_buffers` must not write to that image subresource as an attachment
///  - If this command is called within a render pass instance and any recorded command in a given
///    element of `command_buffers` will write to an image subresource as an attachment, commands
///    recorded in elements of `command_buffers` at a higher index must not read from the memory
///    backing that image subresource in any other way
///  - If this command is called within a render pass instance and any recorded command in a given
///    element of `command_buffers` will read from an image subresource used as an attachment in
///    any way other than as an attachment, commands recorded in elements of `command_buffers` at a
///    higher index must not write to that image subresource as an attachment
///  - If `command_buffers` contains any suspended render pass instances, there must be no action
///    or synchronization commands between that render pass instance and any render pass instance
///    that resumes it
///  - If `command_buffers` contains any suspended render pass instances, there must be no render
///    pass instances between that render pass instance and any render pass instance that resumes
///    it
///  - If the `variable_sample_locations` limit is not supported, and any element of
///    s`command_buffers` contains any suspended render pass instances, where a graphics pipeline
///    has been bound, any pipelines bound in the render pass instance that resumes it, or any
///    subsequent render pass instances that resume from that one and so on, must use the same
///    sample locations
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`],
///    its [`VkRenderingInfo::flags`] parameter must have included
///    [`VkRenderingContentsFlag::SecondaryCommandBuffers`]
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`],
///    the render passes specified in the `begin_info.inheritance_info.render_pass` members of the
///    [`VkBeginCommandBuffer`] commands used to begin recording each element of `command_buffers`
///    must be [`VK_NULL_HANDLE`]
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`],
///    the flags member of the [`VkCommandBufferInheritanceRenderingInfo`] structure included in
///    the `next` chain of [`VkCommandBufferBeginInfo::inheritance_info`] used to begin recording
///    each element of `command_buffers` must be equal to the [`VkRenderingInfo::flags`] parameter
///    to [`VkCmdBeginRendering`], excluding [`VkRenderingContentsFlag::SecondaryCommandBuffers`]
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`],
///    the `color_attachment_count` member of the [`VkCommandBufferInheritanceRenderingInfo`]
///    structure included in the `next` chain of [`VkCommandBufferBeginInfo::inheritance_info`]
///    used to begin recording each element of `command_buffers` must be equal to the
///    [`VkRenderingInfo::color_attachment_count`] parameter to [`VkCmdBeginRendering`]
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`],
///    if the `image_view` member of an element of the [`VkRenderingInfo::color_attachments`]
///    parameter to [`VkCmdBeginRendering`] is not [`VK_NULL_HANDLE`], the corresponding element of
///    the `color_attachment_formats` member of the [`VkCommandBufferInheritanceRenderingInfo`]
///    structure included in the `next` chain of [`VkCommandBufferBeginInfo::inheritance_info`]
///    used to begin recording each element of `command_buffers` must be equal to the format used
///    to create that image view
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`],
///    if the `image_view` member of an element of the [`VkRenderingInfo::color_attachments`]
///    parameter to [`VkCmdBeginRendering`] is [`VK_NULL_HANDLE`], the corresponding element of the
///    `color_attachment_formats` member of the [`VkCommandBufferInheritanceRenderingInfo`]
///    structure included in the `next` chain of [`VkCommandBufferBeginInfo::inheritance_info`]
///    used to begin recording each element of `command_buffers` must be [`VkFormat::Undefined`]
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`],
///    if the [`VkRenderingInfo.depth_attachment.image_view`] parameter to [`VkCmdBeginRendering`]
///    is not [`VK_NULL_HANDLE`], the value of the `depth_attachment_format` member of the
///    [`VkCommandBufferInheritanceRenderingInfo`] structure included in the `next` chain of
///    [`VkCommandBufferBeginInfo::inheritance_info`] used to begin recording each element of
///    `command_buffers` must be equal to the format used to create that image view
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`],
///    if the [`VkRenderingInfo.stencil_attachment.image_view`] parameter to
///    [`VkCmdBeginRendering`] is not [`VK_NULL_HANDLE`], the value of the
///    `stencil_attachment_format` member of the [`VkCommandBufferInheritanceRenderingInfo`]
///    structure included in the `next` chain of [`VkCommandBufferBeginInfo::inheritance_info`]
///    used to begin recording each element of `command_buffers` must be equal to the format used
///    to create that image view
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`]
///    and the [`VkRenderingInfo.depth_attachment.image_view`] parameter to [`VkCmdBeginRendering`]
///    was [`VK_NULL_HANDLE`], the value of the `depth_attachment_format` member of the
///    [`VkCommandBufferInheritanceRenderingInfo`] structure included in the `next` chain of
///    [`VkCommandBufferBeginInfo::inheritance_info`] used to begin recording each element of
///    `command_buffers` must be [`VkFormat::Undefined`]
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`]
///    and the [`VkRenderingInfo.stencil_attachment.image_view`] parameter to
///    [`VkCmdBeginRendering`] was [`VK_NULL_HANDLE`], the value of the `stencil_attachment_format`
///    member of the [`VkCommandBufferInheritanceRenderingInfo`] structure included in the `next`
///    chain of [`VkCommandBufferBeginInfo::inheritance_info`] used to begin recording each element
///    of `command_buffers` must be [`VkFormat::Undefined`]
///  - If the current render pass instance was begun with [`VkCmdBeginRendering`] and contains a
///    custom resolve, then each element of `command_buffers` must have been recorded with a
///    [`VkCustomResolveCreateInfoExt`] struct chained to its [`[`VkCommandBufferInheritanceInfo`]`]
///  - If the current render pass instance was begun with [`VkCmdBeginRendering`] and does not
///    contain a custom resolve, then each element of `command_buffers` must not have been
///    recorded with a [`VkCustomResolveCreateInfoExt`] struct chained to its
///    [`VkCommandBufferInheritanceInfo`]
///  - If the current render pass instance was begun with [`VkCmdBeginRendering`] and
///    [`VkCmdBeginCustomResolveExt`] has been recorded in the render pass instance, then each
///    element of `command_buffers` must have been recorded with
///    [`VkCustomResolveCreateInfoExt::custom_resolve`] as [`VK_TRUE`]
///  - If the current render pass instance was begun with [`VkCmdBeginRendering`] and contains a
///    custom resolve, and [`VkCmdBeginCustomResolveExt`] has not been recorded in the render pass
///    instance, then each element of `command_buffers` must have been recorded with
///    [`VkCustomResolveCreateInfoExt::custom_resolve`] as [`VK_FALSE`]
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`]
///    that contains a custom resolve, the `color_attachment_count` member of the
///    [`VkCustomResolveCreateInfoExt`] structure included in the `next` chain of
///    [`VkCommandBufferBeginInfo::inheritance_info`] used to begin recording each element of
///    `command_buffers` must be equal to the [`VkRenderingInfo::color_attachment_count`] parameter
///    to [`VkCmdBeginRendering`]
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`]
///    that contains a custom resolve, if the resolveImageView member of an element of the
///    [`VkRenderingInfo::color_attachments`] parameter to [`VkCmdBeginRendering`] is not
///    [`VK_NULL_HANDLE`], the corresponding element of the `color_attachment_formats` member of
///    the [`VkCustomResolveCreateInfoExt`] structure included in the `next` chain of
///    [`VkCommandBufferBeginInfo::inheritance_info`] used to begin recording each element of
///    `command_buffers` must be equal to the format used to create that image view
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`]
///    that contains a custom resolve, if the resolveImageView member of an element of the
///    [`VkRenderingInfo::color_attachments`] parameter to [`VkCmdBeginRendering`] is
///    [`VK_NULL_HANDLE`], the corresponding element of the `color_attachment_formats` member of
///    the [`VkCustomResolveCreateInfoExt`] structure included in the `next` chain of
///    [`VkCommandBufferBeginInfo::inheritance_info`] used to begin recording each element of
///    `command_buffers` must be [`VkFormat::Undefined`]
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`],
///    if the [`VkRenderingInfo.depth_attachment.image_view`] parameter to [`VkCmdBeginRendering`]
///    is not [`VK_NULL_HANDLE`], the value of the `depth_attachment_format` member of the
///    [`VkCommandBufferInheritanceRenderingInfo`] structure included in the `next` chain of
///    [`VkCommandBufferBeginInfo::inheritance_info`] used to begin recording each element of
///    `command_buffers` must be equal to the format used to create that image view
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`]
///    that contains a custom resolve, if the
///    [`VkRenderingInfo::stencil_attachment.resolve_image_view`] parameter to
///    [`VkCmdBeginRendering`] is not [`VK_NULL_HANDLE`], the value of the
///    `stencil_attachment_format` member of the [`VkCustomResolveCreateInfoExt`] structure
///    included in the `next` chain of [`VkCommandBufferBeginInfo::inheritance_info`] used to begin
///    recording each element of `command_buffers` must be equal to the format used to create that
///    image view
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`]
///    that contains a custom resolve and the
///    [`VkRenderingInfo::depth_attachment.resolve_image_view`] parameter to
///    [`VkCmdBeginRendering`] was [`VK_NULL_HANDLE`], the value of the `depth_attachment_format`
///    member of the [`VkCustomResolveCreateInfoExt`] structure included in the `next` chain of
///    [`VkCommandBufferBeginInfo::inheritance_info`] used to begin recording each element of
///    `command_buffers` must be [`VkFormat::Undefined`]
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`]
///    that contains a custom resolve and the
///    [`VkRenderingInfo::stencil_attachment.resolve_image_view`] parameter to
///    [`VkCmdBeginRendering`] was [`VK_NULL_HANDLE`], the value of the `stencil_attachment_format`
///    member of the [`VkCustomResolveCreateInfoExt`] structure included in the `next` chain of
///    [`VkCommandBufferBeginInfo::inheritance_info`] used to begin recording each element of
///    `command_buffers` must be [`VkFormat::Undefined`]
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`]
///    that contains a custom resolve and the
///    [`VkRenderingInfo::stencil_attachment.resolve_image_view`] parameter to
///    [`VkCmdBeginRendering`] was [`VK_NULL_HANDLE`], the value of the `stencil_attachment_format`
///    member of the [`VkCustomResolveCreateInfoExt`] structure included in the `next` chain of
///    [`VkCommandBufferBeginInfo::inheritance_info`] used to begin recording each element of
///    `command_buffers` must be [`VkFormat::Undefined`]
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`],
///    [`VkCmdBeginCustomResolveExt`] has been recorded in the render pass instance, and the
///    [`VkRenderingInfo::depth_attachment.resolve_image_view`] parameter to
///    [`VkCmdBeginRendering`] is not [`VK_NULL_HANDLE`], the value of
///    [`VkCommandBufferInheritanceRenderingInfo::rasterization_samples`] must be equal to the
///    sample count used to create that image view
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`],
///    [`VkCmdBeginCustomResolveExt`] has been recorded in the render pass instance, and the
///    [`VkRenderingInfo::stencil_attachment.resolve_image_view`] parameter to
///    [`VkCmdBeginRendering`] is not [`VK_NULL_HANDLE`], the value of
///    [`VkCommandBufferInheritanceRenderingInfo::rasterization_samples`] must be equal to the
///    sample count used to create that image view
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`],
///    the viewMask member of the [`VkCommandBufferInheritanceRenderingInfo`] structure included in
///    the `next` chain of [`VkCommandBufferBeginInfo::inheritance_info`] used to begin recording
///    each element of `command_buffers` must be equal to the [`VkRenderingInfo::view_mask`]
///    parameter to [`VkCmdBeginRendering`]
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`]
///    and the `next` chain of [`VkCommandBufferInheritanceInfo`] includes a
///    [`VkAttachmentSampleCountInfoAmd`] or [`VkAttachmentSampleCountInfoNv`] structure, if the
///    `image_view` member of an element of the [`VkRenderingInfo::color_attachments`] parameter to
///    [`VkCmdBeginRendering`] is not [`VK_NULL_HANDLE`], the corresponding element of the
///    `color_attachment_samples` member of the [`VkAttachmentSampleCountInfoAmd`] or
///    [`VkAttachmentSampleCountInfoNv`] structure included in the `next` chain of
///    [`VkCommandBufferBeginInfo::inheritance_info`] used to begin recording each element of
///    `command_buffers` must be equal to the sample count used to create that image view
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`]
///    and the `next` chain of [`VkCommandBufferInheritanceInfo`] includes a
///    [`VkAttachmentSampleCountInfoAmd`] or [`VkAttachmentSampleCountInfoNv`] structure, if the
///    [`VkRenderingInfo.depth_attachment.image_view`] parameter to [`VkCmdBeginRendering`] is not
///    [`VK_NULL_HANDLE`], the value of the `depth_stencil_attachment_samples` member of the
///    [`VkAttachmentSampleCountInfoAmd`] or [`VkAttachmentSampleCountInfoNv`] structure included
///    in the `next` chain of [`VkCommandBufferBeginInfo::inheritance_info`] used to begin
///    recording each element of `command_buffers` must be equal to the sample count used to create
///    that image view
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`]
///    and the `next` chain of [`VkCommandBufferInheritanceInfo`] includes a
///    [`VkAttachmentSampleCountInfoAmd`] or [`VkAttachmentSampleCountInfoNv`] structure, if the
///    [`VkRenderingInfo.stencil_attachment.image_view`] parameter to [`VkCmdBeginRendering`] is
///    not [`VK_NULL_HANDLE`], the value of the `depth_stencil_attachment_samples` member of the
///    [`VkAttachmentSampleCountInfoAmd`] or [`VkAttachmentSampleCountInfoNv`] structure included
///    in the `next` chain of [`VkCommandBufferBeginInfo::inheritance_info`] used to begin
///    recording each element of `command_buffers` must be equal to the sample count used to create
///    that image view
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`],
///    and the `next` chain of [`VkCommandBufferInheritanceInfo`] does not include a
///    [`VkAttachmentSampleCountInfoAmd`] or [`VkAttachmentSampleCountInfoNv`] structure,
///    [`VkCmdBeginCustomResolveExt`] has not yet been recorded in the render pass instance, and if
///    the `image_view` member of an element of the [`VkRenderingInfo::color_attachments`]
///    parameter to [`VkCmdBeginRendering`] is not [`VK_NULL_HANDLE`], the value of
///    [`VkCommandBufferInheritanceRenderingInfo::rasterization_samples`] must be equal to the
///    sample count used to create that image view
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`]
///    and the `next` chain of [`VkCommandBufferInheritanceInfo`] does not include a
///    [`VkAttachmentSampleCountInfoAmd`] or [`VkAttachmentSampleCountInfoNv`] structure,
///    [`VkCmdBeginCustomResolveExt`] has not yet been recorded in the render pass instance, and if
///    the [`VkRenderingInfo.depth_attachment.image_view`] parameter to [`VkCmdBeginRendering`] is
///    not [`VK_NULL_HANDLE`], the value of
///    [`VkCommandBufferInheritanceRenderingInfo::rasterization_samples`] must be equal to the
///    sample count used to create that image view
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`]
///    and the `next` chain of [`VkCommandBufferInheritanceInfo`] does not include a
///    [`VkAttachmentSampleCountInfoAmd`] or [`VkAttachmentSampleCountInfoNv`] structure,
///    [`VkCmdBeginCustomResolveExt`] has not yet been recorded in the render pass instance, and if
///    the [`VkRenderingInfo.stencil_attachment.image_view`] parameter to [`VkCmdBeginRendering`] is
///    not [`VK_NULL_HANDLE`], the value of
///    [`VkCommandBufferInheritanceRenderingInfo::rasterization_samples`] must be equal to the
///    sample count used to create that image view
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`],
///    with any color attachment using a resolve mode of
///    [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`], the `next` chain of
///    [`VkCommandBufferInheritanceInfo`] used to create each element of `command_buffers` must
///    include a [`VkExternalFormatAndroid`] structure with an `external_format` matching that used
///    to create the resolve attachment in the render pass
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`]
///    with any color attachment using a resolve mode of
///    [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`], and the `next` chain of
///    [`VkCommandBufferInheritanceInfo`] does not include a [`VkAttachmentSampleCountInfoAmd`] or
///    [`VkAttachmentSampleCountInfoNv`] structure, the value of
///    [`VkCommandBufferInheritanceRenderingInfo::rasterization_samples`] must be
///    [`VkSampleCountFlag::_1`]
///  - `command_buffer` must not be a secondary command buffer unless the `nested_command_buffer`
///    feature is enabled
///  - If the `nested_command_buffer` feature is enabled, and `command_buffer` is a secondary
///    command buffer, the command buffer nesting level of each element of `command_buffers` must
///    be less than `max_command_buffer_nesting_level`
///  - If the `nested_command_buffer_rendering` feature is not enabled, and `command_buffer` is a
///    secondary command buffer, `command_buffer` must not have been recorded with
///    [`VkCommandBufferUsageFlag::RenderPassContinue`]
///  - If the `nested_command_buffer`SimultaneousUse feature is not enabled, and `command_buffer`
///    is a secondary command buffer, each element of `command_buffers` must not have been recorded
///    with [`VkCommandBufferUsageFlag::SimultaneousUse`]
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`],
///    the color attachment mapping state specified by [`VkRenderingAttachmentLocationInfo`] in the
///    inheritance info of each element of `command_buffers` and in the current state of
///    `command_buffer` must match
///  - If this command is called within a render pass instance begun with [`VkCmdBeginRendering`],
///    the input attachment mapping state specified by [`VkRenderingInputAttachmentIndexInfo`] in
///    the inheritance info of each element of `command_buffers` and in the current state of
///    `command_buffer` must match
///  - If this command is called within a render pass instance, the memory member of the
///    [`VkTileMemoryBindInfoQcom`] structure included in the `next` chain of
///    [`VkCommandBufferBeginInfo::inheritance_info`] used to begin recording each element of
///    `command_buffers` must be equal to the active bound tile memory object in `command_buffer`
///  - If this command is being recorded within a render pass instance with tile shading enabled,
///    all elements of `command_buffers` must have been recorded with
///    [`VkTileShadingFlag::RenderPassEnableQcom`] included in
///    [`VkRenderPassTileShadingCreateInfoQcom::flags`]
///  - If the per-tile execution model is enabled, all elements of `command_buffers` must have been
///    recorded with [`VkTileShadingFlag::RenderPassPerTileExecutionQcom`] included in
///    [`VkRenderPassTileShadingCreateInfoQcom::flags`]
///  - If this command is being recorded within a render pass instance, the `tile_apron_size` used
///    to create the render pass instance must equal the
///    [`VkRenderPassTileShadingCreateInfoQcom::tile_apron_size`] used to record all elements of
///    `command_buffers`
///  - If any element of `command_buffers` was recorded with
///    [`VkTileShadingFlag::RenderPassEnableQcom`] included in
///    [`VkRenderPassTileShadingCreateInfoQcom::flags`], this command must be recorded in a render
///    pass that has tile shading enabled
///  - If any element of `command_buffers` was recorded with
///    [`VkTileShadingFlag::RenderPassPerTileExecutionQcom`] included in
///    [`VkRenderPassTileShadingCreateInfoQcom::flags, per-tile execution model must be enabled
///  - If this command is not being recorded into a render pass instance, the
///    [`VkRenderPassTileShadingCreateInfoQcom::tile_apron_size`] that was recorded into all
///    elements of `command_buffers` must equal (0,0)
///  - If there is a sampler descriptor heap bound to `command_buffer`, each element of
///    `command_buffers` must have been recorded with a value of
///    [`VkCommandBufferInheritanceDescriptorHeapInfoExt::sampler_heap_bind_info`] that is either
///    [`null`] or a pointer to a bind info that is identical to that set via the last call to
///    [`VkCmdBindSamplerHeapExt`]
///  - If there is a resource descriptor heap bound to `command_buffer`, each element of
///    `command_buffers` must have been recorded with a value of
///    [`VkCommandBufferInheritanceDescriptorHeapInfoExt::resource_heap_bind_info`] that is either
///    [`null`] or a pointer to a bind info that is identical to that set via the last call to
///    [`VkCmdBindResourceHeapExts`]
///  - If there is no sampler descriptor heap bound to `command_buffer`, each element of
///    `command_buffers` must have been recorded with a value of
///    [`VkCommandBufferInheritanceDescriptorHeapInfoExt::sampler_heap_bind_info`] set to [`null`]
///  - If there is no resource descriptor heap bound to `command_buffer`, each element of
///    `command_buffers` must have been recorded with a value of
///    [`VkCommandBufferInheritanceDescriptorHeapInfoExt::resource_heap_bind_info`] set to [`null`]
///  - `command_buffer` must not have any shader instrumentation active
///
/// # Valid Usage (Implicit)
///  - `command_buffer` must be a valid [`VkCommandBuffer`] handle
///  - `command_buffers` must be a valid pointer to an array of `command_buffer_count` valid
///    [`VkCommandBuffer`] handles
///  - `command_buffer` must be in the recording state
///  - The [`VkCommandPool`] that `command_buffer` was allocated from must support
///    [`VkQueueFlag::Compute`], [`VkQueueFlag::Graphics`], or [`VkQueueFlag::Transfer`] operations
///  - This command must only be called outside of a video coding scope
///  - `command_buffer_count` must be greater than 0
///  - Both of `command_buffer`, and the elements of `command_buffers` must have been created,
///    allocated, or retrieved from the same [`VkDevice`]
///
/// # Host Synchronization
///  - Host access to `command_buffer` must be externally synchronized
///  - Host access to the [`VkCommandPool`] that `command_buffer` was allocated from must be
///    externally synchronized
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkCmdExecuteCommands = unsafe extern "system" fn(
    command_buffer: VkCommandBuffer,
    command_buffer_count: u32,
    command_buffers: *const VkCommandBuffer,
);

/// The name of [`VkCmdExecuteCommands`]
pub const VK_CMD_EXECUTE_COMMANDS: &CStr = c"vkCmdExecuteCommands";
