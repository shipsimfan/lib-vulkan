//! The [`khr_surface`] extension is an instance extension. It introduces [`VkSurfaceKHR`] objects,
//! which abstract native platform surface or window objects for use with Vulkan. It also provides
//! a way to determine whether a queue family in a physical device supports presenting to
//! particular surface.
//!
//! Separate extensions for each platform provide the mechanisms for creating [`VkSurfaceKHR`]
//! objects, but once created they may be used in this and other platform-independent extensions,
//! in particular the [`khr_swapchain`] extension.

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_surface, khr_swapchain};

mod constants;
mod enumerations;
mod functions;
mod structures;
mod types;

pub use constants::*;
pub use enumerations::*;
pub use functions::*;
pub use structures::*;
pub use types::*;
