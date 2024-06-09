use crate::{
    VkDebugUtilsLabelEXT, VkDebugUtilsMessengerCallbackDataFlagsEXT, VkDebugUtilsObjectNameInfoEXT,
    VkStructureType,
};
use std::{
    ffi::{c_char, c_void},
    ptr::null,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{ext_debug_utils, VkDebugUtilsMessageTypeFlagsEXT};

/// Structure specifying parameters returned to the callback
///
/// Since adding queue and command buffer labels behaves like pushing and popping onto a stack, the
/// order of both `queue_labels` and `cmd_buf_labels` is based on the order the labels were
/// defined. The result is that the first label in either `queue_labels` or `cmd_buf_labels` will
/// be the first defined (and therefore the oldest) while the last label in each list will be the
/// most recent.
///
/// Provided by [`ext_debug_utils`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    pub next: *const c_void,

    /// `flags` is 0 and is reserved for future use.
    pub flags: VkDebugUtilsMessengerCallbackDataFlagsEXT,

    /// `message_id_name` is [`null`] or a null-terminated UTF-8 string that identifies the
    /// particular message ID that is associated with the provided message. If the message
    /// corresponds to a validation layer message, then this string may contain the portion of the
    /// Vulkan specification that is believed to have been violated.
    pub message_id_name: *const c_char,

    /// `message_id_number` is the ID number of the triggering message. If the message corresponds
    /// to a validation layer message, then this number is related to the internal number
    /// associated with the message being triggered.
    pub message_id_number: i32,

    /// `message` is [`null`] if `message_types` is equal to
    /// [`VkDebugUtilsMessageTypeFlagsEXT::DeviceAddressBindingBitEXT`], or a null-terminated UTF-8
    /// string detailing the trigger conditions.
    pub message: *const c_char,

    /// `queue_label_count` is a count of items contained in the `queue_labels` array.
    pub queue_label_count: u32,

    /// `queue_labels` is [`null`] or a pointer to an array of [`VkDebugUtilsLabelEXT`] active in
    /// the current [`VkQueue`] at the time the callback was triggered.
    pub queue_labels: *const VkDebugUtilsLabelEXT,

    /// `cmd_buf_label_count` is a count of items contained in the `cmd_buf_labels` array.
    pub cmd_buf_label_count: u32,

    /// `cmd_buf_labels` is [`null`] or a pointer to an array of [`VkDebugUtilsLabelEXT`] active in
    /// the current [`VkCommandBuffer`] at the time the callback was triggered.
    pub cmd_buf_labels: *const VkDebugUtilsLabelEXT,

    /// `object_count` is a count of items contained in the `objects` array.
    pub object_count: u32,

    /// `objects` is a pointer to an array of [`VkDebugUtilsObjectNameInfoEXT`] objects related to
    /// the detected issue. The array is roughly in order or importance, but the 0th element is
    /// always guaranteed to be the most important object for this message.
    pub objects: *const VkDebugUtilsObjectNameInfoEXT,
}

impl Default for VkDebugUtilsMessengerCallbackDataEXT {
    fn default() -> Self {
        VkDebugUtilsMessengerCallbackDataEXT {
            r#type: VkStructureType::DebugUtilsMessengerCallbackDataEXT,
            next: null(),
            flags: 0,
            message_id_name: null(),
            message_id_number: 0,
            message: null(),
            queue_label_count: 0,
            queue_labels: null(),
            cmd_buf_label_count: 0,
            cmd_buf_labels: null(),
            object_count: 0,
            objects: null(),
        }
    }
}
