use crate::{
    VkDescriptorSetLayout, VkPipelineLayoutCreateFlags, VkPushConstantRange, VkStructureType,
};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkPipelineLayoutCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkPipelineLayoutCreateFlags,
    pub(crate) set_layout_count: u32,
    pub(crate) p_set_layouts: *const VkDescriptorSetLayout,
    pub(crate) push_constant_range_count: u32,
    pub(crate) p_push_constant_ranges: *const VkPushConstantRange,
}
