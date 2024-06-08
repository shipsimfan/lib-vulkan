use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Application-defined memory free function
///
/// # Parameters
///  - `user_data` is the value specified for `VkAllocationCallbacks::user_data` in the allocator
///    specified by the application.
///  - `memory` is the allocation to be freed.
///
/// # Description
/// `memory` may be [`null_mut`], which the callback must handle safely. If `memory` is not
/// [`null_mut`], it must be a pointer previously allocated by `allocation` or `reallocation`. The
/// application should free this memory.
///
/// Provided by [`VK_VERSION_1_0`]
pub type VkFreeFunction = extern "system" fn(user_data: *mut c_void, memory: *mut c_void);
