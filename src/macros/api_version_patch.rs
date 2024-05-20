// rustdoc imports
#[allow(unused_imports)]
use crate::{vk_api_version_patch, VK_VERSION_1_0};

/// [`vk_api_version_patch`] extracts the API patch version number from a packed version number
///
/// Provided by [`VK_VERSION_1_0`]
#[macro_export]
macro_rules! vk_api_version_patch {
    ($version: expr) => {
        $version as u32 & 0xFFF
    };
}
