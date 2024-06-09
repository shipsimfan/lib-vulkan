// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Supported physical device types
///
/// The physical device type is advertised for informational purposes only, and does not directly
/// affect the operation of the system. However, the device type may correlate with other
/// advertised properties or capabilities of the system, such as how many memory heaps there are.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkPhysicalDeviceType {
    /// The device does not match any other available types.
    Other = 0,

    /// The device is typically one embedded in or tightly coupled with the host.
    IntegratedGPU = 1,

    /// The device is typically a separate processor connected to the host via an interlink.
    DiscreteGPU = 2,

    /// The device is typically a virtual node in a virtualization environment.
    VirtualGPU = 3,

    /// The device is typically running on the same processors as the host.
    CPU = 4,
}
