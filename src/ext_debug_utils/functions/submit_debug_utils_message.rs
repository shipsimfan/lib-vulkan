use crate::{
    VkInstance,
    ext_debug_utils::{
        VkDebugUtilsMessageSeverityFlagExt, VkDebugUtilsMessageTypeFlagsExt,
        VkDebugUtilsMessengerCallbackDataExt,
    },
};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VkObjectType,
    ext_debug_utils::{self, VkDebugUtilsMessageTypeFlagExt},
};

/// Inject a message into a debug stream
///
/// # Parameters
///  - `instance` is the debug stream’s [`VkInstance`].
///  - `message_severity` is a [`VkDebugUtilsMessageSeverityFlagExt`] value specifying the
///    severity of this event/message.
///  - `message_types` is a bitmask of [`VkDebugUtilsMessageTypeFlagExt`] specifying which type
///    of event(s) to identify with this message.
///  - `callback_data` contains all the callback related data in the
///    [`VkDebugUtilsMessengerCallbackDataExt`] structure.
///
/// # Description
/// The call will propagate through the layers and generate callback(s) as indicated by the
/// message’s flags. The parameters are passed on to the callback in addition to the `user_data`
/// value that was defined at the time the messenger was registered.
///
/// # Valid Usage
///  - The `object_type` member of each element of `callback_data->objects` must not be
///    [`VkObjectType::Unknown`]
///
/// # Valid Usage (Implicit)
///  - `instance` must be a valid [`VkInstance`] handle
///  - `message_severity` must be a valid [`VkDebugUtilsMessageSeverityFlagExt`] value
///  - `message_types` must be a valid combination of [`VkDebugUtilsMessageTypeFlagExt`] values
///  - `message_types` must not be 0
///  - `callback_data` must be a valid pointer to a valid [`VkDebugUtilsMessengerCallbackDataExt`]
///    structure
///
/// Provided by [`ext_debug_utils`]
pub type VkSubmitDebugUtilsMessageExt = extern "system" fn(
    instance: VkInstance,
    message_severity: VkDebugUtilsMessageSeverityFlagExt,
    message_types: VkDebugUtilsMessageTypeFlagsExt,
    callback_data: *const VkDebugUtilsMessengerCallbackDataExt,
);

/// The name of [`VkSubmitDebugUtilsMessageExt`]
pub const VK_SUBMIT_DEBUG_UTILS_MESSAGE_EXT: &CStr = c"vkSubmitDebugUtilsMessageEXT";
