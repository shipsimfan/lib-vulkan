use crate::{
    VkPipelineShaderStageCreateFlags, VkShaderModule, VkShaderStageFlagBits, VkSpecializationInfo,
    VkStructureType,
};
use std::ffi::{c_char, c_void};

pub(crate) struct VkPipelineShaderStageCreateInfo {
    s_type: VkStructureType,
    p_next: *const c_void,
    flags: VkPipelineShaderStageCreateFlags,
    stage: VkShaderStageFlagBits,
    module: VkShaderModule,
    p_name: *const c_char,
    p_specialization_info: *const VkSpecializationInfo,
}
