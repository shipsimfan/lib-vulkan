use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_3;

flags! {
    /// Bitmask of [`VkRenderingFlag`]
    ///
    /// # Description
    /// [`VkRenderingFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkRenderingFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    pub struct VkRenderingFlags;

    /// Bitmask specifying additional properties of a dynamic render pass instance
    ///
    /// # Description
    /// The contents of `rendering_info` must match between suspended render pass instances and the
    /// render pass instances that resume them, other than the presence or absence of the
    /// [`VkRenderingFlag::Resuming`], [`VkRenderingFlag::Suspending`], and
    /// [`VkRenderingFlag::ContentsSecondaryCommandBuffers`] flags. No action or synchronization
    /// commands, or other render pass instances, are allowed between suspending and resuming
    /// render pass instances.
    ///
    /// Provided by [`VK_VERSION_1_3`]
    pub enum VkRenderingFlag {
        /// [`VkRenderingFlag::ContentsSecondaryCommandBuffers`] specifies that draw calls for
        /// the render pass instance will be recorded in secondary command buffers. If the
        /// `nested_command_buffer` feature is enabled, the draw calls can come from both inline
        /// and [`VkCmdExecuteCommands`].
        ContentsSecondaryCommandBuffers = 0x00000001,

        /// [`VkRenderingFlag::Suspending`] specifies that the render pass instance will be
        /// suspended.
        Suspending = 0x00000002,

        /// [`VkRenderingFlag::Resuming`] specifies that the render pass instance is resuming an
        /// earlier suspended render pass instance.
        Resuming = 0x00000004,

        /// [`VkRenderingFlag::EnableLegacyDithering`] specifies that Legacy Dithering is
        /// enabled for the render pass instance.
        ///
        /// Provided by [`ext_legacy_dithering`] with ([`khr_dynamic_rendering`] or
        /// [`VK_VERSION_1_3`]) and ([`khr_maintenance5`] or [`VK_VERSION_1_4`])
        EnableLegacyDithering = 0x00000008,

        /// [`VkRenderingFlag::ContentsInlineKhr`] specifies that draw calls for the render pass
        /// instance can be recorded inline within the current command buffer. This can be combined
        /// with the [`VkRenderingFlag::ContentsSecondaryCommandBuffers`] bit to allow draw
        /// calls to be recorded both inline and in secondary command buffers.
        ///
        /// Provided by [`khr_maintenance7`]
        ContentsInlineKhr = 0x00000010,

        /// [`VkRenderingFlag::PerLayerFragmentDensityValve`] specifies that the render pass can
        /// be used with layered fragment density maps.
        ///
        /// Provided by [`valve_fragment_density_map_layered`]
        PerLayerFragmentDensityValve = 0x00000020,

        /// [`VkRenderingFlag::FragmentRegionExt`] specifies that the render pass can access
        /// samples which are not covered in its `sample_mask`.
        ///
        /// Provided by [`ext_custom_resolve`] with [`khr_dynamic_rendering`] or [`VK_VERSION_1_3`]
        FragmentRegionExt = 0x00000040,

        /// [`VkRenderingFlag::CustomResolveExt`] specifies that the render pass contains a
        /// custom resolve. When this bit is set, [`VkCmdBeginCustomResolveExt`] can be called.
        ///
        /// Provided by [`ext_custom_resolve`] with [`khr_dynamic_rendering`] or [`VK_VERSION_1_3`]
        CustomResolveExt = 0x00000080,

        /// [`VkRenderingFlag::LocalReadConcurrentAccessControlKhr`] specifies that
        /// [`VkRenderingAttachmentFlag::InputAttachmentFeedbackKhr`] will always be specified
        /// for any attachment which invokes the behavior described by that flag.
        ///
        /// Provided by [`khr_maintenance10`] with ([`VK_VERSION_1_4`] or
        /// [`khr_dynamic_rendering_local_read`]) and ([`VK_VERSION_1_3`] or
        /// [`khr_dynamic_rendering`])
        LocalReadConcurrentAccessControlKhr = 0x00000100,
    }
}
