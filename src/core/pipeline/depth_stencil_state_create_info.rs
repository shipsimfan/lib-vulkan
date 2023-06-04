use std::ptr::null;

use crate::{
    bindings::{
        VkPipelineDepthStencilStateCreateFlags, VkPipelineDepthStencilStateCreateInfo,
        VkStructureType,
    },
    CompareOp, StencilOpState,
};

pub struct PipelineDepthStencilStateCreateInfo {
    pub depth_test_enable: bool,
    pub depth_write_enable: bool,
    pub depth_compare_op: CompareOp,
    pub depth_bounds_test_enable: bool,
    pub stencil_test_enable: bool,
    pub front: StencilOpState,
    pub back: StencilOpState,
    pub min_depth_bounds: f32,
    pub max_depth_boudns: f32,
}

impl PipelineDepthStencilStateCreateInfo {
    pub(super) fn into_binding(&self) -> VkPipelineDepthStencilStateCreateInfo {
        VkPipelineDepthStencilStateCreateInfo {
            s_type: VkStructureType::PipelineDepthStencilCreateInfo,
            p_next: null(),
            flags: VkPipelineDepthStencilStateCreateFlags::default(),
            depth_test_enable: self.depth_test_enable as u32,
            depth_write_enable: self.depth_write_enable as u32,
            depth_compare_op: self.depth_compare_op,
            depth_bounds_test_enable: self.depth_bounds_test_enable as u32,
            stencil_test_enable: self.stencil_test_enable as u32,
            front: self.front.clone(),
            back: self.back.clone(),
            min_depth_bounds: self.min_depth_bounds,
            max_depth_bounds: self.max_depth_boudns,
        }
    }
}
