use crate::VkFlags;

impl VkFlags {
    /// Creates a new [`VkFlags`] with no flags set
    pub const fn empty() -> VkFlags {
        VkFlags::new(0)
    }

    /// Create new [`VkFlags`] from `f`
    pub const fn new(f: u32) -> VkFlags {
        VkFlags(f)
    }
}

impl const Default for VkFlags {
    fn default() -> Self {
        VkFlags::empty()
    }
}

impl const From<u32> for VkFlags {
    fn from(flags: u32) -> Self {
        VkFlags::new(flags)
    }
}
