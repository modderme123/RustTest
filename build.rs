use std::process::Command;

fn main() {
    Command::new("./glslc")
        .args(&["src/shader.frag", "-o", "src/shader.frag.spv"]).status().unwrap();
    Command::new("./glslc")
        .args(&["src/shader.vert", "-o", "src/shader.vert.spv"]).status().unwrap();

    println!("cargo:rerun-if-changed=shader.frag");
    println!("cargo:rerun-if-changed=shader.vert");
}
