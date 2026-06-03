use crate::{
    VkAttachmentLoadOp, VkAttachmentStoreOp, VkClearValue, VkImageLayout, VkImageView,
    VkResolveModeFlag, VkStructureType, util::NextChain,
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_FALSE, VK_NULL_HANDLE, VK_TRUE, VK_VERSION_1_3, VkDevice, VkDeviceMemory, VkFormat,
    VkImageUsageFlag, VkMemoryHeap, VkRenderingFlag, VkRenderingInfo, VkSampleCountFlag,
};

/// Structure specifying attachment information
///
/// # Description
/// Values in `image_view` are loaded and stored according to the values of `load_op` and
/// `store_op`, within the render area for each device specified in [`VkRenderingInfo`]. If
/// `image_view` is [`VK_NULL_HANDLE`], and `resolve_mode` is not
/// [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`], other members of this structure are
/// ignored; writes to this attachment will be discarded, and no load, store, or multisample
/// resolve operations will be performed.
///
/// If `resolve_mode` is [`VkResolveModeFlag::None`], then `resolve_image_view` is ignored. If
/// `resolve_mode` is not [`VkResolveModeFlag::None`], and `resolve_image_view` is not
/// [`VK_NULL_HANDLE`], a render pass multisample resolve operation is defined for the attachment
/// subresource. If `resolve_mode` is [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`],
/// and the `null_color_attachment_with_external_format_resolve` limit is [`VK_TRUE`], values are
/// only undefined once load operations have completed.
///
/// The contents of a resolve attachment within the render area become undefined at the time
/// [`VkCmdBeginCustomResolveExt`] is called if all of the following conditions are true:
///  - [`VkRenderingFlag::CustomResolveExt`] is set.
///  - The attachment sets `resolve_mode` to [`VkResolveModeFlag::CustomExt`].
///
/// This affects color, depth, and stencil attachments. In addition, there is an implicit store
/// operation of [`VkAttachmentStoreOp::Store`] for these attachments.
///
/// Store and resolve operations are only performed at the end of a render pass instance that does
/// not specify the [`VkRenderingFlag::Suspending`] flag. If the
/// [`VkRenderingFlag::CustomResolveExt`] is specified and an attachment uses the
/// [`VkResolveModeFlag::CustomExt`] resolve mode, the resolve attachment will only be written
/// by draws recorded following a call to [`VkCmdBeginCustomResolveExt`].
///
/// Load operations are only performed at the beginning of a render pass instance that does not
/// specify the [`VkRenderingFlag::Resuming`] flag.
///
/// Image contents at the end of a suspended render pass instance remain defined for access by a
/// resuming render pass instance.
///
/// If the `null_color_attachment_with_external_format_resolve` limit is [`VK_TRUE`], and
/// `resolve_mode` is [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`], values in the
/// color attachment will be loaded from the resolve attachment at the start of rendering, and may
/// also be reloaded any time after a resolve occurs or the resolve attachment is written to; if
/// this occurs it must happen-before any writes to the color attachment are performed which
/// happen-after the resolve that triggers this. If any color component in the external format is
/// subsampled, values will be read from the nearest sample in the image when they are loaded.
///
/// Provided by [`VK_VERSION_1_3`]
#[repr(C)]
#[derive(Clone)]
pub struct VkRenderingAttachmentInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::RenderingAttachmentInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of [`VkAttachmentFeedbackLoopInfoExt`]
    ///    or [`VkRenderingAttachmentFlagsInfoKhr`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `image_view` is the image view that will be used for rendering.
    ///
    /// # Valid Usage
    ///  - If all of the following are true, `image_view` must not have a sample count of
    ///    [`VkSampleCountFlag::_1`]:
    ///    - `image_view` is not [`VK_NULL_HANDLE`]
    ///    - `resolve_mode` is not [`VkResolveModeFlag::None`]
    ///    - the `next` chain of [`VkRenderingInfo`] does not include a
    ///      [`VkMultisampledRenderToSingleSampledInfoExt`] structure with the
    ///      `multisampled_render_to_single_sampled_enable` field equal to [`VK_TRUE`]
    ///  - If `image_view` is not [`VK_NULL_HANDLE`], `resolve_image_view` is not
    ///    [`VK_NULL_HANDLE`], and `resolve_mode` is neither [`VkResolveModeFlag::CustomExt`]
    ///    nor [`VkResolveModeFlag::None`], `image_view` and `resolve_image_view` must have the
    ///    same [`VkFormat`]
    ///  - If feedback loop is enabled for the attachment identified by `image_view`, then
    ///    `image_view` must have been created with a usage value including
    ///    [`VkImageUsageFlag::AttachmentFeedbackLoopExt`], either
    ///    [`VkImageUsageFlag::ColorAttachment`] or
    ///    [`VkImageUsageFlag::DepthStencilAttachment`], and either
    ///    [`VkImageUsageFlag::InputAttachment`] or [`VkImageUsageFlag::Sampled`]
    ///  - If `resolve_mode` is [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`] and
    ///    `null_color_attachment_with_external_format_resolve` is [`VK_TRUE`], `image_view` must
    ///    be [`VK_NULL_HANDLE`]
    ///  - If `resolve_mode` is [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`] and
    ///    `null_color_attachment_with_external_format_resolve` is [`VK_FALSE`], `image_view` must
    ///    be a valid [`VkImageView`]
    ///  - If `resolve_mode` is [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`] and
    ///    `null_color_attachment_with_external_format_resolve` is [`VK_FALSE`], `image_view` must
    ///    have a format equal to the value of
    ///    [`VkAndroidHardwareBufferFormatResolvePropertiesAndroid::color_attachment_format`] as
    ///    returned by a call to [`VkGetAndroidHardwareBufferPropertiesAndroid`] for the Android
    ///    hardware buffer that was used to create `resolve_image_view`
    ///  - If the `next` chain includes a [`VkRenderingAttachmentFlagsInfoKhr`] structure, and
    ///    `flags` includes [`VkRenderingAttachmentFlag::ResolveSkipTransferFunctionKhr`] or
    ///    [`VkRenderingAttachmentFlag::ResolveEnableTransferFunctionKhr`], `image_view` must
    ///    have a format using sRGB encoding
    ///  - If the `next` chain includes a [`VkRenderingAttachmentFlagsInfoKhr`] structure, and
    ///    `flags` includes [`VkRenderingAttachmentFlag::InputAttachmentFeedbackKhr`],
    ///    `image_view` must have an image that was created with the
    ///    [`VkImageUsageFlag::InputAttachment`] usage flag set
    ///
    /// # Valid Usage (Implicit)
    ///  - If `image_view` is not [`VK_NULL_HANDLE`], `image_view` must be a valid [`VkImageView`]
    ///    handle
    ///  - Both of `image_view`, and `resolve_image_view` that are valid handles of non-ignored
    ///    parameters must have been created, allocated, or retrieved from the same [`VkDevice`]
    pub image_view: VkImageView,

    /// `image_layout` is the layout that `image_view` will be in during rendering.
    ///
    /// # Valid Usage
    ///  - If `image_view` is not [`VK_NULL_HANDLE`], `image_layout` must not be
    ///    [`VkImageLayout::Undefined`], [`VkImageLayout::ShaderReadOnlyOptimal`],
    ///    [`VkImageLayout::TransferSrcOptimal`], [`VkImageLayout::ZeroInitializedExt`],
    ///    [`VkImageLayout::TransferDstOptimal`], or [`VkImageLayout::Preinitialized`]
    ///  - If `image_view` is not [`VK_NULL_HANDLE`], `image_layout` must not be
    ///    [`VkImageLayout::ShadingRateOptimalNv`]
    ///  - If `image_view` is not [`VK_NULL_HANDLE`], `image_layout` must not be
    ///    [`VkImageLayout::FragmentDensityMapOptimalExt`]
    ///  - If `image_view` is not [`VK_NULL_HANDLE`], `image_layout` must not be
    ///    [`VkImageLayout::FragmentShadingRateAttachmentOptimalKhr`]
    ///  - If `image_view` is not [`VK_NULL_HANDLE`], `image_layout` must not be
    ///    [`VkImageLayout::PresentSrcKhr`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `image_layout` must be a valid [`VkImageLayout`] value
    pub image_layout: VkImageLayout,

    /// `resolve_mode` is a [`VkResolveModeFlag`] value defining how data written to `image_view`
    /// will be resolved into `resolve_image_view`.
    ///
    /// # Valid Usage
    ///  - If `image_view` is not [`VK_NULL_HANDLE`] and has a non-integer color format,
    ///    `resolve_mode` must be [`VkResolveModeFlag::None`] or
    ///    [`VkResolveModeFlag::CustomExt`] or [`VkResolveModeFlag::Average`]
    ///  - If `image_view` is not [`VK_NULL_HANDLE`] and has an integer color format,
    ///    `resolve_mode` must be [`VkResolveModeFlag::None`] or
    ///    [`VkResolveModeFlag::CustomExt`] or [`VkResolveModeFlag::SampleZero`]
    ///  - If all of the following are true, then `resolve_mode` must not be
    ///    [`VkResolveModeFlag::None`]:
    ///    - `image_view` is not [`VK_NULL_HANDLE`]
    ///    - `image_view` has a sample count of [`VkSampleCountFlag::_1`]
    ///    - the `next` chain of [`VkRenderingInfo`] includes a
    ///      [`VkMultisampledRenderToSingleSampledInfoExt`] structure with the
    ///      `multisampled_render_to_single_sampled_enable` field equal to [`VK_TRUE`]
    ///  - If the `external_format_resolve` feature is not enabled, `resolve_mode` must not be
    ///    [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`]
    ///  - If the `next` chain includes a [`VkRenderingAttachmentFlagsInfoKhr`] structure, and
    ///    `flags` includes [`VkRenderingAttachmentFlag::ResolveSkipTransferFunctionKhr`] or
    ///    [`VkRenderingAttachmentFlag::ResolveEnableTransferFunctionKhr`], `resolve_mode` must
    ///    be equal to [`VkResolveModeFlag::Average`]
    ///
    /// # Valid Usage (Implicit)
    ///  - If `resolve_mode` is not 0, `resolve_mode` must be a valid [`VkResolveModeFlag`] value
    pub resolve_mode: VkResolveModeFlag,

    /// `resolve_image_view` is an image view used to write resolved data at the end of rendering.
    ///
    /// # Valid Usage
    ///  - If all of the following are true, `resolve_image_view` must not be [`VK_NULL_HANDLE`]:
    ///    - `image_view` is not [`VK_NULL_HANDLE`]
    ///    - `resolve_mode` is not [`VkResolveModeFlag::None`]
    ///    - the `next` chain of [`VkRenderingInfo`] does not include a
    ///      [`VkMultisampledRenderToSingleSampledInfoExt`] structure with the
    ///      `multisampled_render_to_single_sampled_enable` field equal to [`VK_TRUE`]
    ///  - If `image_view` is not [`VK_NULL_HANDLE`], `resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], the `next` chain of [`VkRenderingInfo`] includes a
    ///    [`VkMultisampledRenderToSingleSampledInfoExt`] structure with the
    ///    `multisampled_render_to_single_sampled_enable` field equal to [`VK_TRUE`], and
    ///    `image_view` has a sample count of [`VkSampleCountFlag::_1`], `resolve_image_view`
    ///    must be [`VK_NULL_HANDLE`]
    ///  - If `image_view` is not [`VK_NULL_HANDLE`], `resolve_image_view` is not
    ///    [`VK_NULL_HANDLE`], and `resolve_mode` is not [`VkResolveModeFlag::None`],
    ///    `resolve_image_view` must have a sample count of [`VkSampleCountFlag::_1`]
    ///  - If `image_view` is not [`VK_NULL_HANDLE`], `resolve_image_view` is not
    ///    [`VK_NULL_HANDLE`], and `resolve_mode` is neither [`VkResolveModeFlag::CustomExt`]
    ///    nor [`VkResolveModeFlag::None`], `image_view` and `resolve_image_view` must have the
    ///    same [`VkFormat`]
    ///  - If `resolve_mode` is [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`],
    ///    `resolve_image_view` must be a valid image view
    ///  - If the `null_color_attachment_with_external_format_resolve` property is [`VK_TRUE`] and
    ///    `resolve_mode` is [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`],
    ///    `resolve_image_view` must have been created with an image with a samples value of
    ///    [`VkSampleCountFlag::_1`]
    ///  - If `resolve_mode` is [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`],
    ///    `resolve_image_view` must have been created with an external format specified by
    ///    [`VkExternalFormatAndroid`]
    ///  - If `resolve_mode` is [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`],
    ///    `resolve_image_view` must have been created with a `subresource_range.layer_count` of 1
    ///  - If `resolve_image_view` is not [`VK_NULL_HANDLE`], the underlying resource must not be
    ///    bound to a [`VkDeviceMemory`] object allocated from a [`VkMemoryHeap`] with the
    ///    [`VkMemoryHeapFlag::TileMemoryQcom`] property
    ///
    /// # Valid Usage (Implicit)
    ///  - If `resolve_image_view` is not [`VK_NULL_HANDLE`], `resolve_image_view` must be a valid
    ///    [`VkImageView`] handle
    ///  - Both of `image_view`, and `resolve_image_view` that are valid handles of non-ignored
    ///    parameters must have been created, allocated, or retrieved from the same [`VkDevice`]
    pub resolve_image_view: VkImageView,

    /// `resolve_image_layout` is the layout that `resolve_image_view` will be in during rendering.
    ///
    /// # Valid Usage
    ///  - If `image_view` is not [`VK_NULL_HANDLE`] and `resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], `resolve_image_layout` must not be
    ///    [`VkImageLayout::Undefined`], [`VkImageLayout::DepthStencilReadOnlyOptimal`],
    ///    [`VkImageLayout::ShaderReadOnlyOptimal`], [`VkImageLayout::TransferSrcOptimal`],
    ///    [`VkImageLayout::ZeroInitializedExt`], [`VkImageLayout::TransferDstOptimal`], or
    ///    [`VkImageLayout::Preinitialized`]
    ///  - If `image_view` is not [`VK_NULL_HANDLE`] and `resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], `resolve_image_layout` must not be
    ///    [`VkImageLayout::DepthReadOnlyOptimal`] or [`VkImageLayout::StencilReadOnlyOptimal`]
    ///  - If `image_view` is not [`VK_NULL_HANDLE`] and `resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], `resolve_image_layout` must not be
    ///    [`VkImageLayout::ShadingRateOptimalNv`]
    ///  - If `image_view` is not [`VK_NULL_HANDLE`] and `resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], `resolve_image_layout` must not be
    ///    [`VkImageLayout::FragmentDensityMapOptimalExt`]
    ///  - If `image_view` is not [`VK_NULL_HANDLE`] and `resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], `resolve_image_layout` must not be
    ///    [`VkImageLayout::ReadOnlyOptimal`]
    ///  - If `image_view` is not [`VK_NULL_HANDLE`] and `resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], `resolve_image_layout` must not be
    ///    [`VkImageLayout::FragmentShadingRateAttachmentOptimalKhr`]
    ///  - If `image_view` is not [`VK_NULL_HANDLE`] and `resolve_mode` is not
    ///    [`VkResolveModeFlag::None`], `resolve_image_layout` must not be
    ///    [`VkImageLayout::PresentSrcKhr`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `resolve_image_layout` must be a valid [`VkImageLayout`] value
    pub resolve_image_layout: VkImageLayout,

    /// `load_op` is a [`VkAttachmentLoadOp`] value defining the load operation for the attachment.
    ///
    /// # Valid Usage (Implicit)
    ///  - `load_op` must be a valid [`VkAttachmentLoadOp`] value
    pub load_op: VkAttachmentLoadOp,

    /// `store_op` is a [`VkAttachmentStoreOp`] value defining the store operation for the
    /// attachment.
    ///
    /// # Valid Usage (Implicit)
    ///  - `store_op` must be a valid [`VkAttachmentStoreOp`] value
    pub store_op: VkAttachmentStoreOp,

    /// `clear_value` is a [`VkClearValue`] structure defining values used to clear `image_view`
    /// when `load_op` is [`VkAttachmentLoadOp::Clear`].
    pub clear_value: VkClearValue,
}

impl const Default for VkRenderingAttachmentInfo {
    fn default() -> Self {
        VkRenderingAttachmentInfo {
            r#type: VkStructureType::RenderingAttachmentInfo,
            next: null(),
            image_view: VkImageView::null(),
            image_layout: VkImageLayout::Undefined,
            resolve_mode: VkResolveModeFlag::None,
            resolve_image_view: VkImageView::null(),
            resolve_image_layout: VkImageLayout::Undefined,
            load_op: VkAttachmentLoadOp::DontCare,
            store_op: VkAttachmentStoreOp::DontCare,
            clear_value: VkClearValue::default(),
        }
    }
}

impl NextChain for VkRenderingAttachmentInfo {
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
