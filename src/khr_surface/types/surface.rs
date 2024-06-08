use crate::vk_define_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_surface;

vk_define_handle!(
    /// Opaque handle to a surface object.
    ///
    /// The [`khr_surface`] extension declares the [`VkSurfaceKHR`] object, and provides a function
    /// for destroying [`VkSurfaceKHR`] objects. Separate platform-specific extensions each provide
    /// a function for creating a [`VkSurfaceKHR`] object for the respective platform. From the
    /// application’s perspective this is an opaque handle, just like the handles of other Vulkan
    /// objects.
    ///
    /// Provided by [`khr_surface`]
    VkSurfaceKHR
);
