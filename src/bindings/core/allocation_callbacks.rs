#![allow(unused)]

use std::ffi::c_void;

pub(crate) type VkAllocationFunction = extern "system" fn(
    p_user_data: *mut c_void,
    size: usize,
    alignment: usize,
    allocation_scope: VkSystemAllocationScope,
) -> *mut c_void;

pub(crate) type VkReallocationFunction = extern "system" fn(
    p_user_data: *mut c_void,
    p_original: *mut c_void,
    size: usize,
    alignment: usize,
    allocation_scope: VkSystemAllocationScope,
) -> *mut c_void;

pub(crate) type VkFreeFunction =
    extern "system" fn(p_user_data: *mut c_void, p_memory: *mut c_void);

pub(crate) type VkInternalAllocationNotification = extern "system" fn(
    p_user_data: *mut c_void,
    size: usize,
    allocation_type: VkInternalAllocationType,
    allocation_scope: VkSystemAllocationScope,
);

pub(crate) type VkInternalFreeNotification = extern "system" fn(
    p_user_data: *mut c_void,
    size: usize,
    allocation_type: VkInternalAllocationType,
    allocation_scope: VkSystemAllocationScope,
);

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum VkSystemAllocationScope {
    Command = 0,
    Object = 1,
    Cache = 2,
    Device = 3,
    Instance = 4,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum VkInternalAllocationType {
    Executable = 0,
}

pub(crate) struct VkAllocationCallbacks {
    pub(crate) p_user_data: *mut c_void,
    pub(crate) allocation: VkAllocationFunction,
    pub(crate) reallocation: VkReallocationFunction,
    pub(crate) free: VkFreeFunction,
    pub(crate) internal_allocation: VkInternalAllocationNotification,
    pub(crate) internal_free: VkInternalFreeNotification,
}
