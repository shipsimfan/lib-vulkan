// rustdoc imports
#[allow(unused_imports)]
use crate::{vk_api_version_patch, vk_version_patch, VK_VERSION_1_0};

/// [`vk_version_patch`] extracts the API patch version number from a packed version number
///
/// DEPRECATED: This define is deprecated. [`vk_api_version_patch`] should be used instead.
///
/// Provided by [`VK_VERSION_1_0`]
#[macro_export]
macro_rules! vk_version_patch {
    ($version: expr) => {
        $version as u32 & 0xFFF
    };
}
