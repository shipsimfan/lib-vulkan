use crate::bindings::VK_MAX_EXTENSION_NAME_SIZE;

#[repr(C)]
pub struct VkExtensionProperties {
    extension_name: [u8; VK_MAX_EXTENSION_NAME_SIZE],
    spec_version: u32,
}

impl VkExtensionProperties {
    pub fn extension_name(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.extension_name) }.trim_matches('\0')
    }

    pub fn spec_version(&self) -> u32 {
        self.spec_version
    }
}
