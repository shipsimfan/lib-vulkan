use crate::{Instance, Loader, Result, SwapchainFunctions, VkDevice};

pub(super) struct ChildFunctions {
    pub(super) swapchain_functions: Option<SwapchainFunctions>,
}

impl ChildFunctions {
    pub(crate) fn load<L: Loader>(
        instance: &Instance<L>,
        device: VkDevice,
        extension_list: &[String],
    ) -> Result<Self> {
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
            swapchain_functions,
        })
    }
}
