use super::os;
use crate::{VkGetInstanceProcAddr, VkInstance, VK_GET_INSTANCE_PROC_ADDR};
use std::ffi::CStr;

/// A loaded Vulkan driver
pub struct Driver {
    /// The driver implemented for the specific operating system
    #[allow(unused)]
    os: os::Driver,

    /// The function to get other functions with
    get_instance_proc_addr: VkGetInstanceProcAddr,
}

impl Driver {
    /// Creates a new [`Driver`] from the given OS `driver`
    pub(super) fn new(driver: os::Driver) -> Self {
        let get_instance_proc_addr = unsafe {
            std::mem::transmute(
                driver
                    .icd_get_global_proc_addr(VK_GET_INSTANCE_PROC_ADDR)
                    .unwrap(),
            )
        };

        Driver {
            os: driver,
            get_instance_proc_addr,
        }
    }

    /// Gets an instance procedure. See [`VkGetInstanceProcAddr`] for more information.
    pub fn get_instance_proc_addr(
        &self,
        instance: VkInstance,
        name: &CStr,
    ) -> Option<extern "system" fn()> {
        (self.get_instance_proc_addr)(instance, name.as_ptr())
    }
}
