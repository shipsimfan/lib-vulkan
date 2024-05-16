//! Cross-platform loader of vulkan drivers

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod driver;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
use windows as os;

pub use driver::Driver;

/// Loads all the Vulkan drivers in the system
pub fn load_drivers() -> Vec<Driver> {
    os::load_drivers()
        .into_iter()
        .map(|driver| Driver::new(driver))
        .collect()
}
