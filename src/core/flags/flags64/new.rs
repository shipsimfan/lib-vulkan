use crate::VkFlags64;

impl VkFlags64 {
    /// Creates a new [`VkFlags64`] with no flags set
    pub const fn empty() -> VkFlags64 {
        VkFlags64::new(0)
    }

    /// Create new [`VkFlags64`] from `f`
    pub const fn new(f: u64) -> VkFlags64 {
        VkFlags64(f)
    }
}

impl const Default for VkFlags64 {
    fn default() -> Self {
        VkFlags64::empty()
    }
}

impl const From<u64> for VkFlags64 {
    fn from(flags: u64) -> Self {
        VkFlags64::new(flags)
    }
}
