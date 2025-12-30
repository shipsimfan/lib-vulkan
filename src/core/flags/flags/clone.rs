use crate::VkFlags;

impl const Clone for VkFlags {
    fn clone(&self) -> Self {
        VkFlags::from_u32(self.0)
    }
}

impl Copy for VkFlags {}
