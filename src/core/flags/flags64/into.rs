use crate::VkFlags64;

const impl Into<u64> for VkFlags64 {
    fn into(self) -> u64 {
        self.0
    }
}
