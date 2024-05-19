#[test]
fn load_drivers() {
    let driver = vulkan::loader::load_drivers();

    println!("Drivers loaded: {}", driver.len());
}
