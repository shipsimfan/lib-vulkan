#[repr(C)]
#[derive(Clone, Copy)]
pub enum VkQueueGlobalPriorityKHR {
    Low = 128,
    Medium = 256,
    High = 512,
    Realtime = 1024,
}
