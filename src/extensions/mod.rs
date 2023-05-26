mod surface;
#[cfg(target_os = "windows")]
mod win32_surface;

pub use surface::Surface;
#[cfg(target_os = "windows")]
pub use win32_surface::Win32SurfaceCreateInfo;

pub(crate) use surface::*;
#[cfg(target_os = "windows")]
pub(crate) use win32_surface::*;
