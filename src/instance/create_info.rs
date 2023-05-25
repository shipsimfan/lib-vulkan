use crate::{ApplicationInfo, InstanceCreateFlags};

pub struct InstanceCreateInfo {
    pub flags: InstanceCreateFlags,
    pub application_info: Option<ApplicationInfo>,
    pub enabled_layers: Vec<String>,
    pub enabled_extensions: Vec<String>,
}

impl Default for InstanceCreateInfo {
    fn default() -> Self {
        InstanceCreateInfo {
            flags: InstanceCreateFlags::default(),
            application_info: None,
            enabled_layers: Vec::new(),
            enabled_extensions: Vec::new(),
        }
    }
}
