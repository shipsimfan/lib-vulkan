#[cfg(target_os = "windows")]
fn add_vulkan_sdk_library() {
    let base_path: std::path::PathBuf = std::env::var("VULKAN_SDK")
        .expect("Vulkan SDK cannot be found, verify it is installed")
        .into();

    let library_path = base_path.join("Lib");

    println!("cargo:rustc-link-search=native={}", library_path.display());
}

fn main() {
    #[cfg(target_os = "windows")]
    add_vulkan_sdk_library();
}
