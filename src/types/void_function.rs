/// Placeholder function pointer type returned by queries
///
/// This type is returned from command function pointer queries, and *must* be cast to an actual
/// command function pointer before use.
pub type VkVoidFunction = extern "system" fn();
