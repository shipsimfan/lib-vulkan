use crate::{
    Device, Loader, NativeLoader, Result, VkResult, VkShaderModule, VkShaderModuleCreateFlags,
    VkShaderModuleCreateInfo, VkStructureType,
};
use std::{ptr::null, sync::Arc};

mod functions;

pub(crate) use functions::ShaderModuleFunctions;

pub struct ShaderModule<L: Loader = NativeLoader> {
    handle: VkShaderModule,
    device: Arc<Device<L>>,
}

impl<L: Loader> ShaderModule<L> {
    pub(crate) fn create(device: Arc<Device<L>>, code: &[u8]) -> Result<Self> {
        let create_shader_module = device.shader_module_functions().create_shader_module;

        let create_info = VkShaderModuleCreateInfo {
            s_type: VkStructureType::ShaderModuleCreateInfo,
            p_next: null(),
            flags: VkShaderModuleCreateFlags::default(),
            code_size: code.len(),
            p_code: code.as_ptr().cast(),
        };

        let mut handle = None;
        let handle =
            match (create_shader_module)(device.handle(), &create_info, null(), &mut handle) {
                VkResult::Success => handle.unwrap(),
                result => return Err(result),
            };

        Ok(ShaderModule { handle, device })
    }
}

impl<L: Loader> Drop for ShaderModule<L> {
    fn drop(&mut self) {
        (self.device.shader_module_functions().destroy_shader_module)(
            self.device.handle(),
            self.handle,
            null(),
        )
    }
}
