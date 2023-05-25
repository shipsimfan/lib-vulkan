use crate::{
    ExtensionProperties, Library, Loader, NativeLoader, Result, VkApplicationInfo,
    VkCreateInstance, VkInstance, VkInstanceCreateInfo, VkResult, VkStructureType,
};
use functions::InstanceFunctions;
use std::{
    ffi::{CStr, CString},
    ptr::{null, null_mut},
    sync::Arc,
};

mod application_info;
mod create_info;
mod functions;

pub use application_info::ApplicationInfo;
pub use create_info::InstanceCreateInfo;

pub struct Instance<L: Loader = NativeLoader> {
    handle: VkInstance,
    #[allow(unused)]
    library: Arc<Library<L>>,

    functions: InstanceFunctions,
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
                p_application_name: application_name
                    .as_ref()
                    .map(|name| name.as_ptr())
                    .unwrap_or(null()),
                application_version: application_info
                    .version
                    .map(|version| version.as_u32())
                    .unwrap_or(0),
                p_engine_name: engine_name
                    .as_ref()
                    .map(|name| name.as_ptr())
                    .unwrap_or(null()),
                engine_version: application_info
                    .engine_version
                    .map(|version| version.as_u32())
                    .unwrap_or(0),
                api_version: application_info.api_version.as_u32(),
            }
        });

        // Prepare the create info
        let enabled_layers: Vec<_> = create_info
            .enabled_layers
            .into_iter()
            .map(|layer| CString::new(layer).unwrap())
            .collect();
        let enabled_layer_ptrs: Vec<_> =
            enabled_layers.iter().map(|layer| layer.as_ptr()).collect();
        let enabled_extensions: Vec<_> = create_info
            .enabled_extensions
            .into_iter()
            .map(|extension| CString::new(extension).unwrap())
            .collect();
        let enabled_extension_ptrs: Vec<_> = enabled_extensions
            .iter()
            .map(|extension| extension.as_ptr())
            .collect();

        let create_info = VkInstanceCreateInfo {
            s_type: VkStructureType::InstanceCreateInfo,
            p_next: null(),
            flags: create_info.flags,
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
        let handle = match (create_instance)(&create_info, null(), &mut handle) {
            VkResult::Success => handle.unwrap(),
            result => return Err(result),
        };

        // Create the instance
        let functions = InstanceFunctions::load(library.loader(), handle)?;

        Ok(Arc::new(Instance {
            handle,
            library,

            functions,
        }))
    }

    pub fn enumerate_extension_properties(
        &self,
        layer_name: Option<&str>,
    ) -> Result<Vec<ExtensionProperties>> {
        let layer_name = layer_name.map(|name| CString::new(name).unwrap());
        let p_layer_name = layer_name
            .as_ref()
            .map(|name| name.as_ptr())
            .unwrap_or(null());

        let mut count = 0;
        match (self.functions.enumerate_extension_properties)(p_layer_name, &mut count, null_mut())
        {
            VkResult::Success => {}
            result => return Err(result),
        }

        let mut extensions = Vec::with_capacity(count as usize);
        loop {
            match (self.functions.enumerate_extension_properties)(
                p_layer_name,
                &mut count,
                extensions.as_mut_ptr(),
            ) {
                VkResult::Success => {
                    unsafe { extensions.set_len(count as usize) };
                    return Ok(extensions
                        .into_iter()
                        .map(|extension| ExtensionProperties {
                            name: CStr::from_bytes_until_nul(&extension.extension_name)
                                .unwrap()
                                .to_string_lossy()
                                .to_string(),
                            spec_version: extension.spec_version.into(),
                        })
                        .collect());
                }
                VkResult::Incomplete => {
                    extensions.reserve(count as usize);
                }
                result => return Err(result),
            }
        }
    }
}

impl<L: Loader> Drop for Instance<L> {
    fn drop(&mut self) {
        (self.functions.destroy_instance)(self.handle, null())
    }
}
