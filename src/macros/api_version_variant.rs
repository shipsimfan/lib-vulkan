// rustdoc imports
#[allow(unused_imports)]
use crate::{vk_api_version_variant, VK_VERSION_1_0};

/// [`vk_api_version_variant`] extracts the API variant number from a packed version number
///
/// Provided by [`VK_VERSION_1_0`]
#[macro_export]
macro_rules! vk_api_version_variant {
    ($version: expr) => {
        $version as u32 >> 29
    };
}
