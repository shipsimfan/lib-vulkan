use crate::VkFlags64;
use std::ops::BitOr;

impl VkFlags64 {
    /// Set the flags `f`
    pub const fn set<F: [const] Into<VkFlags64>>(&mut self, f: F) {
        self.0 |= f.into().0;
    }
}

impl<F: [const] Into<VkFlags64>> const BitOr<F> for VkFlags64 {
    type Output = VkFlags64;

    fn bitor(self, rhs: F) -> Self::Output {
        VkFlags64::new(self.0 | rhs.into().0)
    }
}

impl const BitOr<VkFlags64> for u64 {
    type Output = VkFlags64;

    fn bitor(self, rhs: VkFlags64) -> Self::Output {
        VkFlags64::new(self | rhs.0)
    }
}
