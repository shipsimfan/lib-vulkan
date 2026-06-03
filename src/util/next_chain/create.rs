use crate::util::{NextChain, NextChainMut};
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};

/// Create a Vulkan `next` chain from an iterator of `NextChain` elements, returning a pointer to
/// the head of the chain
pub fn create_next_chain<'a, I: IntoIterator<Item = &'a mut (dyn NextChain + 'a)>>(
    chain: I,
) -> *const c_void {
    let mut first_prev: Option<(*const c_void, &mut dyn NextChain)> = None;
    for element in chain.into_iter() {
        if let Some((first, prev)) = first_prev {
            prev.set_next(Some(element));
            first_prev = Some((first, element));
        } else {
            first_prev = Some((element.as_ptr(), element));
        }
    }
    first_prev.map_or(null(), |(first, last)| {
        last.set_next(None);
        first
    })
}

/// Create a Vulkan `next` chain from an iterator of `NextChainMut` elements, returning a pointer
/// to the head of the chain
pub fn create_next_chain_mut<'a, I: IntoIterator<Item = &'a mut (dyn NextChainMut + 'a)>>(
    chain: I,
) -> *mut c_void {
    let mut first_prev: Option<(*mut c_void, &mut dyn NextChainMut)> = None;
    for element in chain.into_iter() {
        if let Some((first, prev)) = first_prev {
            prev.set_next(Some(element));
            first_prev = Some((first, element));
        } else {
            first_prev = Some((element.as_mut_ptr(), element));
        }
    }
    first_prev.map_or(null_mut(), |(first, last)| {
        last.set_next(None);
        first
    })
}
