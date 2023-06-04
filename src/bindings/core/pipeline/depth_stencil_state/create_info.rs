use crate::{
    VkBool32, VkCompareOp, VkPipelineDepthStencilStateCreateFlags, VkStencilOpState,
    VkStructureType,
};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkPipelineDepthStencilStateCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkPipelineDepthStencilStateCreateFlags,
    pub(crate) depth_test_enable: VkBool32,
    pub(crate) depth_write_enable: VkBool32,
    pub(crate) depth_compare_op: VkCompareOp,
    pub(crate) depth_bounds_test_enable: VkBool32,
    pub(crate) stencil_test_enable: VkBool32,
    pub(crate) front: VkStencilOpState,
    pub(crate) back: VkStencilOpState,
    pub(crate) min_depth_bounds: f32,
    pub(crate) max_depth_bounds: f32,
}
