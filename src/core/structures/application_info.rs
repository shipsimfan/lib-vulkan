use crate::{VkStructureType, VK_VERSION_1_0};
use std::{ffi::c_char, os::raw::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::VkResult;

/// Structure specifying application information
///
/// # Description
/// Vulkan 1.0 implementations were required to return [`VkResult::VkErrorIncompatibleDriver`] if
/// `api_version` was larger than 1.0. Implementations that support Vulkan 1.1 or later must not
/// return [`VkResult::VkErrorIncompatibleDriver`] for any value of `api_version`.
///
/// As long as the instance supports at least Vulkan 1.1, an application can use different versions
/// of Vulkan with an instance than it does with a device or physical device.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkApplicationInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `application_name` is [`null`] or is a pointer to a null-terminated UTF-8 string containing
    /// the name of the application.
    pub application_name: *const c_char,

    /// `application_version` is an unsigned integer variable containing the developer-supplied
    /// version number of the application.
    pub application_version: u32,

    /// `engine_name` is [`null`] or is a pointer to a null-terminated UTF-8 string containing the
    /// name of the engine (if any) used to create the application.
    pub engine_name: *const c_char,

    /// `engine_version` is an unsigned integer variable containing the developer-supplied version
    /// number of the engine used to create the application.
    pub engine_version: u32,

    /// `api_version` must be the highest version of Vulkan that the application is designed to
    /// use. The patch version number specified in `api_version` is ignored when creating an
    /// instance object. The variant version of the instance must match that requested in
    /// `api_version`.
    pub api_version: u32,
}

impl Default for VkApplicationInfo {
    fn default() -> Self {
        VkApplicationInfo {
            r#type: VkStructureType::ApplicationInfo,
            next: null(),
            application_name: null(),
            application_version: 0,
            engine_name: null(),
            engine_version: 0,
            api_version: VK_VERSION_1_0,
        }
    }
}
