use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=config.json");

    fs::copy("config.json", "target/config.json")
        .expect("Failed to copy config.json to target folder");
}
