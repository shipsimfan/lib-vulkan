// rustdoc imports
#[allow(unused_imports)]
use crate::khr_surface;

/// Presentation mode supported for a surface
///
/// Provided by [`khr_surface`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkPresentModeKHR {
    /// [`VkPresentModeKHR::ImmediateModeKHR`] specifies that the presentation engine does not wait
    /// for a vertical blanking period to update the current image, meaning this mode may result in
    /// visible tearing. No internal queuing of presentation requests is needed, as the requests
    /// are applied immediately.
    ImmediateModeKHR = 0,

    /// [`VkPresentModeKHR::MailboxKHR`] specifies that the presentation engine waits for the next
    /// vertical blanking period to update the current image. Tearing cannot be observed. An
    /// internal single-entry queue is used to hold pending presentation requests. If the queue is
    /// full when a new presentation request is received, the new request replaces the existing
    /// entry, and any images associated with the prior entry become available for reuse by the
    /// application. One request is removed from the queue and processed during each vertical
    /// blanking period in which the queue is non-empty.
    MailboxKHR = 1,

    /// [`VkPresentModeKHR::FIFOKHR`] specifies that the presentation engine waits for the next
    /// vertical blanking period to update the current image. Tearing cannot be observed. An
    /// internal queue is used to hold pending presentation requests. New requests are appended to
    /// the end of the queue, and one request is removed from the beginning of the queue and
    /// processed during each vertical blanking period in which the queue is non-empty. This is the
    /// only value of presentMode that is required to be supported.
    FIFOKHR = 2,

    /// [`VkPresentModeKHR::FIFORelaxedKHR`] specifies that the presentation engine generally waits
    /// for the next vertical blanking period to update the current image. If a vertical blanking
    /// period has already passed since the last update of the current image then the presentation
    /// engine does not wait for another vertical blanking period for the update, meaning this mode
    /// may result in visible tearing in this case. This mode is useful for reducing visual stutter
    /// with an application that will mostly present a new image before the next vertical blanking
    /// period, but may occasionally be late, and present a new image just after the next vertical
    /// blanking period. An internal queue is used to hold pending presentation requests. New
    /// requests are appended to the end of the queue, and one request is removed from the
    /// beginning of the queue and processed during or after each vertical blanking period in which
    /// the queue is non-empty.
    FIFORelaxedKHR = 3,

    /// [`VkPresentModeKHR::DemandRefreshKHR`] specifies that the presentation engine and
    /// application have concurrent access to a single image, which is referred to as a shared
    /// presentable image. The presentation engine is only required to update the current image
    /// after a new presentation request is received. Therefore the application must make a
    /// presentation request whenever an update is required. However, the presentation engine may
    /// update the current image at any point, meaning this mode may result in visible tearing.
    ///
    /// Provided by [`khr_shared_presentable_image`]
    DemandRefreshKHR = 1000111000,

    /// [`VkPresentModeKHR::ContinousRefreshKHR`] specifies that the presentation engine and
    /// application have concurrent access to a single image, which is referred to as a shared
    /// presentable image. The presentation engine periodically updates the current image on its
    /// regular refresh cycle. The application is only required to make one initial presentation
    /// request, after which the presentation engine must update the current image without any need
    /// for further presentation requests. The application can indicate the image contents have
    /// been updated by making a presentation request, but this does not guarantee the timing of
    /// when it will be updated. This mode may result in visible tearing if rendering to the image
    /// is not timed correctly.
    ///
    /// Provided by [`khr_shared_presentable_image`]
    ContinousRefreshKHR = 1000111001,
}
