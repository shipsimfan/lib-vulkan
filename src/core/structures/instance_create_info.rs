use crate::{VkApplicationInfo, VkInstanceCreateFlags, VkStructureType, util::NextChain};
use std::{
    ffi::{c_char, c_void},
    ptr::null,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkCreateInstance, VkInstanceCreateFlag};

/// Structure specifying parameters of a newly created instance
///
/// # Description
/// To capture events that occur while creating or destroying an instance, an application can link
/// a [`VkDebugReportCallbackCreateInfoEXT`] structure or a [`VkDebugUtilsMessengerCreateInfoEXT`]
/// structure to the `next` element of the [`VkInstanceCreateInfo`] structure given to
/// [`VkCreateInstance`]. This callback is only valid for the duration of the [`VkCreateInstance`]
/// and the [`VkDestroyInstance`] call. Use [`VkCreateDebugReportCallbackEXT`] or
/// [`VkCreateDebugUtilsMessengerEXT`] to create persistent callback objects.
///
/// An application can add additional drivers by including the [`VkDirectDriverLoadingListLunarG`]
/// struct to the `next` element of the [`VkInstanceCreateInfo`] structure given to
/// [`VkCreateInstance`].
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkInstanceCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::InstanceCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If the `next` chain includes a [`VkExportMetalObjectCreateInfoExt`] structure, its
    ///    `export_object_type` member must be either
    ///    [`VkExportMetalObjectTypeFlagExt::MetalDeviceExt`] or
    ///    [`VkExportMetalObjectTypeFlagExt::MetalCommandQueueExt`]
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of
    ///    [`VkDebugReportCallbackCreateInfoExt`], [`VkDebugUtilsMessengerCreateInfoExt`],
    ///    [`VkDirectDriverLoadingListLunarG`], [`VkExportMetalObjectCreateInfoExt`],
    ///    [`VkLayerSettingsCreateInfoExt`], [`VkValidationFeaturesExt`], or
    ///    [`VkValidationFlagsExt`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique, with the
    ///    exception of structures of type [`VkDebugUtilsMessengerCreateInfoExt`],
    ///    [`VkExportMetalObjectCreateInfoExt`], or [`VkLayerSettingsCreateInfoExt`]
    pub next: *const c_void,

    /// `flags` is a bitmask of [`VkInstanceCreateFlag`] indicating the behavior of the
    /// instance.
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be a valid combination of [`VkInstanceCreateFlag`] values
    pub flags: VkInstanceCreateFlags,

    /// `application_info` is [`null`] or a pointer to a [`VkApplicationInfo`] structure. If not
    /// [`null`], this information helps implementations recognize behavior inherent to classes of
    /// applications. [`VkApplicationInfo`] is defined in detail below.
    ///
    /// # Valid Usage (Implicit)
    ///  - If `application_info` is not [`null`], `application_info` must be a valid pointer to a
    ///    valid [`VkApplicationInfo`] structure
    pub application_info: *const VkApplicationInfo,

    /// `enabled_layer_count` is the number of global layers to enable.
    pub enabled_layer_count: u32,

    /// `enabled_layer_names` is a pointer to an array of `enabled_layer_count` null-terminated
    /// UTF-8 strings containing the names of layers to enable for the created instance. The layers
    /// are loaded in the order they are listed in this array, with the first array element being
    /// the closest to the application, and the last array element being the closest to the driver.
    ///
    /// # Valid Usage (Implicit)
    ///  - If `enabled_layer_count` is not 0, `enabled_layer_names` must be a valid pointer to an
    ///    array of `enabled_layer_count` null-terminated UTF-8 strings
    pub enabled_layer_names: *const *const c_char,

    /// `enabled_extension_count` is the number of global extensions to enable.
    pub enabled_extension_count: u32,

    /// `enabled_extension_names` is a pointer to an array of `enabled_extension_count`
    /// null-terminated UTF-8 strings containing the names of extensions to enable.
    ///
    /// # Valid Usage
    ///  - If the `next` chain of [`VkInstanceCreateInfo`] includes a
    ///    [`VkDebugReportCallbackCreateInfoExt`] structure, the list of enabled extensions in
    ///    `enabled_extension_names` must contain "VK_EXT_debug_report"
    ///  - If the `next` chain of [`VkInstanceCreateInfo`] includes a
    ///    [`VkDebugUtilsMessengerCreateInfoExt`] structure, the list of enabled extensions in
    ///    `enabled_extension_names` must contain "VK_EXT_debug_utils"
    ///  - If flags has the [`VkInstanceCreateFlag::EnumeratePortabilityKhr`] bit set, the list
    ///    of enabled extensions in `enabled_extension_names` must contain
    ///    "VK_KHR_portability_enumeration"
    ///  - If the `next` chain of [`VkInstanceCreateInfo`] includes a
    ///    [`VkDirectDriverLoadingListLunarG`] structure, the list of enabled extensions in
    ///    `enabled_extension_names` must contain "VK_LUNARG_direct_driver_loading"
    ///  - If the `next` chain of [`VkInstanceCreateInfo`] includes a
    ///    [`VkLayerSettingsCreateInfoExt`] structure, the list of enabled extensions in
    ///    `enabled_extension_names` must contain "VK_EXT_layer_settings"
    ///  - If the `next` chain of [`VkInstanceCreateInfo`] includes a [`VkValidationFeaturesExt`]
    ///    structure, the list of enabled extensions in `enabled_extension_names` must contain
    ///    "VK_EXT_validation_features"
    ///  - If the `next` chain of [`VkInstanceCreateInfo`] includes a [`VkValidationFlagsExt`]
    ///    structure, the list of enabled extensions in `enabled_extension_names` must contain
    ///    "VK_EXT_validation_flags"
    ///
    /// # Valid Usage (Implicit)
    ///  - If `enabled_extension_count` is not 0, `enabled_extension_names` must be a valid pointer
    ///    to an array of `enabled_extension_count` null-terminated UTF-8 strings
    pub enabled_extension_names: *const *const c_char,
}

impl const Default for VkInstanceCreateInfo {
    fn default() -> Self {
        VkInstanceCreateInfo {
            r#type: VkStructureType::InstanceCreateInfo,
            next: null(),
            flags: VkInstanceCreateFlags::empty(),
            application_info: null(),
            enabled_layer_count: 0,
            enabled_layer_names: null(),
            enabled_extension_count: 0,
            enabled_extension_names: null(),
        }
    }
}

impl NextChain for VkInstanceCreateInfo {
    fn structure_type(&self) -> VkStructureType {
        self.r#type
    }

    fn next(&self) -> *const c_void {
        self.next
    }
    
    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: Option<&dyn NextChain>) {
        self.next = next.map_or(null(), |n| n.as_ptr());
    }
}
