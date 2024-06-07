use crate::VK_MAX_EXTENSION_NAME_SIZE;
use std::ffi::c_char;

/// Structure specifying an extension properties
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkExtensionProperties {
    /// `extension_name` is an array of [`VK_MAX_EXTENSION_NAME_SIZE`] [`c_char`] containing a
    /// null-terminated UTF-8 string which is the name of the extension.
    pub extension_name: [c_char; VK_MAX_EXTENSION_NAME_SIZE],

    /// `spec_version` is the version of this extension. It is an integer, incremented with 
    /// backward compatible changes.
    pub spec_version: u32,
}

impl Default for VkExtensionProperties {
    fn default() -> Self {
        VkExtensionProperties {
            extension_name: [0; VK_MAX_EXTENSION_NAME_SIZE],
            spec_version: 0,
        }
    }
}
