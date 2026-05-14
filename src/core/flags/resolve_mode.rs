use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_2, VK_VERSION_1_3};

flags! {
    /// Bitmask of [`VkResolveModeFlag`]
    ///
    /// # Description
    /// [`VkResolveModeFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkResolveModeFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_2`]
    pub struct VkResolveModeFlags;

    /// Bitmask indicating supported depth and stencil resolve modes
    ///
    /// # Description
    /// If no resolve mode is otherwise specified, [`VkResolveModeFlag::Average`] is used.
    ///
    /// If [`VkResolveModeFlag::Average`] is used, and the source format is a floating-point or
    /// normalized type, the sample values for each pixel are resolved with implementation-defined
    /// numerical precision.
    ///
    /// If the numeric format of the resolve attachment uses sRGB encoding, the implementation
    /// should convert samples from nonlinear to linear before averaging samples as described in
    /// the “sRGB EOTF” section of the Khronos Data Format Specification. In this case, the
    /// implementation must convert the linear averaged value to nonlinear before writing the
    /// resolved result to resolve attachment. If the `maintenance10` feature is enabled, whether a
    /// nonlinear to linear conversion happens for sRGB resolve is defined by
    /// `resolve_srgb_format_applies_transfer_function`. This behavior can be overridden with
    /// appropriate `VK_*RESOLVE{SKIP,ENABLE}_TRANSFER_FUNCTION_BIT_KHR` flag usage.
    ///
    /// Provided by [`VK_VERSION_1_2`]
    pub enum VkResolveModeFlag {
        /// [`VkResolveModeFlag::None`] specifies that no resolve operation is done.
        None = 0,

        /// [`VkResolveModeFlag::SampleZero`] specifies that result of the resolve operation is
        /// equal to the value of sample 0.
        SampleZero = 0x00000001,

        /// [`VkResolveModeFlag::Average`] specifies that result of the resolve operation is the
        /// average of the sample values.
        Average = 0x00000002,

        /// [`VkResolveModeFlag::Min`] specifies that result of the resolve operation is the
        /// minimum of the sample values.
        Min = 0x00000004,

        /// [`VkResolveModeFlag::Max`] specifies that result of the resolve operation is the
        /// maximum of the sample values.
        Max = 0x00000008,

        /// [`VkResolveModeFlag::ExternalFormatDownsampleAndroid`] specifies that rather than a
        /// multisample resolve, a single sampled color attachment will be downsampled into a
        /// Y′CBCR format image specified by an external Android format. Unlike other resolve
        /// modes, implementations can resolve multiple times during rendering, or even bypass
        /// writing to the color attachment altogether, as long as the final value is resolved to
        /// the resolve attachment. Values in the G, B, and R channels of the color attachment will
        /// be written to the Y, CB, and CR channels of the external format image, respectively.
        /// Chroma values are calculated as if sampling with a linear filter from the color
        /// attachment at full rate, at the location the chroma values sit according to
        /// [`VkPhysicalDeviceExternalFormatResolvePropertiesAndroid::external_format_resolve_chroma_offset_x`],
        /// [`VkPhysicalDeviceExternalFormatResolvePropertiesAndroid::external_format_resolve_chroma_offset_y`],
        /// and the chroma sample rate of the resolved image.
        ///
        /// Provided by [`android_external_format_resolve`] with [`khr_dynamic_rendering`] or
        /// [`VK_VERSION_1_3`]
        ExternalFormatDownsampleAndroid = 0x00000010,

        /// [`VkResolveModeFlag::CustomExt`] specifies that the attachment will be resolved by
        /// shaders in the render pass instead of fixed-function operations.
        ///
        /// Provided by [`ext_custom_resolve`] with [`khr_dynamic_rendering`] or [`VK_VERSION_1_3`]
        CustomExt = 0x00000020,
    }
}
