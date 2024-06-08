// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Available image formats
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[non_exhaustive]
#[allow(non_camel_case_types, missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VkFormat {
    Undefined = 0,
    R4G4UNormPack8 = 1,
    R4G4B4A4UNormPack16 = 2,
    B4G4R4A4UNormPack16 = 3,
    R5G6B5UNormPack16 = 4,
    B5G6R5UNormPack16 = 5,
    R5G5B5A1UNormPack16 = 6,
    B5G5R5A1UNormPack16 = 7,
    A1R5G5B5UNormPack16 = 8,
    R8UNorm = 9,
    R8SNorm = 10,
    R8UScaled = 11,
    R8SScaled = 12,
    R8UInt = 13,
    R8SInt = 14,
    R8SRGB = 15,
    R8G8UNorm = 16,
    R8G8SNorm = 17,
    R8G8UScaled = 18,
    R8G8SScaled = 19,
    R8G8UInt = 20,
    R8G8SInt = 21,
    R8G8SRGB = 22,
    R8G8B8UNorm = 23,
    R8G8B8SNorm = 24,
    R8G8B8UScaled = 25,
    R8G8B8SScaled = 26,
    R8G8B8UInt = 27,
    R8G8B8SInt = 28,
    R8G8B8SRGB = 29,
    B8G8R8UNorm = 30,
    B8G8R8SNorm = 31,
    B8G8R8UScaled = 32,
    B8G8R8SScaled = 33,
    B8G8R8UInt = 34,
    B8G8R8SInt = 35,
    B8G8R8SRGB = 36,
    R8G8B8A8UNorm = 37,
    R8G8B8A8SNorm = 38,
    R8G8B8A8UScaled = 39,
    R8G8B8A8SScaled = 40,
    R8G8B8A8UInt = 41,
    R8G8B8A8SInt = 42,
    R8G8B8A8SRGB = 43,
    B8G8R8A8UNorm = 44,
    B8G8R8A8SNorm = 45,
    B8G8R8A8UScaled = 46,
    B8G8R8A8SScaled = 47,
    B8G8R8A8UInt = 48,
    B8G8R8A8SInt = 49,
    B8G8R8A8SRGB = 50,
    A8B8G8R8UNormPack32 = 51,
    A8B8G8R8SNormPack32 = 52,
    A8B8G8R8UScaledPack32 = 53,
    A8B8G8R8SScaledPack32 = 54,
    A8B8G8R8UIntPack32 = 55,
    A8B8G8R8SIntPack32 = 56,
    A8B8G8R8SRGB_Pack32 = 57,
    A2R10G10B10UNormPack32 = 58,
    A2R10G10B10SNormPack32 = 59,
    A2R10G10B10UScaledPack32 = 60,
    A2R10G10B10SScaledPack32 = 61,
    A2R10G10B10UIntPack32 = 62,
    A2R10G10B10SIntPack32 = 63,
    A2B10G10R10UNormPack32 = 64,
    A2B10G10R10SNormPack32 = 65,
    A2B10G10R10UScaledPack32 = 66,
    A2B10G10R10SScaledPack32 = 67,
    A2B10G10R10UIntPack32 = 68,
    A2B10G10R10SIntPack32 = 69,
    R16UNorm = 70,
    R16SNorm = 71,
    R16UScaled = 72,
    R16SScaled = 73,
    R16UInt = 74,
    R16SInt = 75,
    R16SFloat = 76,
    R16G16UNorm = 77,
    R16G16SNorm = 78,
    R16G16UScaled = 79,
    R16G16SScaled = 80,
    R16G16UInt = 81,
    R16G16SInt = 82,
    R16G16SFloat = 83,
    R16G16B16UNorm = 84,
    R16G16B16SNorm = 85,
    R16G16B16UScaled = 86,
    R16G16B16SScaled = 87,
    R16G16B16UInt = 88,
    R16G16B16SInt = 89,
    R16G16B16SFloat = 90,
    R16G16B16A16UNorm = 91,
    R16G16B16A16SNorm = 92,
    R16G16B16A16UScaled = 93,
    R16G16B16A16SScaled = 94,
    R16G16B16A16UInt = 95,
    R16G16B16A16SInt = 96,
    R16G16B16A16SFloat = 97,
    R32UInt = 98,
    R32SInt = 99,
    R32SFloat = 100,
    R32G32UInt = 101,
    R32G32SInt = 102,
    R32G32SFloat = 103,
    R32G32B32UInt = 104,
    R32G32B32SInt = 105,
    R32G32B32SFloat = 106,
    R32G32B32A32UInt = 107,
    R32G32B32A32SInt = 108,
    R32G32B32A32SFloat = 109,
    R64UInt = 110,
    R64SInt = 111,
    R64SFloat = 112,
    R64G64UInt = 113,
    R64G64SInt = 114,
    R64G64SFloat = 115,
    R64G64B64UInt = 116,
    R64G64B64SInt = 117,
    R64G64B64SFloat = 118,
    R64G64B64A64UInt = 119,
    R64G64B64A64SInt = 120,
    R64G64B64A64SFloat = 121,
    B10G11R11UFloatPack32 = 122,
    E5B9G9R9UFloatPack32 = 123,
    D16UNorm = 124,
    X8D24UNormPack32 = 125,
    D32SFloat = 126,
    S8UInt = 127,
    D16UNormS8UInt = 128,
    D24UNormS8UInt = 129,
    D32SFloatS8UInt = 130,
    BC1RGB_UNormBlock = 131,
    BC1RGB_SRGB_Block = 132,
    BC1RGBA_UNormBlock = 133,
    BC1RGBA_SRGB_Block = 134,
    BC2UNormBlock = 135,
    BC2SRGB_Block = 136,
    BC3UNormBlock = 137,
    BC3SRGB_Block = 138,
    BC4UNormBlock = 139,
    BC4SNormBlock = 140,
    BC5UNormBlock = 141,
    BC5SNormBlock = 142,
    BC6H_UFloatBlock = 143,
    BC6H_SFloatBlock = 144,
    BC7UNormBlock = 145,
    BC7SRGB_Block = 146,
    ETC2R8G8B8UNormBlock = 147,
    ETC2R8G8B8SRGB_Block = 148,
    ETC2R8G8B8A1UNormBlock = 149,
    ETC2R8G8B8A1SRGB_Block = 150,
    ETC2R8G8B8A8UNormBlock = 151,
    ETC2R8G8B8A8SRGB_Block = 152,
    EAC_R11UNormBlock = 153,
    EAC_R11SNormBlock = 154,
    EAC_R11G11UNormBlock = 155,
    EAC_R11G11SNormBlock = 156,
    ASTC4x4UNormBlock = 157,
    ASTC4x4SRGB_Block = 158,
    ASTC5x4UNormBlock = 159,
    ASTC5x4SRGB_Block = 160,
    ASTC5x5UNormBlock = 161,
    ASTC5x5SRGB_Block = 162,
    ASTC6x5UNormBlock = 163,
    ASTC6x5SRGB_Block = 164,
    ASTC6x6UNormBlock = 165,
    ASTC6x6SRGB_Block = 166,
    ASTC8x5UNormBlock = 167,
    ASTC8x5SRGB_Block = 168,
    ASTC8x6UNormBlock = 169,
    ASTC8x6SRGB_Block = 170,
    ASTC8x8UNormBlock = 171,
    ASTC8x8SRGB_Block = 172,
    ASTC10x5UNormBlock = 173,
    ASTC10x5SRGB_Block = 174,
    ASTC10x6UNormBlock = 175,
    ASTC10x6SRGB_Block = 176,
    ASTC10x8UNormBlock = 177,
    ASTC10x8SRGB_Block = 178,
    ASTC10x10UNormBlock = 179,
    ASTC10x10SRGB_Block = 180,
    ASTC12x10UNormBlock = 181,
    ASTC12x10SRGB_Block = 182,
    ASTC12x12UNormBlock = 183,
    ASTC12x12SRGB_Block = 184,

    /// Provided by [`VK_VERSION_1_1`]
    G8B8G8R8_422UNorm = 1000156000,

    /// Provided by [`VK_VERSION_1_1`]
    B8G8R8G8_422UNorm = 1000156001,

    /// Provided by [`VK_VERSION_1_1`]
    G8B8R8_3PLANE420UNorm = 1000156002,

    /// Provided by [`VK_VERSION_1_1`]
    G8B8R8_2PLANE420UNorm = 1000156003,

    /// Provided by [`VK_VERSION_1_1`]
    G8B8R8_3PLANE422UNorm = 1000156004,

    /// Provided by [`VK_VERSION_1_1`]
    G8B8R8_2PLANE422UNorm = 1000156005,

    /// Provided by [`VK_VERSION_1_1`]
    G8B8R8_3PLANE444UNorm = 1000156006,

    /// Provided by [`VK_VERSION_1_1`]
    R10X6UNormPack16 = 1000156007,

    /// Provided by [`VK_VERSION_1_1`]
    R10X6G10X6UNorm2Pack16 = 1000156008,

    /// Provided by [`VK_VERSION_1_1`]
    R10X6G10X6B10X6A10X6UNorm4Pack16 = 1000156009,

    /// Provided by [`VK_VERSION_1_1`]
    G10X6B10X6G10X6R10X6_422UNorm4Pack16 = 1000156010,

    /// Provided by [`VK_VERSION_1_1`]
    B10X6G10X6R10X6G10X6_422UNorm4Pack16 = 1000156011,

    /// Provided by [`VK_VERSION_1_1`]
    G10X6B10X6R10X6_3PLANE420UNorm3Pack16 = 1000156012,

    /// Provided by [`VK_VERSION_1_1`]
    G10X6B10X6R10X6_2PLANE420UNorm3Pack16 = 1000156013,

    /// Provided by [`VK_VERSION_1_1`]
    G10X6B10X6R10X6_3PLANE422UNorm3Pack16 = 1000156014,

    /// Provided by [`VK_VERSION_1_1`]
    G10X6B10X6R10X6_2PLANE422UNorm3Pack16 = 1000156015,

    /// Provided by [`VK_VERSION_1_1`]
    G10X6B10X6R10X6_3PLANE444UNorm3Pack16 = 1000156016,

    /// Provided by [`VK_VERSION_1_1`]
    R12X4UNormPack16 = 1000156017,

    /// Provided by [`VK_VERSION_1_1`]
    R12X4G12X4UNorm2Pack16 = 1000156018,

    /// Provided by [`VK_VERSION_1_1`]
    R12X4G12X4B12X4A12X4UNorm4Pack16 = 1000156019,

    /// Provided by [`VK_VERSION_1_1`]
    G12X4B12X4G12X4R12X4_422UNorm4Pack16 = 1000156020,

    /// Provided by [`VK_VERSION_1_1`]
    B12X4G12X4R12X4G12X4_422UNorm4Pack16 = 1000156021,

    /// Provided by [`VK_VERSION_1_1`]
    G12X4B12X4R12X4_3PLANE420UNorm3Pack16 = 1000156022,

    /// Provided by [`VK_VERSION_1_1`]
    G12X4B12X4R12X4_2PLANE420UNorm3Pack16 = 1000156023,

    /// Provided by [`VK_VERSION_1_1`]
    G12X4B12X4R12X4_3PLANE422UNorm3Pack16 = 1000156024,

    /// Provided by [`VK_VERSION_1_1`]
    G12X4B12X4R12X4_2PLANE422UNorm3Pack16 = 1000156025,

    /// Provided by [`VK_VERSION_1_1`]
    G12X4B12X4R12X4_3PLANE444UNorm3Pack16 = 1000156026,

    /// Provided by [`VK_VERSION_1_1`]
    G16B16G16R16_422UNorm = 1000156027,

    /// Provided by [`VK_VERSION_1_1`]
    B16G16R16G16_422UNorm = 1000156028,

    /// Provided by [`VK_VERSION_1_1`]
    G16B16R16_3PLANE420UNorm = 1000156029,

    /// Provided by [`VK_VERSION_1_1`]
    G16B16R16_2PLANE420UNorm = 1000156030,

    /// Provided by [`VK_VERSION_1_1`]
    G16B16R16_3PLANE422UNorm = 1000156031,

    /// Provided by [`VK_VERSION_1_1`]
    G16B16R16_2PLANE422UNorm = 1000156032,

    /// Provided by [`VK_VERSION_1_1`]
    G16B16R16_3PLANE444UNorm = 1000156033,

    /// Provided by [`VK_VERSION_1_3`]
    G8B8R8_2PLANE444UNorm = 1000330000,

    /// Provided by [`VK_VERSION_1_3`]
    G10X6B10X6R10X6_2PLANE444UNorm3Pack16 = 1000330001,

    /// Provided by [`VK_VERSION_1_3`]
    G12X4B12X4R12X4_2PLANE444UNorm3Pack16 = 1000330002,

    /// Provided by [`VK_VERSION_1_3`]
    G16B16R16_2PLANE444UNorm = 1000330003,

    /// Provided by [`VK_VERSION_1_3`]
    A4R4G4B4UNormPack16 = 1000340000,

    /// Provided by [`VK_VERSION_1_3`]
    A4B4G4R4UNormPack16 = 1000340001,

    /// Provided by [`VK_VERSION_1_3`]
    ASTC4x4SFloatBlock = 1000066000,

    /// Provided by [`VK_VERSION_1_3`]
    ASTC5x4SFloatBlock = 1000066001,

    /// Provided by [`VK_VERSION_1_3`]
    ASTC5x5SFloatBlock = 1000066002,

    /// Provided by [`VK_VERSION_1_3`]
    ASTC6x5SFloatBlock = 1000066003,

    /// Provided by [`VK_VERSION_1_3`]
    ASTC6x6SFloatBlock = 1000066004,

    /// Provided by [`VK_VERSION_1_3`]
    ASTC8x5SFloatBlock = 1000066005,

    /// Provided by [`VK_VERSION_1_3`]
    ASTC8x6SFloatBlock = 1000066006,

    /// Provided by [`VK_VERSION_1_3`]
    ASTC8x8SFloatBlock = 1000066007,

    /// Provided by [`VK_VERSION_1_3`]
    ASTC10x5SFloatBlock = 1000066008,

    /// Provided by [`VK_VERSION_1_3`]
    ASTC10x6SFloatBlock = 1000066009,

    /// Provided by [`VK_VERSION_1_3`]
    ASTC10x8SFloatBlock = 1000066010,

    /// Provided by [`VK_VERSION_1_3`]
    ASTC10x10SFloatBlock = 1000066011,

    /// Provided by [`VK_VERSION_1_3`]
    ASTC12x10SFloatBlock = 1000066012,

    /// Provided by [`VK_VERSION_1_3`]
    ASTC12x12SFloatBlock = 1000066013,

    /// Provided by [`img_format_pvrtc`]
    PVRTC1_2BPP_UNormBlockIMG = 1000054000,

    /// Provided by [`img_format_pvrtc`]
    PVRTC1_4BPP_UNormBlockIMG = 1000054001,

    /// Provided by [`img_format_pvrtc`]
    PVRTC2_2BPP_UNormBlockIMG = 1000054002,

    /// Provided by [`img_format_pvrtc`]
    PVRTC2_4BPP_UNormBlockIMG = 1000054003,

    /// Provided by [`img_format_pvrtc`]
    PVRTC1_2BPP_SRGB_BlockIMG = 1000054004,

    /// Provided by [`img_format_pvrtc`]
    PVRTC1_4BPP_SRGB_BlockIMG = 1000054005,

    /// Provided by [`img_format_pvrtc`]
    PVRTC2_2BPP_SRGB_BlockIMG = 1000054006,

    /// Provided by [`img_format_pvrtc`]
    PVRTC2_4BPP_SRGB_BlockIMG = 1000054007,

    /// Provided by [`nv_optical_flow`]
    R16G16SFixed5NV = 1000464000,

    /// Provided by [`khr_maintenance5`]
    A1B5G5R5UNormPack16KHR = 1000470000,

    /// Provided by [`khr_maintenance5`]
    A8UNormKHR = 1000470001,
}
