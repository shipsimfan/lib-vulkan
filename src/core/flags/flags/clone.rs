use crate::VkFlags;

const impl Clone for VkFlags {
    fn clone(&self) -> Self {
        VkFlags::new(self.0)
    }
}

impl Copy for VkFlags {}
