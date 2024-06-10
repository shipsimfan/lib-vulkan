//! The [`khr_win32_surface`] extension is an instance extension. It provides a mechanism to create
//! a [`VkSurfaceKHR`] object (defined by the [`khr_surface`] extension) that refers to a Win32
//! [`HWND`], as well as a query to determine support for rendering to the windows desktop.

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_surface, khr_win32_surface, VkSurfaceKHR};

mod constants;
mod functions;
mod structures;
mod types;

pub use constants::*;
pub use functions::*;
pub use structures::*;
pub use types::*;
