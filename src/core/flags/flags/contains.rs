use crate::VkFlags;

impl VkFlags {
    /// Do these flags contain `flags`
    pub const fn contains<F: [const] Into<VkFlags>>(&self, flags: F) -> bool {
        self.0 & flags.into().0 != 0
    }
}
