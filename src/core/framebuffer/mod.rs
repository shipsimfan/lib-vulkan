use crate::{Device, ImageView, Loader, NativeLoader, RenderPass, Result, VkFramebuffer, VkResult};
use std::{ptr::null, sync::Arc};

mod create_info;
mod functions;

pub use create_info::FramebufferCreateInfo;

pub(crate) use functions::FramebufferFunctions;

pub struct Framebuffer<L: Loader = NativeLoader> {
    handle: VkFramebuffer,
    render_pass: Arc<RenderPass<L>>,
    image_views: Vec<ImageView<L>>,
    device: Arc<Device<L>>,
}

impl<L: Loader> Framebuffer<L> {
    pub(crate) fn create(
        device: Arc<Device<L>>,
        create_info: FramebufferCreateInfo<L>,
    ) -> Result<Self> {
        let (vk_create_info, _handles) = create_info.into_binding();

        let mut handle = None;
        let handle = match (device.framebuffer_functions().create_framebuffer)(
            device.handle(),
            &vk_create_info,
            null(),
            &mut handle,
        ) {
            VkResult::Success => handle.unwrap(),
            result => return Err(result),
        };

        let (render_pass, image_views) = create_info.consume();

        Ok(Framebuffer {
            handle,
            device,
            image_views,
            render_pass,
        })
    }
}

impl<L: Loader> Drop for Framebuffer<L> {
    fn drop(&mut self) {
        (self.device.framebuffer_functions().destroy_framebuffer)(
            self.device.handle(),
            self.handle,
            null(),
        )
    }
}
