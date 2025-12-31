use crate::VkResult;

impl VkResult {
    /// Gets a string representation of the error
    pub const fn as_str(&self) -> Option<&'static str> {
        #[allow(unreachable_patterns)]
        Some(match self {
            VkResult::VkSuccess => "command successfully completed",
            VkResult::VkNotReady => "a fence or query has not yet completed",
            VkResult::VkTimeout => "a wait operation has not completed in the specified time",
            VkResult::VkEventSet => "an event is signaled",
            VkResult::VkEventReset => "an event is unsignaled",
            VkResult::VkIncomplete => "a return array was too small for the result",
            VkResult::VkErrorOutOfHostMemory => "a host memory allocation has failed",
            VkResult::VkErrorOutOfDeviceMemory => "a device memory allocation has failed",
            VkResult::VkErrorInitializationFailed => {
                "initialization of an object could not be completed for implementation-specific reasons"
            }
            VkResult::VkErrorDeviceLost => "the logical or physical device has been lost",
            VkResult::VkErrorMemoryMapFailed => "mapping of a memory object has failed",
            VkResult::VkErrorLayerNotPresent => {
                "a requested layer is not present or could not be loaded"
            }
            VkResult::VkErrorExtensionNotPresent => "a requested extension is not supported",
            VkResult::VkErrorFeatureNotPresent => "a requested feature is not supported",
            VkResult::VkErrorIncompatibleDriver => {
                "the requested version of Vulkan is not supported by the driver or is otherwise incompatible for implementation-specific reasons"
            }
            VkResult::VkErrorTooManyObjects => {
                "too many objects of the type have already been created"
            }
            VkResult::VkErrorFormatNotSupported => {
                "a requested format is not supported on this device"
            }
            VkResult::VkErrorFragmentedPool => {
                "a pool allocation has failed due to fragmentation of the pool’s memory"
            }
            VkResult::VkErrorUnknown => {
                "an unknown error has occurred; either the application has provided invalid input, or an implementation failure has occurred"
            }
            VkResult::VkErrorOutOfPoolMemory => "a pool memory allocation has failed",
            VkResult::VkErrorInvalidExternalHandle => {
                "an external handle is not a valid handle of the specified type"
            }
            VkResult::VkErrorFragmentation => {
                "a descriptor pool creation has failed due to fragmentation"
            }
            VkResult::VkErrorInvalidOpaqueCaptureAddress => {
                "a buffer creation or memory allocation failed because the requested address is not available"
            }
            VkResult::VkPipelineCompileRequired => {
                "a requested pipeline creation would have required compilation, but the application requested compilation to not be performed"
            }
            VkResult::VkErrorNotPermitted => {
                "The driver implementation has denied a request to acquire a priority above the default priority"
            }
            VkResult::VkErrorSurfaceLostKhr => "a surface is no longer available",
            VkResult::VkErrorNativeWindowInUseKhr => {
                "the requested window is already in use by Vulkan or another API in a manner which prevents it from being used again"
            }
            VkResult::VkSuboptimalKhr => {
                "a swapchain no longer matches the surface properties exactly, but *can* still be used to present to the surface successfully"
            }
            VkResult::VkErrorOutOfDateKhr => {
                "a surface has changed in such a way that it is no longer compatible with the swapchain, and further presentation requests using the swapchain will fail"
            }
            VkResult::VkErrorIncompatibleDisplayKhr => {
                "the display used by a swapchain does not use the same presentable image layout, or is incompatible in a way that prevents sharing an image"
            }
            VkResult::VkErrorValidationFailedExt => {
                "a command failed because invalid usage was detected by the implementation or a validation-layer"
            }
            VkResult::VkErrorInvalidShaderNv => "one or more shaders failed to compile or link",
            VkResult::VkErrorImageUsageNotSupportedKhr => {
                "the requested image usage flags are not supported"
            }
            VkResult::VkErrorVideoPictureLayoutNotSupportedKhr => {
                "the requested video picture layout is not supported"
            }
            VkResult::VkErrorVideoProfileOperationNotSupportedKhr => {
                "a video profile operation is not supported"
            }
            VkResult::VkErrorVideoProfileFormatNotSupportedKhr => {
                "format parameters requested are not supported"
            }
            VkResult::VkErrorVideoProfileCodecNotSupportedKhr => {
                "codec-specific parameters requested are not supported"
            }
            VkResult::VkErrorVideoStdVersionNotSupportedKhr => {
                "the specified video Std header version is not supported"
            }
            VkResult::VkErrorFullScreenExclusiveModeLostExt => {
                "an operation on a swapchain created with full-screen exclusive failed as it did not have exclusive full-screen access"
            }
            VkResult::VkThreadIdleKhr => {
                "a deferred operation is not complete but there is currently no work for this thread to do at the time of this call"
            }
            VkResult::VkThreadDoneKhr => {
                "a deferred operation is not complete but there is no work remaining to assign to additional threads"
            }
            VkResult::VkOperationDeferredKhr => {
                "a deferred operation was requested and at least some of the work was deferred"
            }
            VkResult::VkOperationNotDeferredKhr => {
                "a deferred operation was requested and no operations were deferred"
            }
            VkResult::VkErrorInvalidVideoStdParametersKhr => {
                "the specified Video Std parameters do not adhere to the syntactic or semantic requirements of the used video compression standard, or values derived from parameters according to the rules defined by the used video compression standard do not adhere to the capabilities of the video compression standard or the implementation"
            }
            VkResult::VkErrorCompressionExhaustedExt => {
                "an image creation failed because internal resources required for compression are exhausted"
            }
            VkResult::VkIncompatibleShaderBinaryExt => {
                "the provided binary shader code is not compatible with this device"
            }
            _ => return None,
        })
    }
}
