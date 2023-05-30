use crate::{
    VkBool32, VkPipelineInputAssemblyStateCreateFlags, VkPrimitiveTopology, VkStructureType,
};
use std::ffi::c_void;

#[repr(C)]
pub(crate) struct VkPipelineInputAssemblyStateCreateInfo {
    s_type: VkStructureType,
    p_next: *const c_void,
    flags: VkPipelineInputAssemblyStateCreateFlags,
    topology: VkPrimitiveTopology,
    primitive_restart_enable: VkBool32,
}
