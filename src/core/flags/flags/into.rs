use crate::VkFlags;

impl const Into<u32> for VkFlags {
    fn into(self) -> u32 {
        self.0
    }
}
