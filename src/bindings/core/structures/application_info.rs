use crate::{assert_null_terminated, bindings::VkStructureType, VkVersion};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
    ptr::NonNull,
};

#[repr(C)]
pub struct VkApplicationInfo<'a> {
    s_type: VkStructureType,
    p_next: Option<NonNull<c_void>>,
    p_application_name: Option<NonNull<u8>>,
    application_version: u32,
    p_engine_name: Option<NonNull<u8>>,
    engine_version: u32,
    api_version: VkVersion,
    phantom: PhantomData<&'a ()>,
}

impl<'a> VkApplicationInfo<'a> {
    pub fn new(
        application_name: Option<&'a str>,
        application_version: u32,
        engine_name: Option<&'a str>,
        engine_version: u32,
        api_version: VkVersion,
    ) -> Self {
        if let Some(application_name) = application_name {
            assert_null_terminated!(application_name);
        }

        if let Some(engine_name) = engine_name {
            assert_null_terminated!(engine_name);
        }

        VkApplicationInfo {
            s_type: VkStructureType::ApplicationInfo,
            p_next: None,
            p_application_name: application_name.map(|application_name| unsafe {
                NonNull::new_unchecked(application_name.as_ptr() as *mut _)
            }),
            application_version,
            p_engine_name: engine_name.map(|engine_name| unsafe {
                NonNull::new_unchecked(engine_name.as_ptr() as *mut _)
            }),
            engine_version,
            api_version,
            phantom: PhantomData,
        }
    }

    pub fn application_name(&self) -> Option<&str> {
        self.p_application_name.map(|application_name| unsafe {
            CStr::from_ptr(application_name.as_ptr() as *const _)
                .to_str()
                .unwrap()
        })
    }

    pub fn application_version(&self) -> u32 {
        self.application_version
    }

    pub fn engine_name(&self) -> Option<&str> {
        self.p_engine_name.map(|engine_name| unsafe {
            CStr::from_ptr(engine_name.as_ptr() as *const _)
                .to_str()
                .unwrap()
        })
    }

    pub fn engine_version(&self) -> u32 {
        self.engine_version
    }

    pub fn api_version(&self) -> VkVersion {
        self.api_version
    }
}
