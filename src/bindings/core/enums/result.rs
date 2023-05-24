#[non_exhaustive]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkResult {
    Success = 0,
    NotReady = 1,
    Timeout = 2,
    EventSet = 3,
    EventReset = 4,
    Incomplete = 5,
    Suboptimal = 1000001003,
    PipelineCompileRequired = 1000297000,
    ErrorOutOfHostMemory = -1,
    ErrorOutOfDeviceMemory = -2,
    ErrorInitializationFailed = -3,
    ErrorDeviceLost = -4,
    ErrorMemoryMapFailed = -5,
    ErrorLayerNotPresent = -6,
    ErrorExtensionNotPresent = -7,
    ErrorFeatureNotPresent = -8,
    ErrorIncompatibleDriver = -9,
    ErrorTooManyObjects = -10,
    ErrorFormatNotSupported = -11,
    ErrorFragmentedPool = -12,
    ErrorUnknown = -13,
    ErrorSurfaceLost = -1000000000,
    ErrorNativeWindowInUse = -1000000001,
    ErrorOutOfDate = -1000001004,
    ErrorOutOfPoolMemory = -1000069000,
    ErrorInvalidExternalHandle = -1000072003,
    ErrorFragmentation = -1000161000,
    ErrorInvalidOpaqueCaptureAddress = -1000257000,
}

impl VkResult {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl std::error::Error for VkResult {}

impl std::fmt::Debug for VkResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for VkResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use VkResult::*;

        write!(
            f,
            "{}",
            match self {
                Success => "Success",
                NotReady => "Not ready",
                Timeout => "Timeout",
                EventSet => "Event set",
                EventReset => "Event reset",
                Incomplete => "Incomplete",
                Suboptimal => "Suboptimal",
                PipelineCompileRequired => "Pipeline compile required",
                ErrorOutOfHostMemory => "Out of host memory",
                ErrorOutOfDeviceMemory => "Out of device memory",
                ErrorInitializationFailed => "Initialization failed",
                ErrorDeviceLost => "Device lost",
                ErrorMemoryMapFailed => "Memory map failed",
                ErrorLayerNotPresent => "Layer not present",
                ErrorExtensionNotPresent => "Extension not present",
                ErrorFeatureNotPresent => "Feature not present",
                ErrorIncompatibleDriver => "Incompatible driver",
                ErrorTooManyObjects => "Too many objects",
                ErrorFormatNotSupported => "Format not supported",
                ErrorFragmentedPool => "Fragmented pool",
                ErrorUnknown => "Unknown",
                ErrorSurfaceLost => "Surface lost",
                ErrorNativeWindowInUse => "Native window in use",
                ErrorOutOfDate => "Out of date",
                ErrorOutOfPoolMemory => "Out of pool memory",
                ErrorInvalidExternalHandle => "Invalid external handle",
                ErrorFragmentation => "Fragmentation",
                ErrorInvalidOpaqueCaptureAddress => "Invalid opaque capture address",
                #[allow(unreachable_patterns)]
                _ => return write!(f, "Unknown ({:#010X})", *self as u32),
            }
        )
    }
}
