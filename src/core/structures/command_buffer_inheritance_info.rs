use crate::{
    VkBool32, VkFramebuffer, VkQueryControlFlags, VkQueryPipelineStatisticFlags, VkRenderPass,
    VkStructureType, util::NextChain,
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_FALSE, VK_NULL_HANDLE, VK_TRUE, VK_VERSION_1_0, VkCommandBuffer, VkDevice};

/// Structure specifying command buffer inheritance information
///
/// # Description
/// If the [`VkCommandBuffer`] will not be executed within a render pass instance, or if the render
/// pass instance was begun with [`VkCmdBeginRendering`], `render_pass`, `subpass`, and
/// `framebuffer` are ignored.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkCommandBufferInheritanceInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::CommandBufferInheritanceInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of [`VkAttachmentSampleCountInfoAmd`],
    ///    [`VkCommandBufferInheritanceConditionalRenderingInfoExt`],
    ///    [`VkCommandBufferInheritanceDescriptorHeapInfoExt`],
    ///    [`VkCommandBufferInheritanceRenderPassTransformInfoQcom`],
    ///    [`VkCommandBufferInheritanceRenderingInfo`],
    ///    [`VkCommandBufferInheritanceViewportScissorInfoNv`], [`VkCustomResolveCreateInfoExt`],
    ///    [`VkExternalFormatAndroid`], [`VkExternalFormatOhos`],
    ///    [`VkMultiviewPerViewAttributesInfoNvx`], [`VkRenderPassTileShadingCreateInfoQcom`],
    ///    [`VkRenderingAttachmentLocationInfo`], [`VkRenderingInputAttachmentIndexInfo`], or
    ///    [`VkTileMemoryBindInfoQcom`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `render_pass` is a [`VkRenderPass`] object defining which render passes the
    /// [`VkCommandBuffer`] will be compatible with and can be executed within.
    ///
    /// # Valid Usage (Implicit)
    ///  - Both of `framebuffer`, and `render_pass` that are valid handles of non-ignored
    ///    parameters must have been created, allocated, or retrieved from the same [`VkDevice`]
    pub render_pass: VkRenderPass,

    /// `subpass` is the index of the subpass within the render pass instance that the
    /// [`VkCommandBuffer`] will be executed within.
    pub subpass: u32,

    /// `framebuffer` can refer to the [`VkFramebuffer`] object that the [`VkCommandBuffer`] will
    /// be rendering to if it is executed within a render pass instance. It can be
    /// [`VK_NULL_HANDLE`] if the framebuffer is not known.
    ///
    /// # Valid Usage (Implicit)
    ///  - Both of `framebuffer`, and `render_pass` that are valid handles of non-ignored
    ///    parameters must have been created, allocated, or retrieved from the same [`VkDevice`]
    pub framebuffer: VkFramebuffer,

    /// `occlusion_query_enable` specifies whether the command buffer can be executed while an
    /// occlusion query is active in the primary command buffer. If this is [`VK_TRUE`], then this
    /// command buffer can be executed whether the primary command buffer has an occlusion query
    /// active or not. If this is [`VK_FALSE`], then the primary command buffer must not have an
    /// occlusion query active.
    ///
    /// # Valid Usage
    ///  - If the `inherited_queries` feature is not enabled, `occlusion_query_enable` must be
    ///    [`VK_FALSE`]
    pub occlusion_query_enable: VkBool32,

    /// `query_flags` specifies the query flags that can be used by an active occlusion query in the primary command buffer when this secondary command buffer is executed. If this value includes the VK_QUERY_CONTROL_PRECISE_BIT bit, then the active query can return boolean results or actual sample counts. If this bit is not set, then the active query must not use the VK_QUERY_CONTROL_PRECISE_BIT bit.
    ///
    /// # Valid Usage
    ///  - If the inheritedQueries feature is enabled, `query_flags` must be a valid combination of VkQueryControlFlags values
    ///  - If the inheritedQueries feature is not enabled, `query_flags` must be 0
    pub query_flags: VkQueryControlFlags,

    /// pipelineStatistics is a bitmask of VkQueryPipelineStatisticFlags specifying the set of pipeline statistics that can be counted by an active query in the primary command buffer when this secondary command buffer is executed. If this value includes a given bit, then this command buffer can be executed whether the primary command buffer has a pipeline statistics query active that includes this bit or not. If this value excludes a given bit, then the active pipeline statistics query must not be from a query pool that counts that statistic.
    ///
    /// # Valid Usage
    ///  - If the pipelineStatisticsQuery feature is enabled, pipelineStatistics must be a valid combination of VkQueryPipelineStatisticFlags values
    ///  - If the pipelineStatisticsQuery feature is not enabled, pipelineStatistics must be 0
    pub pipeline_statistics: VkQueryPipelineStatisticFlags,
}

const impl Default for VkCommandBufferInheritanceInfo {
    fn default() -> Self {
        VkCommandBufferInheritanceInfo {
            r#type: VkStructureType::CommandBufferInheritanceInfo,
            next: null(),
            render_pass: VkRenderPass::null(),
            subpass: 0,
            framebuffer: VkFramebuffer::null(),
            occlusion_query_enable: 0,
            query_flags: VkQueryControlFlags::default(),
            pipeline_statistics: VkQueryPipelineStatisticFlags::default(),
        }
    }
}

impl NextChain for VkCommandBufferInheritanceInfo {
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
