use crate::VkFlags;

impl VkFlags {
    /// Creates a new [`VkFlags`] with no flags set
    pub const fn new() -> VkFlags {
        VkFlags::from_u32(0)
    }

    /// Create new [`VkFlags`] from `f`
    pub const fn from_u32(f: u32) -> VkFlags {
        VkFlags(f)
    }
}

impl const Default for VkFlags {
    fn default() -> Self {
        VkFlags::new()
    }
}

impl const From<u32> for VkFlags {
    fn from(flags: u32) -> Self {
        VkFlags::from_u32(flags)
    }
}
