use vulkan::VkVersion;

#[test]
fn triangle() {
    let vulkan = vulkan::Vulkan::new(vulkan::NativeLoader::new().unwrap()).unwrap();

    let instance = vulkan
        .create_instance(&vulkan::VkInstanceCreateInfo::new(
            vulkan::VkInstanceCreateFlags::new(&[]),
            Some(&vulkan::VkApplicationInfo::new(
                Some("Triangle Test\0"),
                1,
                None,
                0,
                VkVersion::new(0, 1, 0, 0),
            )),
            &[],
            &[],
        ))
        .unwrap();
}
