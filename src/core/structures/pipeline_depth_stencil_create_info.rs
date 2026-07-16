use crate::{
    VK_FALSE, VkBool32, VkCompareOp, VkPipelineDepthStencilStateCreateFlags, VkStencilOpState,
    VkStructureType, util::NextChain,
};
use std::{
    ffi::{c_float, c_void},
    ptr::null,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_TRUE, VK_VERSION_1_0, VkCullModeFlag, VkPipelineDepthStencilStateCreateFlag};

/// Structure specifying parameters of a newly created pipeline depth stencil state
///
/// # Valid Usage
///  - If the [`khr_portability_subset`] extension is enabled, and
///    [`VkPhysicalDevicePortabilitySubsetFeaturesKhr::separate_stencil_mask_ref`] is [`VK_FALSE`],
///    and the value of [`VkPipelineDepthStencilStateCreateInfo::stencil_test_enable`] is
///    [`VK_TRUE`], and the value of [`VkPipelineRasterizationStateCreateInfo::cull_mode`] is
///    [`VkCullModeFlag::None`], the value of reference in each of the [`VkStencilOpState`] structs
///    in `front` and `back` must be the same
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct VkPipelineDepthStencilStateCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PipelineDepthStencilStateCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`]
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkPipelineDepthStencilStateCreateFlag`]s specifying additional
    /// depth/stencil state information.
    ///
    /// # Valid Usage
    ///  - If the `rasterization_order_depth_attachment_access` feature is not enabled, `flags`
    ///    must not include
    ///    [`VkPipelineDepthStencilStateCreateFlag::RasterizationOrderAttachmentDepthAcccessExt`]
    ///  - If the `rasterization_order_stencil_attachment_access` feature is not enabled, `flags`
    ///    must not include
    ///    [`VkPipelineDepthStencilStateCreateFlag::RasterizationOrderAttachmentStencilAcccessExt`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of [`VkPipelineDepthStencilStateCreateFlag`] values
    pub flags: VkPipelineDepthStencilStateCreateFlags,

    /// `depth_test_enable` controls whether depth testing is enabled.
    pub depth_test_enable: VkBool32,

    /// `depth_write_enable` controls whether depth writes are enabled when `depth_test_enable` is
    /// [`VK_TRUE`]. Depth writes are always disabled when `depth_test_enable` is [`VK_FALSE`].
    pub depth_write_enable: VkBool32,

    /// `depth_compare_op` is a [`VkCompareOp`] value specifying the comparison operator to use in
    /// the Depth Comparison step of the depth test.
    ///
    /// # Valid Usage (Implicit)
    ///  - `depth_compare_op` must be a valid [`VkCompareOp`] value
    pub depth_compare_op: VkCompareOp,

    /// `depth_bounds_test_enable` controls whether depth bounds testing is enabled.
    ///
    /// # Valid Usage
    ///  - If the `depth_bounds` feature is not enabled, `depth_bounds_test_enable` must be
    ///    [`VK_FALSE`]
    pub depth_bounds_test_enable: VkBool32,

    /// `stencil_test_enable` controls whether stencil testing is enabled.
    pub stencil_test_enable: VkBool32,

    /// `front` is a [`VkStencilOpState`] value controlling the front parameter of the stencil
    /// test.
    ///
    /// # Valid Usage (Implicit)
    ///  - `front` must be a valid [`VkStencilOpState`] structure
    pub front: VkStencilOpState,

    /// `back` is a [`VkStencilOpState`] value controlling the back parameter of the stencil test.
    ///
    /// # Valid Usage (Implicit)
    ///  - `back` must be a valid [`VkStencilOpState`] structure
    pub back: VkStencilOpState,

    /// `min_depth_bounds` is the minimum depth bound used in the depth bounds test.
    pub min_depth_bounds: c_float,

    /// `max_depth_bounds` is the maximum depth bound used in the depth bounds test.
    pub max_depth_bounds: c_float,
}

const impl Default for VkPipelineDepthStencilStateCreateInfo {
    fn default() -> Self {
        VkPipelineDepthStencilStateCreateInfo {
            r#type: VkStructureType::PipelineDepthStencilStateCreateInfo,
            next: null(),
            flags: VkPipelineDepthStencilStateCreateFlags::empty(),
            depth_test_enable: VK_FALSE,
            depth_write_enable: VK_FALSE,
            depth_compare_op: VkCompareOp::Never,
            depth_bounds_test_enable: VK_FALSE,
            stencil_test_enable: VK_FALSE,
            front: VkStencilOpState::default(),
            back: VkStencilOpState::default(),
            min_depth_bounds: 0.0,
            max_depth_bounds: 0.0,
        }
    }
}

impl NextChain for VkPipelineDepthStencilStateCreateInfo {
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
