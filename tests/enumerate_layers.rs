#[test]
pub fn enumerate_layers() {
    let vulkan = vulkan::Vulkan::new_native().unwrap();

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
            layer.layer_name().to_string_lossy(),
            layer.description().to_string_lossy(),
            layer.implementation_version(),
            layer.spec_version()
        );

        let extensions =
            match vulkan.enumerate_instance_extension_properties(Some(layer.layer_name())) {
                Ok(extensions) => extensions,
                Err(_) => continue,
            };
        for extension in extensions {
            println!(
                "    Extension: {} v{}",
                extension.extension_name().to_string_lossy(),
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
            extension.extension_name().to_string_lossy(),
            extension.spec_version()
        )
    }
}
