use crate::flags;

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

flags! {
    /// Bitmask of [`VkDescriptorPoolCreateFlag`]s
    ///
    /// # Description
    /// [`VkDescriptorPoolCreateFlags`] is a bitmask type for setting a mask of zero or more
    /// [`VkDescriptorPoolCreateFlag`]s.
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub struct VkDescriptorPoolCreateFlags;

    /// Bitmask specifying certain supported operations on a descriptor pool
    ///
    /// Provided by [`VK_VERSION_1_0`]
    pub enum VkDescriptorPoolCreateFlag {
        /// [`VkDescriptorPoolCreateFlag::FreeDescriptorSet`] specifies that descriptor sets can
        /// return their individual allocations to the pool, i.e. all of
        /// [`VkAllocateDescriptorSets`], [`VkFreeDescriptorSets`], and [`VkResetDescriptorPool`]
        /// are allowed. Otherwise, descriptor sets allocated from the pool must not be
        /// individually freed back to the pool, i.e. only [`VkAllocateDescriptorSets`] and
        /// [`VkResetDescriptorPool`] are allowed.
        FreeDescriptorSet = 0x00000001,

        /// [`VkDescriptorPoolCreateFlag::UpdateAfterBind`] specifies that descriptor sets
        /// allocated from this pool can include bindings with the
        /// [`VkDescriptorBindingFlag::UpdateAfterBind`] bit set. It is valid to allocate
        /// descriptor sets that have bindings that do not set the
        /// [`VkDescriptorBindingFlag::UpdateAfterBind`] bit from a pool that has
        /// [`VkDescriptorPoolCreateFlag::UpdateAfterBind`] set.
        ///
        /// Provided by [`VK_VERSION_1_2`]
        UpdateAfterBind = 0x00000002,

        /// [`VkDescriptorPoolCreateFlag::HostOnlyExt`] specifies that this descriptor pool and the
        /// descriptor sets allocated from it reside entirely in host memory and cannot be bound.
        /// Similar to descriptor sets allocated without this flag, applications can copy-from and
        /// copy-to descriptors sets allocated from this descriptor pool. Descriptor sets allocated
        /// from this pool are partially exempt from the external synchronization requirement in
        /// [`VkUpdateDescriptorSetWithTemplateKhr`] and [`VkUpdateDescriptorSets`]. Descriptor
        /// sets and their descriptors can be updated concurrently in different threads, though the
        /// same descriptor must not be updated concurrently by two threads.
        ///
        /// Provided by [`ext_mutable_descriptor_type`]
        HostOnlyExt = 0x00000004,

        /// [`VkDescriptorPoolCreateFlag::AllowOverallocationSetsNv`] specifies that the
        /// implementation should allow the application to allocate more than
        /// [`VkDescriptorPoolCreateInfo::max_sets`] descriptor set objects from the descriptor
        /// pool as available resources allow. The implementation may use the maxSets value to
        /// allocate the initial available sets, but using zero is permitted.
        ///
        /// Provided by [`nv_descriptor_pool_overallocation`]
        AllowOverallocationSetsNv = 0x00000008,

        /// [`VkDescriptorPoolCreateFlag::AllowOverallocationPoolsNv`] specifies that the
        /// implementation should allow the application to allocate more descriptors from the pool
        /// than was specified by the [`VkDescriptorPoolSize::descriptor_count`] for any descriptor
        /// type as specified by [`VkDescriptorPoolCreateInfo::pool_size_count`] and
        /// [`VkDescriptorPoolCreateInfo::pool_sizes`], as available resources allow. The
        /// implementation may use the `descriptor_count` for each descriptor type to allocate the
        /// initial pool, but the application is allowed to set the `pool_size_count` to zero, or
        /// any of the `descriptor_count` values in the `pool_sizes` array to zero.
        ///
        /// Provided by [`nv_descriptor_pool_overallocation`]
        AllowOverallocationPoolsNv = 0x00000010,
    }
}
