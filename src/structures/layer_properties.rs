use crate::{VK_MAX_DESCRIPTION_SIZE, VK_MAX_EXTENSION_NAME_SIZE};
use std::ffi::c_char;

/// Structure specifying layer properties
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkLayerProperties {
    /// `layer_name` is an array of [`VK_MAX_EXTENSION_NAME_SIZE`] [`c_char`] containing a
    /// null-terminated UTF-8 string which is the name of the layer. Use this name in the
    /// `enabled_layer_names` array passed in the [`VkInstanceCreateInfo`] structure to enable this
    /// layer for an instance.
    pub layer_name: [c_char; VK_MAX_EXTENSION_NAME_SIZE],

    /// `spec_version` is the Vulkan version the layer was written to.
    pub spec_version: u32,

    /// `implementation_version` is the version of this layer. It is an integer, increasing with
    /// backward compatible changes.
    pub implementation_version: u32,

    /// `description` is an array of [`VK_MAX_DESCRIPTION_SIZE`] char containing a null-terminated
    /// UTF-8 string which provides additional details that can be used by the application to
    /// identify the layer.
    pub description: [c_char; VK_MAX_DESCRIPTION_SIZE],
}
