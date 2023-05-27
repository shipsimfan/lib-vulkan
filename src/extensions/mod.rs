mod surface;
mod swapchain;
#[cfg(target_os = "windows")]
mod win32_surface;

pub use surface::Surface;
pub use swapchain::{Swapchain, SwapchainCreateInfo};
#[cfg(target_os = "windows")]
pub use win32_surface::Win32SurfaceCreateInfo;

pub(crate) use surface::*;
pub(crate) use swapchain::*;
#[cfg(target_os = "windows")]
pub(crate) use win32_surface::*;
