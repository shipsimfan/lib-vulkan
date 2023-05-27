use crate::{
    Device, Loader, NativeLoader, Result, VkBool32, VkResult, VkStructureType,
    VkSwapchainCreateFlagsKHR, VkSwapchainCreateInfoKHR, VkSwapchainKHR,
};
use std::{ptr::null, sync::Arc};

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
    ) -> Result<Self> {
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

        Ok(Swapchain { handle, device })
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
