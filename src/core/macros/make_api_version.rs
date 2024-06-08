// rustdoc imports
#[allow(unused_imports)]
use crate::{vk_make_api_version, VK_VERSION_1_0};

/// [`vk_make_api_version`] constructs an API version number
///
/// Provided by [`VK_VERSION_1_0`]
#[macro_export]
macro_rules! vk_make_api_version {
    ($variant: expr, $major: expr, $minor: expr, $patch: expr) => {
        ($variant as u32) << 29 | ($major as u32) << 22 | ($minor as u32) << 12 | $patch as u32
    };
}
