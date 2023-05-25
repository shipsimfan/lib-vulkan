use crate::{Loader, Result, VkCreateInstance, VkResult};

pub(super) struct LibraryFunctions {
    pub(super) create_instance: VkCreateInstance,
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

        Ok(LibraryFunctions { create_instance })
    }
}
