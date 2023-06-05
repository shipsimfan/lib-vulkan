use crate::{CommandPool, Loader, NativeLoader, VkCommandBuffer};
use std::sync::Arc;

pub struct CommandBuffer<L: Loader = NativeLoader> {
    handle: VkCommandBuffer,
    command_pool: Arc<CommandPool<L>>,
}

impl<L: Loader> CommandBuffer<L> {
    pub(crate) fn new(handle: VkCommandBuffer, command_pool: Arc<CommandPool<L>>) -> Self {
        CommandBuffer {
            handle,
            command_pool,
        }
    }
}
