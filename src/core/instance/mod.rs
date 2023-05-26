use crate::{
    opt_cstring_as_ptr, opt_version_as_u32, string_vec_to_cstring_vec, Library, Loader,
    NativeLoader, PhysicalDevice, PhysicalDeviceFunctions, Result, Surface, SurfaceFunctions,
    VkApplicationInfo, VkCreateInstance, VkDevice, VkInstance, VkInstanceCreateFlags,
    VkInstanceCreateInfo, VkResult, VkStructureType,
};
use child_functions::ChildFunctions;
use functions::InstanceFunctions;
use std::{
    ffi::CString,
    ptr::{null, null_mut},
    sync::Arc,
};

mod application_info;
mod child_functions;
mod create_info;
mod functions;

pub use application_info::ApplicationInfo;
pub use create_info::InstanceCreateInfo;

pub struct Instance<L: Loader = NativeLoader> {
    handle: VkInstance,
    #[allow(unused)]
    library: Arc<Library<L>>,

    functions: InstanceFunctions,

    child_functions: ChildFunctions,
}

impl<L: Loader> Instance<L> {
    pub(crate) fn create_instance(
        create_info: InstanceCreateInfo,
        create_instance: VkCreateInstance,
        library: Arc<Library<L>>,
    ) -> Result<Arc<Self>> {
        // Prepare the application info
        let mut application_name = None;
        let mut engine_name = None;
        let application_info = create_info.application_info.map(|application_info| {
            application_name = application_info
                .name
                .map(|name| CString::new(name).unwrap());
            engine_name = application_info
                .engine_name
                .map(|name| CString::new(name).unwrap());

            VkApplicationInfo {
                s_type: VkStructureType::ApplicationInfo,
                p_next: null(),
                p_application_name: opt_cstring_as_ptr!(application_name),
                application_version: opt_version_as_u32!(application_info.version),
                p_engine_name: opt_cstring_as_ptr!(engine_name),
                engine_version: opt_version_as_u32!(application_info.engine_version),
                api_version: application_info.api_version.as_u32(),
            }
        });

        // Prepare the create info
        let (enabled_layers, enabled_layer_ptrs) =
            string_vec_to_cstring_vec!(create_info.enabled_layers);
        let (enabled_extensions, enabled_extension_ptrs) =
            string_vec_to_cstring_vec!(create_info.enabled_extensions.clone());

        let vk_create_info = VkInstanceCreateInfo {
            s_type: VkStructureType::InstanceCreateInfo,
            p_next: null(),
            flags: VkInstanceCreateFlags::default(),
            p_application_info: application_info
                .as_ref()
                .map(|application_info| application_info as *const _)
                .unwrap_or(null()),
            enabled_layer_count: enabled_layers.len() as u32,
            pp_enabled_layers: if enabled_layers.len() == 0 {
                null()
            } else {
                enabled_layer_ptrs.as_ptr()
            },
            enabled_extension_count: enabled_extensions.len() as u32,
            pp_enabled_extension_names: if enabled_extensions.len() == 0 {
                null()
            } else {
                enabled_extension_ptrs.as_ptr()
            },
        };

        // Call vkCreateInstance function
        let mut handle = None;
        let handle = match (create_instance)(&vk_create_info, null(), &mut handle) {
            VkResult::Success => handle.unwrap(),
            result => return Err(result),
        };

        // Create the instance
        let functions = InstanceFunctions::load(library.loader(), handle)?;
        let child_functions =
            ChildFunctions::load(library.loader(), handle, &create_info.enabled_extensions)?;

        Ok(Arc::new(Instance {
            handle,
            library,

            functions,
            child_functions,
        }))
    }

    pub fn enumerate_physical_devices(self: &Arc<Self>) -> Result<Vec<PhysicalDevice<L>>> {
        let mut count = 0;
        match (self.functions.enumerate_physical_devices)(self.handle, &mut count, null_mut()) {
            VkResult::Success => {}
            result => return Err(result),
        }

        let mut physical_devices = Vec::with_capacity(count as usize);
        loop {
            match (self.functions.enumerate_physical_devices)(
                self.handle,
                &mut count,
                physical_devices.as_mut_ptr(),
            ) {
                VkResult::Success => {
                    unsafe { physical_devices.set_len(count as usize) };
                    return Ok(physical_devices
                        .into_iter()
                        .map(|physical_device| PhysicalDevice::new(physical_device, self.clone()))
                        .collect());
                }
                VkResult::Incomplete => physical_devices.reserve(count as usize),
                result => return Err(result),
            }
        }
    }

    #[cfg(target_os = "windows")]
    pub fn create_win32_surface(
        self: &Arc<Self>,
        create_info: crate::Win32SurfaceCreateInfo,
    ) -> Result<Surface<L>> {
        crate::create_win32_surface(self.clone(), create_info)
    }

    pub(crate) fn physical_device_functions(&self) -> &PhysicalDeviceFunctions {
        &self.child_functions.physical_device
    }

    pub(crate) fn surface_functions(&self) -> &SurfaceFunctions {
        self.child_functions.surface_functions.as_ref().unwrap()
    }

    #[cfg(target_os = "windows")]
    pub(crate) fn win32_surface_functions(&self) -> &crate::Win32SurfaceFunctions {
        self.child_functions
            .win32_surface_functions
            .as_ref()
            .unwrap()
    }

    pub(crate) fn get_device_proc_addr(&self, device: VkDevice, name: &str) -> Option<*const ()> {
        let name = CString::new(name).unwrap();
        (self.functions.get_device_proc_addr)(device, name.as_ptr()).map(|f| f as _)
    }

    pub(crate) fn handle(&self) -> VkInstance {
        self.handle
    }
}

impl<L: Loader> Drop for Instance<L> {
    fn drop(&mut self) {
        (self.functions.destroy_instance)(self.handle, null())
    }
}
