use crate::{VkVersion, VK_API_VERSION_1_0};

pub struct ApplicationInfo {
    pub name: Option<String>,
    pub version: Option<VkVersion>,
    pub engine_name: Option<String>,
    pub engine_version: Option<VkVersion>,
    pub api_version: VkVersion,
}

impl Default for ApplicationInfo {
    fn default() -> Self {
        ApplicationInfo {
            name: None,
            version: None,
            engine_name: None,
            engine_version: None,
            api_version: VK_API_VERSION_1_0,
        }
    }
}
