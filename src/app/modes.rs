// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use std::fmt;
use egui::Ui;
use tracing::debug;

use crate::editor::{TextEditor, Editor2D, Editor3D};

/// 编辑器模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    Text2D,
    Visual2D,
    Visual3D,
}

impl fmt::Display for EditorMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditorMode::Text2D => write!(f, "Text"),
            EditorMode::Visual2D => write!(f, "2D"),
            EditorMode::Visual3D => write!(f, "3D"),
        }
    }
}

/// 模式管理器
pub struct ModeManager {
    current_mode: EditorMode,
    text_editor: Option<TextEditor>,
    editor_2d: Option<Editor2D>,
    editor_3d: Option<Editor3D>,
}

impl ModeManager {
    pub fn new() -> Self {
        Self {
            current_mode: EditorMode::Text2D,
            text_editor: Some(TextEditor::new()),
            editor_2d: None,
            editor_3d: None,
        }
    }

    /// 切换模式
    pub fn switch(&mut self, mode: EditorMode) {
        if self.current_mode == mode {
            return;
        }

        debug!("Switching to mode: {:?}", mode);

        // 释放当前模式资源
        match self.current_mode {
            EditorMode::Text2D => self.text_editor = None,
            EditorMode::Visual2D => self.editor_2d = None,
            EditorMode::Visual3D => self.editor_3d = None,
        }

        // 初始化新模式
        match mode {
            EditorMode::Text2D => {
                if self.text_editor.is_none() {
                    self.text_editor = Some(TextEditor::new());
                }
            }
            EditorMode::Visual2D => {
                if self.editor_2d.is_none() {
                    self.editor_2d = Some(Editor2D::new());
                }
            }
            EditorMode::Visual3D => {
                if self.editor_3d.is_none() {
                    self.editor_3d = Some(Editor3D::new());
                }
            }
        }

        self.current_mode = mode;

        // 强制垃圾回收
        #[cfg(feature = "gc")]
        std::mem::take(&mut self);
    }

    /// 更新当前模式
    pub fn update(&mut self, dt: f32) {
        match self.current_mode {
            EditorMode::Text2D => {
                if let Some(editor) = &mut self.text_editor {
                    editor.update(dt);
                }
            }
            EditorMode::Visual2D => {
                if let Some(editor) = &mut self.editor_2d {
                    editor.update(dt);
                }
            }
            EditorMode::Visual3D => {
                if let Some(editor) = &mut self.editor_3d {
                    editor.update(dt);
                }
            }
        }
    }

    /// 渲染当前模式
    pub fn render(&mut self, ui: &mut Ui) {
        match self.current_mode {
            EditorMode::Text2D => {
                if let Some(editor) = &mut self.text_editor {
                    editor.render(ui);
                }
            }
            EditorMode::Visual2D => {
                if let Some(editor) = &mut self.editor_2d {
                    editor.render(ui);
                }
            }
            EditorMode::Visual3D => {
                if let Some(editor) = &mut self.editor_3d {
                    editor.render(ui);
                }
            }
        }
    }

    pub fn current_mode(&self) -> EditorMode {
        self.current_mode
    }
}
