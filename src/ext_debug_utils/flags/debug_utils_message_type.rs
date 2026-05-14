use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

flags! {
    /// Bitmask of [`VkDebugUtilsMessageTypeFlagExt`]
    ///
    /// # Description
    /// [`VkDebugUtilsMessageTypeFlagsExt`] is a bitmask type for setting a mask of zero or more
    /// [`VkDebugUtilsMessageTypeFlagExt`].
    ///
    /// Provided by [`ext_debug_utils`]
    pub struct VkDebugUtilsMessageTypeFlagsExt;

    /// Bitmask specifying which types of events cause a debug messenger callback
    ///
    /// Provided by [`ext_debug_utils`]
    pub enum VkDebugUtilsMessageTypeFlagExt {
        /// [`VkDebugUtilsMessageTypeFlagExt::GeneralExt`] specifies that some general event has
        /// occurred. This is typically a non-specification, non-performance event.
        GeneralExt = 0x00000001,

        /// [`VkDebugUtilsMessageTypeFlagExt::ValidationExt`] specifies that something has
        /// occurred during validation against the Vulkan specification that may indicate invalid
        /// behavior.
        ValidationExt = 0x00000002,

        /// [`VkDebugUtilsMessageTypeFlagExt::PerformanceExt`] specifies a potentially
        /// non-optimal use of Vulkan, e.g. using [`VkCmdClearColorImage`] when setting
        /// [`VkAttachmentDescription::load_op`] to [`VkAttachment::LoadOpClear`] would have
        /// worked.
        PerformanceExt = 0x00000004,

        /// [`VkDebugUtilsMessageTypeFlagExt::AddressBindingExt`] specifies that the
        /// implementation has modified the set of GPU-visible virtual addresses associated with a
        /// Vulkan object.
        ///
        /// Provided by [`ext_device_address_binding_report`]
        AddressBindingExt = 0x00000008,
    }
}
