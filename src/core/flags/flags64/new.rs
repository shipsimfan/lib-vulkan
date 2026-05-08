use crate::VkFlags64;

impl VkFlags64 {
    /// Creates a new [`VkFlags64`] with no flags set
    pub const fn new() -> VkFlags64 {
        VkFlags64::from_u64(0)
    }

    /// Create new [`VkFlags64`] from `f`
    pub const fn from_u64(f: u64) -> VkFlags64 {
        VkFlags64(f)
    }
}

impl const Default for VkFlags64 {
    fn default() -> Self {
        VkFlags64::new()
    }
}

impl const From<u64> for VkFlags64 {
    fn from(flags: u64) -> Self {
        VkFlags64::from_u64(flags)
    }
}
