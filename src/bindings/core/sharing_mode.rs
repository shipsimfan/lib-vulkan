#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkSharingMode {
    Exclusive = 0,
    Concurrent = 1,
}
