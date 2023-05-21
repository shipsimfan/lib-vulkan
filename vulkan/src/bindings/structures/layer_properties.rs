use crate::{
    bindings::{VK_MAX_DESCRIPTION_SIZE, VK_MAX_EXTENSION_NAME_SIZE},
    VkVersion,
};

#[repr(C)]
#[derive(Clone)]
pub struct VkLayerProperties {
    layer_name: [u8; VK_MAX_EXTENSION_NAME_SIZE],
    spec_version: VkVersion,
    implementation_version: u32,
    description: [u8; VK_MAX_DESCRIPTION_SIZE],
}

impl VkLayerProperties {
    pub fn layer_name(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.layer_name) }
    }

    pub fn spec_version(&self) -> VkVersion {
        self.spec_version
    }

    pub fn implementation_version(&self) -> u32 {
        self.implementation_version
    }

    pub fn description(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.description) }
    }
}
