use crate::{
    VkBool32, VkPipelineInputAssemblyStateCreateFlags, VkPrimitiveTopology, VkStructureType,
};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkPipelineInputAssemblyStateCreateInfo {
    pub(crate) s_type: VkStructureType,
    pub(crate) p_next: *const c_void,
    pub(crate) flags: VkPipelineInputAssemblyStateCreateFlags,
    pub(crate) topology: VkPrimitiveTopology,
    pub(crate) primitive_restart_enable: VkBool32,
}
