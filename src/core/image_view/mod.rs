use crate::{
    Device, Image, Loader, NativeLoader, Result, VkImageView, VkImageViewCreateFlags,
    VkImageViewCreateInfo, VkResult, VkStructureType,
};
use std::{ptr::null, sync::Arc};

mod create_info;
mod functions;

pub use create_info::ImageViewCreateInfo;

pub(crate) use functions::ImageViewFunctions;

pub struct ImageView<L: Loader = NativeLoader> {
    handle: VkImageView,
    image: Arc<Image<L>>,
}

impl<L: Loader> ImageView<L> {
    pub(crate) fn create_image_view(
        device: &Device<L>,
        create_info: ImageViewCreateInfo<L>,
    ) -> Result<Self> {
        let create_image_view = device.image_view_functions().create_image_view;

        let vk_create_info = VkImageViewCreateInfo {
            s_type: VkStructureType::ImageViewCreateInfo,
            p_next: null(),
            flags: VkImageViewCreateFlags::default(),
            image: create_info.image.handle(),
            view_type: create_info.r#type,
            format: create_info.format,
            components: create_info.components,
            subresource_range: create_info.subresource_range,
        };

        let mut handle = None;
        let handle =
            match (create_image_view)(device.handle(), &vk_create_info, null(), &mut handle) {
                VkResult::Success => handle.unwrap(),
                result => return Err(result),
            };

        Ok(ImageView {
            handle,
            image: create_info.image,
        })
    }
}

impl<L: Loader> Drop for ImageView<L> {
    fn drop(&mut self) {
        (self
            .image
            .swapchain()
            .device()
            .image_view_functions()
            .destroy_image_view)(
            self.image.swapchain().device().handle(),
            self.handle,
            null(),
        )
    }
}
