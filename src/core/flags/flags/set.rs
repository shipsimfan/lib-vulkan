use crate::VkFlags;
use std::ops::BitOr;

impl VkFlags {
    /// Set the flags `f`
    pub const fn set<F: [const] Into<VkFlags>>(&mut self, f: F) {
        self.0 |= f.into().0;
    }
}

impl<F: [const] Into<VkFlags>> const BitOr<F> for VkFlags {
    type Output = VkFlags;

    fn bitor(self, rhs: F) -> Self::Output {
        VkFlags::new(self.0 | rhs.into().0)
    }
}

impl const BitOr<VkFlags> for u32 {
    type Output = VkFlags;

    fn bitor(self, rhs: VkFlags) -> Self::Output {
        VkFlags::new(self | rhs.0)
    }
}
