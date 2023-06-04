use crate::{
    PrimitiveTopology, VkPipelineInputAssemblyStateCreateFlags,
    VkPipelineInputAssemblyStateCreateInfo, VkStructureType,
};
use std::ptr::null;

pub struct PipelineInputAssemblyStateCreateInfo {
    pub topology: PrimitiveTopology,
    pub primitive_restart_enabled: bool,
}

impl PipelineInputAssemblyStateCreateInfo {
    pub(super) fn into_binding(&self) -> VkPipelineInputAssemblyStateCreateInfo {
        VkPipelineInputAssemblyStateCreateInfo {
            s_type: VkStructureType::PipelineInputAssemblyStateCreaetInfo,
            p_next: null(),
            flags: VkPipelineInputAssemblyStateCreateFlags::default(),
            topology: self.topology,
            primitive_restart_enable: self.primitive_restart_enabled as u32,
        }
    }
}

impl Default for PipelineInputAssemblyStateCreateInfo {
    fn default() -> Self {
        PipelineInputAssemblyStateCreateInfo {
            topology: PrimitiveTopology::TriangleList,
            primitive_restart_enabled: false,
        }
    }
}
