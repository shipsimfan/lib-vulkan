#[non_exhaustive]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkPresentModeKHR {
    Immediate = 0,
    Mailbox = 1,
    FIFO = 2,
    FIFORelaxed = 3,
}
