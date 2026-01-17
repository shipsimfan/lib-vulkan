//! The [`khr_wayland_surface`] extension is an instance extension. It provides a mechanism to
//! create a [`VkSurfaceKhr`] object (defined by the [`khr_surface`] extension) that refers to a
//! Wayland [`wl_surface`], as well as a query to determine support for rendering to a Wayland
//! compositor.

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    khr_surface::{self, VkSurfaceKhr},
    khr_wayland_surface,
};
#[allow(unused_imports)]
use wayland::wl_surface;

mod constants;
mod flags;
mod functions;
mod structures;

pub use constants::*;
pub use flags::*;
pub use functions::*;
pub use structures::*;
