use crate::{Loader, Result, VkDestroyInstance, VkInstance, VkResult};

pub(super) struct InstanceFunctions {
    pub(super) destroy_instance: VkDestroyInstance,
}

macro_rules! load_function {
    ($loader: ident, $instance: ident, $name: literal) => {
        $loader
            .get_instance_proc_addr(Some($instance), $name)
            .map(|function| unsafe { std::mem::transmute(function) })
            .ok_or(VkResult::IncompatibleDriver)
    };
}

impl InstanceFunctions {
    pub(super) fn load<L: Loader>(loader: &L, instance: VkInstance) -> Result<Self> {
        let destroy_instance = load_function!(loader, instance, "vkDestroyInstance")?;

        Ok(InstanceFunctions { destroy_instance })
    }
}
