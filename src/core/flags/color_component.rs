use crate::macros::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

flags! {
    /// Bitmask of [`VkColorComponentFlag`]s
    ///
    /// # Description
    /// [`VkColorComponentFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkColorComponentFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkColorComponentFlags;

    /// Bitmask controlling which components are written to the framebuffer
    ///
    /// # Description
    /// The color write mask operation is applied regardless of whether blending is enabled.
    ///
    /// The color write mask operation is applied only if Color Write Enable is enabled for the
    /// respective attachment. Otherwise the color write mask is ignored and writes to all
    /// components of the attachment are disabled.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkColorComponentFlag {
        /// [`VkColorComponentFlag::R`] specifies that the R value is written to the color
        /// attachment for the appropriate sample. Otherwise, the value in memory is unmodified.
        R = 0x00000001,

        /// [`VkColorComponentFlag::G`] specifies that the G value is written to the color
        /// attachment for the appropriate sample. Otherwise, the value in memory is unmodified.
        G = 0x00000002,

        /// [`VkColorComponentFlag::B`] specifies that the B value is written to the color
        /// attachment for the appropriate sample. Otherwise, the value in memory is unmodified.
        B = 0x00000004,

        /// [`VkColorComponentFlag::A`] specifies that the A value is written to the color
        /// attachment for the appropriate sample. Otherwise, the value in memory is unmodified.
        A = 0x00000008,
    }
}
