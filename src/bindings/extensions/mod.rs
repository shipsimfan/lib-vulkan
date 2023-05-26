mod surface;
#[cfg(target_os = "windows")]
mod win32_surface;

pub use surface::{VkColorSpaceKHR, VkSurfaceFormatKHR};

pub(crate) use surface::*;
#[cfg(target_os = "windows")]
pub(crate) use win32_surface::*;
