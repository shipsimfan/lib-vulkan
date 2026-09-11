// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_1, VK_VERSION_1_3, VK_VERSION_1_4};

/// Available image formats
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkFormat {
    Undefined = 0,
    R4G4UnormPack8 = 1,
    R4G4B4A4UnormPack16 = 2,
    B4G4R4A4UnormPack16 = 3,
    R5G6B5UnormPack16 = 4,
    B5G6R5UnormPack16 = 5,
    R5G5B5A1UnormPack16 = 6,
    B5G5R5A1UnormPack16 = 7,
    A1R5G5B5UnormPack16 = 8,
    R8Unorm = 9,
    R8Snorm = 10,
    R8Uscaled = 11,
    R8Sscaled = 12,
    R8Uint = 13,
    R8Sint = 14,
    R8Srgb = 15,
    R8G8Unorm = 16,
    R8G8Snorm = 17,
    R8G8Uscaled = 18,
    R8G8Sscaled = 19,
    R8G8Uint = 20,
    R8G8Sint = 21,
    R8G8Srgb = 22,
    R8G8B8Unorm = 23,
    R8G8B8Snorm = 24,
    R8G8B8Uscaled = 25,
    R8G8B8Sscaled = 26,
    R8G8B8Uint = 27,
    R8G8B8Sint = 28,
    R8G8B8Srgb = 29,
    B8G8R8Unorm = 30,
    B8G8R8Snorm = 31,
    B8G8R8Uscaled = 32,
    B8G8R8Sscaled = 33,
    B8G8R8Uint = 34,
    B8G8R8Sint = 35,
    B8G8R8Srgb = 36,
    R8G8B8A8Unorm = 37,
    R8G8B8A8Snorm = 38,
    R8G8B8A8Uscaled = 39,
    R8G8B8A8Sscaled = 40,
    R8G8B8A8Uint = 41,
    R8G8B8A8Sint = 42,
    R8G8B8A8Srgb = 43,
    B8G8R8A8Unorm = 44,
    B8G8R8A8Snorm = 45,
    B8G8R8A8Uscaled = 46,
    B8G8R8A8Sscaled = 47,
    B8G8R8A8Uint = 48,
    B8G8R8A8Sint = 49,
    B8G8R8A8Srgb = 50,
    A8B8G8R8UnormPack32 = 51,
    A8B8G8R8SnormPack32 = 52,
    A8B8G8R8UscaledPack32 = 53,
    A8B8G8R8SscaledPack32 = 54,
    A8B8G8R8UintPack32 = 55,
    A8B8G8R8SintPack32 = 56,
    A8B8G8R8SrgbPack32 = 57,
    A2R10G10B10UnormPack32 = 58,
    A2R10G10B10SnormPack32 = 59,
    A2R10G10B10UscaledPack32 = 60,
    A2R10G10B10SscaledPack32 = 61,
    A2R10G10B10UintPack32 = 62,
    A2R10G10B10SintPack32 = 63,
    A2B10G10R10UnormPack32 = 64,
    A2B10G10R10SnormPack32 = 65,
    A2B10G10R10UscaledPack32 = 66,
    A2B10G10R10SscaledPack32 = 67,
    A2B10G10R10UintPack32 = 68,
    A2B10G10R10SintPack32 = 69,
    R16Unorm = 70,
    R16Snorm = 71,
    R16Uscaled = 72,
    R16Sscaled = 73,
    R16Uint = 74,
    R16Sint = 75,
    R16Sfloat = 76,
    R16G16Unorm = 77,
    R16G16Snorm = 78,
    R16G16Uscaled = 79,
    R16G16Sscaled = 80,
    R16G16Uint = 81,
    R16G16Sint = 82,
    R16G16Sfloat = 83,
    R16G16B16Unorm = 84,
    R16G16B16Snorm = 85,
    R16G16B16Uscaled = 86,
    R16G16B16Sscaled = 87,
    R16G16B16Uint = 88,
    R16G16B16Sint = 89,
    R16G16B16Sfloat = 90,
    R16G16B16A16Unorm = 91,
    R16G16B16A16Snorm = 92,
    R16G16B16A16Uscaled = 93,
    R16G16B16A16Sscaled = 94,
    R16G16B16A16Uint = 95,
    R16G16B16A16Sint = 96,
    R16G16B16A16Sfloat = 97,
    R32Uint = 98,
    R32Sint = 99,
    R32Sfloat = 100,
    R32G32Uint = 101,
    R32G32Sint = 102,
    R32G32Sfloat = 103,
    R32G32B32Uint = 104,
    R32G32B32Sint = 105,
    R32G32B32Sfloat = 106,
    R32G32B32A32Uint = 107,
    R32G32B32A32Sint = 108,
    R32G32B32A32Sfloat = 109,
    R64Uint = 110,
    R64Sint = 111,
    R64Sfloat = 112,
    R64G64Uint = 113,
    R64G64Sint = 114,
    R64G64Sfloat = 115,
    R64G64B64Uint = 116,
    R64G64B64Sint = 117,
    R64G64B64Sfloat = 118,
    R64G64B64A64Uint = 119,
    R64G64B64A64Sint = 120,
    R64G64B64A64Sfloat = 121,
    B10G11R11UfloatPack32 = 122,
    E5B9G9R9UfloatPack32 = 123,
    D16Unorm = 124,
    X8D24UnormPack32 = 125,
    D32Sfloat = 126,
    S8Uint = 127,
    D16UnormS8Uint = 128,
    D24UnormS8Uint = 129,
    D32SfloatS8Uint = 130,
    Bc1RgbUnormBlock = 131,
    Bc1RgbSrgbBlock = 132,
    Bc1RgbaUnormBlock = 133,
    Bc1RgbaSrgbBlock = 134,
    Bc2UnormBlock = 135,
    Bc2SrgbBlock = 136,
    Bc3UnormBlock = 137,
    Bc3SrgbBlock = 138,
    Bc4UnormBlock = 139,
    Bc4SnormBlock = 140,
    Bc5UnormBlock = 141,
    Bc5SnormBlock = 142,
    Bc6HUfloatBlock = 143,
    Bc6HSfloatBlock = 144,
    Bc7UnormBlock = 145,
    Bc7SrgbBlock = 146,
    Etc2R8G8B8UnormBlock = 147,
    Etc2R8G8B8SrgbBlock = 148,
    Etc2R8G8B8A1UnormBlock = 149,
    Etc2R8G8B8A1SrgbBlock = 150,
    Etc2R8G8B8A8UnormBlock = 151,
    Etc2R8G8B8A8SrgbBlock = 152,
    EacR11UnormBlock = 153,
    EacR11SnormBlock = 154,
    EacR11G11UnormBlock = 155,
    EacR11G11SnormBlock = 156,
    Astc4x4UnormBlock = 157,
    Astc4x4SrgbBlock = 158,
    Astc5x4UnormBlock = 159,
    Astc5x4SrgbBlock = 160,
    Astc5x5UnormBlock = 161,
    Astc5x5SrgbBlock = 162,
    Astc6x5UnormBlock = 163,
    Astc6x5SrgbBlock = 164,
    Astc6x6UnormBlock = 165,
    Astc6x6SrgbBlock = 166,
    Astc8x5UnormBlock = 167,
    Astc8x5SrgbBlock = 168,
    Astc8x6UnormBlock = 169,
    Astc8x6SrgbBlock = 170,
    Astc8x8UnormBlock = 171,
    Astc8x8SrgbBlock = 172,
    Astc10x5UnormBlock = 173,
    Astc10x5SrgbBlock = 174,
    Astc10x6UnormBlock = 175,
    Astc10x6SrgbBlock = 176,
    Astc10x8UnormBlock = 177,
    Astc10x8SrgbBlock = 178,
    Astc10x10UnormBlock = 179,
    Astc10x10SrgbBlock = 180,
    Astc12x10UnormBlock = 181,
    Astc12x10SrgbBlock = 182,
    Astc12x12UnormBlock = 183,
    Astc12x12SrgbBlock = 184,

    /// Provided by [`VK_VERSION_1_1`]
    G8B8G8R8_422Unorm = 1000156000,

    /// Provided by [`VK_VERSION_1_1`]
    B8G8R8G8_422Unorm = 1000156001,

    /// Provided by [`VK_VERSION_1_1`]
    G8B8R8_3Plane420Unorm = 1000156002,

    /// Provided by [`VK_VERSION_1_1`]
    G8B8R8_2Plane420Unorm = 1000156003,

    /// Provided by [`VK_VERSION_1_1`]
    G8B8R8_3Plane422Unorm = 1000156004,

    /// Provided by [`VK_VERSION_1_1`]
    G8B8R8_2Plane422Unorm = 1000156005,

    /// Provided by [`VK_VERSION_1_1`]
    G8B8R8_3Plane444Unorm = 1000156006,

    /// Provided by [`VK_VERSION_1_1`]
    R10X6UnormPack16 = 1000156007,

    /// Provided by [`VK_VERSION_1_1`]
    R10X6G10X6Unorm2Pack16 = 1000156008,

    /// Provided by [`VK_VERSION_1_1`]
    R10X6G10X6B10X6A10X6Unorm4Pack16 = 1000156009,

    /// Provided by [`VK_VERSION_1_1`]
    G10X6B10X6G10X6R10X6_422Unorm4Pack16 = 1000156010,

    /// Provided by [`VK_VERSION_1_1`]
    B10X6G10X6R10X6G10X6_422Unorm4Pack16 = 1000156011,

    /// Provided by [`VK_VERSION_1_1`]
    G10X6B10X6R10X6_3Plane420Unorm3Pack16 = 1000156012,

    /// Provided by [`VK_VERSION_1_1`]
    G10X6B10X6R10X6_2Plane420Unorm3Pack16 = 1000156013,

    /// Provided by [`VK_VERSION_1_1`]
    G10X6B10X6R10X6_3Plane422Unorm3Pack16 = 1000156014,

    /// Provided by [`VK_VERSION_1_1`]
    G10X6B10X6R10X6_2Plane422Unorm3Pack16 = 1000156015,

    /// Provided by [`VK_VERSION_1_1`]
    G10X6B10X6R10X6_3Plane444Unorm3Pack16 = 1000156016,

    /// Provided by [`VK_VERSION_1_1`]
    R12X4UnormPack16 = 1000156017,

    /// Provided by [`VK_VERSION_1_1`]
    R12X4G12X4Unorm2Pack16 = 1000156018,

    /// Provided by [`VK_VERSION_1_1`]
    R12X4G12X4B12X4A12X4Unorm4Pack16 = 1000156019,

    /// Provided by [`VK_VERSION_1_1`]
    G12X4B12X4G12X4R12X4_422Unorm4Pack16 = 1000156020,

    /// Provided by [`VK_VERSION_1_1`]
    B12X4G12X4R12X4G12X4_422Unorm4Pack16 = 1000156021,

    /// Provided by [`VK_VERSION_1_1`]
    G12X4B12X4R12X4_3Plane420Unorm3Pack16 = 1000156022,

    /// Provided by [`VK_VERSION_1_1`]
    G12X4B12X4R12X4_2Plane420Unorm3Pack16 = 1000156023,

    /// Provided by [`VK_VERSION_1_1`]
    G12X4B12X4R12X4_3Plane422Unorm3Pack16 = 1000156024,

    /// Provided by [`VK_VERSION_1_1`]
    G12X4B12X4R12X4_2Plane422Unorm3Pack16 = 1000156025,

    /// Provided by [`VK_VERSION_1_1`]
    G12X4B12X4R12X4_3Plane444Unorm3Pack16 = 1000156026,

    /// Provided by [`VK_VERSION_1_1`]
    G16B16G16R16_422Unorm = 1000156027,

    /// Provided by [`VK_VERSION_1_1`]
    B16G16R16G16_422Unorm = 1000156028,

    /// Provided by [`VK_VERSION_1_1`]
    G16B16R16_3Plane420Unorm = 1000156029,

    /// Provided by [`VK_VERSION_1_1`]
    G16B16R16_2Plane420Unorm = 1000156030,

    /// Provided by [`VK_VERSION_1_1`]
    G16B16R16_3Plane422Unorm = 1000156031,

    /// Provided by [`VK_VERSION_1_1`]
    G16B16R16_2Plane422Unorm = 1000156032,

    /// Provided by [`VK_VERSION_1_1`]
    G16B16R16_3Plane444Unorm = 1000156033,

    /// Provided by [`VK_VERSION_1_3`]
    G8B8R8_2Plane444Unorm = 1000330000,

    /// Provided by [`VK_VERSION_1_3`]
    G10X6B10X6R10X6_2Plane444Unorm3Pack16 = 1000330001,

    /// Provided by [`VK_VERSION_1_3`]
    G12X4B12X4R12X4_2Plane444Unorm3Pack16 = 1000330002,

    /// Provided by [`VK_VERSION_1_3`]
    G16B16R16_2Plane444Unorm = 1000330003,

    /// Provided by [`VK_VERSION_1_3`]
    A4R4G4B4UnormPack16 = 1000340000,

    /// Provided by [`VK_VERSION_1_3`]
    A4B4G4R4UnormPack16 = 1000340001,

    /// Provided by [`VK_VERSION_1_3`]
    Astc4x4SfloatBlock = 1000066000,

    /// Provided by [`VK_VERSION_1_3`]
    Astc5x4SfloatBlock = 1000066001,

    /// Provided by [`VK_VERSION_1_3`]
    Astc5x5SfloatBlock = 1000066002,

    /// Provided by [`VK_VERSION_1_3`]
    Astc6x5SfloatBlock = 1000066003,

    /// Provided by [`VK_VERSION_1_3`]
    Astc6x6SfloatBlock = 1000066004,

    /// Provided by [`VK_VERSION_1_3`]
    Astc8x5SfloatBlock = 1000066005,

    /// Provided by [`VK_VERSION_1_3`]
    Astc8x6SfloatBlock = 1000066006,

    /// Provided by [`VK_VERSION_1_3`]
    Astc8x8SfloatBlock = 1000066007,

    /// Provided by [`VK_VERSION_1_3`]
    Astc10x5SfloatBlock = 1000066008,

    /// Provided by [`VK_VERSION_1_3`]
    Astc10x6SfloatBlock = 1000066009,

    /// Provided by [`VK_VERSION_1_3`]
    Astc10x8SfloatBlock = 1000066010,

    /// Provided by [`VK_VERSION_1_3`]
    Astc10x10SfloatBlock = 1000066011,

    /// Provided by [`VK_VERSION_1_3`]
    Astc12x10SfloatBlock = 1000066012,

    /// Provided by [`VK_VERSION_1_3`]
    Astc12x12SfloatBlock = 1000066013,

    /// Provided by [`VK_VERSION_1_4`]
    A1B5G5R5UnormPack16 = 1000470000,

    /// Provided by [`img_format_pvrtc`]
    Pvrtc1_2BppUnormBlockImg = 1000054000,

    /// Provided by [`img_format_pvrtc`]
    Pvrtc1_4BppUnormBlockImg = 1000054001,

    /// Provided by [`img_format_pvrtc`]
    PVRTC2_2BppUnormBlockImg = 1000054002,

    /// Provided by [`img_format_pvrtc`]
    PVRTC2_4BppUnormBlockImg = 1000054003,

    /// Provided by [`img_format_pvrtc`]
    Pvrtc1_2BppSrgbBlockImg = 1000054004,

    /// Provided by [`img_format_pvrtc`]
    Pvrtc1_4BppSrgbBlockImg = 1000054005,

    /// Provided by [`img_format_pvrtc`]
    PVRTC2_2BppSrgbBlockImg = 1000054006,

    /// Provided by [`img_format_pvrtc`]
    PVRTC2_4BppSrgbBlockImg = 1000054007,

    /// Provided by [`nv_optical_flow`]
    R16G16SFixed5Nv = 1000464000,

    /// Provided by [`khr_maintenance5`]
    A8UnormKhr = 1000470001,
}
