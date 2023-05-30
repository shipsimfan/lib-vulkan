use crate::{
    VkBool32, VkPipelineMultisampleStateCreateFlags, VkSampleCountFlagBits, VkSampleMask,
    VkStructureType,
};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkPipelineMultisampleStateCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkPipelineMultisampleStateCreateFlags,
    pub(crate) rasterization_samples: VkSampleCountFlagBits,
    pub(crate) sample_shading_enable: VkBool32,
    pub(crate) min_shading_enable: f32,
    pub(crate) p_sample_mask: *const VkSampleMask,
    pub(crate) alpha_to_converage_enable: VkBool32,
    pub(crate) alpha_to_one_enable: VkBool32,
}
