use crate::{
    Instance, Loader, Result, Surface, VkResult, VkStructureType, VkWin32SurfaceCreateFlagsKHR,
    VkWin32SurfaceCreateInfoKHR,
};
use std::{ptr::null, sync::Arc};

mod create_info;
mod functions;

pub use create_info::Win32SurfaceCreateInfo;

pub(crate) use functions::Win32SurfaceFunctions;

pub(crate) fn create_win32_surface<L: Loader>(
    instance: Arc<Instance<L>>,
    create_info: Win32SurfaceCreateInfo,
) -> Result<Surface<L>> {
    // Prepare the create info
    let create_info = VkWin32SurfaceCreateInfoKHR {
        s_type: VkStructureType::Win32SurfaceCreateInfo,
        p_next: null(),
        flags: VkWin32SurfaceCreateFlagsKHR::default(),
        h_instance: create_info.h_instance,
        h_wnd: create_info.h_wnd,
    };

    // Call vkCreateWin32SurfaceKHR
    let mut handle = None;
    let handle = match (instance.win32_surface_functions().create_win32_surface)(
        instance.handle(),
        &create_info,
        null(),
        &mut handle,
    ) {
        VkResult::Success => handle.unwrap(),
        result => return Err(result),
    };

    // Create the surface
    Ok(Surface::new(handle, instance))
}
