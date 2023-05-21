#[test]
pub fn enumerate_layers() {
    let loader = native::NativeLoader::new().unwrap();
    let vulkan = vulkan::Vulkan::new(loader).unwrap();

    let layers = vulkan.enumerate_instance_layer_properties().unwrap();

    println!("Found {} layers:", layers.len());
    for layer in layers {
        println!(
            "  Layer: {} - {} (v{} for Vulkan {})",
            layer.layer_name(),
            layer.description(),
            layer.implementation_version(),
            layer.spec_version()
        );
    }
}
