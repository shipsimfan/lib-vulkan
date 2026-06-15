// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Framebuffer blending factors
///
/// # Description
/// In the definitions, the following conventions are used:
///  - `Rs0`, `Gs0`, `Bs0` and `As0` represent the first source color R, G, B, and A components,
///    respectively, for the fragment output location corresponding to the color attachment being
///    blended.
///  - `Rs1`, `Gs1`, `Bs1` and `As1` represent the second source color R, G, B, and A components,
///    respectively, used in dual source blending modes, for the fragment output location
///    corresponding to the color attachment being blended.
///  - `Rd`, `Gd`, `Bd` and `Ad` represent the R, G, B, and A components of the destination color.
///    That is, the color currently in the corresponding color attachment for this fragment/sample.
///  - `Rc`, `Gc`, `Bc` and `Ac` represent the blend constant R, G, B, and A components,
///    respectively.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkBlendFactor {
    /// `(0, 0, 0, 0)`
    Zero = 0,

    /// `(1, 1, 1, 1)`
    One = 1,

    /// `(Rs0, Gs0, Bs0, As0)`
    SrcColor = 2,

    /// `(1 - Rs0, 1 - Gs0, 1 - Bs0, 1 - As0)`
    OneMinusSrcColor = 3,

    /// `(Rd, Gd, Bd, Ad)`
    DstColor = 4,

    /// `(1 - Rd, 1 - Gd, 1 - Bd, 1 - Ad)`
    OneMinusDstColor = 5,

    /// `(As0, As0, As0, As0)`
    SrcAlpha = 6,

    /// `(1 - As0, 1 - As0, 1 - As0, 1 - As0)`
    OneMinusSrcAlpha = 7,

    /// `(Ad, Ad, Ad, Ad)`
    DstAlpha = 8,

    /// `(1 - Ad, 1 - Ad, 1 - Ad, 1 - Ad)`
    OneMinusDstAlpha = 9,

    /// `(Rc, Gc, Bc, Ac)`
    ConstantColor = 10,

    /// `(1 - Rc, 1 - Gc, 1 - Bc, 1 - Ac)`
    OneMinusConstantColor = 11,

    /// `(Ac, Ac, Ac, Ac)`
    ConstantAlpha = 12,

    /// `(1 - Ac, 1 - Ac, 1 - Ac, 1 - Ac)`
    OneMinusConstantAlpha = 13,

    /// `(f, f, f, 1)`, where `f = min(As0, 1 - Ad)`
    SrcAlphaSaturate = 14,

    /// `(Rs1, Gs1, Bs1, As1)`
    Src1Color = 15,

    /// `(1 - Rs1, 1 - Gs1, 1 - Bs1, 1 - As1)`
    OneMinusSrc1Color = 16,

    /// `(As1, As1, As1, As1)`
    Src1Alpha = 17,

    /// `(1 - As1, 1 - As1, 1 - As1, 1 - As1)`
    OneMinusSrc1Alpha = 18,
}
