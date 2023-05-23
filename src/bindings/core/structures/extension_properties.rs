use crate::bindings::VK_MAX_EXTENSION_NAME_SIZE;
use std::ffi::CStr;

#[repr(C)]
pub struct VkExtensionProperties {
    extension_name: [u8; VK_MAX_EXTENSION_NAME_SIZE],
    spec_version: u32,
}

impl VkExtensionProperties {
    pub fn extension_name(&self) -> &CStr {
        CStr::from_bytes_until_nul(&self.extension_name).unwrap()
    }

    pub fn spec_version(&self) -> u32 {
        self.spec_version
    }
}
