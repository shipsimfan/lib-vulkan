#[cfg(all(target_os = "windows", feature = "link"))]
fn add_vulkan_sdk_library() {
    let base_path: std::path::PathBuf = std::env::var("VULKAN_SDK")
        .expect("Vulkan SDK cannot be found, verify it is installed")
        .into();

    let library_path = base_path.join("Lib");

    println!("cargo:rustc-link-search=native={}", library_path.display());
}

fn main() {
    #[cfg(all(target_os = "windows", feature = "link"))]
    add_vulkan_sdk_library();
}
