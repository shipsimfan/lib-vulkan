use crate::{Instance, Loader, Result, VkCreateImageView, VkDestroyImageView, VkDevice};

pub(crate) struct ImageViewFunctions {
    pub(super) create_image_view: VkCreateImageView,
    pub(super) destroy_image_view: VkDestroyImageView,
}

macro_rules! load_function {
    ($instance: ident, $device: ident, $name: literal) => {
        $instance
            .get_device_proc_addr($device, $name)
            .map(|function| unsafe { std::mem::transmute(function) })
            .ok_or(crate::VkResult::IncompatibleDriver)
    };
}

impl ImageViewFunctions {
    pub(crate) fn load<L: Loader>(instance: &Instance<L>, device: VkDevice) -> Result<Self> {
        let create_image_view = load_function!(instance, device, "vkCreateImageView")?;
        let destroy_image_view = load_function!(instance, device, "vkDestroyImageView")?;

        Ok(ImageViewFunctions {
            create_image_view,
            destroy_image_view,
        })
    }
}
