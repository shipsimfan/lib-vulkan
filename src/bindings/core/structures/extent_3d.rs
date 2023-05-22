#[repr(C)]
#[derive(Clone, PartialEq, Eq)]
pub struct VkExtent3D {
    width: u32,
    height: u32,
    depth: u32,
}

impl VkExtent3D {
    pub const fn new(width: u32, height: u32, depth: u32) -> Self {
        VkExtent3D {
            width,
            height,
            depth,
        }
    }

    pub const fn width(&self) -> u32 {
        self.width
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub const fn depth(&self) -> u32 {
        self.depth
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn set_depth(&mut self, depth: u32) {
        self.depth = depth;
    }
}
