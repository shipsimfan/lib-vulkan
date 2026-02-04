use crate::{
    VkBool32,
    ext_debug_utils::{
        VkDebugUtilsMessageSeverityFlagExt, VkDebugUtilsMessageTypeFlagsExt,
        VkDebugUtilsMessengerCallbackDataExt,
    },
};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_FALSE, VK_TRUE, ext_debug_utils,
    ext_debug_utils::{VkDebugUtilsMessageTypeFlagExt, VkDebugUtilsMessengerExt},
};

/// Application-defined debug messenger callback function
///
/// # Parameters
///  - `message_severity` specifies the [`VkDebugUtilsMessageSeverityFlagExt`] that triggered this
///    callback.
///  - `message_types` is a bitmask of [`VkDebugUtilsMessageTypeFlagExt`] specifying which type of
///    event(s) triggered this callback.
///  - `callback_data` contains all the callback related data in the
///    [`VkDebugUtilsMessengerCallbackDataExt`] structure.
///  - `user_data` is the user data provided when the [`VkDebugUtilsMessengerExt`] was created.
///
/// # Description
/// The callback returns a [`VkBool32`], which is interpreted in a layer-specified manner. The
/// application should always return [`VK_FALSE`]. The [`VK_TRUE`] value is reserved for use in
/// layer development.
///
/// # Valid Usage
///  - The callback must not make calls to any Vulkan commands
///
/// Provided by [`ext_debug_utils`]
pub type VkDebugUtilsMessengerCallbackExt = unsafe extern "system" fn(
    message_severity: VkDebugUtilsMessageSeverityFlagExt,
    message_types: VkDebugUtilsMessageTypeFlagsExt,
    callback_data: *const VkDebugUtilsMessengerCallbackDataExt,
    user_data: *mut c_void,
) -> VkBool32;
