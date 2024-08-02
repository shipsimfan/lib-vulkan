//! The [`khr_swapchain`] extension is the device-level companion to the [`khr_surface`] extension.
//! It introduces [`VkSwapchainKHR`] objects, which provide the ability to present rendering
//! results to a surface.

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_surface, khr_swapchain};

mod constants;
mod types;

pub use constants::*;
pub use types::*;
