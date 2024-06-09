use crate::{
    VkBool32, VkDebugUtilsMessageSeverityFlagsEXT, VkDebugUtilsMessageTypeFlagsEXT,
    VkDebugUtilsMessengerCallbackDataEXT, VkDebugUtilsMessengerCallbackEXT,
    VkDebugUtilsMessengerCreateFlagsEXT, VkStructureType, VK_FALSE,
};
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    ext_debug_utils, VkDebugUtilsMessageSeverityFlagBitsEXT, VkDebugUtilsMessageTypeFlagBitsEXT,
    VkDebugUtilsMessengerEXT,
};

/// Structure specifying parameters of a newly created debug messenger
///
/// For each [`VkDebugUtilsMessengerEXT`] that is created the
/// `VkDebugUtilsMessengerCreateInfoEXT::message_severity` and
/// `VkDebugUtilsMessengerCreateInfoEXT::message_type` determine when that
/// `VkDebugUtilsMessengerCreateInfoEXT::user_callback` is called. The process to determine if the
/// user’s `user_callback` is triggered when an event occurs is as follows:
///  1. The implementation will perform a bitwise AND of the event’s
///     [`VkDebugUtilsMessageSeverityFlagBitsEXT`] with the `message_severity` provided during
///     creation of the [`VkDebugUtilsMessengerEXT`] object.
///    a. If the value is 0, the message is skipped.
///  2. The implementation will perform bitwise AND of the event’s
///     [`VkDebugUtilsMessageTypeFlagBitsEXT`] with the messageType provided during the creation of
///     the [`VkDebugUtilsMessengerEXT`] object
///    a. If the value is 0, the message is skipped.
///  3. The callback will trigger a debug message for the current event
///
/// The callback will come directly from the component that detected the event, unless some other
/// layer intercepts the calls for its own purposes (filter them in a different way, log to a
/// system error log, etc.).
///
/// An application can receive multiple callbacks if multiple [`VkDebugUtilsMessengerEXT`] objects
/// are created. A callback will always be executed in the same thread as the originating Vulkan
/// call.
///
/// A callback can be called from multiple threads simultaneously (if the application is making
/// Vulkan calls from multiple threads).
///
/// Provided by [`ext_debug_utils`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `flags` is 0 and is reserved for future use.
    pub flags: VkDebugUtilsMessengerCreateFlagsEXT,

    /// `message_severity` is a bitmask of [`VkDebugUtilsMessageSeverityFlagBitsEXT`] specifying
    /// which severity of event(s) will cause this callback to be called.
    pub message_severity: VkDebugUtilsMessageSeverityFlagsEXT,

    /// `message_type` is a bitmask of [`VkDebugUtilsMessageTypeFlagBitsEXT`] specifying which type
    /// of event(s) will cause this callback to be called.
    pub message_type: VkDebugUtilsMessageTypeFlagsEXT,

    /// `user_callback` is the application callback function to call.
    pub user_callback: VkDebugUtilsMessengerCallbackEXT,

    /// `user_data` is user data to be passed to the callback.
    pub user_data: *mut c_void,
}

impl Default for VkDebugUtilsMessengerCreateInfoEXT {
    fn default() -> Self {
        VkDebugUtilsMessengerCreateInfoEXT {
            r#type: VkStructureType::DebugUtilsMessengerCreateInfoEXT,
            next: null(),
            flags: 0,
            message_severity: 0,
            message_type: 0,
            user_callback: default,
            user_data: null_mut(),
        }
    }
}

extern "system" fn default(
    _: VkDebugUtilsMessageSeverityFlagBitsEXT,
    _: VkDebugUtilsMessageTypeFlagsEXT,
    _: *const VkDebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> VkBool32 {
    VK_FALSE
}
