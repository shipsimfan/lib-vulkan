use crate::{
    VkPipelineVertexInputStateCreateFlags, VkStructureType, VkVertexInputAttributeDescription,
    VkVertexInputBindingDescription,
};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkPipelineVertexInputStateCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkPipelineVertexInputStateCreateFlags,
    pub(crate) vertex_binding_description_count: u32,
    pub(crate) p_vertex_binding_descriptions: *const VkVertexInputBindingDescription,
    pub(crate) vertex_attribute_description_count: u32,
    pub(crate) p_vertext_attribute_descriptions: *const VkVertexInputAttributeDescription,
}
