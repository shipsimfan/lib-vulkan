use crate::VK_MAX_EXTENSION_NAME_SIZE;

#[repr(C)]
pub(crate) struct VkExtensionProperties {
    pub(crate) extension_name: [u8; VK_MAX_EXTENSION_NAME_SIZE],
    pub(crate) spec_version: u32,
}
