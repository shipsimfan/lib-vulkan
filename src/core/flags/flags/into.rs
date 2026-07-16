use crate::VkFlags;

const impl Into<u32> for VkFlags {
    fn into(self) -> u32 {
        self.0
    }
}
