use crate::{
    CullModeFlags, FrontFace, PolygonMode, VkPipelineRasterizationStateCreateFlags,
    VkPipelineRasterizationStateCreateInfo, VkStructureType,
};
use std::ptr::null;

pub struct PipelineRasterizationStateCreateInfo {
    pub depth_clamp_enable: bool,
    pub rasterizer_discard_enable: bool,
    pub polygon_mode: PolygonMode,
    pub cull_mode: CullModeFlags,
    pub front_face: FrontFace,
    pub depth_bias_enable: bool,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub line_width: f32,
}

impl PipelineRasterizationStateCreateInfo {
    pub(super) fn into_binding(&self) -> VkPipelineRasterizationStateCreateInfo {
        VkPipelineRasterizationStateCreateInfo {
            s_type: VkStructureType::PipelineRasterizationStateCreateInfo,
            p_next: null(),
            flags: VkPipelineRasterizationStateCreateFlags::default(),
            depth_clamp_enable: self.depth_clamp_enable as u32,
            rasterizer_discard_enable: self.rasterizer_discard_enable as u32,
            polygon_mode: self.polygon_mode,
            cull_mode: self.cull_mode,
            front_face: self.front_face,
            depth_bias_enable: self.depth_bias_enable as u32,
            depth_bias_constant_factor: self.depth_bias_constant_factor,
            depth_bias_clamp: self.depth_bias_clamp,
            depth_bias_slope_factor: self.depth_bias_slope_factor,
            line_width: self.line_width,
        }
    }
}

impl Default for PipelineRasterizationStateCreateInfo {
    fn default() -> Self {
        PipelineRasterizationStateCreateInfo {
            depth_clamp_enable: false,
            rasterizer_discard_enable: false,
            polygon_mode: PolygonMode::default(),
            cull_mode: CullModeFlags::default(),
            front_face: FrontFace::default(),
            depth_bias_enable: false,
            depth_bias_constant_factor: 0.0,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 0.0,
            line_width: 1.0,
        }
    }
}
