use std::ptr::null_mut;
use vulkan::{vkGetInstanceProcAddr, VK_GET_INSTANCE_PROC_ADDR};

#[test]
fn loader() {
    unsafe { vkGetInstanceProcAddr(null_mut(), VK_GET_INSTANCE_PROC_ADDR.as_ptr()) }.unwrap();
}
