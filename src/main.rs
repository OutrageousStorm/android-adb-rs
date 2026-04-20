// Rust ADB wrapper
// Usage: cargo build && ./target/debug/adb-rs devices

fn main() {
    println!("ADB Rust Wrapper v0.1");
    
    let output = std::process::Command::new("adb")
        .arg("devices")
        .output()
        .expect("Failed to run adb");
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    println!("{}", stdout);
}
