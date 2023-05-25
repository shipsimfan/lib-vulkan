use crate::{VK_MAX_DESCRIPTION_SIZE, VK_MAX_EXTENSION_NAME_SIZE};

#[repr(C)]
pub(crate) struct VkLayerProperties {
    pub(crate) layer_name: [u8; VK_MAX_EXTENSION_NAME_SIZE],
    pub(crate) spec_version: u32,
    pub(crate) implementation_version: u32,
    pub(crate) description: [u8; VK_MAX_DESCRIPTION_SIZE],
}
