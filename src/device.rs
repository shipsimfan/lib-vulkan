use crate::{
    bindings::{self, VkDestroyDevice, VkGetDeviceQueue},
    get_device_proc_addr,
    image_view::VkImageViewFunctions,
    swapchain::VkSwapchainKHRFunctions,
    Loader, NativeLoader, Result, VkImageView, VkImageViewCreateInfo, VkInstance, VkQueue,
    VkResult, VkSwapchainCreateInfoKHR, VkSwapchainKHR,
};
use std::sync::Arc;

pub struct VkDevice<L: Loader = NativeLoader> {
    inner: bindings::VkDevice,
    #[allow(unused)]
    instance: Arc<VkInstance<L>>,

    destroy_device: VkDestroyDevice,
    get_device_queue: VkGetDeviceQueue,

    image_view_functions: VkImageViewFunctions,
    swapchain_functions: Option<VkSwapchainKHRFunctions>,
}

impl<L: Loader> VkDevice<L> {
    pub(crate) fn new(
        inner: bindings::VkDevice,
        instance: Arc<VkInstance<L>>,
        swapchain_enabled: bool,
    ) -> Result<Arc<Self>> {
        let get_proc_addr = instance.get_device_proc_addr();

        let destroy_device = get_device_proc_addr!(get_proc_addr, inner, "vkDestroyDevice")?;
        let get_device_queue = get_device_proc_addr!(get_proc_addr, inner, "vkGetDeviceQueue")?;

        let image_view_functions = VkImageViewFunctions::get(get_proc_addr, inner)?;
        let swapchain_functions = if swapchain_enabled {
            Some(VkSwapchainKHRFunctions::get(get_proc_addr, inner)?)
        } else {
            None
        };

        Ok(Arc::new(VkDevice {
            inner,
            instance,

            destroy_device,
            get_device_queue,

            image_view_functions,
            swapchain_functions,
        }))
    }

    pub fn get_device_queue(
        self: &Arc<Self>,
        queue_family_index: u32,
        queue_index: u32,
    ) -> VkQueue<L> {
        let mut queue = None;
        (self.get_device_queue)(self.inner, queue_family_index, queue_index, &mut queue);
        VkQueue::new(queue.unwrap(), self.clone())
    }

    pub fn create_image_view(
        self: &Arc<Self>,
        create_info: &VkImageViewCreateInfo,
    ) -> Result<VkImageView<L>> {
        let mut view = None;
        match (self.image_view_functions.create_image_view)(
            self.inner,
            create_info,
            None,
            &mut view,
        ) {
            VkResult::Success => Ok(VkImageView::new(view.unwrap(), self.clone())),
            result => Err(result),
        }
    }

    pub fn create_swapchain(
        self: &Arc<Self>,
        create_info: &VkSwapchainCreateInfoKHR,
    ) -> Result<VkSwapchainKHR<L>> {
        let mut swapchain = None;
        match (self.swapchain_functions.as_ref().unwrap().create_swapchain)(
            self.inner,
            create_info,
            None,
            &mut swapchain,
        ) {
            VkResult::Success => Ok(VkSwapchainKHR::new(swapchain.unwrap(), self.clone())),
            result => Err(result),
        }
    }

    pub(crate) fn inner(&self) -> bindings::VkDevice {
        self.inner
    }

    pub(crate) fn image_view_functions(&self) -> &VkImageViewFunctions {
        &self.image_view_functions
    }

    pub(crate) fn swapchain_functions(&self) -> Option<&VkSwapchainKHRFunctions> {
        self.swapchain_functions.as_ref()
    }
}

impl<L: Loader> Drop for VkDevice<L> {
    fn drop(&mut self) {
        (self.destroy_device)(self.inner, None)
    }
}
