#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkPresentModeKHR {
    Immediate = 0,
    Mailbox = 1,
    FIFO = 2,
    FIFORelaxed = 3,
    SharedDemandRefresh = 1000111000,
    ContinuousRefresh = 1000111001,
}
