#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkResult {
    Success = 0,
    NotReady = 1,
    Timeout = 2,
    EventSet = 3,
    EventReset = 4,
    Incomplete = 5,

    OutOfHostMemory = -1,
    OutOfDeviceMemory = -2,
    InitializationFailed = -3,
    DeviceLost = -4,
    MemoryMapFailed = -5,
    LayerNotPresent = -6,
    ExtensionNotPresent = -7,
    FeatureNotPresent = -8,
    IncompatibleDriver = -9,
    TooManyObjects = -10,
    FormatNotSupported = -11,
    FragmentedPool = -12,
    Unknown = -13,
}

impl VkResult {
    pub const fn is_error(&self) -> bool {
        use VkResult::*;

        match self {
            Success | NotReady | Timeout | EventSet | EventReset | Incomplete => false,
            _ => true,
        }
    }

    pub const fn message(&self) -> &'static str {
        use VkResult::*;

        match self {
            Success => "Success",
            NotReady => "Not ready",
            Timeout => "Timeout",
            EventSet => "Event set",
            EventReset => "Event reset",
            Incomplete => "Incomplete",

            OutOfHostMemory => "Out of host memory",
            OutOfDeviceMemory => "Out of device memory",
            InitializationFailed => "Initialization failed",
            DeviceLost => "Device lost",
            MemoryMapFailed => "Memory map failed",
            LayerNotPresent => "Layer not present",
            ExtensionNotPresent => "Extension not present",
            FeatureNotPresent => "Feature not present",
            IncompatibleDriver => "Incompatible driver",
            TooManyObjects => "Too many objects",
            FormatNotSupported => "Format not supported",
            FragmentedPool => "Fragmented pool",
            Unknown => "Unknown",
        }
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
        write!(f, "{}", self.message())
    }
}
