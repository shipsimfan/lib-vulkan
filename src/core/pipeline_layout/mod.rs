use crate::{
    Device, Loader, NativeLoader, Result, VkPipelineLayout, VkPipelineLayoutCreateFlags,
    VkPipelineLayoutCreateInfo, VkResult, VkStructureType,
};
use std::{ptr::null, sync::Arc};

mod create_info;
mod functions;

pub use create_info::PipelineLayoutCreateInfo;

pub(crate) use functions::PipelineLayoutFunctions;

pub struct PipelineLayout<L: Loader = NativeLoader> {
    handle: VkPipelineLayout,
    device: Arc<Device<L>>,
}

impl<L: Loader> PipelineLayout<L> {
    pub(crate) fn create(
        device: Arc<Device<L>>,
        create_info: PipelineLayoutCreateInfo,
    ) -> Result<Self> {
        let create_info = VkPipelineLayoutCreateInfo {
            s_type: VkStructureType::PipelineLayoutCreateInfo,
            p_next: null(),
            flags: VkPipelineLayoutCreateFlags::default(),
            set_layout_count: 0,
            p_set_layouts: null(),
            push_constant_range_count: create_info.push_constant_ranges.len() as u32,
            p_push_constant_ranges: if create_info.push_constant_ranges.len() == 0 {
                null()
            } else {
                create_info.push_constant_ranges.as_ptr()
            },
        };

        let mut handle = None;
        let handle = match (device.pipeline_layout_functions().create_pipeline_layout)(
            device.handle(),
            &create_info,
            null(),
            &mut handle,
        ) {
            VkResult::Success => handle.unwrap(),
            result => return Err(result),
        };

        Ok(PipelineLayout { handle, device })
    }
}

impl<L: Loader> Drop for PipelineLayout<L> {
    fn drop(&mut self) {
        (self
            .device
            .pipeline_layout_functions()
            .destroy_pipeline_layout)(self.device.handle(), self.handle, null());
    }
}
