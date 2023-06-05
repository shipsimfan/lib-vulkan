use crate::{
    bindings::{VkCommandBufferAllocateInfo, VkStructureType},
    CommandBuffer, CommandBufferLevel, Device, Loader, NativeLoader, Result, VkCommandPool,
    VkResult,
};
use std::{
    ptr::null,
    sync::{Arc, Mutex},
};

mod create_info;
mod functions;

pub use create_info::CommandPoolCreateInfo;

pub(crate) use functions::CommandPoolFunctions;

pub struct CommandPool<L: Loader = NativeLoader> {
    handle: Mutex<VkCommandPool>,
    device: Arc<Device<L>>,
}

impl<L: Loader> CommandPool<L> {
    pub(crate) fn create(
        device: Arc<Device<L>>,
        create_info: CommandPoolCreateInfo,
    ) -> Result<Arc<Self>> {
        let create_info = create_info.into_binding();

        let mut handle = None;
        let handle = match (device.command_pool_functions().create_command_pool)(
            device.handle(),
            &create_info,
            null(),
            &mut handle,
        ) {
            VkResult::Success => Mutex::new(handle.unwrap()),
            result => return Err(result),
        };

        Ok(Arc::new(CommandPool { handle, device }))
    }

    pub fn allocate_command_buffers(
        self: &Arc<Self>,
        level: CommandBufferLevel,
        count: u32,
    ) -> Result<Vec<CommandBuffer<L>>> {
        let handle = self.handle.lock().unwrap();
        let allocate_info = VkCommandBufferAllocateInfo {
            s_type: VkStructureType::CommandBufferAllocateInfo,
            p_next: null(),
            command_pool: *handle,
            level,
            command_buffer_count: count,
        };

        let mut command_buffers = Vec::with_capacity(count as usize);
        match (self
            .device
            .command_pool_functions()
            .allocate_command_buffers)(
            self.device.handle(),
            &allocate_info,
            command_buffers.as_mut_ptr(),
        ) {
            VkResult::Success => {
                unsafe { command_buffers.set_len(count as usize) };
                Ok(command_buffers
                    .into_iter()
                    .map(|handle| CommandBuffer::new(handle, self.clone()))
                    .collect())
            }
            result => Err(result),
        }
    }
}

impl<L: Loader> Drop for CommandPool<L> {
    fn drop(&mut self) {
        (self.device.command_pool_functions().destroy_command_pool)(
            self.device.handle(),
            *self.handle.lock().unwrap(),
            null(),
        )
    }
}
