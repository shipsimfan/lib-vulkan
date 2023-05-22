use super::{VkApplicationInfo, VkInstanceCreateFlags};
use crate::bindings::VkStructureType;
use std::{ffi::c_void, ptr::NonNull};

#[repr(C)]
pub struct VkInstanceCreateInfo<'a> {
    s_type: VkStructureType,
    p_next: Option<NonNull<c_void>>,
    flags: VkInstanceCreateFlags,
    p_application_info: Option<&'a VkApplicationInfo<'a>>,
    enabled_layer_count: u32,
    pp_enabled_layers: Option<NonNull<*const u8>>,
    enabled_extension_count: u32,
    pp_enabled_extensions: Option<NonNull<*const u8>>,
}

impl<'a> VkInstanceCreateInfo<'a> {
    pub fn new(
        flags: VkInstanceCreateFlags,
        application_info: Option<&'a VkApplicationInfo<'a>>,
        enabled_layers: &'a [*const u8],
        enabled_extensions: &'a [*const u8],
    ) -> Self {
        VkInstanceCreateInfo {
            s_type: VkStructureType::InstanceCreateInfo,
            p_next: None,
            flags,
            p_application_info: application_info,
            enabled_layer_count: enabled_layers.len() as u32,
            pp_enabled_layers: if enabled_layers.len() == 0 {
                None
            } else {
                Some(unsafe { NonNull::new_unchecked(enabled_layers.as_ptr() as *mut _) })
            },
            enabled_extension_count: enabled_extensions.len() as u32,
            pp_enabled_extensions: if enabled_extensions.len() == 0 {
                None
            } else {
                Some(unsafe { NonNull::new_unchecked(enabled_extensions.as_ptr() as *mut _) })
            },
        }
    }

    pub fn flags(&self) -> VkInstanceCreateFlags {
        self.flags
    }

    pub fn application_info(&self) -> Option<&VkApplicationInfo<'a>> {
        self.p_application_info
    }

    pub fn enabled_layers(&self) -> Option<&[*const u8]> {
        self.pp_enabled_layers.map(|pp_enabled_layers| unsafe {
            std::slice::from_raw_parts(
                pp_enabled_layers.as_ptr(),
                self.enabled_layer_count as usize,
            )
        })
    }

    pub fn enabled_extensions(&self) -> Option<&[*const u8]> {
        self.pp_enabled_extensions
            .map(|pp_enabled_extensions| unsafe {
                std::slice::from_raw_parts(
                    pp_enabled_extensions.as_ptr(),
                    self.enabled_extension_count as usize,
                )
            })
    }
}
