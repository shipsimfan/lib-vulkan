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
        /// [`VkDebugUtilsMessageTypeFlagExt::GeneralBitExt`] specifies that some general event has
        /// occurred. This is typically a non-specification, non-performance event.
        GeneralBitExt = 0x00000001,

        /// [`VkDebugUtilsMessageTypeFlagExt::ValidationBitExt`] specifies that something has
        /// occurred during validation against the Vulkan specification that may indicate invalid
        /// behavior.
        ValidationBitExt = 0x00000002,

        /// [`VkDebugUtilsMessageTypeFlagExt::PerformanceBitExt`] specifies a potentially
        /// non-optimal use of Vulkan, e.g. using [`VkCmdClearColorImage`] when setting
        /// [`VkAttachmentDescription::load_op`] to [`VkAttachment::LoadOpClear`] would have
        /// worked.
        PerformanceBitExt = 0x00000004,

        /// [`VkDebugUtilsMessageTypeFlagExt::AddressBindingBitExt`] specifies that the
        /// implementation has modified the set of GPU-visible virtual addresses associated with a
        /// Vulkan object.
        ///
        /// Provided by [`ext_device_address_binding_report`]
        AddressBindingBitExt = 0x00000008,
    }
}
