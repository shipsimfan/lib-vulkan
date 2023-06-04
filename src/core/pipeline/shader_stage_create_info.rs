use crate::{
    bindings::{
        VkPipelineShaderStageCreateFlags, VkPipelineShaderStageCreateInfo, VkStructureType,
    },
    Loader, NativeLoader, ShaderModule, ShaderStageFlagBits,
};
use std::{ffi::CString, ptr::null, sync::Arc};

pub struct PipelineShaderStageCreateInfo<L: Loader = NativeLoader> {
    pub stage: ShaderStageFlagBits,
    pub module: Arc<ShaderModule<L>>,
    pub name: String,
}

impl<L: Loader> PipelineShaderStageCreateInfo<L> {
    pub(super) fn into_binding(&self) -> (VkPipelineShaderStageCreateInfo, CString) {
        let name = CString::new(self.name.clone()).unwrap();

        (
            VkPipelineShaderStageCreateInfo {
                s_type: VkStructureType::PipelineShaderStageCreateInfo,
                p_next: null(),
                flags: VkPipelineShaderStageCreateFlags::default(),
                stage: self.stage,
                module: self.module.handle(),
                p_name: name.as_ptr(),
                p_specialization_info: null(),
            },
            name,
        )
    }
}

impl<L: Loader> Clone for PipelineShaderStageCreateInfo<L> {
    fn clone(&self) -> Self {
        PipelineShaderStageCreateInfo {
            stage: self.stage,
            module: self.module.clone(),
            name: self.name.clone(),
        }
    }
}
