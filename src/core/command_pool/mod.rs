use crate::{Device, Loader, NativeLoader, Result, VkCommandPool, VkResult};
use std::{ptr::null, sync::Arc};

mod create_info;
mod functions;

pub use create_info::CommandPoolCreateInfo;

pub(crate) use functions::CommandPoolFunctions;

pub struct CommandPool<L: Loader = NativeLoader> {
    handle: VkCommandPool,
    device: Arc<Device<L>>,
}

impl<L: Loader> CommandPool<L> {
    pub(crate) fn create(
        device: Arc<Device<L>>,
        create_info: CommandPoolCreateInfo,
    ) -> Result<Self> {
        let create_info = create_info.into_binding();

        let mut handle = None;
        let handle = match (device.command_pool_functions().create_command_pool)(
            device.handle(),
            &create_info,
            null(),
            &mut handle,
        ) {
            VkResult::Success => handle.unwrap(),
            result => return Err(result),
        };

        Ok(CommandPool { handle, device })
    }
}

impl<L: Loader> Drop for CommandPool<L> {
    fn drop(&mut self) {
        (self.device.command_pool_functions().destroy_command_pool)(
            self.device.handle(),
            self.handle,
            null(),
        )
    }
}
