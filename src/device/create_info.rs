use crate::{DeviceQueueCreateInfo, PhysicalDeviceFeatures};

pub struct DeviceCreateInfo {
    pub queue_create_infos: Vec<DeviceQueueCreateInfo>,
    pub enabled_layers: Vec<String>,
    pub enabled_extensions: Vec<String>,
    pub enabled_features: Option<PhysicalDeviceFeatures>,
}

impl Default for DeviceCreateInfo {
    fn default() -> Self {
        DeviceCreateInfo {
            queue_create_infos: Vec::new(),
            enabled_layers: Vec::new(),
            enabled_extensions: Vec::new(),
            enabled_features: None,
        }
    }
}
