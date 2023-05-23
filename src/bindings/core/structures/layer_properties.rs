use crate::{
    bindings::{VK_MAX_DESCRIPTION_SIZE, VK_MAX_EXTENSION_NAME_SIZE},
    VkVersion,
};
use std::ffi::CStr;

#[repr(C)]
pub struct VkLayerProperties {
    layer_name: [u8; VK_MAX_EXTENSION_NAME_SIZE],
    spec_version: VkVersion,
    implementation_version: u32,
    description: [u8; VK_MAX_DESCRIPTION_SIZE],
}

impl VkLayerProperties {
    pub fn layer_name(&self) -> &CStr {
        CStr::from_bytes_until_nul(&self.layer_name).unwrap()
    }

    pub fn spec_version(&self) -> VkVersion {
        self.spec_version
    }

    pub fn implementation_version(&self) -> u32 {
        self.implementation_version
    }

    pub fn description(&self) -> &CStr {
        CStr::from_bytes_until_nul(&self.description).unwrap()
    }
}
