use crate::{
    Loader, Result, VkDestroySurfaceKHR, VkGetPhysicalDeviceSurfaceSupportKHR, VkInstance,
};

pub(crate) struct SurfaceFunctions {
    pub(super) destroy_surface: VkDestroySurfaceKHR,
    pub(crate) get_physical_device_surface_support: VkGetPhysicalDeviceSurfaceSupportKHR,
}

macro_rules! load_function {
    ($loader: ident, $instance: ident, $name: literal) => {
        $loader
            .get_instance_proc_addr(Some($instance), $name)
            .map(|function| unsafe { std::mem::transmute(function) })
            .ok_or(crate::VkResult::IncompatibleDriver)
    };
}

impl SurfaceFunctions {
    pub(crate) fn load<L: Loader>(loader: &L, instance: VkInstance) -> Result<Self> {
        let destroy_surface = load_function!(loader, instance, "vkDestroySurfaceKHR")?;
        let get_physical_device_surface_support =
            load_function!(loader, instance, "vkGetPhysicalDeviceSurfaceSupportKHR")?;

        Ok(SurfaceFunctions {
            destroy_surface,
            get_physical_device_surface_support,
        })
    }
}
