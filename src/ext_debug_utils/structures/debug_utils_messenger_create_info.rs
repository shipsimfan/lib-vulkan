use crate::{
    VK_FALSE, VkBool32, VkStructureType,
    ext_debug_utils::{
        VkDebugUtilsMessageSeverityFlagsExt, VkDebugUtilsMessageTypeFlagsExt,
        VkDebugUtilsMessengerCallbackDataExt, VkDebugUtilsMessengerCallbackExt,
        VkDebugUtilsMessengerCreateFlagsExt,
    },
    util::NextChain,
};
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils::{
    self, VkDebugUtilsMessageSeverityFlagExt, VkDebugUtilsMessageTypeFlagExt,
    VkDebugUtilsMessengerExt,
};

/// Structure specifying parameters of a newly created debug messenger
///
/// # Description
/// For each [`VkDebugUtilsMessengerExt`] that is created the
/// [`VkDebugUtilsMessengerCreateInfoExt::message_severity`] and
/// [`VkDebugUtilsMessengerCreateInfoExt::message_type`] determine when that
/// [`VkDebugUtilsMessengerCreateInfoExt::user_callback`] is called. The process to determine if
/// the user’s `user_callback` is triggered when an event occurs is as follows:
///  1. The implementation will perform a bitwise AND of the event’s
///     [`VkDebugUtilsMessageSeverityFlagExt`] with the `message_severity` provided during creation
///     of the [`VkDebugUtilsMessengerExt`] object.
///    a. If the value is 0, the message is skipped.
///  2. The implementation will perform bitwise AND of the event’s
///     [`VkDebugUtilsMessageTypeFlagExt`] with the `message_type` provided during the creation of
///     the [`VkDebugUtilsMessengerExt`] object
///    a. If the value is 0, the message is skipped.
///  3. The callback will trigger a debug message for the current event
///
/// The callback will come directly from the component that detected the event, unless some other
/// layer intercepts the calls for its own purposes (filter them in a different way, log to a
/// system error log, etc.).
///
/// An application can receive multiple callbacks if multiple [`VkDebugUtilsMessengerExt`] objects
/// are created. A callback will always be executed in the same thread as the originating Vulkan
/// call.
///
/// A callback can be called from multiple threads simultaneously (if the application is making
/// Vulkan calls from multiple threads).
///
/// Provided by [`ext_debug_utils`]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VkDebugUtilsMessengerCreateInfoExt {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::DebugUtilsMessengerCreateInfoExt`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `flags` is 0 and is reserved for future use.
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be 0
    pub flags: VkDebugUtilsMessengerCreateFlagsExt,

    /// `message_severity` is a bitmask of [`VkDebugUtilsMessageSeverityFlagExt`] specifying which
    /// severity of event(s) will cause this callback to be called.
    ///
    /// # Valid Usage (Implicit)
    ///  - `message_severity` must be a valid combination of [`VkDebugUtilsMessageSeverityFlagExt`]
    ///    values
    ///  - `message_severity` must not be 0
    pub message_severity: VkDebugUtilsMessageSeverityFlagsExt,

    /// `message_type` is a bitmask of [`VkDebugUtilsMessageTypeFlagExt`] specifying which type
    /// of event(s) will cause this callback to be called.
    ///
    /// # Valid Usage (Implicit)
    ///  - `message_type` must be a valid combination of [`VkDebugUtilsMessageTypeFlagExt`] values
    ///  - `message_type` must not be 0
    pub message_type: VkDebugUtilsMessageTypeFlagsExt,

    /// `user_callback` is the application callback function to call.
    ///
    /// # Valid Usage (Implicit)
    ///  - `user_callback` must be a valid [`VkDebugUtilsMessengerCallbackExt`] value
    pub user_callback: VkDebugUtilsMessengerCallbackExt,

    /// `user_data` is user data to be passed to the callback.
    pub user_data: *mut c_void,
}

impl const Default for VkDebugUtilsMessengerCreateInfoExt {
    fn default() -> Self {
        VkDebugUtilsMessengerCreateInfoExt {
            r#type: VkStructureType::DebugUtilsMessengerCreateInfoExt,
            next: null(),
            flags: VkDebugUtilsMessengerCreateFlagsExt::empty(),
            message_severity: VkDebugUtilsMessageSeverityFlagsExt::empty(),
            message_type: VkDebugUtilsMessageTypeFlagsExt::empty(),
            user_callback: default,
            user_data: null_mut(),
        }
    }
}

unsafe extern "system" fn default(
    _: VkDebugUtilsMessageSeverityFlagExt,
    _: VkDebugUtilsMessageTypeFlagsExt,
    _: *const VkDebugUtilsMessengerCallbackDataExt,
    _: *mut c_void,
) -> VkBool32 {
    VK_FALSE
}

impl NextChain for VkDebugUtilsMessengerCreateInfoExt {
    fn next(&self) -> *const c_void {
        self.next
    }

    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: *const c_void) {
        self.next = next;
    }
}
