// rustdoc imports
#[allow(unused_imports)]
use crate::ext_debug_utils;

/// Bitmask specifying which types of events cause a debug messenger callback
///
/// Provided by [`ext_debug_utils`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkDebugUtilsMessageTypeFlagBitsEXT {
    /// [`VkDebugUtilsMessageTypeFlagBitsEXT::GeneralBitEXT`] specifies that some general event has
    /// occurred. This is typically a non-specification, non-performance event.
    GeneralBitEXT = 0x00000001,

    /// [`VkDebugUtilsMessageTypeFlagBitsEXT::ValidationBitEXT`] specifies that something has
    /// occurred during validation against the Vulkan specification that may indicate invalid
    /// behavior.
    ValidationBitEXT = 0x00000002,

    /// [`VkDebugUtilsMessageTypeFlagBitsEXT::PerformanceBitEXT`] specifies a potentially
    /// non-optimal use of Vulkan, e.g. using vkCmdClearColorImage when setting
    /// `VkAttachmentDescription::load_op` to [`VkAttachment::LoadOpClear`] would have worked.
    PerformanceBitEXT = 0x00000004,

    /// [`VkDebugUtilsMessageTypeFlagBitsEXT::AddressBindingBitEXT`] specifies that the
    /// implementation has modified the set of GPU-visible virtual addresses associated with a
    /// Vulkan object.
    ///
    /// Provided by [`ext_device_address_binding_report`]
    AddressBindingBitEXT = 0x00000008,
}
