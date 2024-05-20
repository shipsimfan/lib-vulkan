// rustdoc imports
#[allow(unused_imports)]
use crate::{vk_api_version_major, vk_version_major, VK_VERSION_1_0};

/// [`vk_version_major`] extracts the API major version number from a packed version number
///
/// DEPRECATED: This define is deprecated. [`vk_api_version_major`] should be used instead.
///
/// Provided by [`VK_VERSION_1_0`]
#[macro_export]
macro_rules! vk_version_major {
    ($version: expr) => {
        $version as u32 >> 22
    };
}
