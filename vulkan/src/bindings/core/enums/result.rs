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
    ErrorOutOfPoolMemory = -1000069000,
    ErrorInvalidExternalHandle = -1000072003,
    ErrorFragmentation = -1000161000,
    ErrorInvalidOpaqueCaptureAddress = -1000257000,
    PipelineCompileRequired = 1000297000,
    ErrorSurfaceLostKHR = -1000000000,
    ErrorNativeWindowInUseKHR = -1000000001,
    SuboptimalKHR = 1000001003,
    ErrorOutOfDateKHR = -1000001004,
    ErrorIncompatibleDisplayKHR = -1000003001,
    ErrorImageUsageNotSupportedKHR = -1000023000,
    ErrorVideoPictureLayoutNotSupportedKHR = -1000023001,
    ErrorVideoProfileOperationNotSupportedKHR = -1000023002,
    ErrorVideoProfileFormatNotSupportedKHR = -1000023003,
    ErrorVideoProfileCodecNotSupportedKHR = -1000023004,
    ErrorVideoStdVersionNotSupportedKHR = -1000023005,
    ErrorNotPermittedKHR = -1000174001,
    ThreadIdleKHR = 1000268000,
    ThreadDoneKHR = 1000268001,
    OperationDeferredKHR = 1000268002,
    OperationNotDeferredKHR = 1000268003,
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
                ErrorOutOfPoolMemory => "Out of pool memory",
                ErrorInvalidExternalHandle => "Invalid external handle",
                ErrorFragmentation => "Fragmentation",
                ErrorInvalidOpaqueCaptureAddress => "Invalid opaque capture address",
                PipelineCompileRequired => "Pipeline compile required",
                ErrorSurfaceLostKHR => "Surface lost",
                ErrorNativeWindowInUseKHR => "Native window in use",
                SuboptimalKHR => "Suboptimal",
                ErrorOutOfDateKHR => "Out of date",
                ErrorIncompatibleDisplayKHR => "Incompatible display",
                ErrorImageUsageNotSupportedKHR => "Image usage not supported",
                ErrorVideoPictureLayoutNotSupportedKHR => "Video picture layout not supported",
                ErrorVideoProfileOperationNotSupportedKHR =>
                    "Video profile operation not supported",
                ErrorVideoProfileFormatNotSupportedKHR => "Video profile format not supported",
                ErrorVideoProfileCodecNotSupportedKHR => "Video profile codec not supported",
                ErrorVideoStdVersionNotSupportedKHR => "Video standard version not supported",
                ErrorNotPermittedKHR => "Not permitted",
                ThreadIdleKHR => "Thread idle",
                ThreadDoneKHR => "Thread done",
                OperationDeferredKHR => "Operation deferred",
                OperationNotDeferredKHR => "Operation not deferred",
                #[allow(unreachable_patterns)]
                _ => return write!(f, "Unknown ({:#010X})", *self as u32),
            }
        )
    }
}
