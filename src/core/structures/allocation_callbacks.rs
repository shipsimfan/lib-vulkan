use crate::{
    VkAllocationFunction, VkFreeFunction, VkInternalAllocationNotification,
    VkInternalFreeNotification, VkReallocationFunction,
};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure containing callback function pointers for memory allocation
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VkAllocationCallbacks {
    /// `user_data` is a value to be interpreted by the implementation of the callbacks. When any
    /// of the callbacks in [`VkAllocationCallbacks`] are called, the Vulkan implementation will
    /// pass this value as the first parameter to the callback. This value can vary each time an
    /// allocator is passed into a command, even when the same object takes an allocator in
    /// multiple commands.
    pub user_data: *mut c_void,

    /// `allocation` is a [`VkAllocationFunction`] pointer to an application-defined memory
    /// allocation function.
    ///
    /// # Valid Usage
    ///  - `allocation` must be a valid pointer to a valid application-defined
    ///    [`VkAllocationFunction`]
    pub allocation: VkAllocationFunction,

    /// `reallocation` is a [`VkReallocationFunction`] pointer to an application-defined memory
    /// reallocation function.
    ///
    /// # Valid Usage
    ///  - `reallocation` must be a valid pointer to a valid application-defined
    ///    [`VkReallocationFunction`]
    pub reallocation: VkReallocationFunction,

    /// `free` is a [`VkFreeFunction`] pointer to an application-defined memory free function.
    ///
    /// # Valid Usage
    ///  - `free` must be a valid pointer to a valid application-defined [`VkFreeFunction`]
    pub free: VkFreeFunction,

    /// `internal_allocation` is a [`VkInternalAllocationNotification`] pointer to an
    /// application-defined function that is called by the implementation when the implementation
    /// makes internal allocations.
    ///
    /// # Valid Usage
    ///  - If either of `internal_allocation` or `internal_free` is not [`None`], both must be
    ///    valid callbacks
    pub internal_allocation: Option<VkInternalAllocationNotification>,

    /// `internal_free` is a [`VkInternalFreeNotification`] pointer to an application-defined
    /// function that is called by the implementation when the implementation frees internal
    /// allocations.
    ///
    /// # Valid Usage
    ///  - If either of `internal_allocation` or `internal_free` is not [`None`], both must be
    ///    valid callbacks
    pub internal_free: Option<VkInternalFreeNotification>,
}
