use crate::{Device, Loader, NativeLoader, Result, VkRenderPass, VkResult};
use std::{ptr::null, sync::Arc};

mod create_info;
mod functions;
mod subpass_description;

pub use create_info::RenderPassCreateInfo;
pub use subpass_description::SubpassDescription;

pub(crate) use functions::RenderPassFunctions;

pub struct RenderPass<L: Loader = NativeLoader> {
    handle: VkRenderPass,
    device: Arc<Device<L>>,
}

impl<L: Loader> RenderPass<L> {
    pub(crate) fn create(
        device: Arc<Device<L>>,
        create_info: RenderPassCreateInfo,
    ) -> Result<Arc<Self>> {
        let (create_info, _subpasses) = create_info.into_binding();

        let mut handle = None;
        let handle = match (device.render_pass_functions().create_render_pass)(
            device.handle(),
            &create_info,
            null(),
            &mut handle,
        ) {
            VkResult::Success => handle.unwrap(),
            result => return Err(result),
        };

        Ok(Arc::new(RenderPass { handle, device }))
    }

    pub(crate) fn handle(&self) -> VkRenderPass {
        self.handle
    }
}

impl<L: Loader> Drop for RenderPass<L> {
    fn drop(&mut self) {
        (self.device.render_pass_functions().destroy_render_pass)(
            self.device.handle(),
            self.handle,
            null(),
        )
    }
}
