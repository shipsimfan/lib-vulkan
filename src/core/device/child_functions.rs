use crate::{ImageViewFunctions, Instance, Loader, Result, SwapchainFunctions, VkDevice};

pub(super) struct ChildFunctions {
    pub(super) image_view_functions: ImageViewFunctions,

    pub(super) swapchain_functions: Option<SwapchainFunctions>,
}

impl ChildFunctions {
    pub(crate) fn load<L: Loader>(
        instance: &Instance<L>,
        device: VkDevice,
        extension_list: &[String],
    ) -> Result<Self> {
        let image_view_functions = ImageViewFunctions::load(instance, device)?;

        let mut swapchain_functions = None;

        for extension in extension_list {
            match extension.as_str() {
                "VK_KHR_swapchain" => {
                    swapchain_functions = Some(SwapchainFunctions::load(instance, device)?)
                }
                _ => {}
            }
        }

        Ok(ChildFunctions {
            image_view_functions,

            swapchain_functions,
        })
    }
}
