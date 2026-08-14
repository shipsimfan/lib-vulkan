use crate::{VkDevice, VkImage, VkMemoryRequirements};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VkImageCreateFlag};

/// Returns the memory requirements for specified Vulkan object
///
/// # Parameters
///  - `device` is the logical device that owns the image.
///  - `image` is the image to query.
///  - `memory_requirements` is a pointer to a [`VkMemoryRequirements`] structure in which the
///    memory requirements of the image object are returned.
///
/// # Valid Usage
///  - `image` must not have been created with the [`VkImageCreateFlag::Disjoint`] flag set
///  - If image was created with the [`VkExternalMemoryHandleType::HardwareBufferAndroid`] external
///    memory handle type, then `image` must be bound to memory
///  - If image was created with the [`VkExternalMemoryHandleType::ScreenBufferQnx`] external
///    memory handle type, then `image` must be bound to memory
///
/// # Valid Usage (Implicit)
///  - `device` must be a valid [`VkDevice`] handle
///  - `image` must be a valid [`VkImage`] handle
///  - `memory_requirements` must be a valid pointer to a [`VkMemoryRequirements`] structure
///  - `image` must have been created, allocated, or retrieved from `device`
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkGetImageMemoryRequirements = unsafe extern "system" fn(
    device: VkDevice,
    image: VkImage,
    memory_requirements: *mut VkMemoryRequirements,
);

/// The name of [`VkGetImageMemoryRequirements`]
pub const VK_GET_IMAGE_MEMORY_REQUIREMENTS: &CStr = c"vkGetImageMemoryRequirements";
