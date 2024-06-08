// rustdoc imports
#[allow(unused_imports)]
use crate::khr_surface;

/// Supported color space of the presentation engine
///
/// Provided by [`khr_surface`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkColorSpaceKHR {
    /// [`VkColorSpaceKHR::SRGBNonlinearKHR`] specifies support for the sRGB color space.
    SRGBNonlinearKHR = 0,

    /// [`VkColorSpaceKHR::DisplayP3NonlinearEXT`] specifies support for the Display-P3 color space
    /// to be displayed using an sRGB-like EOTF (defined below).
    ///
    /// Provided by [`swapchain_colorspace`]
    DisplayP3NonlinearEXT = 1000104001,

    /// [`VkColorSpaceKHR::ExtendedSRGBLinearEXT`] specifies support for the extended sRGB color
    /// space to be displayed using a linear EOTF.
    ///
    /// Provided by [`swapchain_colorspace`]
    ExtendedSRGBLinearEXT = 1000104002,

    /// [`VkColorSpaceKHR::DisplayP3LinearEXT`] specifies support for the Display-P3 color space to
    /// be displayed using a linear EOTF.
    ///
    /// Provided by [`swapchain_colorspace`]
    DisplayP3LinearEXT = 1000104003,

    /// [`VkColorSpaceKHR::DciP3NonlinearEXT`] specifies support for the DCI-P3 color space to be
    /// displayed using the DCI-P3 EOTF. Note that values in such an image are interpreted as XYZ
    /// encoded color data by the presentation engine.
    ///
    /// Provided by [`swapchain_colorspace`]
    DciP3NonlinearEXT = 1000104004,

    /// [`VkColorSpaceKHR::BT709LinearEXT`] specifies support for the BT709 color space to be
    /// displayed using a linear EOTF.
    ///
    /// Provided by [`swapchain_colorspace`]
    BT709LinearEXT = 1000104005,

    /// [`VkColorSpaceKHR::BT709NonlinearEXT`] specifies support for the BT709 color space to be
    /// displayed using the SMPTE 170M EOTF.
    ///
    /// Provided by [`swapchain_colorspace`]
    BT709NonlinearEXT = 1000104006,

    /// [`VkColorSpaceKHR::BT2020LinearEXT`] specifies support for the BT2020 color space to be
    /// displayed using a linear EOTF.
    ///
    /// Provided by [`swapchain_colorspace`]
    BT2020LinearEXT = 1000104007,

    /// [`VkColorSpaceKHR::HDR10St2084EXT`] specifies support for the HDR10 (BT2020 color) space to
    /// be displayed using the SMPTE ST2084 Perceptual Quantizer (PQ) EOTF.
    ///
    /// Provided by [`swapchain_colorspace`]
    HDR10St2084EXT = 1000104008,

    /// [`VkColorSpaceKHR::DolbyvisionEXT`] specifies support for the Dolby Vision (BT2020 color
    /// space), proprietary encoding, to be displayed using the SMPTE ST2084 EOTF.
    ///
    /// Provided by [`swapchain_colorspace`]
    DolbyvisionEXT = 1000104009,

    /// [`VkColorSpaceKHR::HDR10HlgEXT`] specifies support for the HDR10 (BT2020 color space) to be
    /// displayed using the Hybrid Log Gamma (HLG) EOTF.
    ///
    /// Provided by [`swapchain_colorspace`]
    HDR10HlgEXT = 1000104010,

    /// [`VkColorSpaceKHR::AdobeRGBLinearEXT`] specifies support for the AdobeRGB color space to be
    /// displayed using a linear EOTF.
    ///
    /// Provided by [`swapchain_colorspace`]
    AdobeRGBLinearEXT = 1000104011,

    /// [`VkColorSpaceKHR::AdobeRGBNonlinearEXT`] specifies support for the AdobeRGB color space to
    /// be displayed using the Gamma 2.2 EOTF.
    ///
    ///  Provided by [`swapchain_colorspace`]
    AdobeRGBNonlinearEXT = 1000104012,

    /// [`VkColorSpaceKHR::PassThroughEXT`] specifies that color components are used “as is”. This
    /// is intended to allow applications to supply data for color spaces not described here.
    ///
    /// Provided by [`swapchain_colorspace`]
    PassThroughEXT = 1000104013,

    /// [`VkColorSpaceKHR::ExtendedSRGBNonlinearEXT`] specifies support for the extended sRGB color
    /// space to be displayed using an sRGB EOTF.
    ///
    /// Provided by [`swapchain_colorspace`]
    ExtendedSRGBNonlinearEXT = 1000104014,

    /// [`VkColorSpaceKHR::DisplayNativeAMD`] specifies support for the display’s native color
    /// space. This matches the color space expectations of AMD’s FreeSync2 standard, for displays
    /// supporting it.
    ///
    /// Provided by [`amd_display_native_hdr`]
    DisplayNativeAMD = 1000213000,
}
