#[test]
fn load_drivers() {
    let driver = vulkan_loader::load_drivers().unwrap();

    println!("Drivers loaded: {}", driver.len());
}
