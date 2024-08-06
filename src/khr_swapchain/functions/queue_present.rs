use crate::{VkPresentInfoKHR, VkQueue, VkResult};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{khr_swapchain, VkAcquireNextImageKHR};

/// Queue an image for presentation
///
/// # Parameters
///  - `queue` is a queue that is capable of presentation to the target surface’s platform on the
///    same device as the image’s swapchain.
///  - `present_info` is a pointer to a [`VkPresentInfoKHR`] structure specifying parameters of the
///    presentation.
///
/// # Description
/// The result codes [`VkResult::VkErrorOutOfDateKHR`] and [`VkResult::VkSuboptimalKHR`] have the
/// same meaning when returned by [`VkQueuePresentKHR`] as they do when returned by
/// [`VkAcquireNextImageKHR`]. If any swapchain member of `present_info` was created with
/// [`VkFullscreenExclusiveEXT::ApplicationControlledEXT`],
/// [`VkResult::VkErrorFullScreenExclusiveModeLostEXT`] will be returned if that swapchain does not
/// have exclusive full-screen access, possibly for implementation-specific reasons outside of the
/// application’s control. If multiple swapchains are presented, the result code is determined by
/// applying the following rules in order:
///  - If the device is lost, [`VkResult::VkErrorDeviceLost`] is returned.
///  - If any of the target surfaces are no longer available the error
///    [`VkResult::VkErrorSurfaceLostKHR`] is returned.
///  - If any of the presents would have a result of [`VkResult::VkErrorOutOfDateKHR`] if issued
///    separately then [`VkResult::VkErrorOutOfDateKHR`] is returned.
///  - If any of the presents would have a result of
///    [`VkResult::VkErrorFullScreenExclusiveModeLostEXT`] if issued separately then
///    [`VkResult::VkErrorFullScreenExclusiveModeLostEXT`] is returned.
///  - If any of the presents would have a result of [`VkResult::VkSuboptimalKHR`] if issued
///    separately then [`VkResult::VkSuboptimalKHR`] is returned.
///  - Otherwise [`VkResult::VkSuccess`] is returned.
///
/// Any writes to memory backing the images referenced by the `image_indices` and `swapchains`
/// members of `present_info`, that are available before [`VkQueuePresentKHR`] is executed, are
/// automatically made visible to the read access performed by the presentation engine. This
/// automatic visibility operation for an image happens-after the semaphore signal operation, and
/// happens-before the presentation engine accesses the image.
///
/// Presentation is a read-only operation that will not affect the content of the presentable
/// images. Upon reacquiring the image and transitioning it away from the
/// [`VkImageLayoutKHR::PresentSrcKhr`] layout, the contents will be the same as they were prior to
/// transitioning the image to the present source layout and presenting it. However, if a mechanism
/// other than Vulkan is used to modify the platform window associated with the swapchain, the
/// content of all presentable images in the swapchain becomes undefined.
///
/// Calls to [`VkQueuePresentKHR`] may block, but must return in finite time. The processing of the
/// presentation happens in issue order with other queue operations, but semaphores must be used to
/// ensure that prior rendering and other commands in the specified queue complete before the
/// presentation begins. The presentation command itself does not delay processing of subsequent
/// commands on the queue. However, presentation requests sent to a particular queue are always
/// performed in order. Exact presentation timing is controlled by the semantics of the
/// presentation engine and native platform in use.
///
/// If an image is presented to a swapchain created from a display surface, the mode of the
/// associated display will be updated, if necessary, to match the mode specified when creating the
/// display surface. The mode switch and presentation of the specified image will be performed as
/// one atomic operation.
///
/// Queueing an image for presentation defines a set of queue operations, including waiting on the
/// semaphores and submitting a presentation request to the presentation engine. However, the scope
/// of this set of queue operations does not include the actual processing of the image by the
/// presentation engine.
///
/// If [`VkQueuePresentKHR`] fails to enqueue the corresponding set of queue operations, it may
/// return [`VkResult::VkErrorOutOfHostMemory`] or [`VkResult::VkErrorOutOfDeviceMemory`]. If it
/// does, the implementation must ensure that the state and contents of any resources or
/// synchronization primitives referenced is unaffected by the call or its failure.
///
/// If [`VkQueuePresentKHR`] fails in such a way that the implementation is unable to make that
/// guarantee, the implementation must return [`VkResult::VkErrorDeviceLost`].
///
/// However, if the presentation request is rejected by the presentation engine with an error
/// [`VkResult::VkErrorOUT_OF_DATE_KHR`], [`VkResult::VkErrorFullScreenExclusiveModeLostEXT`], or
/// [`VkResult::VkErrorSurfaceLostKHR`], the set of queue operations are still considered to be
/// enqueued and thus any semaphore wait operation specified in [`VkPresentInfoKHR`] will execute
/// when the corresponding queue operation is complete.
///
/// [`VkQueuePresentKHR`] releases the acquisition of the images referenced by `image_indices`. The
/// queue family corresponding to the queue [`VkQueuePresentKHR`] is executed on must have
/// ownership of the presented images as defined in Resource Sharing. [`VkQueuePresentKHR`] does
/// not alter the queue family ownership, but the presented images must not be used again before
/// they have been reacquired using [`VkAcquireNextImageKHR`].
///
/// Provided by [`khr_swapchain`]
pub type VkQueuePresentKHR =
    extern "system" fn(queue: VkQueue, present_info: *const VkPresentInfoKHR) -> VkResult;

/// The name of [`VkQueuePresentKHR`]
pub const VK_QUEUE_PRESENT_KHR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"vkQueuePresentKHR\0") };
