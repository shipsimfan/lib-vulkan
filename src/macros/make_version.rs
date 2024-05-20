// rustdoc imports
#[allow(unused_imports)]
use crate::{vk_make_api_version, vk_make_version, VK_VERSION_1_0};

/// [`vk_make_version`] constructs an API version number
///
/// Provided by [`VK_VERSION_1_0`]
///
/// DEPRECATED: This define is deprecated. [`vk_make_api_version`] should be used instead.
#[macro_export]
macro_rules! vk_make_version {
    ( $major: expr, $minor: expr, $patch: expr) => {
        $major as u32 << 22 | $minor as u32 << 12 | $patch as u32
    };
}
