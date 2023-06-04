#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkAttachmentStoreOp {
    Store = 0,
    DontCare = 1,
}
