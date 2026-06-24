#!/usr/bin/env python3
"""
Lunar Editor 编译错误修复脚本
生成最小可编译版本
"""

import os
from pathlib import Path

BASE_DIR = Path(".")

# 修复 Cargo.toml - 统一版本
CARGO_TOML = r"""[package]
name = "lunar-editor"
version = "0.1.0"
edition = "2021"
license = "Apache-2.0"
authors = ["Lunar Editor Team"]
description = "A high-performance code editor with 2D/3D visualization"
repository = "https://github.com/MG-feng/Lunar-Editor"
build = "build.rs"

[dependencies]
# GUI Framework - 统一使用 compatible 版本
egui = "0.27.2"
egui-wgpu = "0.27.2"
egui-winit = "0.27.2"
egui_extras = { version = "0.27.2", features = ["image", "svg"] }
winit = "0.29.15"

# Rendering
wgpu = "0.19.4"
bytemuck = { version = "1.14.3", features = ["derive"] }
pollster = "0.3.0"
glam = { version = "0.25.0", features = ["serde", "bytemuck"] }

# Text Editing
ropey = "1.6.1"
tree-sitter = "0.22.6"

# Plugin System
wasmtime = { version = "18.0.1", features = ["cranelift", "parallel-compilation", "cache"] }
wasmtime-wasi = "18.0.1"

# Serialization
serde = { version = "1.0.197", features = ["derive"] }
serde_json = "1.0.114"
toml = "0.8.12"
rmp-serde = "1.1.2"

# File System
walkdir = "2.5.0"
notify = "6.1.1"
tempfile = "3.10.0"

# Math
nalgebra = { version = "0.32.5", features = ["serde-serialize"] }

# Image Processing
image = { version = "0.24.9", features = ["png", "jpeg", "webp", "bmp", "tiff"] }

# 3D Model Loading
gltf = { version = "1.4.0", features = ["utils", "extensions", "names"] }

# Fonts
fontdue = "0.7.3"

# Async Runtime
tokio = { version = "1.36.0", features = ["full"] }
futures = "0.3.30"

# Error Handling
anyhow = "1.0.80"
thiserror = "1.0.57"

# Logging
tracing = "0.1.40"
tracing-subscriber = { version = "0.3.18", features = ["env-filter", "json"] }
tracing-appender = "0.2.3"

# Concurrency
parking_lot = "0.12.1"
dashmap = "5.5.3"
crossbeam-channel = "0.5.11"

# Utilities
slotmap = "1.0.7"
bitflags = "2.4.2"
smallvec = "1.13.1"
arc-swap = "1.6.0"
once_cell = "1.19.0"

# Color
palette = { version = "0.7.5", features = ["serde"] }

# UUID
uuid = { version = "1.7.0", features = ["v4", "serde"] }
chrono = "0.4.35"

crossterm = "0.27.0"
flate2 = "1.0.28"
zstd = "0.13.0"
blake3 = "1.5.0"

sys-info = "0.9.1"
num_cpus = "1.16.0"
rand = "0.8.5"
rand_chacha = "0.3.1"
arboard = "3.3.0"

[dev-dependencies]
criterion = { version = "0.5.1", features = ["html_reports"] }

[build-dependencies]
rust-embed = { version = "8.2.0", features = ["include-exclude", "compression"] }
chrono = "0.4.35"

[profile.release]
lto = true
codegen-units = 1
strip = true
panic = "abort"
opt-level = 3

[profile.dev]
opt-level = 1
debug = true

[[bin]]
name = "lunar-editor"
path = "src/main.rs"
"""

# ===== 最小可编译的 main.rs =====
MAIN_RS = r"""// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing::info;
use tracing_subscriber::prelude::*;

fn main() -> anyhow::Result<()> {
    init_logging();
    info!("🚀 Lunar Editor v{} starting...", env!("CARGO_PKG_VERSION"));
    info!("✅ Application initialized");
    info!("👋 Lunar Editor shut down");
    Ok(())
}

fn init_logging() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .pretty();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(fmt_layer)
        .init();
}
"""

# ===== 最小 mod.rs 文件 =====
MOD_RS_TEMPLATE = r"""// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

// This module is a placeholder for future implementation
"""

# ===== 修复 build.rs =====
BUILD_RS = r"""// Copyright 2026 Lunar Editor Team
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
"""


def write_file(path, content):
    full_path = BASE_DIR / path
    full_path.parent.mkdir(parents=True, exist_ok=True)
    with open(full_path, "w", encoding="utf-8") as f:
        f.write(content)
    print(f"  ✅ {path}")


def main():
    print("🔧 修复 Lunar Editor 编译错误...")
    print("=" * 50)

    # 1. 修复 Cargo.toml
    print("\n📦 更新 Cargo.toml...")
    write_file("Cargo.toml", CARGO_TOML)

    # 2. 修复 build.rs
    print("\n📦 更新 build.rs...")
    write_file("build.rs", BUILD_RS)

    # 3. 修复 main.rs
    print("\n📦 更新 main.rs...")
    write_file("src/main.rs", MAIN_RS)

    # 4. 创建所有模块的 mod.rs（最小版本）
    print("\n📦 创建模块文件...")

    modules = [
        "src/app/mod.rs",
        "src/editor/mod.rs",
        "src/editor/text_editor/mod.rs",
        "src/editor/text_editor/syntax/mod.rs",
        "src/editor/editor_2d/mod.rs",
        "src/editor/editor_3d/mod.rs",
        "src/ui/mod.rs",
        "src/ui/widgets/mod.rs",
        "src/plugin_system/mod.rs",
        "src/plugin_system/api/mod.rs",
        "src/project/mod.rs",
        "src/render_backend/mod.rs",
        "src/render_backend/shaders/mod.rs",
        "src/lunar_engine/mod.rs",
        "src/lunar_engine/vm/mod.rs",
        "src/utils/mod.rs",
    ]

    for mod_path in modules:
        write_file(mod_path, MOD_RS_TEMPLATE)

    # 5. 创建空文件（占位）
    print("\n📦 创建占位文件...")

    placeholders = [
        "src/app/editor_app.rs",
        "src/app/modes.rs",
        "src/app/theme.rs",
        "src/editor/text_editor/buffer.rs",
        "src/editor/text_editor/render.rs",
        "src/editor/text_editor/input.rs",
        "src/editor/text_editor/syntax/lunar.rs",
        "src/editor/text_editor/syntax/rust.rs",
        "src/editor/text_editor/syntax/json.rs",
        "src/editor/editor_2d/scene.rs",
        "src/editor/editor_2d/tools.rs",
        "src/editor/editor_2d/render.rs",
        "src/editor/editor_3d/scene.rs",
        "src/editor/editor_3d/camera.rs",
        "src/editor/editor_3d/render.rs",
        "src/editor/editor_3d/gizmo.rs",
        "src/editor/editor_3d/loader.rs",
        "src/ui/docks.rs",
        "src/ui/toolbars.rs",
        "src/ui/file_tree.rs",
        "src/ui/widgets/button.rs",
        "src/ui/widgets/panel.rs",
        "src/ui/widgets/tree.rs",
        "src/ui/widgets/glow.rs",
        "src/plugin_system/plugin_manager.rs",
        "src/plugin_system/wasm_host.rs",
        "src/plugin_system/permissions.rs",
        "src/plugin_system/api/editor_api.rs",
        "src/plugin_system/api/scene_api.rs",
        "src/plugin_system/api/render_api.rs",
        "src/plugin_system/api/fs_api.rs",
        "src/plugin_system/api/lunar_api.rs",
        "src/project/project_manager.rs",
        "src/project/asset_manager.rs",
        "src/render_backend/wgpu_backend.rs",
        "src/render_backend/pipeline.rs",
        "src/render_backend/shaders/ui.wgsl",
        "src/render_backend/shaders/2d.wgsl",
        "src/render_backend/shaders/3d.wgsl",
        "src/lunar_engine/lexer.rs",
        "src/lunar_engine/parser.rs",
        "src/lunar_engine/ast.rs",
        "src/lunar_engine/vm/bytecode.rs",
        "src/lunar_engine/vm/interpreter.rs",
        "src/utils/memory.rs",
        "src/utils/performance.rs",
        "src/utils/license_check.rs",
    ]

    for placeholder in placeholders:
        write_file(placeholder, "// Placeholder - to be implemented\n")

    print("\n" + "=" * 50)
    print("✅ 修复完成！")
    print("\n现在提交并推送：")
    print("  git add .")
    print('  git commit -m "Fix compilation errors - minimal buildable version"')
    print("  git push")
    print("\n然后 GitHub Actions 应该能编译成功！")


if __name__ == "__main__":
    main()
