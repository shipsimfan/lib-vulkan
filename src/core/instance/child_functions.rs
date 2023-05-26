use crate::{Loader, PhysicalDeviceFunctions, Result, SurfaceFunctions, VkInstance};

pub(super) struct ChildFunctions {
    pub(super) physical_device: PhysicalDeviceFunctions,

    pub(super) surface_functions: Option<SurfaceFunctions>,
    #[cfg(target_os = "windows")]
    pub(super) win32_surface_functions: Option<crate::Win32SurfaceFunctions>,
}

impl ChildFunctions {
    pub(crate) fn load<L: Loader>(
        loader: &L,
        instance: VkInstance,
        extension_list: &[String],
    ) -> Result<Self> {
        let physical_device = PhysicalDeviceFunctions::load(loader, instance)?;

        let mut surface_functions = None;
        #[cfg(target_os = "windows")]
        let mut win32_surface_functions = None;

        for extension in extension_list {
            match extension.as_str() {
                "VK_KHR_surface" => {
                    surface_functions = Some(SurfaceFunctions::load(loader, instance)?)
                }
                #[cfg(target_os = "windows")]
                "VK_KHR_win32_surface" => {
                    win32_surface_functions =
                        Some(crate::Win32SurfaceFunctions::load(loader, instance)?)
                }
                _ => {}
            }
        }

        Ok(ChildFunctions {
            physical_device,
            surface_functions,
            #[cfg(target_os = "windows")]
            win32_surface_functions,
        })
    }
}
