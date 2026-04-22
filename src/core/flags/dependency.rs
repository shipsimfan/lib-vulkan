use crate::macros::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_0, VK_VERSION_1_1};

flags! {
    /// Bitmask of [`VkDependencyFlags`]
    ///
    /// # Description
    /// [`VkDependencyFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkDependencyFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkDependencyFlags;


    /// Bitmask specifying how execution and memory dependencies are formed
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkDependencyFlag {
        /// [`VkDependencyFlag::ByRegionBit`] specifies that dependencies will be split into
        /// multiple framebuffer-local regions according to the (x,y,layer,sample) coordinates.
        ByRegionBit = 0x00000001,

        /// [`VkDependencyFlag::ViewLocalBit`] specifies that dependencies will be split into
        /// multiple framebuffer-local regions according to the view.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        ViewLocalBit = 0x00000002,

        /// [`VkDependencyFlag::DeviceGroupBit`] specifies that dependencies are non-device-local.
        ///
        /// Provided by [`VK_VERSION_1_1`]
        DeviceGroupBit = 0x00000004,

        /// [`VkDependencyFlag::FeedbackLoopBitExt`] specifies that the render pass will write to
        /// and read from the same image with feedback loop enabled.
        ///
        /// Provided by [`ext_attachment_feedback_loop_layout`]
        FeedbackLoopBitExt = 0x00000008,

        /// [`VkDependencyFlag::QueueFamilyOwnershipTransferUseAllStagesBitKhr`] specifies that
        /// source and destination stages are not ignored when performing a queue family ownership
        /// transfer.
        ///
        /// Provided by [`khr_maintenance8`]
        QueueFamilyOwnershipTransferUseAllStagesBitKhr = 0x00000020,

        /// [`VkDependencyFlag::AsymmetricEventBitKhr`] specifies that the access scopes of
        /// [`VkCmdSetEvent2`] and [`VkCmdWaitEvents2`] do not need to match for a given event when
        /// it is specified in both commands, and the access scope of [`VkCmdSetEvent2`] is empty.
        ///
        /// Provided by [`khr_maintenance9`]
        AsymmetricEventBitKhr = 0x00000040,
    }
}
