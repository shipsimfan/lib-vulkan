#[repr(C)]
#[derive(Clone, PartialEq, Eq)]
pub struct VkExtent2D {
    width: u32,
    height: u32,
}

impl VkExtent2D {
    pub const fn new(width: u32, height: u32) -> Self {
        VkExtent2D { width, height }
    }

    pub const fn width(&self) -> u32 {
        self.width
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}
