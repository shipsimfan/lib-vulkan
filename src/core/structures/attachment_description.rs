use crate::{
    VkAttachmentDescriptionFlags, VkAttachmentLoadOp, VkAttachmentStoreOp, VkFormat, VkImageLayout,
    VkSampleCountFlag,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_TRUE, VK_VERSION_1_0, VkAttachmentDescriptionFlag};

/// Structure specifying an attachment description
///
/// # Description
/// If the attachment uses a color format, then `load_op` and `store_op` are used, and
/// `stencil_load_op` and `stencil_store_op` are ignored. If the format has depth and/or stencil
/// components, `load_op` and `store_op` apply only to the depth data, while `stencil_load_op` and
/// `stencil_store_op` define how the stencil data is handled. `load_op` and `stencil_load_op`
/// define the load operations for the attachment. `store_op` and `stencil_store_op` define the
/// store operations for the attachment. If an attachment is not used by any subpass, `load_op`,
/// `store_op`, `stencil_store_op`, and `stencil_load_op` will be ignored for that attachment, and
/// no load or store ops will be performed. However, any transition specified by `initial_layout`
/// and `final_layout` will still be executed.
///
/// If `flags` includes [`VkAttachmentDescriptionFlag::MayAlias`], then the attachment is
/// treated as if it shares physical memory with another attachment in the same render pass. This
/// information limits the ability of the implementation to reorder certain operations (like layout
/// transitions and the `load_op`) such that it is not improperly reordered against other uses of
/// the same physical memory via a different attachment. This is described in more detail below.
///
/// If a render pass uses multiple attachments that alias the same device memory, those attachments
/// must each include the [`VkAttachmentDescriptionFlag::MayAlias`] bit in their attachment
/// description `flags`. Attachments aliasing the same memory occurs in multiple ways:
///  - Multiple attachments being assigned the same image view as part of framebuffer creation.
///  - Attachments using distinct image views that correspond to the same image subresource of an
///    image.
///  - Attachments using views of distinct image subresources which are bound to overlapping memory
///    ranges.
///
/// Multiple attachments that alias the same memory must not be used in a single subpass. A given
/// attachment index must not be used multiple times in a single subpass, with one exception: two
/// subpass attachments can use the same attachment index if at least one use is as an input
/// attachment and neither use is as a resolve or preserve attachment. In other words, the same
/// view can be used simultaneously as an input and color or depth/stencil attachment, but must not
/// be used as multiple color or depth/stencil attachments nor as resolve or preserve attachments.
///
/// If a set of attachments alias each other, then all except the first to be used in the render
/// pass must use an `initial_layout` of [`VkImageLayout::Undefined`], since the earlier uses of
/// the other aliases make their contents undefined. Once an alias has been used and a different
/// alias has been used after it, the first alias must not be used in any later subpasses. However,
/// an application can assign the same image view to multiple aliasing attachment indices, which
/// allows that image view to be used multiple times even if other aliases are used in between.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkAttachmentDescription {
    /// `flags` is a bitmask of [`VkAttachmentDescriptionFlag`] specifying additional properties of
    /// the attachment.
    ///
    /// # Valid Usage
    ///  - If `flags` includes [`VkAttachmentDescriptionFlag::ResolveSkipTransferFunctionKhr`],
    ///    `flags` must not include
    ///    [`VkAttachmentDescriptionFlag::ResolveEnableTransferFunctionKhr`]
    ///  - If `flags` includes [`VkAttachmentDescriptionFlag::ResolveSkipTransferFunctionKhr`]
    ///    or [`VkAttachmentDescriptionFlag::ResolveEnableTransferFunctionKhr`],
    ///    `resolve_srgb_format_supports_transfer_function_control` must be [`VK_TRUE`]
    ///  - If `flags` includes [`VkAttachmentDescriptionFlag::ResolveSkipTransferFunctionKhr`]
    ///    or [`VkAttachmentDescriptionFlag::ResolveEnableTransferFunctionKhr`], `maintenance10`
    ///    must be enabled
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of [`VkAttachmentDescriptionFlag`] values
    pub flags: VkAttachmentDescriptionFlags,

    /// `format` is a [`VkFormat`] value specifying the format of the image view that will be used
    /// for the attachment.
    ///
    /// # Valid Usage
    ///  - If `flags` includes [`VkAttachmentDescriptionFlag::ResolveSkipTransferFunctionKhr`]
    ///    or [`VkAttachmentDescriptionFlag::ResolveEnableTransferFunctionKhr`], `format` must
    ///    use sRGB encoding
    ///  - format must not be [`VkFormat::Undefined`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `format` must be a valid [`VkFormat`] value
    pub format: VkFormat,

    /// `samples` is a [`VkSampleCountFlag`] value specifying the number of samples of the image.
    ///
    /// # Valid Usage
    ///  - `samples` must be a valid [`VkSampleCountFlag`] value that is set in
    ///    `image_create_sample_counts` (as defined in Image Creation Limits) for the given format
    ///  - If `flags` includes [`VkAttachmentDescriptionFlag::ResolveSkipTransferFunctionKhr`]
    ///    or [`VkAttachmentDescriptionFlag::ResolveEnableTransferFunctionKhr`], `samples` must
    ///    be [`VkSampleCountFlag::_1`]
    ///
    /// # Valid Usage (Implicit)
    ///  - samples must be a valid [`VkSampleCountFlag`] value
    pub samples: VkSampleCountFlag,

    /// `load_op` is a [`VkAttachmentLoadOp`] value specifying how the contents of color and depth
    /// components of the attachment are treated at the beginning of the subpass where it is first
    /// used.
    ///
    /// # Valid Usage (Implicit)
    ///  - `load_op` must be a valid [`VkAttachmentLoadOp`] value
    pub load_op: VkAttachmentLoadOp,

    /// `store_op` is a [`VkAttachmentStoreOp`] value specifying how the contents of color and
    /// depth components of the attachment are treated at the end of the subpass where it is last
    /// used.
    ///
    /// # Valid Usage (Implicit)
    ///  - `store_op` must be a valid [`VkAttachmentStoreOp`] value
    pub store_op: VkAttachmentStoreOp,

    /// `stencil_load_op` is a [`VkAttachmentLoadOp`] value specifying how the contents of stencil
    /// components of the attachment are treated at the beginning of the subpass where it is first
    /// used.
    ///
    /// # Valid Usage (Implicit)
    ///  - `stencil_load_op` must be a valid [`VkAttachmentLoadOp`] value
    pub stencil_load_op: VkAttachmentLoadOp,

    /// `stencil_store_op` is a [`VkAttachmentStoreOp`] value specifying how the contents of
    /// stencil components of the attachment are treated at the end of the last subpass where it is
    /// used.
    ///
    /// # Valid Usage (Implicit)
    ///  - `stencil_store_op` must be a valid [`VkAttachmentStoreOp`] value
    pub stencil_store_op: VkAttachmentStoreOp,

    /// `initial_layout` is the layout the attachment image subresource will be in when a render
    /// pass instance begins.
    ///
    /// # Valid Usage
    ///  - If `format` includes a color or depth component and `load_op` is
    ///    [`VkAttachmentLoadOp::Load`], then `initial_layout` must not be
    ///    [`VkImageLayout::Undefined`]
    ///  - If `format` is a color format, `initial_layout` must not be
    ///    [`VkImageLayout::DepthStencilAttachmentOptimal`] or
    ///    [`VkImageLayout::DepthStencilReadOnlyOptimal`]
    ///  - If `format` is a depth/stencil format, `initial_layout` must not be
    ///    [`VkImageLayout::ColorAttachmentOptimal`]
    ///  - If `format` is a color format, `initial_layout` must not be
    ///    [`VkImageLayout::DepthAttachmentStencilReadOnlyOptimal`] or
    ///    [`VkImageLayout::DepthReadOnlyStencilAttacmentOptimal`]
    ///  - If the [`separate_depth_stencil_layouts`] feature is not enabled, `initial_layout` must
    ///    not be [`VkImageLayout::DepthAttachmentOptimal`],
    ///    [`VkImageLayout::DepthReadOnlyOptimal`], [`VkImageLayout::StencilAttachmentOptimal`], or
    ///    [`VkImageLayout::StencilReadOnlyOptimal`]
    ///  - If `format` is a color format, `initial_layout` must not be
    ///    [`VkImageLayout::DepthAttachmentOptimal`], [`VkImageLayout::DepthReadOnlyOptimal`],
    ///    [`VkImageLayout::StencilAttachmentOptimal`], or
    ///    [`VkImageLayout::StencilReadOnlyOptimal`]
    ///  - If `format` is a depth/stencil format which includes both depth and stencil components,
    ///    `initial_layout` must not be [`VkImageLayout::StencilAttachmentOptimal`] or
    ///    [`VkImageLayout::StencilReadOnlyOptimal`]
    ///  - If `format` is a depth/stencil format which includes only the depth component,
    ///    `initial_layout` must not be [`VkImageLayout::StencilAttachmentOptimal`] or
    ///    [`VkImageLayout::StencilReadOnlyOptimal`]
    ///  - If the [`synchronization2`] feature is not enabled, `initial_layout` must not be
    ///    [`VkImageLayout::AttachmentOptimalKhr`] or [`VkImageLayout::ReadOnlyOptimalKhr`]
    ///  - If the [`attachment_feedback_loop_layout`] feature is not enabled, `initial_layout` must
    ///    not be [`VkImageLayout::AttachmentFeedbackLoopOptimalExt`]
    ///  - If the [`dynamic_rendering_local_read`] feature is not enabled, `initial_layout` must
    ///    not be [`VkImageLayout::RenderingLocalRead`]
    ///  - If `format` includes a stencil component and `stencil_load_op` is
    ///    [`VkAttachmentLoadOp::Load`], then `initial_layout` must not be
    ///    [`VkImageLayout::Undefined`]
    ///  - If `format` is a depth/stencil format which includes only the stencil component,
    ///    `initial_layout` must not be [`VkImageLayout::DepthAttachmentOptimal`] or
    ///    [`VkImageLayout::DepthReadOnlyOptimal`]
    ///  - If `format` is a depth/stencil format which includes both depth and stencil components,
    ///    `initial_layout` must not be [`VkImageLayout::DepthAttachmentOptimal`] or
    ///    [`VkImageLayout::DepthReadOnlyOptimal`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `initial_layout` must be a valid [`VkImageLayout`] value
    pub initial_layout: VkImageLayout,

    /// `final_layout` is the layout the attachment image subresource will be transitioned to when
    /// a render pass instance ends.
    ///
    /// # Valid Usage
    ///  - `final_layout` must not be [`VkImageLayout::Undefined`] or
    ///    [`VkImageLayout::ZeroInitializedExt`] or [`VkImageLayout::Preinitialized`]
    ///  - If `format` is a color format, `final_layout` must not be
    ///    [`VkImageLayout::DepthStencilAttachmentOptimal`] or
    ///    [`VkImageLayout::DepthStencilReadOnlyOptimal`]
    ///  - If `format` is a depth/stencil format, `final_layout` must not be
    ///    [`VkImageLayout::ColorAttachmentOptimal`]
    ///  - If `format` is a color format, `final_layout` must not be
    ///    [`VkImageLayout::DepthAttachmentStencilReadOnlyOptimal`] or
    ///    [`VkImageLayout::DepthReadOnlyStencilAttacmentOptimal`]
    ///  - If the [`separate_depth_stencil_layouts`] feature is not enabled, `final_layout` must
    ///    not be [`VkImageLayout::DepthAttachmentOptimal`],
    ///    [`VkImageLayout::DepthReadOnlyOptimal`], [`VkImageLayout::StencilAttachmentOptimal`], or
    ///    [`VkImageLayout::StencilReadOnlyOptimal`]
    ///  - If `format` is a color format, `final_layout` must not be
    ///    [`VkImageLayout::DepthAttachmentOptimal`], [`VkImageLayout::DepthReadOnlyOptimal`],
    ///    [`VkImageLayout::StencilAttachmentOptimal`], or
    ///    [`VkImageLayout::StencilReadOnlyOptimal`]
    ///  - If `format` is a depth/stencil format which includes both depth and stencil components,
    ///    `final_layout` must not be [`VkImageLayout::StencilAttachmentOptimal`] or
    ///    [`VkImageLayout::StencilReadOnlyOptimal`]
    ///  - If `format` is a depth/stencil format which includes only the depth component,
    ///    `final_layout` must not be [`VkImageLayout::StencilAttachmentOptimal`] or
    ///    [`VkImageLayout::StencilReadOnlyOptimal`]
    ///  - If the [`synchronization2`] feature is not enabled, `final_layout` must not be
    ///    [`VkImageLayout::AttachmentOptimalKhr`] or [`VkImageLayout::ReadOnlyOptimalKhr`]
    ///  - If the [`attachment_feedback_loop_layout`] feature is not enabled, `final_layout` must
    ///    not be [`VkImageLayout::AttachmentFeedbackLoopOptimalExt`]
    ///  - If the [`dynamic_rendering_local_read`] feature is not enabled, `final_layout` must not
    ///    be [`VkImageLayout::RenderingLocalRead`]
    ///  - If `format` is a depth/stencil format which includes only the stencil component,
    ///    `final_layout` must not be [`VkImageLayout::DepthAttachmentOptimal`] or
    ///    [`VkImageLayout::DepthReadOnlyOptimal`]
    ///  - If `format` is a depth/stencil format which includes both depth and stencil components,
    ///    `final_layout` must not be [`VkImageLayout::DepthAttachmentOptimal`] or
    ///    [`VkImageLayout::DepthReadOnlyOptimal`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `final_layout` must be a valid [`VkImageLayout`] value
    pub final_layout: VkImageLayout,
}
