// rustdoc imports
#[allow(unused_imports)]
use crate::{vk_api_version_minor, VK_VERSION_1_0};

/// [`vk_api_version_minor`] extracts the API minor version number from a packed version number
///
/// Provided by [`VK_VERSION_1_0`]
#[macro_export]
macro_rules! vk_api_version_minor {
    ($version: expr) => {
        ($version as u32 >> 12) & 0x3FF
    };
}
