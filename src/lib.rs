//! Vulkan raw bindings

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
//
// Constant impl features
#![feature(const_clone)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_default)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
//
// Other features
#![feature(c_size_t)]

use macros::*;

pub mod ext_debug_utils;
pub mod ext_extended_dynamic_state;
pub mod khr_surface;
pub mod khr_swapchain;
#[cfg(all(target_os = "linux", feature = "wayland"))]
pub mod khr_wayland_surface;
#[cfg(all(target_os = "windows", feature = "win32"))]
pub mod khr_win32_surface;

mod core;
mod macros;

pub use core::*;
