// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

/// Bitmask specifying which severities of events cause a debug messenger callback
///
/// Provided by [`ext_debug_utils`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkDebugUtilsMessageSeverityFlagBitsEXT {
    /// [`VkDebugUtilsMessageSeverityFlagBitsEXT::VerboseBitEXT`] specifies the most verbose output
    /// indicating all diagnostic messages from the Vulkan loader, layers, and drivers should be
    /// captured.
    VerboseBitEXT = 0x00000001,

    /// [`VkDebugUtilsMessageSeverityFlagBitsEXT::InfoBitEXT`] specifies an informational message
    /// such as resource details that may be handy when debugging an application.
    InfoBitEXT = 0x00000010,

    /// [`VkDebugUtilsMessageSeverityFlagBitsEXT::WarningBitEXT`] specifies use of Vulkan that may
    /// expose an app bug. Such cases may not be immediately harmful, such as a fragment shader
    /// outputting to a location with no attachment. Other cases may point to behavior that is
    /// almost certainly bad when unintended such as using an image whose memory has not been
    /// filled. In general if you see a warning but you know that the behavior is intended/desired,
    /// then simply ignore the warning.
    WarningBitEXT = 0x00000100,

    /// [`VkDebugUtilsMessageSeverityFlagBitsEXT::ErrorBitEXT`] specifies that the application has
    /// violated a valid usage condition of the specification.
    ErrorBitEXT = 0x00001000,
}
