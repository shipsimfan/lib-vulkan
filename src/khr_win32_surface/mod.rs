//! The [`khr_win32_surface`] extension is an instance extension. It provides a mechanism to create
//! a [`VkSurfaceKhr`] object (defined by the [`khr_surface`] extension) that refers to a Win32
//! [`HWND`], as well as a query to determine support for rendering to the windows desktop.

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    khr_surface::{self, VkSurfaceKhr},
    khr_win32_surface,
};
#[allow(unused_imports)]
use win32::HWND;

mod constants;
mod flags;
mod functions;
mod structures;

pub use constants::*;
pub use flags::*;
pub use functions::*;
pub use structures::*;
