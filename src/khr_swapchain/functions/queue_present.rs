use crate::{VkQueue, VkResult, khr_swapchain::VkPresentInfoKhr};
use std::ffi::CStr;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    khr_surface::VkGetPhysicalDeviceSurfaceSupportKhr,
    khr_swapchain::{self, VkAcquireNextImageKhr},
};

/// Queue an image for presentation
///
/// # Parameters
///  - `queue` is a queue that is capable of presentation to the target surface’s platform on the
///    same device as the image’s swapchain.
///  - `present_info` is a pointer to a [`VkPresentInfoKhr`] structure specifying parameters of the
///    presentation.
///
/// # Description
/// The result codes [`VkResult::VkErrorOutOfDateKhr`] and [`VkResult::VkSuboptimalKhr`] have the
/// same meaning when returned by [`VkQueuePresentKhr`] as they do when returned by
/// [`VkAcquireNextImageKhr`]. If any swapchain member of `present_info` was created with
/// [`VkFullscreenExclusiveExt::ApplicationControlledExt`],
/// [`VkResult::VkErrorFullScreenExclusiveModeLostExt`] will be returned if that swapchain does not
/// have exclusive full-screen access, possibly for implementation-specific reasons outside of the
/// application’s control. If multiple swapchains are presented, the result code is determined by
/// applying the following rules in order:
///  - If the device is lost, [`VkResult::VkErrorDeviceLost`] is returned.
///  - If any of the target surfaces are no longer available the error
///    [`VkResult::VkErrorSurfaceLostKhr`] is returned.
///  - If any of the presents would have a result of [`VkResult::VkErrorOutOfDateKhr`] if issued
///    separately then [`VkResult::VkErrorOutOfDateKhr`] is returned.
///  - If any of the presents would have a result of
///    [`VkResult::VkErrorFullScreenExclusiveModeLostExt`] if issued separately then
///    [`VkResult::VkErrorFullScreenExclusiveModeLostExt`] is returned.
///  - If any of the presents would have a result of [`VkResult::VkSuboptimalKhr`] if issued
///    separately then [`VkResult::VkSuboptimalKhr`] is returned.
///  - Otherwise [`VkResult::VkSuccess`] is returned.
///
/// Any writes to memory backing the images referenced by the `image_indices` and `swapchains`
/// members of `present_info`, that are available before [`VkQueuePresentKhr`] is executed, are
/// automatically made visible to the read access performed by the presentation engine. This
/// automatic visibility operation for an image happens-after the semaphore signal operation, and
/// happens-before the presentation engine accesses the image.
///
/// Presentation is a read-only operation that will not affect the content of the presentable
/// images. Upon reacquiring the image and transitioning it away from the
/// [`VkImageLayoutKhr::PresentSrcKhr`] layout, the contents will be the same as they were prior to
/// transitioning the image to the present source layout and presenting it. However, if a mechanism
/// other than Vulkan is used to modify the platform window associated with the swapchain, the
/// content of all presentable images in the swapchain becomes undefined.
///
/// Calls to [`VkQueuePresentKhr`] may block, but must return in finite time. The processing of the
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
/// If [`VkQueuePresentKhr`] fails to enqueue the corresponding set of queue operations, it may
/// return [`VkResult::VkErrorOutOfHostMemory`] or [`VkResult::VkErrorOutOfDeviceMemory`]. If it
/// does, the implementation must ensure that the state and contents of any resources or
/// synchronization primitives referenced is unaffected by the call or its failure.
///
/// If [`VkQueuePresentKhr`] fails in such a way that the implementation is unable to make that
/// guarantee, the implementation must return [`VkResult::VkErrorDeviceLost`].
///
/// However, if the presentation request is rejected by the presentation engine with an error
/// [`VkResult::VkErrorOutOfDateKhr`], [`VkResult::VkErrorFullScreenExclusiveModeLostExt`], or
/// [`VkResult::VkErrorSurfaceLostKhr`], the set of queue operations are still considered to be
/// enqueued and thus any semaphore wait operation specified in [`VkPresentInfoKhr`] will execute
/// when the corresponding queue operation is complete.
///
/// [`VkQueuePresentKhr`] releases the acquisition of the images referenced by `image_indices`. The
/// queue family corresponding to the queue [`VkQueuePresentKhr`] is executed on must have
/// ownership of the presented images as defined in Resource Sharing. [`VkQueuePresentKhr`] does
/// not alter the queue family ownership, but the presented images must not be used again before
/// they have been reacquired using [`VkAcquireNextImageKhr`].
///
/// # Valid Usage
///  - Each element of `swapchains` member of `present_info` must be a swapchain that is created
///    for a surface for which presentation is supported from queue as determined using a call to
///    [`VkGetPhysicalDeviceSurfaceSupportKhr`]
///  - If more than one member of `swapchains` was created from a display surface, all display
///    surfaces referenced that refer to the same display must use the same display mode
///  - If more than one member of `swapchains` was created from a display surface, all display
///    surfaces referenced that refer to the same display must use the same `stereo_type`
///  - When a semaphore wait operation referring to a binary semaphore defined by the elements of
///    the `wait_semaphores` member of `present_info` executes on queue, there must be no other
///    queues waiting on the same semaphore
///  - All elements of the `wait_semaphores` member of `present_info` must be created with a
///    [`VkSemaphoreType`] of [`VkSemaphoreType::Binary`]
///  - All elements of the `wait_semaphores` member of `present_info` must reference a semaphore
///    signal operation that has been submitted for execution and any semaphore signal operations
///    on which it depends must have also been submitted for execution
///
/// # Valid Usage (Implicit)
///  - `queue` must be a valid [`VkQueue`] handle
///  - `present_info` must be a valid pointer to a valid [`VkPresentInfoKhr`] structure
///
/// # Host Synchronization
///  - Host access to `queue` must be externally synchronized
///
/// Provided by [`khr_swapchain`]
pub type VkQueuePresentKhr =
    extern "system" fn(queue: VkQueue, present_info: *const VkPresentInfoKhr) -> VkResult;

/// The name of [`VkQueuePresentKhr`]
pub const VK_QUEUE_PRESENT_KHR: &CStr = c"vkQueuePresentKHR";
