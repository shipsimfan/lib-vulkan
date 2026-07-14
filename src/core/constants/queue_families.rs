// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Ignored queue family index sentinel
///
/// The special queue family index [`VK_QUEUE_FAMILY_IGNORED`] indicates that a queue family
/// parameter or member is ignored.
///
/// Provided by [`VK_VERSION_1_0`]
pub const VK_QUEUE_FAMILY_IGNORED: u32 = !0;
