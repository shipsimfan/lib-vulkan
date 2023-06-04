#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkAttachmentLoadOp {
    Load = 0,
    Clear = 1,
    DontCare = 2,
}
