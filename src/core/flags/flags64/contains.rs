use crate::VkFlags64;

impl VkFlags64 {
    /// Do these flags contain `flags`
    pub const fn contains<F: [const] Into<VkFlags64>>(&self, flags: F) -> bool {
        self.0 & flags.into().0 != 0
    }
}
