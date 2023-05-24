#[repr(C)]
pub struct VkOffset2D {
    x: i32,
    y: i32,
}

impl VkOffset2D {
    pub const fn new(x: i32, y: i32) -> Self {
        VkOffset2D { x, y }
    }

    pub const fn x(&self) -> i32 {
        self.x
    }

    pub const fn y(&self) -> i32 {
        self.y
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}
