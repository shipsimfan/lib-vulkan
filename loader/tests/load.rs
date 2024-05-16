#[test]
fn load_drivers() {
    let driver = vulkan_loader::load_drivers();

    println!("Drivers loaded: {}", driver.len());
}
