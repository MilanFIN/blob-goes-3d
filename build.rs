use std::process::Command;
use std::io::{self, Write};

fn main() {
    let output = Command::new("python3")
        .arg("scripts/bundle_levels.py")
        .output()
        .expect("Failed to bundle level json files");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    if !output.status.success() {
        panic!("Failed to bundle level json files");
    }

}