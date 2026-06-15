// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Framebuffer blending operations
///
/// # Description
/// Once the source and destination blend factors have been selected, they along with the source
/// and destination components are passed to the blending operations. RGB and alpha components can
/// use different operations.
///
/// In the variant definitions, the following conventions are used:
///  - `Rs0`, `Gs0`, `Bs0` and `As0` represent the first source color R, G, B, and A components,
///    respectively.
///  - `Rd`, `Gd`, `Bd` and `Ad` represent the R, G, B, and A components of the destination color.
///    That is, the color currently in the corresponding color attachment for this fragment/sample.
///  - `Sr`, `Sg`, `Sb` and `Sa` represent the source blend factor R, G, B, and A components,
///    respectively.
///  - `Dr`, `Dg`, `Db` and `Da` represent the destination blend factor R, G, B, and A components,
///    respectively.
///
/// The blending operation produces a new set of values R, G, B and A, which are written to the
/// framebuffer attachment. If blending is not enabled for this attachment, then R, G, B and A are
/// assigned Rs0, Gs0, Bs0 and As0, respectively.
///
/// If the color attachment is fixed-point, the components of the source and destination values and
/// blend factors are each clamped to [0,1] or [-1,1] respectively for an unsigned normalized or
/// signed normalized color attachment prior to evaluating the blend operations. If the color
/// attachment is floating-point, no clamping occurs.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkBlendOp {
    /// `R = Rs0 × Sr + Rd × Dr`
    /// `G = Gs0 × Sg + Gd × Dg`
    /// `B = Bs0 × Sb + Bd × Db`
    /// `A = As0 × Sa + Ad × Da`
    Add = 0,

    /// `R = Rs0 × Sr - Rd × Dr`
    /// `G = Gs0 × Sg - Gd × Dg`
    /// `B = Bs0 × Sb - Bd × Db`
    /// `A = As0 × Sa - Ad × Da`
    Subtract = 1,

    /// `R = Rd × Dr - Rs0 × Sr`
    /// `G = Gd × Dg - Gs0 × Sg`
    /// `B = Bd × Db - Bs0 × Sb`
    /// `A = Ad × Da - As0 × Sa`
    ReverseSubtract = 2,

    /// `R = min(Rs0,Rd)`
    /// `G = min(Gs0,Gd)`
    /// `B = min(Bs0,Bd)`
    /// `A = min(As0,Ad)`
    Min = 3,
    /// `R = max(Rs0,Rd)`
    /// `G = max(Gs0,Gd)`
    /// `B = max(Bs0,Bd)`
    /// `A = max(As0,Ad)`
    Max = 4,

    /// Provided by [`ext_blend_operation_advanced`]
    ZeroExt = 1000148000,

    /// Provided by [`ext_blend_operation_advanced`]
    SrcExt = 1000148001,

    /// Provided by [`ext_blend_operation_advanced`]
    DstExt = 1000148002,

    /// Provided by [`ext_blend_operation_advanced`]
    SrcOverExt = 1000148003,

    /// Provided by [`ext_blend_operation_advanced`]
    DstOverExt = 1000148004,

    /// Provided by [`ext_blend_operation_advanced`]
    SrcInExt = 1000148005,

    /// Provided by [`ext_blend_operation_advanced`]
    DstInExt = 1000148006,

    /// Provided by [`ext_blend_operation_advanced`]
    SrcOutExt = 1000148007,

    /// Provided by [`ext_blend_operation_advanced`]
    DstOutExt = 1000148008,

    /// Provided by [`ext_blend_operation_advanced`]
    SrcAtopExt = 1000148009,

    /// Provided by [`ext_blend_operation_advanced`]
    DstAtopExt = 1000148010,

    /// Provided by [`ext_blend_operation_advanced`]
    XorExt = 1000148011,

    /// Provided by [`ext_blend_operation_advanced`]
    MultiplyExt = 1000148012,

    /// Provided by [`ext_blend_operation_advanced`]
    ScreenExt = 1000148013,

    /// Provided by [`ext_blend_operation_advanced`]
    OverlayExt = 1000148014,

    /// Provided by [`ext_blend_operation_advanced`]
    DarkenExt = 1000148015,

    /// Provided by [`ext_blend_operation_advanced`]
    LightenExt = 1000148016,

    /// Provided by [`ext_blend_operation_advanced`]
    ColorDodgeExt = 1000148017,

    /// Provided by [`ext_blend_operation_advanced`]
    ColorBurnExt = 1000148018,

    /// Provided by [`ext_blend_operation_advanced`]
    HardLightExt = 1000148019,

    /// Provided by [`ext_blend_operation_advanced`]
    SoftLightExt = 1000148020,

    /// Provided by [`ext_blend_operation_advanced`]
    DifferenceExt = 1000148021,

    /// Provided by [`ext_blend_operation_advanced`]
    ExclusionExt = 1000148022,

    /// Provided by [`ext_blend_operation_advanced`]
    InvertExt = 1000148023,

    /// Provided by [`ext_blend_operation_advanced`]
    InvertRgbExt = 1000148024,

    /// Provided by [`ext_blend_operation_advanced`]
    LinearDodgeExt = 1000148025,

    /// Provided by [`ext_blend_operation_advanced`]
    LinearBurnExt = 1000148026,

    /// Provided by [`ext_blend_operation_advanced`]
    VividLightExt = 1000148027,

    /// Provided by [`ext_blend_operation_advanced`]
    LinearLightExt = 1000148028,

    /// Provided by [`ext_blend_operation_advanced`]
    PinLightExt = 1000148029,

    /// Provided by [`ext_blend_operation_advanced`]
    HardMixExt = 1000148030,

    /// Provided by [`ext_blend_operation_advanced`]
    HslHueExt = 1000148031,

    /// Provided by [`ext_blend_operation_advanced`]
    HslSaturationExt = 1000148032,

    /// Provided by [`ext_blend_operation_advanced`]
    HslColorExt = 1000148033,

    /// Provided by [`ext_blend_operation_advanced`]
    HslLuminosityExt = 1000148034,

    /// Provided by [`ext_blend_operation_advanced`]
    PlusExt = 1000148035,

    /// Provided by [`ext_blend_operation_advanced`]
    PlusClampedExt = 1000148036,

    /// Provided by [`ext_blend_operation_advanced`]
    PlusClampedAlphaExt = 1000148037,

    /// Provided by [`ext_blend_operation_advanced`]
    PlusDarkerExt = 1000148038,

    /// Provided by [`ext_blend_operation_advanced`]
    MinusExt = 1000148039,

    /// Provided by [`ext_blend_operation_advanced`]
    MinusClampedExt = 1000148040,

    /// Provided by [`ext_blend_operation_advanced`]
    ContrastExt = 1000148041,

    /// Provided by [`ext_blend_operation_advanced`]
    InvertOvgExt = 1000148042,

    /// Provided by [`ext_blend_operation_advanced`]
    RedExt = 1000148043,

    /// Provided by [`ext_blend_operation_advanced`]
    GreenExt = 1000148044,

    /// Provided by [`ext_blend_operation_advanced`]
    BlueExt = 1000148045,
}
