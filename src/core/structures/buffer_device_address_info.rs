use crate::{VkBuffer, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{VK_VERSION_1_2, VkBufferUsageFlag};

/// Structure specifying the buffer to query an address for
///
/// Provided by [`VK_VERSION_1_2`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkBufferDeviceAddressInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::BufferDeviceAddressInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `next` must be [`null`]
    pub next: *const c_void,

    /// `buffer` specifies the buffer whose address is being queried.
    ///
    /// # Valid Usage
    ///  - `buffer` must have been created with the [`VkBufferUsageFlag::ShaderDeviceAddress`]
    ///    usage flag set
    ///
    /// # Valid Usage (Implicit)
    ///  - `buffer` must be a valid [`VkBuffer`] handle
    pub buffer: VkBuffer,
}

const impl Default for VkBufferDeviceAddressInfo {
    fn default() -> Self {
        VkBufferDeviceAddressInfo {
            r#type: VkStructureType::BufferDeviceAddressInfo,
            next: null(),
            buffer: VkBuffer::null(),
        }
    }
}

impl NextChain for VkBufferDeviceAddressInfo {
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
