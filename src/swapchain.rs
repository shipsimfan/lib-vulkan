use crate::{
    bindings::{
        self, VkAcquireNextImageKHR, VkCreateSwapchainKHR, VkDestroySwapchainKHR,
        VkGetDeviceProcAddr, VkGetSwapchainImagesKHR, VkQueuePresentKHR,
    },
    get_device_proc_addr, Loader, NativeLoader, Result, VkDevice, VkFence, VkImage, VkResult,
    VkSemaphore,
};
use std::{ptr::NonNull, sync::Arc};

pub struct VkSwapchainKHR<L: Loader = NativeLoader> {
    inner: bindings::VkSwapchainKHR,
    device: Arc<VkDevice<L>>,
}

pub(crate) struct VkSwapchainKHRFunctions {
    pub(crate) create_swapchain: VkCreateSwapchainKHR,
    pub(crate) queue_present: VkQueuePresentKHR,

    acquire_next_image: VkAcquireNextImageKHR,
    destroy_swapchain: VkDestroySwapchainKHR,
    get_swapchain_images: VkGetSwapchainImagesKHR,
}

impl<L: Loader> VkSwapchainKHR<L> {
    pub(crate) fn new(inner: bindings::VkSwapchainKHR, device: Arc<VkDevice<L>>) -> Self {
        VkSwapchainKHR { inner, device }
    }

    pub fn acquire_next_image(
        &self,
        timeout: u64,
        semaphore: Option<&VkSemaphore>,
        fence: Option<&VkFence>,
    ) -> Result<u32> {
        let mut image_index = 0;
        match (self
            .device
            .swapchain_functions()
            .unwrap()
            .acquire_next_image)(
            self.device.inner(),
            self.inner,
            timeout,
            semaphore.map(|semaphore| semaphore.inner()),
            fence.map(|fence| fence.inner()),
            &mut image_index,
        ) {
            VkResult::Success => Ok(image_index),
            result => Err(result),
        }
    }

    pub fn get_swapchain_images(&self) -> Result<Vec<VkImage<L>>> {
        let mut count = 0;
        match (self
            .device
            .swapchain_functions()
            .unwrap()
            .get_swapchain_images)(self.device.inner(), self.inner, &mut count, None)
        {
            VkResult::Success => {}
            result => return Err(result),
        }

        let mut images = Vec::with_capacity(count as usize);
        match (self
            .device
            .swapchain_functions()
            .unwrap()
            .get_swapchain_images)(
            self.device.inner(),
            self.inner,
            &mut count,
            Some(unsafe { NonNull::new_unchecked(images.as_mut_ptr()) }),
        ) {
            VkResult::Success | VkResult::Incomplete => {
                unsafe { images.set_len(count as usize) };
                Ok(images
                    .into_iter()
                    .map(|image| VkImage::new(image, self.device.clone()))
                    .collect())
            }
            result => Err(result),
        }
    }

    pub(crate) fn inner(&self) -> bindings::VkSwapchainKHR {
        self.inner
    }
}

impl<L: Loader> Drop for VkSwapchainKHR<L> {
    fn drop(&mut self) {
        (self.device.swapchain_functions().unwrap().destroy_swapchain)(
            self.device.inner(),
            self.inner,
            None,
        );
    }
}

impl VkSwapchainKHRFunctions {
    pub(crate) fn get(
        get_proc_addr: VkGetDeviceProcAddr,
        device: bindings::VkDevice,
    ) -> Result<Self> {
        let acquire_next_image =
            get_device_proc_addr!(get_proc_addr, device, "vkAcquireNextImageKHR")?;
        let create_swapchain =
            get_device_proc_addr!(get_proc_addr, device, "vkCreateSwapchainKHR")?;
        let destroy_swapchain =
            get_device_proc_addr!(get_proc_addr, device, "vkDestroySwapchainKHR")?;
        let get_swapchain_images =
            get_device_proc_addr!(get_proc_addr, device, "vkGetSwapchainImagesKHR")?;
        let queue_present = get_device_proc_addr!(get_proc_addr, device, "vkQueuePresentKHR")?;

        Ok(VkSwapchainKHRFunctions {
            acquire_next_image,
            create_swapchain,
            destroy_swapchain,
            get_swapchain_images,
            queue_present,
        })
    }
}
