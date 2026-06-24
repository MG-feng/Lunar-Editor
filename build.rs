// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use std::env;

fn main() {
    println!("cargo:rerun-if-changed=assets/");
    println!("cargo:rerun-if-changed=build.rs");

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);

    if let Ok(git_hash) = get_git_hash() {
        println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    }
}

fn get_git_hash() -> Result<String, std::io::Error> {
    use std::process::Command;

    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Ok("unknown".to_string())
    }
}
