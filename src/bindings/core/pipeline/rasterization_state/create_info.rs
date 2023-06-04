use crate::{
    bindings::VkBool32, VkCullModeFlags, VkFrontFace, VkPipelineRasterizationStateCreateFlags,
    VkPolygonMode, VkStructureType,
};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkPipelineRasterizationStateCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkPipelineRasterizationStateCreateFlags,
    pub(crate) depth_clamp_enable: VkBool32,
    pub(crate) rasterizer_discard_enable: VkBool32,
    pub(crate) polygon_mode: VkPolygonMode,
    pub(crate) cull_mode: VkCullModeFlags,
    pub(crate) front_face: VkFrontFace,
    pub(crate) depth_bias_enable: VkBool32,
    pub(crate) depth_bias_constant_factor: f32,
    pub(crate) depth_bias_clamp: f32,
    pub(crate) depth_bias_slope_factor: f32,
    pub(crate) line_width: f32,
}
