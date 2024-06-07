// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Allocation type
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkInternalAllocationType {
    /// [`VkInternalAllocationType::Executable`] specifies that the allocation is intended for
    /// execution by the host.
    Executable = 0,
}
