#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkVersion(u32);

pub const VK_API_VERSION_1_0: VkVersion = VkVersion::new(0, 1, 0, 0);
pub const VK_API_VERSION_1_1: VkVersion = VkVersion::new(0, 1, 1, 0);
pub const VK_API_VERSION_1_2: VkVersion = VkVersion::new(0, 1, 2, 0);
pub const VK_API_VERSION_1_3: VkVersion = VkVersion::new(0, 1, 3, 0);

pub const VK_HEADER_VERSION: u16 = 250;
pub const VK_HEADER_VERSION_COMPLETE: VkVersion = VkVersion::new(0, 1, 3, VK_HEADER_VERSION);

macro_rules! opt_version_as_u32 {
    ($version: expr) => {
        $version.map(|version| version.as_u32()).unwrap_or(0)
    };
}

pub(crate) use opt_version_as_u32;

impl VkVersion {
    pub const fn new(variant: u8, major: u8, minor: u16, patch: u16) -> Self {
        VkVersion(
            ((variant as u32) << 29)
                | ((major as u32) << 22)
                | ((minor as u32) << 12)
                | patch as u32,
        )
    }

    pub const fn variant(&self) -> u8 {
        (self.0 >> 29) as u8
    }

    pub const fn major(&self) -> u8 {
        ((self.0 >> 22) & 0x7F) as u8
    }

    pub const fn minor(&self) -> u16 {
        ((self.0 >> 12) & 0x3FF) as u16
    }

    pub const fn patch(&self) -> u16 {
        (self.0 & 0xFFF) as u16
    }

    pub const fn as_u32(&self) -> u32 {
        self.0
    }
}

impl std::fmt::Debug for VkVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for VkVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.variant() > 0 {
            write!(f, "{}.", self.variant())?;
        }

        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

impl From<u32> for VkVersion {
    fn from(value: u32) -> Self {
        VkVersion(value)
    }
}
