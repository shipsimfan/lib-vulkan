use crate::VkFlags64;

impl const Clone for VkFlags64 {
    fn clone(&self) -> Self {
        VkFlags64::new(self.0)
    }
}

impl Copy for VkFlags64 {}
