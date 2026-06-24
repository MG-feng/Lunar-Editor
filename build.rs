// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=assets/");
    println!("cargo:rerun-if-changed=build.rs");

    // 创建必要的目录
    let out_dir = env::var("OUT_DIR").unwrap();
    let assets_dir = Path::new("assets");

    // 确保assets目录存在
    if !assets_dir.exists() {
        fs::create_dir_all(assets_dir).unwrap();
    }

    // 生成编译时间戳
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);

    // 获取Git信息
    if let Ok(git_hash) = get_git_hash() {
        println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    }

    // 设置编译优化标志
    println!("cargo:rustc-cfg=feature=\"build_assets\"");

    // Windows特定优化
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-arg=/SUBSYSTEM:WINDOWS");
    }

    // 启用LTO
    println!("cargo:rustc-cfg=lto");
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
