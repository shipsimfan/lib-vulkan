use crate::bindings::VkFlags;

#[non_exhaustive]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkSurfaceTransformFlagBitsKHR {
    Identity = 0x00000001,
    Rotate90 = 0x00000002,
    Rotate180 = 0x00000004,
    Rotate270 = 0x00000008,
    HorizontalMirror = 0x00000010,
    HorizontalMirrorRotate90 = 0x00000020,
    HorizontalMirrorRotate180 = 0x00000040,
    HorizontalMirrorRotate270 = 0x00000080,
    Inherit = 0x00000100,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkSurfaceTransformFlagsKHR(VkFlags);

impl VkSurfaceTransformFlagsKHR {
    pub const fn new(bits: &[VkSurfaceTransformFlagBitsKHR]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkSurfaceTransformFlagsKHR(flags)
    }

    pub const fn contains(&self, bit: VkSurfaceTransformFlagBitsKHR) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}
