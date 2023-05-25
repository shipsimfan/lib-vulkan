use crate::{Instance, InstanceCreateInfo, Loader, NativeLoader, Result, VkResult};
use functions::LibraryFunctions;
use std::sync::Arc;

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

    pub(crate) fn loader(&self) -> &L {
        &self.loader
    }
}
