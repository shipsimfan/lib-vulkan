use crate::vk_make_api_version;

/// [`VK_VERSION_1_0`] returns the API version number for Vulkan 1.0.0.
///
/// Provided by [`VK_VERSION_1_0`]
pub const VK_VERSION_1_0: u32 = vk_make_api_version!(0, 1, 0, 0);

/// [`VK_VERSION_1_1`] returns the API version number for Vulkan 1.1.0.
///
/// Provided by [`VK_VERSION_1_1`]
pub const VK_VERSION_1_1: u32 = vk_make_api_version!(0, 1, 1, 0);

/// [`VK_VERSION_1_2`] returns the API version number for Vulkan 1.2.0.
///
/// Provided by [`VK_VERSION_1_2`]
pub const VK_VERSION_1_2: u32 = vk_make_api_version!(0, 1, 2, 0);

/// [`VK_VERSION_1_3`] returns the API version number for Vulkan 1.3.0.
///
/// Provided by [`VK_VERSION_1_3`]
pub const VK_VERSION_1_3: u32 = vk_make_api_version!(0, 1, 3, 0);
