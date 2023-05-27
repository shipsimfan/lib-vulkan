use crate::{
    ColorSpace, CompositeAlphaFlagBits, Extent2D, Format, ImageUsageFlags, Loader, NativeLoader,
    PresentMode, SharingMode, Surface, SurfaceTransformFlagBits, Swapchain,
};

pub struct SwapchainCreateInfo<'a, L: Loader = NativeLoader> {
    pub surface: &'a mut Surface<L>,
    pub min_image_count: u32,
    pub image_format: Format,
    pub image_color_space: ColorSpace,
    pub image_extent: Extent2D,
    pub image_array_layers: u32,
    pub image_usage: ImageUsageFlags,
    pub image_sharing_mode: SharingMode,
    pub queue_family_indices: Vec<u32>,
    pub pre_transform: SurfaceTransformFlagBits,
    pub composite_alpha: CompositeAlphaFlagBits,
    pub present_mode: PresentMode,
    pub clipped: bool,
    pub old_swapchain: Option<&'a mut Swapchain<L>>,
}
