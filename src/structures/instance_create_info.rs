use crate::{VkApplicationInfo, VkInstanceCreateFlags, VkStructureType};
use std::{
    ffi::{c_char, c_void},
    ptr::null,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VkCreateInstance, VkInstanceCreateFlagBits, VK_VERSION_1_0};

/// Structure specifying parameters of a newly created instance
///
/// To capture events that occur while creating or destroying an instance, an application can link
/// a [`VkDebugReportCallbackCreateInfoEXT`] structure or a [`VkDebugUtilsMessengerCreateInfoEXT`]
/// structure to the `next` element of the [`VkInstanceCreateInfo`] structure given to
/// [`VkCreateInstance`]. This callback is only valid for the duration of the [`VkCreateInstance`]
/// and the [`VkDestroyInstance`] call. Use [`VkCreateDebugReportCallbackEXT`] or
/// [`VkCreateDebugUtilsMessengerEXT`] to create persistent callback objects.
///
/// An application can add additional drivers by including the [`VkDirectDriverLoadingListLUNARG`]
/// struct to the `next` element of the [`VkInstanceCreateInfo`] structure given to
/// [`VkCreateInstance`].
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkInstanceCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkInstanceCreateFlagBits`] indicating the behavior of the
    /// instance.
    pub flags: VkInstanceCreateFlags,

    /// `application_info` is [`null`] or a pointer to a [`VkApplicationInfo`] structure. If not
    /// [`null`], this information helps implementations recognize behavior inherent to classes of
    /// applications. [`VkApplicationInfo`] is defined in detail below.
    pub application_info: *const VkApplicationInfo,

    /// `enabled_layer_count` is the number of global layers to enable.
    pub enabled_layer_count: u32,

    /// `enabled_layer_names` is a pointer to an array of `enabled_layer_count` null-terminated
    /// UTF-8 strings containing the names of layers to enable for the created instance. The layers
    /// are loaded in the order they are listed in this array, with the first array element being
    /// the closest to the application, and the last array element being the closest to the driver.
    pub enabled_layer_names: *const *const c_char,

    /// `enabled_extension_count` is the number of global extensions to enable.
    pub enabled_extension_count: u32,

    /// `enabled_extension_names` is a pointer to an array of `enabled_extension_count`
    /// null-terminated UTF-8 strings containing the names of extensions to enable.
    pub enabled_extension_names: *const *const c_char,
}

impl Default for VkInstanceCreateInfo {
    fn default() -> Self {
        VkInstanceCreateInfo {
            r#type: VkStructureType::InstanceCreateInfo,
            next: null(),
            flags: 0,
            application_info: null(),
            enabled_layer_count: 0,
            enabled_layer_names: null(),
            enabled_extension_count: 0,
            enabled_extension_names: null(),
        }
    }
}
