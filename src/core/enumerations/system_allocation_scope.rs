// rustdoc imports
#[allow(unused_imports)]
use crate::{VkDevice, VkInstance, VK_VERSION_1_0};

/// Allocation scope
///
/// Each allocation has an allocation scope defining its lifetime and which object it is associated
/// with.
///
/// Most Vulkan commands operate on a single object, or there is a sole object that is being
/// created or manipulated. When an allocation uses an allocation scope of
/// [`VkSystemAllocationScope::Object`] or [`VkSystemAllocationScope::Cache`], the allocation is
/// scoped to the object being created or manipulated.
///
/// When an implementation requires host memory, it will make callbacks to the application using
/// the most specific allocator and allocation scope available:
///  - If an allocation is scoped to the duration of a command, the allocator will use the
///    [`VkSystemAllocationScope::Command`] allocation scope. The most specific allocator available
///    is used: if the object being created or manipulated has an allocator, that object’s
///    allocator will be used, else if the parent [`VkDevice`] has an allocator it will be used,
///    else if the parent [`VkInstance`] has an allocator it will be used. Else,
///  - If an allocation is associated with a VkValidationCacheEXT or VkPipelineCache object, the
///    allocator will use the [`VkSystemAllocationScope::Cache`] allocation scope. The most
///    specific allocator available is used (cache, else device, else instance). Else,
///  - If an allocation is scoped to the lifetime of an object, that object is being created or
///    manipulated by the command, and that object’s type is not [`VkDevice`] or [`VkInstance`],
///    the allocator will use an allocation scope of [`VkSystemAllocationScope::Object`]. The most
///    specific allocator available is used (object, else device, else instance). Else,
///  - If an allocation is scoped to the lifetime of a device, the allocator will use an allocation
///    scope of [`VkSystemAllocationScope::Device`]. The most specific allocator available is used
///    (device, else instance). Else,
///  - If the allocation is scoped to the lifetime of an instance and the instance has an
///    allocator, its allocator will be used with an allocation scope of
///    [`VkSystemAllocationScope::Instance`].
///  - Otherwise an implementation will allocate memory through an alternative mechanism that is
///    unspecified.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkSystemAllocationScope {
    /// [`VkSystemAllocationScope::Command`] specifies that the allocation is scoped to the
    /// duration of the Vulkan command.
    Command = 0,

    /// [`VkSystemAllocationScope::Object`] specifies that the allocation is scoped to the lifetime
    /// of the Vulkan object that is being created or used.
    Object = 1,

    /// [`VkSystemAllocationScope::Cache`] specifies that the allocation is scoped to the lifetime
    /// of a [`VkPipelineCache`] or [`VkValidationCacheEXT`] object.
    Cache = 2,

    /// [`VkSystemAllocationScope::Device`] specifies that the allocation is scoped to the lifetime
    /// of the Vulkan device.
    Device = 3,

    /// [`VkSystemAllocationScope::Instance`] specifies that the allocation is scoped to the
    /// lifetime of the Vulkan instance.
    Instance = 4,
}
