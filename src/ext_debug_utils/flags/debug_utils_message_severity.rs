use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

flags! {
    /// Bitmask of [`VkDebugUtilsMessageSeverityFlagExt`]
    ///
    /// # Description
    /// [`VkDebugUtilsMessageSeverityFlagsExt`] is a bitmask type for setting a mask of zero or
    /// more [`VkDebugUtilsMessageSeverityFlagExt`].
    ///
    /// Provided by [`ext_debug_utils`]
    pub struct VkDebugUtilsMessageSeverityFlagsExt;

    /// Bitmask specifying which severities of events cause a debug messenger callback
    ///
    /// Provided by [`ext_debug_utils`]
    pub enum VkDebugUtilsMessageSeverityFlagExt {
        /// [`VkDebugUtilsMessageSeverityFlagExt::VerboseExt`] specifies the most verbose output
        /// indicating all diagnostic messages from the Vulkan loader, layers, and drivers should
        /// be captured.
        VerboseExt = 0x00000001,

        /// [`VkDebugUtilsMessageSeverityFlagExt::InfoExt`] specifies an informational message
        /// such as resource details that may be handy when debugging an application.
        InfoExt = 0x00000010,

        /// [`VkDebugUtilsMessageSeverityFlagExt::WarningExt`] specifies use of Vulkan that may
        /// expose an app bug. Such cases may not be immediately harmful, such as a fragment shader
        /// outputting to a location with no attachment. Other cases may point to behavior that is
        /// almost certainly bad when unintended such as using an image whose memory has not been
        /// filled. In general if you see a warning but you know that the behavior is
        /// intended/desired, then simply ignore the warning.
        WarningExt = 0x00000100,

        /// [`VkDebugUtilsMessageSeverityFlagExt::ErrorExt`] specifies that the application has
        /// violated a valid usage condition of the specification.
        ErrorExt = 0x00001000,
    }
}
