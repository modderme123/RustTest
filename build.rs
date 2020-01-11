use std::process::Command;

fn main() {
    Command::new("./glslc")
        .args(&["shader.frag", "-o", "src/shader.frag.spv"])
        .status()
        .unwrap();
    Command::new("./glslc")
        .args(&["shader.vert", "-o", "src/shader.vert.spv"])
        .status()
        .unwrap();

    println!("cargo:rerun-if-changed=shader.frag");
    println!("cargo:rerun-if-changed=shader.vert");
}
