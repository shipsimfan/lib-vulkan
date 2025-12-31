use crate::vk_define_handle;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

vk_define_handle!(
    /// Opaque handle to an image object
    ///
    /// Images represent multidimensional - up to 3 - arrays of data which can be used for various
    /// purposes (e.g. attachments, textures), by binding them to a graphics or compute pipeline
    /// via descriptor sets, or by directly specifying them as parameters to certain commands.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    VkImage
);
