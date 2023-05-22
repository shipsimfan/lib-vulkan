use crate::{bindings::VkStructureType, VkDeviceQueueCreateInfo, VkPhysicalDeviceFeatures};
use std::{ffi::c_void, ptr::NonNull};

#[repr(C)]
pub struct VkDeviceCreateInfo<'a> {
    s_type: VkStructureType,
    p_next: Option<NonNull<*const c_void>>,
    flags: u32,
    queue_create_info_count: u32,
    p_queue_create_infos: NonNull<VkDeviceQueueCreateInfo<'a>>,
    enabled_layer_count: u32,
    pp_enabled_layer_names: Option<NonNull<*const u8>>,
    enabled_extension_count: u32,
    pp_enabled_extension_names: Option<NonNull<*const u8>>,
    p_enabled_features: Option<NonNull<VkPhysicalDeviceFeatures>>,
}

impl<'a> VkDeviceCreateInfo<'a> {
    pub fn new(
        queue_create_infos: &'a [VkDeviceQueueCreateInfo<'a>],
        enabled_layers: &'a [*const u8],
        enabled_extensions: &'a [*const u8],
        enabled_features: Option<&'a VkPhysicalDeviceFeatures>,
    ) -> Self {
        assert!(queue_create_infos.len() > 0);

        VkDeviceCreateInfo {
            s_type: VkStructureType::DeviceCreateInfo,
            p_next: None,
            flags: 0,
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: unsafe {
                NonNull::new_unchecked(queue_create_infos.as_ptr() as _)
            },
            enabled_layer_count: enabled_layers.len() as u32,
            pp_enabled_layer_names: if enabled_layers.len() == 0 {
                None
            } else {
                Some(unsafe { NonNull::new_unchecked(enabled_layers.as_ptr() as _) })
            },
            enabled_extension_count: enabled_extensions.len() as u32,
            pp_enabled_extension_names: if enabled_extensions.len() == 0 {
                None
            } else {
                Some(unsafe { NonNull::new_unchecked(enabled_extensions.as_ptr() as _) })
            },
            p_enabled_features: enabled_features.map(|enabled_features| unsafe {
                NonNull::new_unchecked(enabled_features as *const _ as _)
            }),
        }
    }

    pub fn queue_create_infos(&self) -> &[VkDeviceQueueCreateInfo] {
        unsafe {
            std::slice::from_raw_parts(
                self.p_queue_create_infos.as_ptr(),
                self.queue_create_info_count as usize,
            )
        }
    }

    pub fn enabled_layers(&self) -> Option<&[*const u8]> {
        self.pp_enabled_layer_names.map(|p_enabled_layers| unsafe {
            std::slice::from_raw_parts(p_enabled_layers.as_ptr(), self.enabled_layer_count as usize)
        })
    }

    pub fn enabled_extensions(&self) -> Option<&[*const u8]> {
        self.pp_enabled_extension_names
            .map(|p_enabled_extensions| unsafe {
                std::slice::from_raw_parts(
                    p_enabled_extensions.as_ptr(),
                    self.enabled_extension_count as usize,
                )
            })
    }

    pub fn enabled_features(&self) -> Option<&VkPhysicalDeviceFeatures> {
        self.p_enabled_features
            .map(|enabled_features| unsafe { enabled_features.as_ref() })
    }
}
