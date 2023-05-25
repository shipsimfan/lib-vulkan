use crate::{
    Loader, Result, VkCreateInstance, VkEnumerateInstanceExtensionProperties,
    VkEnumerateInstanceLayerProperties, VkResult,
};

pub(super) struct LibraryFunctions {
    pub(super) create_instance: VkCreateInstance,
    pub(super) enumerate_extension_properties: VkEnumerateInstanceExtensionProperties,
    pub(super) enumerate_layer_properties: VkEnumerateInstanceLayerProperties,
}

macro_rules! load_function {
    ($loader: ident, $name: literal) => {
        $loader
            .get_instance_proc_addr(None, $name)
            .map(|function| unsafe { std::mem::transmute(function) })
            .ok_or(VkResult::IncompatibleDriver)
    };
}

impl LibraryFunctions {
    pub(super) fn load<L: Loader>(loader: &L) -> Result<Self> {
        let create_instance = load_function!(loader, "vkCreateInstance")?;
        let enumerate_extension_properties =
            load_function!(loader, "vkEnumerateInstanceExtensionProperties")?;
        let enumerate_layer_properties =
            load_function!(loader, "vkEnumerateInstanceLayerProperties")?;

        Ok(LibraryFunctions {
            create_instance,
            enumerate_extension_properties,
            enumerate_layer_properties,
        })
    }
}
