use crate::{
    string_vec_to_cstring_vec, GraphicsPipelineCreateInfo, ImageView, ImageViewCreateInfo,
    ImageViewFunctions, Instance, Loader, NativeLoader, Pipeline, PipelineFunctions,
    PipelineLayout, PipelineLayoutCreateInfo, PipelineLayoutFunctions, Queue, RenderPass,
    RenderPassCreateInfo, RenderPassFunctions, Result, ShaderModule, ShaderModuleFunctions,
    Swapchain, SwapchainCreateInfo, SwapchainFunctions, VkCreateDevice, VkDevice,
    VkDeviceCreateFlags, VkDeviceCreateInfo, VkDeviceQueueCreateFlags, VkDeviceQueueCreateInfo,
    VkPhysicalDevice, VkResult, VkStructureType,
};
use child_functions::ChildFunctions;
use functions::DeviceFunctions;
use std::{ptr::null, sync::Arc};

mod child_functions;
mod create_info;
mod functions;
mod queue_create_info;

pub use create_info::DeviceCreateInfo;
pub use queue_create_info::DeviceQueueCreateInfo;

pub struct Device<L: Loader = NativeLoader> {
    handle: VkDevice,
    #[allow(unused)]
    instance: Arc<Instance<L>>,

    functions: DeviceFunctions,
    child_functions: ChildFunctions,
}

impl<L: Loader> Device<L> {
    pub(crate) fn create(
        physical_device: VkPhysicalDevice,
        instance: Arc<Instance<L>>,
        create_device: VkCreateDevice,
        create_info: DeviceCreateInfo,
    ) -> Result<Arc<Self>> {
        // Prepare the queue create infos
        let queue_create_infos: Vec<_> = create_info
            .queue_create_infos
            .iter()
            .map(|queue_create_info| VkDeviceQueueCreateInfo {
                s_type: VkStructureType::DeviceQueueCreateInfo,
                p_next: null(),
                flags: VkDeviceQueueCreateFlags::default(),
                queue_family_index: queue_create_info.family_index,
                queue_count: queue_create_info.priorities.len() as u32,
                queue_priorities: queue_create_info.priorities.as_ptr(),
            })
            .collect();

        // Prepare the create info
        let (enabled_layers, enabled_layer_ptrs) =
            string_vec_to_cstring_vec!(create_info.enabled_layers);
        let (enabled_extensions, enabled_extension_ptrs) =
            string_vec_to_cstring_vec!(create_info.enabled_extensions.clone());
        let enabled_features = create_info
            .enabled_features
            .map(|enabled_features| enabled_features.into());

        let vk_create_info = VkDeviceCreateInfo {
            s_type: VkStructureType::DeviceCreateInfo,
            p_next: null(),
            flags: VkDeviceCreateFlags::default(),
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            enabled_layer_count: enabled_layers.len() as u32,
            pp_enabled_layer_names: enabled_layer_ptrs.as_ptr(),
            enabled_extension_count: enabled_extensions.len() as u32,
            pp_enabled_extension_names: enabled_extension_ptrs.as_ptr(),
            p_enabled_features: enabled_features
                .as_ref()
                .map(|enabled_features| enabled_features as *const _)
                .unwrap_or(null()),
        };

        // Call the vkCreateDevice function
        let mut handle = None;
        let handle = match (create_device)(physical_device, &vk_create_info, null(), &mut handle) {
            VkResult::Success => handle.unwrap(),
            result => return Err(result),
        };

        // Create the device
        let functions = DeviceFunctions::load(&instance, handle)?;
        let child_functions =
            ChildFunctions::load(&instance, handle, &create_info.enabled_extensions)?;

        Ok(Arc::new(Device {
            handle,
            instance,

            functions,
            child_functions,
        }))
    }

    pub fn get_queue(self: &Arc<Self>, queue_family_index: u32, queue_index: u32) -> Queue<L> {
        let mut handle = None;
        (self.functions.get_queue)(self.handle, queue_family_index, queue_index, &mut handle);
        Queue::new(handle.unwrap(), self.clone())
    }

    pub fn create_swapchain(
        self: &Arc<Self>,
        create_info: SwapchainCreateInfo,
    ) -> Result<Arc<Swapchain<L>>> {
        Swapchain::create(self.clone(), create_info)
    }

    pub fn create_image_view(&self, create_info: ImageViewCreateInfo<L>) -> Result<ImageView<L>> {
        ImageView::create(self, create_info)
    }

    pub fn create_pipeline(
        self: &Arc<Self>,
        create_info: GraphicsPipelineCreateInfo<L>,
    ) -> Result<Pipeline<L>> {
        Pipeline::create(self.clone(), create_info)
    }

    pub fn create_pipeline_layout(
        self: &Arc<Self>,
        create_info: PipelineLayoutCreateInfo,
    ) -> Result<Arc<PipelineLayout<L>>> {
        PipelineLayout::create(self.clone(), create_info)
    }

    pub fn create_render_pass(
        self: &Arc<Self>,
        create_info: RenderPassCreateInfo,
    ) -> Result<Arc<RenderPass<L>>> {
        RenderPass::create(self.clone(), create_info)
    }

    pub fn create_shader_module(self: &Arc<Self>, code: &[u8]) -> Result<Arc<ShaderModule<L>>> {
        ShaderModule::create(self.clone(), code)
    }

    pub(crate) fn image_view_functions(&self) -> &ImageViewFunctions {
        &self.child_functions.image_view_functions
    }

    pub(crate) fn pipeline_functions(&self) -> &PipelineFunctions {
        &self.child_functions.pipeline_functions
    }

    pub(crate) fn pipeline_layout_functions(&self) -> &PipelineLayoutFunctions {
        &self.child_functions.pipeline_layout_functions
    }

    pub(crate) fn render_pass_functions(&self) -> &RenderPassFunctions {
        &self.child_functions.render_pass_functions
    }

    pub(crate) fn shader_module_functions(&self) -> &ShaderModuleFunctions {
        &self.child_functions.shader_module_functions
    }

    pub(crate) fn swapchain_functions(&self) -> &SwapchainFunctions {
        self.child_functions.swapchain_functions.as_ref().unwrap()
    }

    pub(crate) fn handle(&self) -> VkDevice {
        self.handle
    }
}

impl<L: Loader> Drop for Device<L> {
    fn drop(&mut self) {
        (self.functions.destroy_device)(self.handle, null())
    }
}
