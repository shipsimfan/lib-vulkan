use std::ptr::null;

use crate::{
    bindings::{
        VkPipelineVertexInputStateCreateFlags, VkPipelineVertexInputStateCreateInfo,
        VkStructureType,
    },
    VertexInputAttributeDescription, VertexInputBindingDescription,
};

#[derive(Clone)]
pub struct PipelineVertexInputStateCreateInfo {
    pub bindings: Vec<VertexInputBindingDescription>,
    pub attributes: Vec<VertexInputAttributeDescription>,
}

impl PipelineVertexInputStateCreateInfo {
    pub(super) fn into_binding(&self) -> VkPipelineVertexInputStateCreateInfo {
        VkPipelineVertexInputStateCreateInfo {
            s_type: VkStructureType::PipelineVertexInputStateCreateInfo,
            p_next: null(),
            flags: VkPipelineVertexInputStateCreateFlags::default(),
            vertex_binding_description_count: self.bindings.len() as u32,
            p_vertex_binding_descriptions: if self.bindings.len() == 0 {
                null()
            } else {
                self.bindings.as_ptr()
            },
            vertex_attribute_description_count: self.attributes.len() as u32,
            p_vertext_attribute_descriptions: if self.attributes.len() == 0 {
                null()
            } else {
                self.attributes.as_ptr()
            },
        }
    }
}

impl Default for PipelineVertexInputStateCreateInfo {
    fn default() -> Self {
        PipelineVertexInputStateCreateInfo {
            bindings: Vec::new(),
            attributes: Vec::new(),
        }
    }
}
