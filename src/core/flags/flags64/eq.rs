use crate::VkFlags64;

impl const PartialEq for VkFlags64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl const PartialEq<u64> for VkFlags64 {
    fn eq(&self, other: &u64) -> bool {
        self.0.eq(other)
    }
}

impl Eq for VkFlags64 {}
