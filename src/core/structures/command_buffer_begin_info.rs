use crate::{
    VkCommandBufferInheritanceInfo, VkCommandBufferUsageFlags, VkStructureType, util::NextChain,
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VK_VERSION_1_0, VkCommandBufferUsageFlag, VkCommandPool, VkFramebuffer,
    VkRenderPass,
};

/// Structure specifying a command buffer begin operation
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkCommandBufferBeginInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::CommandBufferBeginInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`] or a pointer to a valid instance of
    ///    [`VkDeviceGroupCommandBufferBeginInfo`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkCommandBufferUsageFlag`]s specifying usage behavior for the
    /// command buffer.
    ///
    /// # Valid Usage
    ///  - If `flags` contains [`VkCommandBufferUsageFlag::RenderPassContinue`], the
    ///    [`VkCommandPool`] that `command_buffer` was allocated from must support graphics
    ///    operations
    ///  - `flags` must be a valid combination of [`VkCommandBufferUsageFlag`]s values
    pub flags: VkCommandBufferUsageFlags,

    /// `inheritance_info` is a pointer to a [`VkCommandBufferInheritanceInfo`] structure, used if
    /// `command_buffer` is a secondary command buffer. If this is a primary command buffer, then
    /// this value is ignored.
    ///
    /// # Valid Usage
    ///  - If `flags` contains [`VkCommandBufferUsageFlag::RenderPassContinue`], the
    ///    `framebuffer` member of `inheritance_info` must be either [`VK_NULL_HANDLE`], or a valid
    ///    [`VkFramebuffer`] that is compatible with the `render_pass` member of `inheritance_info`
    ///  - If `flags` contains [`VkCommandBufferUsageFlag::RenderPassContinue`] and the
    ///    `dynamic_rendering` feature is not enabled, the `render_pass` member of
    ///    `inheritance_info` must not be [`VK_NULL_HANDLE`]
    ///  - If `flags` contains [`VkCommandBufferUsageFlag::RenderPassContinue`] and the
    ///    `render_pass` member of `inheritance_info` is [`VK_NULL_HANDLE`], the `next` chain of
    ///    `inheritance_info` must include a [`VkCommandBufferInheritanceRenderingInfo`] structure
    ///  - If `flags` contains [`VkCommandBufferUsageFlag::RenderPassContinue`], the
    ///    `render_pass` member of `inheritance_info` is [`VK_NULL_HANDLE`], and the `next` chain
    ///    of `inheritance_info` includes a [`VkAttachmentSampleCountInfoAmd`] or
    ///    [`VkAttachmentSampleCountInfoNv`] structure, the `color_attachment_count` member of that
    ///    structure must be equal to the value of
    ///    [`VkCommandBufferInheritanceRenderingInfo::color_attachment_count`]
    ///  - If `flags` contains [`VkCommandBufferUsageFlag::RenderPassContinue`] and the
    ///    `render_pass` member of `inheritance_info` is not [`VK_NULL_HANDLE`], the `render_pass`
    ///    member of `inheritance_info` must be a valid [`VkRenderPass`]
    ///  - If `flags` contains [`VkCommandBufferUsageFlag::RenderPassContinue`] and the
    ///    `render_pass` member of `inheritance_info` is not [`VK_NULL_HANDLE`], the `subpass`
    ///    member of `inheritance_info` must be a valid subpass index within the `render_pass`
    ///    member of `inheritance_info`
    ///  - If `flags` contains [`VkCommandBufferUsageFlag::RenderPassContinue`] , the
    ///    `render_pass` member of `inheritance_info` is not [`VK_NULL_HANDLE`], and `render_pass`
    ///    was created with tile shading enabled,
    ///    [`VkTileShadingRenderPassFlagQcom::EnableQcom`] must be included in
    ///    [`VkRenderPassTileShadingCreateInfoQcom::flags`]
    ///  - If `flags` does not contain [`VkCommandBufferUsageFlag::RenderPassContinue`] , the
    ///    `render_pass` member of `inheritance_info` is [`VK_NULL_HANDLE`], or `render_pass` was
    ///    not created with tile shading enabled,
    ///    [`VkTileShadingRenderPassFlagQcom::EnableQcom`] must not be included in
    ///    [`VkRenderPassTileShadingCreateInfoQcom::flags`]
    ///  - If [`VkTileShadingRenderPassFlagQcom::EnableQcom`] is included in
    ///    [`VkRenderPassTileShadingCreateInfoQcom::flags`],
    ///    [`VkRenderPassTileShadingCreateInfoQcom::tile_apron_size`] must be equal to the
    ///    `tile_apron_size` used to create `render_pass`
    pub inheritance_info: *const VkCommandBufferInheritanceInfo,
}

impl const Default for VkCommandBufferBeginInfo {
    fn default() -> Self {
        VkCommandBufferBeginInfo {
            r#type: VkStructureType::CommandBufferBeginInfo,
            next: null(),
            flags: VkCommandBufferUsageFlags::default(),
            inheritance_info: null(),
        }
    }
}

impl NextChain for VkCommandBufferBeginInfo {
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
