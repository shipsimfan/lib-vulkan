use crate::{
    ComponentMapping, Format, Image, ImageSubresourceRange, ImageViewType, Loader, NativeLoader,
};
use std::sync::Arc;

pub struct ImageViewCreateInfo<L: Loader = NativeLoader> {
    pub image: Arc<Image<L>>,
    pub r#type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresource_range: ImageSubresourceRange,
}
