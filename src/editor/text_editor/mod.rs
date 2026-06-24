// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

mod buffer;
mod render;
mod input;
mod syntax;

pub use buffer::TextBuffer;
pub use render::TextRenderer;
pub use input::TextInputHandler;
pub use syntax::SyntaxHighlighter;

use egui::Ui;
use super::{Editor, EditorConfig};
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::debug;

/// 文本编辑器
pub struct TextEditor {
    buffer: Arc<RwLock<TextBuffer>>,
    renderer: TextRenderer,
    input_handler: TextInputHandler,
    highlighter: SyntaxHighlighter,
    config: EditorConfig,
    cursor_position: (usize, usize),
    selection: Option<((usize, usize), (usize, usize))>,
    dirty: bool,
    file_path: Option<String>,
}

impl TextEditor {
    pub fn new() -> Self {
        debug!("Creating TextEditor");

        let buffer = Arc::new(RwLock::new(TextBuffer::new()));
        let highlighter = SyntaxHighlighter::new();

        Self {
            buffer: buffer.clone(),
            renderer: TextRenderer::new(buffer.clone()),
            input_handler: TextInputHandler::new(buffer.clone()),
            highlighter,
            config: EditorConfig::default(),
            cursor_position: (0, 0),
            selection: None,
            dirty: false,
            file_path: None,
        }
    }

    /// 打开文件
    pub fn open_file(&mut self, path: &str) -> anyhow::Result<()> {
        let content = std::fs::read_to_string(path)?;
        self.buffer.write().set_text(&content);
        self.file_path = Some(path.to_string());
        self.dirty = false;
        self.cursor_position = (0, 0);
        debug!("Opened file: {}", path);
        Ok(())
    }

    /// 保存文件
    pub fn save_file(&mut self) -> anyhow::Result<()> {
        if let Some(path) = &self.file_path {
            let content = self.buffer.read().get_text();
            std::fs::write(path, &content)?;
            self.dirty = false;
            debug!("Saved file: {}", path);
            Ok(())
        } else {
            Err(anyhow::anyhow!("No file path set"))
        }
    }

    /// 保存为
    pub fn save_as(&mut self, path: &str) -> anyhow::Result<()> {
        let content = self.buffer.read().get_text();
        std::fs::write(path, &content)?;
        self.file_path = Some(path.to_string());
        self.dirty = false;
        debug!("Saved as: {}", path);
        Ok(())
    }
}

impl Editor for TextEditor {
    fn update(&mut self, dt: f32) {
        self.input_handler.update(dt);
        self.cursor_position = self.input_handler.cursor_position();
        self.selection = self.input_handler.selection();

        // 标记为脏
        if self.input_handler.is_dirty() {
            self.dirty = true;
        }
    }

    fn render(&mut self, ui: &mut Ui) {
        // 使用滚动区域
        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                // 渲染行号
                if self.config.show_line_numbers {
                    self.renderer.render_line_numbers(ui);
                    ui.separator();
                }

                // 渲染文本
                self.renderer.render(
                    ui,
                    &self.highlighter,
                    self.cursor_position,
                    self.selection,
                );
            });

        // 状态栏
        ui.horizontal(|ui| {
            ui.label(format!("Line: {}", self.cursor_position.0 + 1));
            ui.separator();
            ui.label(format!("Col: {}", self.cursor_position.1 + 1));
            ui.separator();
            ui.label(format!("Lines: {}", self.buffer.read().line_count()));
            ui.separator();
            if self.dirty {
                ui.label("🔴 Modified");
            } else {
                ui.label("✅ Saved");
            }
            if let Some(path) = &self.file_path {
                ui.separator();
                ui.label(format!("📁 {}", path));
            }
        });
    }

    fn memory_usage(&self) -> usize {
        self.buffer.read().memory_usage()
    }

    fn name(&self) -> &'static str {
        "Text Editor"
    }
}

impl Default for TextEditor {
    fn default() -> Self {
        Self::new()
    }
}
