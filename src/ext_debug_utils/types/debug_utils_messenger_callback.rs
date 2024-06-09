use crate::{
    VkBool32, VkDebugUtilsMessageSeverityFlagBitsEXT, VkDebugUtilsMessageTypeFlagsEXT,
    VkDebugUtilsMessengerCallbackDataEXT,
};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    ext_debug_utils, VkDebugUtilsMessageTypeFlagBitsEXT, VkDebugUtilsMessengerEXT, VK_FALSE,
    VK_TRUE,
};

/// Application-defined debug messenger callback function
///
/// # Parameters
///  - `message_severity` specifies the [`VkDebugUtilsMessageSeverityFlagBitsEXT`] that triggered
///    this callback.
///  - `message_types` is a bitmask of [`VkDebugUtilsMessageTypeFlagBitsEXT`] specifying which type
///    of event(s) triggered this callback.
///  - `callback_data` contains all the callback related data in the
///    [`VkDebugUtilsMessengerCallbackDataEXT`] structure.
///  - `user_data` is the user data provided when the [`VkDebugUtilsMessengerEXT`] was created.
///
/// # Description
/// The callback returns a [`VkBool32`], which is interpreted in a layer-specified manner. The
/// application should always return [`VK_FALSE`]. The [`VK_TRUE`] value is reserved for use in
/// layer development.
///
/// Provided by [`ext_debug_utils`]
pub type VkDebugUtilsMessengerCallbackEXT = extern "system" fn(
    message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    message_types: VkDebugUtilsMessageTypeFlagsEXT,
    callback_data: *const VkDebugUtilsMessengerCallbackDataEXT,
    user_data: *mut c_void,
) -> VkBool32;
