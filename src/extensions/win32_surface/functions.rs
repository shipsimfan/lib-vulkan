use crate::{Loader, Result, VkCreateWin32SurfaceKHR, VkInstance};

pub(crate) struct Win32SurfaceFunctions {
    pub(super) create_win32_surface: VkCreateWin32SurfaceKHR,
}

macro_rules! load_function {
    ($loader: ident, $instance: ident, $name: literal) => {
        $loader
            .get_instance_proc_addr(Some($instance), $name)
            .map(|function| unsafe { std::mem::transmute(function) })
            .ok_or(crate::VkResult::IncompatibleDriver)
    };
}

impl Win32SurfaceFunctions {
    pub(crate) fn load<L: Loader>(loader: &L, instance: VkInstance) -> Result<Self> {
        let create_win32_surface = load_function!(loader, instance, "vkCreateWin32SurfaceKHR")?;

        Ok(Win32SurfaceFunctions {
            create_win32_surface,
        })
    }
}
