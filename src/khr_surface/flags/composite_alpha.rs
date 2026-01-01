use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::khr_surface;

flags! {
    /// Bitmask of [`VkCompositeAlphaFlagKhr`]
    ///
    /// # Description
    /// [`VkCompositeAlphaFlagsKhr`] is a bitmask type for setting a mask of zero or more
    /// [`VkCompositeAlphaFlagKhr`].
    ///
    /// Provided by [`khr_surface`]
    pub struct VkCompositeAlphaFlagsKhr;

    /// Alpha compositing modes supported on a device
    ///
    /// Provided by [`khr_surface`]
    pub enum VkCompositeAlphaFlagKhr {
        /// The alpha component, if it exists, of the images is ignored in the compositing process.
        /// Instead, the image is treated as if it has a constant alpha of 1.0.
        OpaqueBitKhr = 0x00000001,

        /// The alpha component, if it exists, of the images is respected in the compositing process.
        /// The non-alpha components of the image are expected to already be multiplied by the alpha
        /// component by the application.
        PreMultipliedBitKhr = 0x00000002,

        /// The alpha component, if it exists, of the images is respected in the compositing process.
        /// The non-alpha components of the image are not expected to already be multiplied by the
        /// alpha component by the application; instead, the compositor will multiply the non-alpha
        /// components of the image by the alpha component during compositing.
        PostMultipliedBitKhr = 0x00000004,

        /// The way in which the presentation engine treats the alpha component in the images is
        /// unknown to the Vulkan API. Instead, the application is responsible for setting the
        /// composite alpha blending mode using native window system commands. If the application does
        /// not set the blending mode using native window system commands, then a platform-specific
        /// default will be used.
        InheritBitKhr = 0x00000008,
    }
}
