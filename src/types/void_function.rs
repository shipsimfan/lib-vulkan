// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Placeholder function pointer type returned by queries
///
/// This type is returned from command function pointer queries, and *must* be cast to an actual
/// command function pointer before use.
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkVoidFunction = extern "system" fn();
