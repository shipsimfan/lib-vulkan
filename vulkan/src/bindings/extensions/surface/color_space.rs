#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VkColorSpaceKHR {
    SRGBNonLinear = 0,
    DisplayP3NonLinear = 1000104001,
    ExtendedSRGBLinear = 1000104002,
    DisplayP3Linear = 1000104003,
    DCIP3NonLinear = 1000104004,
    BT709Linear = 1000104005,
    BT709NonLinear = 1000104006,
    BT2020Linear = 1000104007,
    HDR10ST2084 = 1000104008,
    DolbyVision = 1000104009,
    HDR10HLG = 1000104010,
    AdobeRGBLinear = 1000104011,
    AdobeRGBNonLinear = 1000104012,
    PASSTHROUGH = 1000104013,
    ExtendedSRGBNonLinear = 1000104014,
    DisplayNativeAMD = 1000213000,
}
