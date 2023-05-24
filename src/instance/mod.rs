use crate::{Library, Loader, NativeLoader, Result, VkResult};
use std::{
    ptr::{null, NonNull},
    sync::Arc,
};

mod create_info;

pub use create_info::InstanceCreateInfo;

pub(crate) use create_info::VkInstanceCreateInfo;

extern "system" {
    pub type VkInstanceT;
}

pub type VkInstance = NonNull<VkInstanceT>;

pub struct Instance<L: Loader = NativeLoader> {
    handle: VkInstance,
    library: Arc<Library<L>>,
}

impl<L: Loader> Instance<L> {
    pub(crate) fn create_instance(
        library: Arc<Library<L>>,
        create_info: InstanceCreateInfo,
    ) -> Result<Self> {
        let create_info = create_info.into_vk();
        let mut handle = None;
        match (library.functions().create_instance)(&create_info, null(), &mut handle) {
            VkResult::Success => Ok(Instance {
                handle: handle.unwrap(),
                library,
            }),
            result => Err(result),
        }
    }
}
