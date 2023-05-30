use crate::{CullModeFlags, FrontFace, PolygonMode};

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
