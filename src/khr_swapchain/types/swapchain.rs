use crate::vk_define_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_swapchain;

vk_define_handle!(
    /// Opaque handle to a swapchain object
    ///
    /// A swapchain is an abstraction for an array of presentable images that are associated with a
    /// surface. The presentable images are represented by [`VkImage`] objects created by the
    /// platform. One image (which can be an array image for multiview/stereoscopic-3D surfaces) is
    /// displayed at a time, but multiple images can be queued for presentation. An application
    /// renders to the image, and then queues the image for presentation to the surface.
    ///
    /// A native window cannot be associated with more than one non-retired swapchain at a time.
    /// Further, swapchains cannot be created for native windows that have a non-Vulkan graphics
    /// API surface associated with them.
    ///
    /// The presentable images of a swapchain are owned by the presentation engine. An application
    /// can acquire use of a presentable image from the presentation engine. Use of a presentable
    /// image must occur only after the image is returned by [`VkAcquireNextImageKHR`], and before
    /// it is released by [`VkQueuePresentKHR`]. This includes transitioning the image layout and
    /// rendering commands.
    ///
    /// An application can acquire use of a presentable image with [`VkAcquireNextImageKHR`]. After
    /// acquiring a presentable image and before modifying it, the application must use a
    /// synchronization primitive to ensure that the presentation engine has finished reading from
    /// the image. The application can then transition the image’s layout, queue rendering commands
    /// to it, etc. Finally, the application presents the image with [`VkQueuePresentKHR`], which
    /// releases the acquisition of the image. The application can also release the acquisition of
    /// the image through [`VkReleaseSwapchainImagesEXT`], if the image is not in use by the
    /// device, and skip the present operation.
    ///
    /// The presentation engine controls the order in which presentable images are acquired for use
    /// by the application.
    ///
    /// Provided by [`khr_swapchain`]
    VkSwapchainKHR
);
