use std::ffi::c_float;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure specifying a clear color value
///
/// # Description
/// The four array elements of the clear color map to R, G, B, and A components of image formats,
/// in order.
///
/// If the image has more than one sample, the same value is written to all samples for any pixels
/// being cleared.
///
/// If the image or attachment format has a 64-bit component width, the first 2 array elements of
/// each of the arrays above are reinterpreted as a single 64-bit element for the R component. The
/// next 2 array elements are used in the same way for the G component. In other words, the union
/// behaves as if it had the following additional members:
/// ```ignore
/// pub float64: [std::ffi::c_double; 2],
/// pub int64: [i64; 2],
/// pub uint64: [u64; 2],
/// ```
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Clone, Copy)]
pub union VkClearColorValue {
    /// `float32` are the color clear values when the format of the image or attachment is one of
    /// the numeric formats with a numeric type that is floating-point. Floating-point values are
    /// automatically converted to the format of the image, with the clear value being treated as
    /// linear if the image is sRGB.
    pub float32: [c_float; 4],

    /// `int32` are the color clear values when the format of the image or attachment has a numeric
    /// type that is signed integer (`SINT`). Signed integer values are converted to the format of
    /// the image by casting to the smaller type (with negative 32-bit values mapping to negative
    /// values in the smaller type). If the integer clear value is not representable in the target
    /// type (e.g. would overflow in conversion to that type), the clear value is undefined.
    pub int32: [i32; 4],

    /// `uint32` are the color clear values when the format of the image or attachment has a
    /// numeric type that is unsigned integer (`UINT`). Unsigned integer values are converted to
    /// the format of the image by casting to the integer type with fewer bits.
    pub uint32: [u32; 4],
}

impl const Default for VkClearColorValue {
    fn default() -> Self {
        VkClearColorValue { uint32: [0; 4] }
    }
}
