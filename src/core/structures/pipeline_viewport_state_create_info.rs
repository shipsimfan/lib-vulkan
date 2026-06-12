use crate::{
    VkPipelineViewportStateCreateFlags, VkRect2D, VkStructureType, VkViewport, util::NextChain,
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_TRUE, VK_VERSION_1_0, VkDynamicState, VkPhysicalDeviceLimits};

/// Structure specifying parameters of a newly created pipeline viewport state
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPipelineViewportStateCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PipelineViewportStateCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If the `viewport_w_scaling_enable` member of a
    ///    [`VkPipelineViewportWScalingStateCreateInfoNv`] structure included in the `next` chain
    ///    is [`VK_TRUE`], the `viewport_count` member of the
    ///    [`VkPipelineViewportWScalingStateCreateInfoNv`] structure must be greater than or equal
    ///    to [`VkPipelineViewportStateCreateInfo::viewport_count`]
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of
    ///    [`VkPipelineViewportCoarseSampleOrderStateCreateInfoNv`],
    ///    [`VkPipelineViewportDepthClampControlCreateInfoExt`],
    ///    [`VkPipelineViewportDepthClipControlCreateInfoExt`],
    ///    [`VkPipelineViewportExclusiveScissorStateCreateInfoNv`],
    ///    [`VkPipelineViewportShadingRateImageStateCreateInfoNv`],
    ///    [`VkPipelineViewportSwizzleStateCreateInfoNv`], or
    ///    [`VkPipelineViewportWScalingStateCreateInfoNv`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique
    pub next: *const c_void,

    /// `flags` is reserved for future use.
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be 0
    pub flags: VkPipelineViewportStateCreateFlags,

    /// `viewport_count` is the number of viewports used by the pipeline.
    ///
    /// # Valid Usage
    ///  - If the `multi_viewport` feature is not enabled, `viewport_count` must not be greater
    ///    than 1
    ///  - `viewport_count` must be less than or equal to [`VkPhysicalDeviceLimits::max_viewports`]
    ///  - If `scissor_count` and `viewport_count` are both not dynamic, then `scissor_count` and
    ///    `viewport_count` must be identical
    ///  - If the graphics pipeline is being created with [`VkDynamicState::ViewportWithCount`] set
    ///    then `viewport_count` must be 0, otherwise `viewport_count` must be greater than 0
    pub viewport_count: u32,

    /// `viewports` is a pointer to an array of [`VkViewport`] structures, defining the viewport
    /// transforms. If the viewport state is dynamic, this member is ignored.
    pub viewports: *const VkViewport,

    /// `scissor_count` is the number of scissors and must match the number of viewports.
    ///
    /// # Valid Usage
    ///  - If the `multi_viewport` feature is not enabled, `scissor_count` must not be greater than
    ///    1
    ///  - `scissor_count` must be less than or equal to [`VkPhysicalDeviceLimits::max_viewports`]
    ///  - If `scissor_count` and `viewport_count` are both not dynamic, then `scissor_count` and
    ///    `viewport_count` must be identical
    ///  - If the graphics pipeline is being created with [`VkDynamicState::ScissorWithCount`] set
    ///    then `scissor_count` must be 0, otherwise `scissor_count` must be greater than 0
    pub scissor_count: u32,

    /// `scissors` is a pointer to an array of [`VkRect2D`] structures defining the rectangular
    /// bounds of the scissor for the corresponding viewport. If the scissor state is dynamic, this
    /// member is ignored.
    ///
    /// # Valid Usage
    ///  - The `x` and `y` members of `offset` member of any element of `scissors` must be greater
    ///    than or equal to 0
    ///  - Evaluation of `(offset.x + extent.width)` must not cause a signed integer addition
    ///    overflow for any element of `scissors`
    ///  - Evaluation of `(offset.y + extent.height)` must not cause a signed integer addition
    ///    overflow for any element of `scissors`
    pub scissors: *const VkRect2D,
}

impl const Default for VkPipelineViewportStateCreateInfo {
    fn default() -> Self {
        VkPipelineViewportStateCreateInfo {
            r#type: VkStructureType::PipelineViewportStateCreateInfo,
            next: null(),
            flags: VkPipelineViewportStateCreateFlags::empty(),
            viewport_count: 0,
            viewports: null(),
            scissor_count: 0,
            scissors: null(),
        }
    }
}

impl NextChain for VkPipelineViewportStateCreateInfo {
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
