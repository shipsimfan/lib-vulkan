use crate::{
    Device, Image, Loader, NativeLoader, Result, VkBool32, VkResult, VkStructureType,
    VkSwapchainCreateFlagsKHR, VkSwapchainCreateInfoKHR, VkSwapchainKHR,
};
use std::{
    ptr::{null, null_mut},
    sync::Arc,
};

mod create_info;
mod functions;

pub use create_info::SwapchainCreateInfo;

pub(crate) use functions::SwapchainFunctions;

pub struct Swapchain<L: Loader = NativeLoader> {
    handle: VkSwapchainKHR,
    device: Arc<Device<L>>,
}

impl<L: Loader> Swapchain<L> {
    pub(crate) fn create_swapchain(
        device: Arc<Device<L>>,
        create_info: SwapchainCreateInfo,
    ) -> Result<Arc<Self>> {
        let create_swapchain = device.swapchain_functions().create_swapchain;

        let create_info = VkSwapchainCreateInfoKHR {
            s_type: VkStructureType::SwapchainCreateInfo,
            p_next: null(),
            flags: VkSwapchainCreateFlagsKHR::default(),
            surface: create_info.surface.handle(),
            min_image_count: create_info.min_image_count,
            image_format: create_info.image_format,
            image_color_space: create_info.image_color_space,
            image_extent: create_info.image_extent,
            image_array_layers: create_info.image_array_layers,
            image_usage: create_info.image_usage,
            image_sharing_mode: create_info.image_sharing_mode,
            queue_family_index_count: create_info.queue_family_indices.len() as u32,
            p_queue_family_indices: create_info.queue_family_indices.as_ptr(),
            pre_transform: create_info.pre_transform,
            composite_alpha: create_info.composite_alpha,
            present_mode: create_info.present_mode,
            clipped: create_info.clipped as VkBool32,
            old_swapchain: create_info
                .old_swapchain
                .map(|old_swapchain| old_swapchain.handle),
        };

        let mut swapchain = None;
        let handle = match (create_swapchain)(device.handle(), &create_info, null(), &mut swapchain)
        {
            VkResult::Success => swapchain.unwrap(),
            result => return Err(result),
        };

        Ok(Arc::new(Swapchain { handle, device }))
    }

    pub fn get_images(self: &Arc<Self>) -> Result<Vec<Arc<Image<L>>>> {
        let mut count = 0;
        match (self.device.swapchain_functions().get_swapchain_images)(
            self.device.handle(),
            self.handle,
            &mut count,
            null_mut(),
        ) {
            VkResult::Success => {}
            result => return Err(result),
        }

        let mut images = Vec::with_capacity(count as usize);
        loop {
            match (self.device.swapchain_functions().get_swapchain_images)(
                self.device.handle(),
                self.handle,
                &mut count,
                images.as_mut_ptr(),
            ) {
                VkResult::Success => {
                    unsafe { images.set_len(count as usize) };
                    return Ok(images
                        .into_iter()
                        .map(|image| Image::new(image, self.clone()))
                        .collect());
                }
                VkResult::Incomplete => {
                    images.reserve(count as usize);
                }
                result => return Err(result),
            }
        }
    }

    pub(crate) fn device(&self) -> &Device<L> {
        &self.device
    }
}

impl<L: Loader> Drop for Swapchain<L> {
    fn drop(&mut self) {
        (self.device.swapchain_functions().destroy_swapchain)(
            self.device.handle(),
            self.handle,
            null(),
        )
    }
}
