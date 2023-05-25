use crate::{
    ExtensionProperties, Instance, InstanceCreateInfo, LayerProperties, Loader, NativeLoader,
    Result, VkResult,
};
use functions::LibraryFunctions;
use std::{
    ffi::{CStr, CString},
    ptr::{null, null_mut},
    sync::Arc,
};

mod functions;

pub struct Library<L: Loader = NativeLoader> {
    loader: L,

    functions: LibraryFunctions,
}

impl Library {
    pub fn new_native() -> Result<Arc<Self>> {
        let loader = NativeLoader::new().ok_or(VkResult::IncompatibleDriver)?;
        Library::new(loader)
    }
}

impl<L: Loader> Library<L> {
    pub fn new(loader: L) -> Result<Arc<Self>> {
        let functions = LibraryFunctions::load(&loader)?;

        Ok(Arc::new(Library { loader, functions }))
    }

    pub fn create_instance(
        self: Arc<Self>,
        create_info: InstanceCreateInfo,
    ) -> Result<Arc<Instance<L>>> {
        Instance::create_instance(create_info, self.functions.create_instance, self)
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

    pub fn enumerate_layer_properties(&self) -> Result<Vec<LayerProperties>> {
        let mut count = 0;
        match (self.functions.enumerate_layer_properties)(&mut count, null_mut()) {
            VkResult::Success => {}
            result => return Err(result),
        }

        let mut layers = Vec::with_capacity(count as usize);
        loop {
            match (self.functions.enumerate_layer_properties)(&mut count, layers.as_mut_ptr()) {
                VkResult::Success => {
                    unsafe { layers.set_len(count as usize) };
                    return Ok(layers
                        .into_iter()
                        .map(|layer| LayerProperties {
                            name: CStr::from_bytes_until_nul(&layer.layer_name)
                                .unwrap()
                                .to_string_lossy()
                                .to_string(),
                            version: layer.spec_version.into(),
                            implementation_version: layer.implementation_version,
                            description: CStr::from_bytes_until_nul(&layer.description)
                                .unwrap()
                                .to_string_lossy()
                                .to_string(),
                        })
                        .collect());
                }
                VkResult::Incomplete => {
                    layers.reserve(count as usize);
                }
                result => return Err(result),
            }
        }
    }

    pub(crate) fn loader(&self) -> &L {
        &self.loader
    }
}
