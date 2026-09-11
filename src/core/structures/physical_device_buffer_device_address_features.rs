use crate::{VK_FALSE, VkBool32, VkStructureType, util::NextChainMut};
use std::{ffi::c_void, ptr::null_mut};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_TRUE, VK_VERSION_1_2, VkDevice, VkDeviceCreateInfo, VkGetBufferDeviceAddress,
    VkGetPhysicalDeviceFeatures2, VkPhysicalDeviceFeatures2,
};

/// Structure describing buffer address features that can be supported by an implementation
///
/// # Description
/// If the [`VkPhysicalDeviceBufferDeviceAddressFeatures`] structure is included in the `next`
/// chain of the [`VkPhysicalDeviceFeatures2`] structure passed to
/// [`VkGetPhysicalDeviceFeatures2`], it is filled in to indicate whether each corresponding
/// feature is supported. If the application wishes to use a [`VkDevice`] with any features
/// described by [`VkPhysicalDeviceBufferDeviceAddressFeatures`], it must add an instance of the
/// structure, with the desired feature members set to [`VK_TRUE`], to the `next` chain of
/// [`VkDeviceCreateInfo`] when creating the [`VkDevice`].
///
/// Provided by [`VK_VERSION_1_2`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeatures {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::PhysicalDeviceBufferDeviceAddressFeatures`]
    pub r#type: VkStructureType,

    /// `next` is [`null_mut`] or a pointer to a structure extending this structure.
    pub next: *mut c_void,

    /// `buffer_device_address` indicates that the implementation supports accessing buffer memory
    /// in shaders as storage buffers via an address queried from [`VkGetBufferDeviceAddress`].
    pub buffer_device_address: VkBool32,

    /// `buffer_device_address_capture_replay` indicates that the implementation supports saving
    /// and reusing buffer and device addresses, e.g. for trace capture and replay.
    pub buffer_device_address_capture_replay: VkBool32,

    /// `buffer_device_address_multi_device` indicates that the implementation supports the
    /// `buffer_device_address` , `ray_tracing_pipeline` and `ray_query` features for logical
    /// devices created with multiple physical devices. If this feature is not supported, buffer
    /// and acceleration structure addresses must not be queried on a logical device created with
    /// more than one physical device.
    pub buffer_device_address_multi_device: VkBool32,
}

const impl Default for VkPhysicalDeviceBufferDeviceAddressFeatures {
    fn default() -> Self {
        VkPhysicalDeviceBufferDeviceAddressFeatures {
            r#type: VkStructureType::PhysicalDeviceBufferDeviceAddressFeatures,
            next: null_mut(),
            buffer_device_address: VK_FALSE,
            buffer_device_address_capture_replay: VK_FALSE,
            buffer_device_address_multi_device: VK_FALSE,
        }
    }
}

impl NextChainMut for VkPhysicalDeviceBufferDeviceAddressFeatures {
    fn structure_type(&self) -> VkStructureType {
        self.r#type
    }

    fn next(&mut self) -> *mut c_void {
        self.next
    }

    fn as_mut_ptr(&mut self) -> *mut c_void {
        (self as *mut Self).cast()
    }

    fn set_next(&mut self, next: Option<&mut dyn NextChainMut>) {
        self.next = next.map_or(null_mut(), |n| n.as_mut_ptr());
    }
}
