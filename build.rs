use std::path::PathBuf;

fn main() {
    let base_path: PathBuf = std::env::var("VULKAN_SDK")
        .expect("Vulkan SDK cannot be found, verify it is installed")
        .into();

    let library_path = base_path.join("Lib");

    println!("cargo:rustc-link-search=native={}", library_path.display());
}
