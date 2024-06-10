//! Vulkan raw bindings

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(c_size_t)]

pub mod ext_debug_utils;
pub mod khr_surface;
pub mod khr_win32_surface;

mod core;
mod macros;

pub use core::*;
pub use ext_debug_utils::*;
pub use khr_surface::*;
pub use khr_win32_surface::*;
