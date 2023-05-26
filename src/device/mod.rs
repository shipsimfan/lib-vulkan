use crate::{
    Instance, Loader, NativeLoader, Queue, Result, VkCreateDevice, VkDevice, VkDeviceCreateFlags,
    VkDeviceCreateInfo, VkDeviceQueueCreateFlags, VkDeviceQueueCreateInfo, VkPhysicalDevice,
    VkResult, VkStructureType,
};
use functions::DeviceFunctions;
use std::{ffi::CString, ptr::null, sync::Arc};

mod create_info;
mod functions;
mod queue_create_info;

pub use create_info::DeviceCreateInfo;
pub use queue_create_info::DeviceQueueCreateInfo;

pub struct Device<L: Loader = NativeLoader> {
    handle: VkDevice,
    instance: Arc<Instance<L>>,

    functions: DeviceFunctions,
}

impl<L: Loader> Device<L> {
    pub(crate) fn create_device(
        physical_device: VkPhysicalDevice,
        instance: Arc<Instance<L>>,
        create_device: VkCreateDevice,
        create_info: DeviceCreateInfo,
    ) -> Result<Arc<Self>> {
        // Prepare the create info
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

        let enabled_layers: Vec<_> = create_info
            .enabled_layers
            .into_iter()
            .map(|layer| CString::new(layer).unwrap())
            .collect();
        let enabled_layer_ptrs: Vec<_> =
            enabled_layers.iter().map(|layer| layer.as_ptr()).collect();
        let enabled_extensions: Vec<_> = create_info
            .enabled_extensions
            .into_iter()
            .map(|extension| CString::new(extension).unwrap())
            .collect();
        let enabled_extension_ptrs: Vec<_> = enabled_extensions
            .iter()
            .map(|extension| extension.as_ptr())
            .collect();

        let enabled_features = create_info
            .enabled_features
            .map(|enabled_features| enabled_features.into());

        let create_info = VkDeviceCreateInfo {
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
        let handle = match (create_device)(physical_device, &create_info, null(), &mut handle) {
            VkResult::Success => handle.unwrap(),
            result => return Err(result),
        };

        // Create the device
        let functions = DeviceFunctions::load(&instance, handle)?;

        Ok(Arc::new(Device {
            handle,
            instance,

            functions,
        }))
    }

    pub fn get_queue(self: &Arc<Self>, queue_family_index: u32, queue_index: u32) -> Queue<L> {
        let mut handle = None;
        (self.functions.get_queue)(self.handle, queue_family_index, queue_index, &mut handle);
        Queue::new(handle.unwrap(), self.clone())
    }
}

impl<L: Loader> Drop for Device<L> {
    fn drop(&mut self) {
        (self.functions.destroy_device)(self.handle, null())
    }
}
