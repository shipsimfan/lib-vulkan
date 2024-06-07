use crate::VkSystemAllocationScope;
use core::ffi::{c_size_t, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VkAllocationFunction, VkFreeFunction, VK_VERSION_1_0};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Application-defined memory reallocation function
///
/// # Parameters
///  - `user_data` is the value specified for `VkAllocationCallbacks::user_data` in the allocator
///    specified by the application.
///  - `original` must be either [`null_mut`] or a pointer previously returned by `reallocation` or
///    `allocation` of a compatible allocator.
///  - `size` is the size in bytes of the requested allocation.
///  - `alignment` is the requested alignment of the allocation in bytes and must be a power of
///    two.
///  - `allocation_scope` is a [`VkSystemAllocationScope`] value specifying the allocation scope of
///    the lifetime of the allocation.
///
/// # Description
/// If the reallocation was successful, `reallocation` must return an allocation with enough space
/// for size bytes, and the contents of the original allocation from bytes zero to min(original
/// size, new size) - 1 must be preserved in the returned allocation. If size is larger than the
/// old size, the contents of the additional space are undefined. If satisfying these requirements
/// involves creating a new allocation, then the old allocation should be freed.
///
/// If `original` is [`null_mut`], then `reallocation` must behave equivalently to a call to
/// [`VkAllocationFunction`] with the same parameter values (without `original`).
///
/// If size is zero, then `reallocation` must behave equivalently to a call to [`VkFreeFunction`]
/// with the same `user_data` parameter value, and `memory` equal to `original`.
///
/// If `original` is not [`null_mut`], the implementation must ensure that alignment is equal to
/// the alignment used to originally allocate `original`.
///
/// If this function fails and `original` is not [`null_mut`] the application must not free the old
/// allocation.
///
/// `reallocation` must follow the same rules for return values as [`VkAllocationFunction`].
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkReallocationFunction = extern "system" fn(
    user_data: *mut c_void,
    original: *mut c_void,
    size: c_size_t,
    alignment: c_size_t,
    allocation_scope: VkSystemAllocationScope,
) -> *mut c_void;
