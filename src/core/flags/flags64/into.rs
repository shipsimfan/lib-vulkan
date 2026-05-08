use crate::VkFlags64;

impl const Into<u64> for VkFlags64 {
    fn into(self) -> u64 {
        self.0
    }
}
