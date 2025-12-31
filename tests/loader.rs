use vulkan::{VK_GET_INSTANCE_PROC_ADDR, VkInstance, vkGetInstanceProcAddr};

#[test]
fn loader() {
    unsafe { vkGetInstanceProcAddr(VkInstance::null(), VK_GET_INSTANCE_PROC_ADDR.as_ptr()) }
        .unwrap();
}
