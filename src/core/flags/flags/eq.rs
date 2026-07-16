use crate::VkFlags;

const impl PartialEq for VkFlags {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

const impl PartialEq<u32> for VkFlags {
    fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }
}

impl Eq for VkFlags {}
