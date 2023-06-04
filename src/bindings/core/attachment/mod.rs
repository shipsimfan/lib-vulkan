mod description;
mod description_flags;
mod load_op;
mod reference;
mod store_op;

pub use description::VkAttachmentDescription;
pub use description_flags::{VkAttachmentDescriptionFlagBits, VkAttachmentDescriptionFlags};
pub use load_op::VkAttachmentLoadOp;
pub use reference::VkAttachmentReference;
pub use store_op::VkAttachmentStoreOp;
