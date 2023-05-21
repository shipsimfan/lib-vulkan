use crate::{bindings::VK_MAX_EXTENSION_NAME_SIZE, VkVersion};

#[repr(C)]
pub struct VkExtensionProperties {
    extension_name: [u8; VK_MAX_EXTENSION_NAME_SIZE],
    spec_version: u32,
}

impl VkExtensionProperties {
    pub fn extension_name(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.extension_name) }
    }

    pub fn spec_version(&self) -> u32 {
        self.spec_version
    }
}
