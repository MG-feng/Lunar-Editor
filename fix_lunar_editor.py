#!/usr/bin/env python3
"""
Lunar Editor 修复脚本
修复所有空文件和依赖问题
"""

import os
import sys
from pathlib import Path

# ==================== 文件内容定义 ====================

FILES_CONTENT = {
    # ===== 主入口 =====
    "src/main.rs": r'''// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::all, rust_2018_idioms)]

use std::sync::Arc;
use tracing::{info, error};
use tracing_subscriber::prelude::*;

mod app;
mod editor;
mod ui;
mod plugin_system;
mod project;
mod render_backend;
mod lunar_engine;
mod utils;

use app::EditorApp;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging();

    info!("🚀 Lunar Editor v{} starting...", env!("CARGO_PKG_VERSION"));

    if let Err(e) = check_system_requirements() {
        error!("❌ System requirements check failed: {}", e);
        return Err(e);
    }

    let app = EditorApp::new().await?;

    info!("✅ Application initialized successfully");
    if let Err(e) = app.run().await {
        error!("❌ Application error: {}", e);
        return Err(e);
    }

    info!("👋 Lunar Editor shut down successfully");
    Ok(())
}

fn init_logging() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .pretty();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();
}

fn check_system_requirements() -> anyhow::Result<()> {
    let cpu_count = num_cpus::get();
    info!("💻 System: {} cores", cpu_count);
    Ok(())
}
''',

    # ===== 编辑器模块 =====
    "src/editor/mod.rs": r'''// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

mod text_editor;
mod editor_2d;
mod editor_3d;

pub use text_editor::TextEditor;
pub use editor_2d::Editor2D;
pub use editor_3d::Editor3D;

use egui::Ui;

pub trait Editor: Send + Sync {
    fn update(&mut self, dt: f32);
    fn render(&mut self, ui: &mut Ui);
    fn memory_usage(&self) -> usize;
    fn name(&self) -> &'static str;
}

#[derive(Debug, Clone)]
pub struct EditorConfig {
    pub show_line_numbers: bool,
    pub show_whitespace: bool,
    pub wrap_lines: bool,
    pub font_size: f32,
    pub tab_size: usize,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            show_line_numbers: true,
            show_whitespace: false,
            wrap_lines: false,
            font_size: 14.0,
            tab_size: 4,
        }
    }
}
''',

    # ===== 文本编辑器 input.rs =====
    "src/editor/text_editor/input.rs": r'''// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use parking_lot::RwLock;
use std::sync::Arc;
use egui::{Key, Modifiers};
use tracing::debug;

use super::buffer::TextBuffer;

pub struct TextInputHandler {
    buffer: Arc<RwLock<TextBuffer>>,
    cursor_line: usize,
    cursor_col: usize,
    selection_start: Option<(usize, usize)>,
    selection_end: Option<(usize, usize)>,
    dirty: bool,
}

impl TextInputHandler {
    pub fn new(buffer: Arc<RwLock<TextBuffer>>) -> Self {
        Self {
            buffer,
            cursor_line: 0,
            cursor_col: 0,
            selection_start: None,
            selection_end: None,
            dirty: false,
        }
    }

    pub fn update(&mut self, _dt: f32) {
        self.dirty = false;
    }

    pub fn handle_key(&mut self, key: Key, _modifiers: Modifiers) -> bool {
        let mut buffer = self.buffer.write();

        match key {
            Key::Enter => {
                let pos = self.get_cursor_position(&buffer);
                buffer.insert_text(pos, "\n");
                self.dirty = true;
                self.cursor_line += 1;
                self.cursor_col = 0;
                return true;
            }
            Key::Backspace => {
                let pos = self.get_cursor_position(&buffer);
                if pos > 0 {
                    if self.cursor_col > 0 {
                        self.cursor_col -= 1;
                        buffer.delete_text(pos - 1..pos);
                    } else if self.cursor_line > 0 {
                        self.cursor_line -= 1;
                        let line_end = buffer.line_end(self.cursor_line) - buffer.line_start(self.cursor_line);
                        self.cursor_col = line_end;
                        buffer.delete_text(pos - 1..pos);
                    }
                    self.dirty = true;
                }
                return true;
            }
            Key::ArrowLeft => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                } else if self.cursor_line > 0 {
                    self.cursor_line -= 1;
                    let line_end = buffer.line_end(self.cursor_line) - buffer.line_start(self.cursor_line);
                    self.cursor_col = line_end;
                }
                self.selection_start = None;
                self.selection_end = None;
                return true;
            }
            Key::ArrowRight => {
                let line_end = buffer.line_end(self.cursor_line) - buffer.line_start(self.cursor_line);
                if self.cursor_col < line_end {
                    self.cursor_col += 1;
                } else if self.cursor_line + 1 < buffer.line_count() {
                    self.cursor_line += 1;
                    self.cursor_col = 0;
                }
                self.selection_start = None;
                self.selection_end = None;
                return true;
            }
            Key::ArrowUp => {
                if self.cursor_line > 0 {
                    self.cursor_line -= 1;
                    let line_end = buffer.line_end(self.cursor_line) - buffer.line_start(self.cursor_line);
                    self.cursor_col = self.cursor_col.min(line_end);
                }
                self.selection_start = None;
                self.selection_end = None;
                return true;
            }
            Key::ArrowDown => {
                if self.cursor_line + 1 < buffer.line_count() {
                    self.cursor_line += 1;
                    let line_end = buffer.line_end(self.cursor_line) - buffer.line_start(self.cursor_line);
                    self.cursor_col = self.cursor_col.min(line_end);
                }
                self.selection_start = None;
                self.selection_end = None;
                return true;
            }
            Key::Home => {
                self.cursor_col = 0;
                self.selection_start = None;
                self.selection_end = None;
                return true;
            }
            Key::End => {
                let line_end = buffer.line_end(self.cursor_line) - buffer.line_start(self.cursor_line);
                self.cursor_col = line_end;
                self.selection_start = None;
                self.selection_end = None;
                return true;
            }
            _ => false,
        }
    }

    pub fn handle_char(&mut self, ch: char) {
        if ch.is_control() {
            return;
        }

        let mut buffer = self.buffer.write();
        let pos = self.get_cursor_position(&buffer);
        buffer.insert_text(pos, &ch.to_string());
        self.dirty = true;
        self.cursor_col += 1;
    }

    fn get_cursor_position(&self, buffer: &TextBuffer) -> usize {
        let line_start = buffer.line_start(self.cursor_line);
        line_start + self.cursor_col
    }

    pub fn cursor_position(&self) -> (usize, usize) {
        (self.cursor_line, self.cursor_col)
    }

    pub fn selection(&self) -> Option<((usize, usize), (usize, usize))> {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            Some((start, end))
        } else {
            None
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}
''',

    # ===== 3D编辑器 gizmo.rs =====
    "src/editor/editor_3d/gizmo.rs": r'''// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use glam::{Vec3, Mat4};
use egui::{Ui, Painter, Color32, Stroke};

/// 3D Gizmo - 变换操作器
pub struct Gizmo3D {
    pub enabled: bool,
    pub mode: GizmoMode,
    pub snap: bool,
    pub snap_value: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GizmoMode {
    Translate,
    Rotate,
    Scale,
}

impl Gizmo3D {
    pub fn new() -> Self {
        Self {
            enabled: true,
            mode: GizmoMode::Translate,
            snap: false,
            snap_value: 0.1,
        }
    }

    pub fn update(&mut self, _dt: f32) {
        // 更新gizmo状态
    }

    pub fn render(&self, _ui: &mut Ui, _painter: &Painter, _camera_view: Mat4, _camera_proj: Mat4) {
        if !self.enabled {
            return;
        }

        // TODO: 实现Gizmo渲染
        // 这里使用简化版本，仅绘制坐标轴
    }

    pub fn set_mode(&mut self, mode: GizmoMode) {
        self.mode = mode;
    }

    pub fn toggle_snap(&mut self) {
        self.snap = !self.snap;
    }
}

impl Default for Gizmo3D {
    fn default() -> Self {
        Self::new()
    }
}
''',

    # ===== 3D编辑器 loader.rs =====
    "src/editor/editor_3d/loader.rs": r'''// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use anyhow::{Result, anyhow};
use std::path::Path;
use glam::{Vec3, Vec2};
use tracing::info;

use super::scene::{Scene3D, SceneObject3D, ObjectType3D, Vertex3D, Material, MeshData};

/// 模型加载器
pub struct ModelLoader {
    supported_formats: Vec<String>,
}

impl ModelLoader {
    pub fn new() -> Self {
        Self {
            supported_formats: vec!["gltf".to_string(), "glb".to_string(), "obj".to_string()],
        }
    }

    /// 加载模型
    pub fn load_model(&self, path: &str, scene: &mut Scene3D) -> Result<usize> {
        let path = Path::new(path);

        if !path.exists() {
            return Err(anyhow!("Model file not found: {}", path.display()));
        }

        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        match ext.to_lowercase().as_str() {
            "gltf" | "glb" => self.load_gltf(path, scene),
            "obj" => self.load_obj(path, scene),
            _ => Err(anyhow!("Unsupported format: {}", ext)),
        }
    }

    fn load_gltf(&self, path: &Path, scene: &mut Scene3D) -> Result<usize> {
        info!("Loading GLTF model: {}", path.display());

        // TODO: 实现完整的GLTF加载
        // 这里创建一个占位模型
        let vertices = vec![
            Vertex3D { position: [-0.5, -0.5, 0.0], normal: [0.0, 0.0, 1.0], uv: [0.0, 0.0], tangent: [1.0, 0.0, 0.0, 0.0] },
            Vertex3D { position: [0.5, -0.5, 0.0], normal: [0.0, 0.0, 1.0], uv: [1.0, 0.0], tangent: [1.0, 0.0, 0.0, 0.0] },
            Vertex3D { position: [0.0, 0.5, 0.0], normal: [0.0, 0.0, 1.0], uv: [0.5, 1.0], tangent: [1.0, 0.0, 0.0, 0.0] },
        ];
        let indices = vec![0, 1, 2];

        let material = Material {
            name: "Default".to_string(),
            albedo: Vec3::new(0.5, 0.5, 0.8),
            metallic: 0.0,
            roughness: 0.5,
            albedo_texture: None,
            normal_texture: None,
            metallic_roughness_texture: None,
        };

        let obj_type = ObjectType3D::Mesh {
            vertices,
            indices,
            material,
        };

        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Model".to_string());

        Ok(scene.create_object(obj_type, name))
    }

    fn load_obj(&self, path: &Path, scene: &mut Scene3D) -> Result<usize> {
        info!("Loading OBJ model: {}", path.display());
        // TODO: 实现OBJ加载
        // 暂时返回一个默认网格
        self.load_gltf(path, scene)
    }

    pub fn memory_usage(&self) -> usize {
        std::mem::size_of_val(self)
    }
}

impl Default for ModelLoader {
    fn default() -> Self {
        Self::new()
    }
}
''',

    # ===== Lunar引擎文件 =====
    "src/lunar_engine/lexer.rs": r'''// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

/// Lunar词法分析器 - 接口占位
#[derive(Debug, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(String),
    String(String),
    Symbol(char),
    Comment(String),
    Whitespace,
    Unknown,
}

pub struct Lexer {
    source: String,
    position: usize,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self, source: &str) -> Vec<Token> {
        self.source = source.to_string();
        self.position = 0;
        let mut tokens = Vec::new();

        while self.position < self.source.len() {
            let ch = self.source.chars().nth(self.position).unwrap_or(' ');

            if ch.is_whitespace() {
                self.position += 1;
                continue;
            }

            if ch.is_alphabetic() || ch == '_' {
                let start = self.position;
                while self.position < self.source.len() {
                    let c = self.source.chars().nth(self.position).unwrap_or(' ');
                    if c.is_alphanumeric() || c == '_' {
                        self.position += 1;
                    } else {
                        break;
                    }
                }
                let word = &self.source[start..self.position];
                tokens.push(Token::Identifier(word.to_string()));
                continue;
            }

            if ch.is_ascii_digit() {
                let start = self.position;
                while self.position < self.source.len() {
                    let c = self.source.chars().nth(self.position).unwrap_or(' ');
                    if c.is_ascii_digit() || c == '.' {
                        self.position += 1;
                    } else {
                        break;
                    }
                }
                let num = &self.source[start..self.position];
                tokens.push(Token::Number(num.to_string()));
                continue;
            }

            if ch == '"' || ch == '\'' {
                let quote = ch;
                self.position += 1;
                let start = self.position;
                while self.position < self.source.len() {
                    let c = self.source.chars().nth(self.position).unwrap_or(' ');
                    if c == quote {
                        break;
                    }
                    self.position += 1;
                }
                let string = &self.source[start..self.position];
                tokens.push(Token::String(string.to_string()));
                self.position += 1;
                continue;
            }

            // 处理注释
            if ch == '/' && self.position + 1 < self.source.len() {
                let next = self.source.chars().nth(self.position + 1).unwrap_or(' ');
                if next == '/' {
                    let start = self.position;
                    while self.position < self.source.len() {
                        let c = self.source.chars().nth(self.position).unwrap_or(' ');
                        if c == '\n' || c == '\r' {
                            break;
                        }
                        self.position += 1;
                    }
                    let comment = &self.source[start..self.position];
                    tokens.push(Token::Comment(comment.to_string()));
                    continue;
                }
            }

            // 单个符号
            tokens.push(Token::Symbol(ch));
            self.position += 1;
        }

        tokens
    }
}
''',

    "src/lunar_engine/rust.rs": r'''// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use egui::Color32;

/// Rust语法高亮器
pub struct RustHighlighter {
    keywords: Vec<String>,
}

impl RustHighlighter {
    pub fn new() -> Self {
        let keywords = vec![
            "as".to_string(), "break".to_string(), "const".to_string(),
            "continue".to_string(), "crate".to_string(), "else".to_string(),
            "enum".to_string(), "extern".to_string(), "false".to_string(),
            "fn".to_string(), "for".to_string(), "if".to_string(),
            "impl".to_string(), "in".to_string(), "let".to_string(),
            "loop".to_string(), "match".to_string(), "mod".to_string(),
            "move".to_string(), "mut".to_string(), "pub".to_string(),
            "ref".to_string(), "return".to_string(), "self".to_string(),
            "static".to_string(), "struct".to_string(), "super".to_string(),
            "trait".to_string(), "true".to_string(), "type".to_string(),
            "unsafe".to_string(), "use".to_string(), "where".to_string(),
            "while".to_string(),
        ];

        Self { keywords }
    }

    pub fn highlight(&self, text: &str) -> Vec<(usize, Color32)> {
        let mut result = Vec::new();
        let mut pos = 0;
        let mut current_word = String::new();

        for ch in text.chars() {
            if ch.is_alphabetic() || ch == '_' {
                current_word.push(ch);
            } else {
                if !current_word.is_empty() {
                    if self.keywords.contains(&current_word) {
                        result.push((pos - current_word.len(), Color32::from_rgb(200, 120, 255)));
                    }
                    current_word.clear();
                }
                pos += 1;
            }
            pos += 1;
        }

        result
    }
}
''',

    "src/editor/text_editor/syntax/json.rs": r'''// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use egui::Color32;
use super::SyntaxHighlighter;

pub struct JsonHighlighter;

impl JsonHighlighter {
    pub fn new() -> Self {
        Self
    }
}

impl SyntaxHighlighter for JsonHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, Color32)> {
        let mut result = Vec::new();
        let mut pos = 0;
        let mut in_string = false;
        let mut escaped = false;

        for (i, ch) in text.chars().enumerate() {
            if ch == '"' && !escaped {
                in_string = !in_string;
                if in_string {
                    result.push((i, Color32::from_rgb(100, 255, 100)));
                }
            } else if in_string {
                // 字符串内容
            } else if ch == '{' || ch == '}' {
                result.push((i, Color32::from_rgb(255, 200, 100)));
            } else if ch == '[' || ch == ']' {
                result.push((i, Color32::from_rgb(255, 200, 100)));
            } else if ch == ':' {
                result.push((i, Color32::from_rgb(255, 255, 255)));
            } else if ch == ',' {
                result.push((i, Color32::from_rgb(150, 150, 150)));
            }

            if ch == '\\' && in_string {
                escaped = !escaped;
            } else {
                escaped = false;
            }
            pos += 1;
        }

        result
    }

    fn name(&self) -> &'static str {
        "JSON"
    }
}
''',

    "src/editor/text_editor/syntax/rust.rs": r'''// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use egui::Color32;
use super::SyntaxHighlighter;

pub struct RustHighlighter {
    keywords: Vec<String>,
}

impl RustHighlighter {
    pub fn new() -> Self {
        let keywords = vec![
            "as".to_string(), "break".to_string(), "const".to_string(),
            "continue".to_string(), "crate".to_string(), "else".to_string(),
            "enum".to_string(), "extern".to_string(), "false".to_string(),
            "fn".to_string(), "for".to_string(), "if".to_string(),
            "impl".to_string(), "in".to_string(), "let".to_string(),
            "loop".to_string(), "match".to_string(), "mod".to_string(),
            "move".to_string(), "mut".to_string(), "pub".to_string(),
            "ref".to_string(), "return".to_string(), "self".to_string(),
            "static".to_string(), "struct".to_string(), "super".to_string(),
            "trait".to_string(), "true".to_string(), "type".to_string(),
            "unsafe".to_string(), "use".to_string(), "where".to_string(),
            "while".to_string(),
        ];
        Self { keywords }
    }
}

impl SyntaxHighlighter for RustHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, Color32)> {
        let mut result = Vec::new();
        let mut current_word = String::new();
        let mut word_start = 0;

        for (i, ch) in text.chars().enumerate() {
            if ch.is_alphanumeric() || ch == '_' {
                if current_word.is_empty() {
                    word_start = i;
                }
                current_word.push(ch);
            } else {
                if !current_word.is_empty() {
                    if self.keywords.contains(&current_word) {
                        result.push((word_start, Color32::from_rgb(200, 120, 255)));
                    }
                    current_word.clear();
                }
            }
        }

        result
    }

    fn name(&self) -> &'static str {
        "Rust"
    }
}
''',

    # ===== Shader文件 =====
    "src/render_backend/shaders/ui.wgsl": r'''@group(0) @binding(0)
var<uniform> u_transform: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coord: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = u_transform * vec4<f32>(input.position, 0.0, 1.0);
    output.tex_coord = input.tex_coord;
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
''',

    "src/render_backend/shaders/2d.wgsl": r'''@group(0) @binding(0)
var<uniform> u_transform: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coord: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = u_transform * vec4<f32>(input.position, 0.0, 1.0);
    output.tex_coord = input.tex_coord;
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
''',

    "src/render_backend/shaders/3d.wgsl": r'''@group(0) @binding(0)
var<uniform> u_view_proj: mat4x4<f32>;
@group(0) @binding(1)
var<uniform> u_model: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    let world_pos = u_model * vec4<f32>(input.position, 1.0);
    output.position = u_view_proj * world_pos;
    output.normal = (u_model * vec4<f32>(input.normal, 0.0)).xyz;
    output.uv = input.uv;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let light_dir = normalize(vec3<f32>(1.0, 2.0, 1.0));
    let diffuse = max(dot(input.normal, light_dir), 0.0);
    return vec4<f32>(0.5 + diffuse * 0.5, 0.5, 0.8, 1.0);
}
''',
}

# ==================== 执行修复 ====================

def main():
    base_dir = Path(".")

    print("🔧 Lunar Editor 修复工具")
    print("=" * 50)

    # 1. 修复 Cargo.toml
    print("\n📦 修复 Cargo.toml...")
    cargo_content = r'''[package]
name = "lunar-editor"
version = "0.1.0"
edition = "2021"
license = "Apache-2.0"
authors = ["Lunar Editor Team"]
description = "A high-performance code editor with 2D/3D visualization"
repository = "https://github.com/MG-feng/Lunar-Editor"
build = "build.rs"

[dependencies]
egui = "0.27.2"
egui-wgpu = "0.27.2"
egui-winit = "0.27.2"
egui_extras = { version = "0.27.2", features = ["image", "svg"] }
winit = "0.30.5"

wgpu = "0.19.4"
wgpu-hal = "0.19.4"
bytemuck = { version = "1.14.3", features = ["derive"] }
pollster = "0.3.0"
glam = { version = "0.25.0", features = ["serde", "bytemuck"] }

ropey = "1.6.1"
tree-sitter = "0.22.6"

wasmtime = { version = "18.0.1", features = ["cranelift", "parallel-compilation", "cache"] }
wasmtime-wasi = "18.0.1"

serde = { version = "1.0.197", features = ["derive"] }
serde_json = "1.0.114"
toml = "0.8.12"
rmp-serde = "1.1.2"

walkdir = "2.5.0"
notify = "6.1.1"
tempfile = "3.10.0"

nalgebra = { version = "0.32.5", features = ["serde-serialize"] }

image = { version = "0.24.9", features = ["png", "jpeg", "webp", "bmp", "tiff"] }

gltf = { version = "1.4.0", features = ["utils", "extensions", "names"] }

# fontdue - 修复：移除 layout feature，使用默认配置
fontdue = "0.7.3"

tokio = { version = "1.36.0", features = ["full"] }
futures = "0.3.30"

anyhow = "1.0.80"
thiserror = "1.0.57"

tracing = "0.1.40"
tracing-subscriber = { version = "0.3.18", features = ["env-filter", "json"] }
tracing-appender = "0.2.3"

parking_lot = "0.12.1"
dashmap = "5.5.3"
crossbeam-channel = "0.5.11"

slotmap = "1.0.7"
bitflags = "2.4.2"
smallvec = "1.13.1"
arc-swap = "1.6.0"
once_cell = "1.19.0"

palette = { version = "0.7.5", features = ["serde"] }

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
overflow-checks = false

[profile.dev]
opt-level = 1
debug = true

[[bin]]
name = "lunar-editor"
path = "src/main.rs"
'''

    with open(base_dir / "Cargo.toml", "w", encoding="utf-8") as f:
        f.write(cargo_content)
    print("   ✅ Cargo.toml 已更新")

    # 2. 修复所有空文件
    print("\n📝 填充空文件...")

    for file_path, content in FILES_CONTENT.items():
        full_path = base_dir / file_path
        try:
            with open(full_path, "w", encoding="utf-8") as f:
                f.write(content)
            print(f"   ✅ {file_path}")
        except Exception as e:
            print(f"   ❌ {file_path}: {e}")

    # 3. 创建缺失的 .gitkeep 文件
    print("\n📁 确保目录结构...")
    dirs_to_create = [
        "assets/fonts",
        "assets/icons",
        "assets/themes",
        "plugins",
        "project",
        "scripts",
    ]

    for dir_path in dirs_to_create:
        full_path = base_dir / dir_path
        full_path.mkdir(parents=True, exist_ok=True)
        gitkeep = full_path / ".gitkeep"
        if not gitkeep.exists():
            gitkeep.touch()
        print(f"   ✅ {dir_path}/")

    print("\n" + "=" * 50)
    print("✅ 修复完成！")
    print("\n现在可以运行:")
    print("  cargo clean")
    print("  cargo build")
    print("\n如果还有错误，请将错误信息发给我。")

if __name__ == "__main__":
    main()
