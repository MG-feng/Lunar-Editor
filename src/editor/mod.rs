// Copyright 2026 Lunar Editor Team
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
