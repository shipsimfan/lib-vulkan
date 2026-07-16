use crate::{
    VK_FALSE, VkBool32, VkPipelineInputAssemblyStateCreateFlags, VkPrimitiveTopology,
    VkStructureType, util::NextChain,
};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::VK_VERSION_1_0;

/// Structure specifying parameters of a newly created pipeline input assembly state
///
/// # Description
/// Drawing can be achieved in two modes:
///  - Programmable Mesh Shading, the mesh shader assembles primitives, or
///  - Programmable Primitive Shading, the input primitives are assembled as follows.
///
/// Each draw is made up of zero or more vertices and zero or more instances, which are processed
/// by the device and result in the assembly of primitives. Primitives are assembled according to
/// the `input_assembly_state` member of the [`VkGraphicsPipelineCreateInfo`] structure, which is
/// of type [`VkPipelineInputAssemblyStateCreateInfo`].
///
/// Restarting the assembly of primitives discards the most recent index values if those elements
/// formed an incomplete primitive, and restarts the primitive assembly using the subsequent
/// indices, but only assembling the immediately following element through the end of the
/// originally specified elements. The primitive restart index value comparison is performed before
/// adding the `vertex_offset` value to the index value.
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PipelineInputAssemblyStateCreateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`]
    pub next: *const c_void,

    /// `flags` is reserved for future use.
    ///
    /// # Valid Usage (Implicit)
    ///  - `flags` must be 0
    pub flags: VkPipelineInputAssemblyStateCreateFlags,

    /// `topology` is a [`VkPrimitiveTopology`] defining the primitive topology.
    ///
    /// # Valid Usage
    ///  - If the `primitive_topology_list_restart` feature is not enabled, and `topology` is
    ///    [`VkPrimitiveTopology::PointList`], [`VkPrimitiveTopology::LineList`],
    ///    [`VkPrimitiveTopology::TriangleList`], [`VkPrimitiveTopology::LineListWithAdjacency`],
    ///    or [`VkPrimitiveTopology::TriangleListWithAdjacency`], `primitive_restart_enable` must
    ///    be [`VK_FALSE`]
    ///  - If the `primitive_topology_patch_list_restart` feature is not enabled, and `topology` is
    ///    [`VkPrimitiveTopology::PatchList`], `primitive_restart_enable` must be [`VK_FALSE`]
    ///  - If the `geometry_shader` feature is not enabled, `topology` must not be any of
    ///    [`VkPrimitiveTopology::LineListWithAdjacency`],
    ///    [`VkPrimitiveTopology::LineStripWithAdjacency`],
    ///    [`VkPrimitiveTopology::TriangleListWithAdjacency`] or
    ///    [`VkPrimitiveTopology::TriangleStripWithAdjacency`]
    ///  - If the `tessellation_shader` feature is not enabled, `topology` must not be
    ///    [`VkPrimitiveTopology::PatchList`]
    ///  - If the [`khr_portability_subset`] extension is enabled, and
    ///    [`VkPhysicalDevicePortabilitySubsetFeaturesKhr::triangle_fans`] is [`VK_FALSE`],
    ///    topology must not be [`VkPrimitiveTopology::TriangleFan`]
    ///
    /// # Valid Usage (Implicit)
    ///  - `topology` must be a valid [`VkPrimitiveTopology`] value
    pub topology: VkPrimitiveTopology,

    /// `primitive_restart_enable` controls whether a special vertex index value is treated as
    /// restarting the assembly of primitives. This enable only applies to indexed draws
    /// ([`VkCmdDrawIndexed`], [`VkCmdDrawMultiIndexedExt`], and [`VkCmdDrawIndexedIndirect`]), and
    /// the special index value is either `0xFFFFFFFF` when the `index_type` parameter of
    /// Vulkan 1.4 or [`VkCmdBindIndexBuffer2`] or [`VkCmdBindIndexBuffer`] is equal to
    /// [`VkIndexType::UInt32`]; `0xFF` when `index_type` is equal to [`VkIndexType::UInt8`]; or
    /// `0xFFFF` when `index_type` is equal to [`VkIndexType::UInt16`]. Primitive restart is not
    /// allowed for “list” topologies, unless one of the features
    /// `primitive_topology_patch_list_restart` (for [`VkPrimitiveTopology::PatchList`]) or
    /// `primitive_topology_list_restart` (for all other list topologies) is enabled. If the
    /// `primitive_restart_index` feature is enabled, [`VkCmdSetPrimitiveRestartIndexExt`] can be
    /// used to set a custom index for primitive restart.
    pub primitive_restart_enable: VkBool32,
}

const impl Default for VkPipelineInputAssemblyStateCreateInfo {
    fn default() -> Self {
        VkPipelineInputAssemblyStateCreateInfo {
            r#type: VkStructureType::PipelineInputAssemblyStateCreateInfo,
            next: null(),
            flags: VkPipelineInputAssemblyStateCreateFlags::empty(),
            topology: VkPrimitiveTopology::PointList,
            primitive_restart_enable: VK_FALSE,
        }
    }
}

impl NextChain for VkPipelineInputAssemblyStateCreateInfo {
    fn structure_type(&self) -> VkStructureType {
        self.r#type
    }

    fn next(&self) -> *const c_void {
        self.next
    }

    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: Option<&dyn NextChain>) {
        self.next = next.map_or(null(), |n| n.as_ptr());
    }
}
