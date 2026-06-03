use crate::VkStructureType;
use std::ffi::c_void;

mod create;

pub use create::*;

/// An element that can be part of a Vulkan `next` chain
pub trait NextChain {
    /// Get the structure type of this element
    fn structure_type(&self) -> VkStructureType;

    /// Get a pointer to the next chain element
    fn next(&self) -> *const c_void;

    /// Get a pointer to this element, for use in the `next` field of the previous element
    fn as_ptr(&self) -> *const c_void;

    /// Set the next chain element
    fn set_next(&mut self, next: Option<&dyn NextChain>);
}

/// An element that can be part of a Vulkan `next` chain, with mutable access
pub trait NextChainMut {
    /// Get the structure type of this element
    fn structure_type(&self) -> VkStructureType;

    /// Get a mutable pointer to the next chain element
    fn next(&mut self) -> *mut c_void;

    /// Get a mutable pointer to this element, for use in the `next` field of the previous element
    fn as_mut_ptr(&mut self) -> *mut c_void;

    /// Set the next chain element
    fn set_next(&mut self, next: Option<&mut dyn NextChainMut>);
}
