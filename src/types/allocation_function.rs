use crate::VkSystemAllocationScope;
use core::ffi::{c_size_t, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VkResult, VK_VERSION_1_0};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Application-defined memory allocation function
///
/// # Parameters
///  - `user_data` is the value specified for `VkAllocationCallbacks::user_data` in the allocator
///    specified by the application.
///  - `size` is the size in bytes of the requested allocation.
///  - `alignment` is the requested alignment of the allocation in bytes and must be a power of
///    two.
///  - `allocation_scope` is a [`VkSystemAllocationScope`] value specifying the allocation scope of
///    the lifetime of the allocation.
///
/// # Description
/// If `allocation` is unable to allocate the requested memory, it must return [`null_mut`]. If the
/// allocation was successful, it must return a valid pointer to memory allocation containing at
/// least size bytes, and with the pointer value being a multiple of alignment.
///
/// If `allocation` returns [`null_mut`], and if the implementation is unable to continue correct
/// processing of the current command without the requested allocation, it must treat this as a
/// runtime error, and generate [`VkResult::VkErrorOutOfHostMemory`] at the appropriate time for
/// the command in which the condition was detected, as described in Return Codes.
///
/// If the implementation is able to continue correct processing of the current command without the
/// requested allocation, then it may do so, and must not generate
/// [`VkResult::VkErrorOutOfHostMemory`] as a result of this failed allocation.
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkAllocationFunction = extern "system" fn(
    user_data: *mut c_void,
    size: c_size_t,
    alignment: c_size_t,
    allocation_scope: VkSystemAllocationScope,
) -> *mut c_void;
