use crate::{Loader, NativeLoader, Result, VkResult};
use std::sync::Arc;

pub struct Library<L: Loader = NativeLoader> {
    #[allow(unused)]
    loader: L,
}

impl Library {
    pub fn new_native() -> Result<Arc<Self>> {
        let loader = NativeLoader::new().ok_or(VkResult::IncompatibleDriver)?;
        Library::new(loader)
    }
}

impl<L: Loader> Library<L> {
    pub fn new(loader: L) -> Result<Arc<Self>> {
        Ok(Arc::new(Library { loader }))
    }
}
