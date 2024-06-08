// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_surface, VK_VERSION_1_0, VK_VERSION_1_1, VK_VERSION_1_2, VK_VERSION_1_3};

/// Vulkan command return codes
///
/// While the core Vulkan API is not designed to capture incorrect usage, some circumstances still
/// require return codes. Commands in Vulkan return their status via return codes that are in one
/// of two categories:
///  * Successful completion codes are returned when a command needs to communicate success or
///    status information. All successful completion codes are non-negative values.
///  * Run time error codes are returned when a command needs to communicate a failure that could
///    only be detected at runtime. All runtime error codes are negative values.
///
/// All return codes in Vulkan are reported via [`VkResult`] return values.
///
/// If a command returns a runtime error, unless otherwise specified any output parameters will
/// have undefined contents, except that if the output parameter is a structure with `r#type` and
/// `next` fields, those fields will be unmodified. Any structures chained from pNext will also
/// have undefined contents, except that `r#type` and `next` will be unmodified.
///
/// `VK_ERROR_OUT_OF_*_MEMORY` errors do not modify any currently existing Vulkan objects. Objects
/// that have already been successfully created can still be used by the application.
///
/// [`VkResult::VkErrorUnknown`] will be returned by an implementation when an unexpected error
/// occurs that cannot be attributed to valid behavior of the application and implementation. Under
/// these conditions, it may be returned from any command returning a [`VkResult`].
///
/// Any command returning a [`VkResult`] may return [`VkResult::VkErrorValidationFailedExt`] if a
/// violation of valid usage is detected, even though commands do not explicitly list this as a
/// possible return code.
///
/// Performance-critical commands generally do not have return codes. If a runtime error occurs in
/// such commands, the implementation will defer reporting the error until a specified point. For
/// commands that record into command buffers (`vkCmd*`) runtime errors are reported by
/// [`VkEndCommandBuffer`].
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VkResult {
    /// Command successfully completed
    VkSuccess = 0,

    /// A fence or query has not yet completed
    VkNotReady = 1,

    /// A wait operation has not completed in the specified time
    VkTimeout = 2,

    /// An event is signaled
    VkEventSet = 3,

    /// An event is unsignaled
    VkEventReset = 4,

    /// A return array was too small for the result
    VkIncomplete = 5,

    /// A host memory allocation has failed.
    VkErrorOutOfHostMemory = -1,

    /// A device memory allocation has failed.
    VkErrorOutOfDeviceMemory = -2,

    /// Initialization of an object could not be completed for implementation-specific reasons.
    VkErrorInitializationFailed = -3,

    /// The logical or physical device has been lost.
    VkErrorDeviceLost = -4,

    /// Mapping of a memory object has failed.
    VkErrorMemoryMapFailed = -5,

    /// A requested layer is not present or could not be loaded.
    VkErrorLayerNotPresent = -6,

    /// A requested extension is not supported.
    VkErrorExtensionNotPresent = -7,

    /// A requested feature is not supported.
    VkErrorFeatureNotPresent = -8,

    /// The requested version of Vulkan is not supported by the driver or is otherwise incompatible
    /// for implementation-specific reasons.
    VkErrorIncompatibleDriver = -9,

    /// Too many objects of the type have already been created.
    VkErrorTooManyObjects = -10,

    /// A requested format is not supported on this device.
    VkErrorFormatNotSupported = -11,

    /// A pool allocation has failed due to fragmentation of the pool’s memory. This *must* only be
    /// returned if no attempt to allocate host or device memory was made to accommodate the new
    /// allocation. This *should* be returned in preference to [`VkResult::VkErrorOutOfPoolMemory`],
    /// but only if the implementation is certain that the pool allocation failure was due to
    /// fragmentation.
    VkErrorFragmentedPool = -12,

    /// An unknown error has occurred; either the application has provided invalid input, or an
    /// implementation failure has occurred.
    VkErrorUnknown = -13,

    /// A pool memory allocation has failed. This *must* only be returned if no attempt to allocate
    /// host or device memory was made to accommodate the new allocation. If the failure was
    /// definitely due to fragmentation of the pool, [`VkResult::VkErrorFragmentedPool`] *should*
    /// be returned instead.
    ///
    /// Provided by [`VK_VERSION_1_1`]
    VkErrorOutOfPoolMemory = -1000069000,

    /// An external handle is not a valid handle of the specified type.
    ///
    /// Provided by [`VK_VERSION_1_1`]
    VkErrorInvalidExternalHandle = -1000072003,

    /// A descriptor pool creation has failed due to fragmentation.
    ///
    /// Provided by [`VK_VERSION_1_2`]
    VkErrorFragmentation = -1000161000,

    /// A buffer creation or memory allocation failed because the requested address is not
    /// available. A shader group handle assignment failed because the requested shader group
    /// handle information is no longer valid.
    ///
    /// Provided by [`VK_VERSION_1_2`]
    VkErrorInvalidOpaqueCaptureAddress = -1000257000,

    /// A requested pipeline creation would have required compilation, but the application
    /// requested compilation to not be performed.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    VkPipelineCompileRequired = 1000297000,

    /// A surface is no longer available.
    ///
    /// Provided by [`khr_surface`]
    VkErrorSurfaceLostKhr = -1000000000,

    /// The requested window is already in use by Vulkan or another API in a manner which prevents
    /// it from being used again.
    ///
    /// Provided by [`khr_surface`]
    VkErrorNativeWindowInUseKhr = -1000000001,

    /// A swapchain no longer matches the surface properties exactly, but *can* still be used to
    /// present to the surface successfully.
    ///
    /// Provided by [`khr_swapchain`]
    VkSuboptimalKhr = 1000001003,

    /// A surface has changed in such a way that it is no longer compatible with the swapchain, and
    /// further presentation requests using the swapchain will fail. Applications must query the
    /// new surface properties and recreate their swapchain if they wish to continue presenting to
    /// the surface.
    ///
    /// Provided by [`khr_swapchain`]
    VkErrorOutOfDateKhr = -1000001004,

    /// The display used by a swapchain does not use the same presentable image layout, or is
    /// incompatible in a way that prevents sharing an image.
    ///
    /// Provided by [`khr_display_swapchain`]
    VkErrorIncompatibleDisplayKhr = -1000003001,

    /// A command failed because invalid usage was detected by the implementation or a
    /// validation-layer.
    ///
    /// Provided by [`ext_debug_report`]
    VkErrorValidationFailedExt = -1000011001,

    /// One or more shaders failed to compile or link. More details are reported back to the
    /// application via [`ext_debug_report`] if enabled.
    ///
    /// Provided by [`nv_glsl_shader`]
    VkErrorInvalidShaderNv = -1000012000,

    /// The requested [`VkImageUsageFlags`] are not supported.
    ///
    /// Provided by [`khr_video_queue`]
    VkErrorImageUsageNotSupportedKhr = -1000023000,

    /// The requested video picture layout is not supported.
    ///
    /// Provided by [`khr_video_queue`]
    VkErrorVideoPictureLayoutNotSupportedKhr = -1000023001,

    /// A video profile operation specified via [`VkVideoProfileInfoKHR::videoCodecOperation`] is
    /// not supported.
    ///
    /// Provided by [`khr_video_queue`]
    VkErrorVideoProfileOperationNotSupportedKhr = -1000023002,

    /// Format parameters in a requested [`VkVideoProfileInfoKHR`] chain are not supported.
    ///
    /// Provided by [`khr_video_queue`]
    VkErrorVideoProfileFormatNotSupportedKhr = -1000023003,

    /// Codec-specific parameters in a requested [`VkVideoProfileInfoKHR`] chain are not supported.
    ///
    /// Provided by [`khr_video_queue`]
    VkErrorVideoProfileCodecNotSupportedKhr = -1000023004,

    /// The specified video Std header version is not supported.
    ///
    /// Provided by [`khr_video_queue`]
    VkErrorVideoStdVersionNotSupportedKhr = -1000023005,

    /// An operation on a swapchain created with
    /// [`VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT`] failed as it did not have exclusive
    /// full-screen access. This may occur due to implementation-dependent reasons, outside of the
    /// application’s control.
    ///
    /// Provided by [`ext_full_screen_exclusive`]
    VkErrorFullScreenExclusiveModeLostExt = -1000255000,

    /// A deferred operation is not complete but there is currently no work for this thread to do
    /// at the time of this call.
    ///
    /// Provided by [`khr_deferred_host_operations`]
    VkThreadIdleKhr = 1000268000,

    /// A deferred operation is not complete but there is no work remaining to assign to additional
    /// threads.
    ///
    /// Provided by [`khr_deferred_host_operations`]
    VkThreadDoneKhr = 1000268001,

    /// A deferred operation was requested and at least some of the work was deferred.
    ///
    /// Provided by [`khr_deferred_host_operations`]
    VkOperationDeferredKhr = 1000268002,

    /// A deferred operation was requested and no operations were deferred.
    ///
    /// Provided by [`khr_deferred_host_operations`]
    VkOperationNotDeferredKhr = 1000268003,

    /// The specified Video Std parameters do not adhere to the syntactic or semantic requirements
    /// of the used video compression standard, or values derived from parameters according to the
    /// rules defined by the used video compression standard do not adhere to the capabilities of
    /// the video compression standard or the implementation.
    ///
    /// Provided by [`khr_video_encode_queue`]
    VkErrorInvalidVideoStdParametersKhr = -1000299000,

    /// An image creation failed because internal resources required for compression are exhausted.
    /// This must only be returned when fixed-rate compression is requested.
    ///
    /// Provided by [`ext_image_compression_control`]
    VkErrorCompressionExhaustedExt = -1000338000,

    /// The provided binary shader code is not compatible with this device.
    ///
    /// Provided by [`ext_shader_object`]
    VkIncompatibleShaderBinaryExt = 1000482000,
}
