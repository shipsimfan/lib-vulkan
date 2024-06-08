use crate::{VkInternalAllocationType, VkSystemAllocationScope};
use core::ffi::{c_size_t, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Application-defined memory allocation notification function
///
/// # Parameters
///  - `user_data` is the value specified for `VkAllocationCallbacks::user_data` in the allocator
///    specified by the application.
///  - `size` is the requested size of an allocation.
///  - `allocation_type` is a [`VkInternalAllocationType`] value specifying the requested type of
///    an allocation.
///  - `allocation_scope` is a [`VkSystemAllocationScope`] value specifying the allocation scope of
///    the lifetime of the allocation.
///
/// # Description
/// This is a purely informational callback.
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkInternalAllocationNotification = extern "system" fn(
    user_data: *mut c_void,
    size: c_size_t,
    allocation_type: VkInternalAllocationType,
    allocation_scope: VkSystemAllocationScope,
);
