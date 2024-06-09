use crate::{
    VkDebugUtilsMessageSeverityFlagBitsEXT, VkDebugUtilsMessageTypeFlagsEXT,
    VkDebugUtilsMessengerCallbackDataEXT, VkInstance,
};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{ext_debug_utils, VkDebugUtilsMessageTypeFlagBitsEXT};

/// Inject a message into a debug stream
///
/// # Parameters
///  - `instance` is the debug stream’s [`VkInstance`].
///  - `message_severity` is a [`VkDebugUtilsMessageSeverityFlagBitsEXT`] value specifying the
///    severity of this event/message.
///  - `message_types` is a bitmask of [`VkDebugUtilsMessageTypeFlagBitsEXT`] specifying which type
///    of event(s) to identify with this message.
///  - `callback_data` contains all the callback related data in the
///    [`VkDebugUtilsMessengerCallbackDataEXT`] structure.
///
/// # Description
/// The call will propagate through the layers and generate callback(s) as indicated by the
/// message’s flags. The parameters are passed on to the callback in addition to the `user_data`
/// value that was defined at the time the messenger was registered.
///
/// Provided by [`ext_debug_utils`]
pub type VkSubmitDebugUtilsMessageEXT = extern "system" fn(
    instance: VkInstance,
    message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    message_types: VkDebugUtilsMessageTypeFlagsEXT,
    callback_data: *const VkDebugUtilsMessengerCallbackDataEXT,
);

/// The name of [`VkSubmitDebugUtilsMessageEXT`]
pub const VK_SUBMIT_DEBUG_UTILS_MESSAGE_EXT: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkSubmitDebugUtilsMessageEXT\0") };
