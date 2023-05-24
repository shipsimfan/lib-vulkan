use crate::bindings::VkFlags;

#[non_exhaustive]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkDeviceGroupPresentModeFlagBitsKHR {
    Local = 0x00000001,
    Remote = 0x00000002,
    Sum = 0x00000004,
    LocalMultiDevice = 0x00000008,
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VkDeviceGroupPresentModeFlagsKHR(VkFlags);

impl VkDeviceGroupPresentModeFlagsKHR {
    pub const fn new(bits: &[VkDeviceGroupPresentModeFlagBitsKHR]) -> Self {
        let mut flags = 0;
        let mut i = 0;
        while i < bits.len() {
            flags |= bits[i] as u32;
            i += 1;
        }

        VkDeviceGroupPresentModeFlagsKHR(flags)
    }

    pub const fn contains(&self, bit: VkDeviceGroupPresentModeFlagBitsKHR) -> bool {
        (self.0 as u32 & bit as u32) == bit as u32
    }
}
