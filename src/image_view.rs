use crate::{
    bindings::{self, VkCreateImageView, VkDestroyImageView, VkGetDeviceProcAddr},
    get_device_proc_addr, Loader, NativeLoader, Result, VkDevice,
};
use std::sync::Arc;

pub struct VkImageView<L: Loader = NativeLoader> {
    inner: bindings::VkImageView,
    device: Arc<VkDevice<L>>,
}

pub(crate) struct VkImageViewFunctions {
    pub(crate) create_image_view: VkCreateImageView,
    destroy_image_view: VkDestroyImageView,
}

impl<L: Loader> VkImageView<L> {
    pub(crate) fn new(inner: bindings::VkImageView, device: Arc<VkDevice<L>>) -> Self {
        VkImageView { inner, device }
    }
}

impl<L: Loader> Drop for VkImageView<L> {
    fn drop(&mut self) {
        (self.device.image_view_functions().destroy_image_view)(
            self.device.inner(),
            self.inner,
            None,
        );
    }
}

impl VkImageViewFunctions {
    pub(crate) fn get(
        get_proc_addr: VkGetDeviceProcAddr,
        device: bindings::VkDevice,
    ) -> Result<Self> {
        let create_image_view = get_device_proc_addr!(get_proc_addr, device, "vkCreateImageView")?;
        let destroy_image_view =
            get_device_proc_addr!(get_proc_addr, device, "vkDestroyImageView")?;

        Ok(VkImageViewFunctions {
            create_image_view,
            destroy_image_view,
        })
    }
}
