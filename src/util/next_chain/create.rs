use crate::util::{NextChain, NextChainMut};
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};

/// Create a Vulkan `next` chain from an iterator of `NextChain` elements, returning a pointer to
/// the head of the chain
pub fn create_next_chain<'a, I: IntoIterator<Item = &'a mut dyn NextChain>>(
    chain: I,
) -> *const c_void {
    let mut prev: Option<&mut dyn NextChain> = None;
    for element in chain.into_iter() {
        if let Some(prev) = prev {
            prev.set_next(element.as_ptr());
        }
        prev = Some(element);
    }
    prev.map_or(null(), |element| {
        element.set_next(null());
        element.as_ptr()
    })
}

/// Create a Vulkan `next` chain from an iterator of `NextChainMut` elements, returning a pointer
/// to the head of the chain
pub fn create_next_chain_mut<'a, I: IntoIterator<Item = &'a mut dyn NextChainMut>>(
    chain: I,
) -> *mut c_void {
    let mut prev: Option<&mut dyn NextChainMut> = None;
    for element in chain.into_iter() {
        if let Some(prev) = prev {
            prev.set_next(element.as_mut_ptr());
        }
        prev = Some(element);
    }
    prev.map_or(null_mut(), |element| {
        element.set_next(null_mut());
        element.as_mut_ptr()
    })
}
