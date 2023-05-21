#[test]
pub fn enumerate_layers() {
    let loader = native::NativeLoader::new().unwrap();
    let vulkan = vulkan::Vulkan::new(loader).unwrap();

    match vulkan.enumerate_instance_version() {
        Ok(version) => println!("This system supports Vulkan {}", version),
        Err(vulkan::VkResult::ErrorIncompatibleDriver) => {
            println!("This system supports Vulkan 1.0")
        }
        Err(error) => Err(error).unwrap(),
    }

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

        let extensions = vulkan
            .enumerate_instance_extension_properties(Some(layer.layer_name()))
            .unwrap();
        for extension in extensions {
            println!(
                "    Extension: {} v{}",
                extension.extension_name(),
                extension.spec_version()
            );
        }
    }

    let extensions = vulkan
        .enumerate_instance_extension_properties(None)
        .unwrap();
    println!("\nFound {} base extensions:", extensions.len());
    for extension in extensions {
        println!(
            "  Extension: {} v{}",
            extension.extension_name(),
            extension.spec_version()
        )
    }
}
