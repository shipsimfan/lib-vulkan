use crate::{
    VkPipelineShaderStageCreateFlags, VkShaderModule, VkShaderStageFlagBits, VkSpecializationInfo,
    VkStructureType,
};
use std::ffi::{c_char, c_void};

#[repr(C)]
pub(crate) struct VkPipelineShaderStageCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkPipelineShaderStageCreateFlags,
    pub(crate) stage: VkShaderStageFlagBits,
    pub(crate) module: VkShaderModule,
    pub(crate) p_name: *const c_char,
    pub(crate) p_specialization_info: *const VkSpecializationInfo,
}
