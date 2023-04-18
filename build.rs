use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=config.json");

    let config = slint_build::CompilerConfiguration::new()
        .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer);
    slint_build::compile_with_config("ui/main_window.slint", config).unwrap();
    slint_build::print_rustc_flags().unwrap();

    fs::copy("config.json", "target/config.json")
        .expect("Failed to copy config.json to target folder");
}
