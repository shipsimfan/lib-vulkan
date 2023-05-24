use crate::VkComponentSwizzle;

#[repr(C)]
pub struct VkComponentMapping {
    r: VkComponentSwizzle,
    g: VkComponentSwizzle,
    b: VkComponentSwizzle,
    a: VkComponentSwizzle,
}

impl VkComponentMapping {
    pub const fn new(
        r: VkComponentSwizzle,
        g: VkComponentSwizzle,
        b: VkComponentSwizzle,
        a: VkComponentSwizzle,
    ) -> Self {
        VkComponentMapping { r, g, b, a }
    }
}
