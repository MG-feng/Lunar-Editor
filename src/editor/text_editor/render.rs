// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use egui::{Ui, Color32, FontId, RichText, TextStyle, FontFamily};
use parking_lot::RwLock;
use std::sync::Arc;
use ropey::Rope;

use super::buffer::TextBuffer;
use super::syntax::SyntaxHighlighter;

/// 文本渲染器
pub struct TextRenderer {
    buffer: Arc<RwLock<TextBuffer>>,
    font_size: f32,
    line_height: f32,
}

impl TextRenderer {
    pub fn new(buffer: Arc<RwLock<TextBuffer>>) -> Self {
        Self {
            buffer,
            font_size: 14.0,
            line_height: 20.0,
        }
    }

    /// 渲染行号
    pub fn render_line_numbers(&mut self, ui: &mut Ui) {
        let buffer = self.buffer.read();
        let line_count = buffer.line_count();

        ui.vertical(|ui| {
            for i in 0..line_count {
                let line_num = i + 1;
                ui.label(RichText::new(format!("{:4}", line_num))
                    .color(Color32::from_rgb(80, 90, 110))
                    .size(self.font_size));
            }
        });
    }

    /// 渲染文本
    pub fn render(
        &mut self,
        ui: &mut Ui,
        highlighter: &Box<dyn SyntaxHighlighter>,
        cursor_pos: (usize, usize),
        selection: Option<((usize, usize), (usize, usize))>,
    ) {
        let buffer = self.buffer.read();
        let line_count = buffer.line_count();

        ui.vertical(|ui| {
            for line_idx in 0..line_count {
                let line_text = buffer.get_line(line_idx);

                // 获取高亮
                let highlights = highlighter.highlight(&line_text);

                // 渲染行
                ui.horizontal(|ui| {
                    let mut last_pos = 0;
                    for (pos, color) in highlights {
                        // 渲染未高亮部分
                        if last_pos < pos {
                            let text = &line_text[last_pos..pos];
                            ui.label(RichText::new(text)
                                .color(Color32::from_rgb(220, 220, 230))
                                .size(self.font_size));
                        }
                        // 渲染高亮部分
                        let text = &line_text[pos..pos + 1];
                        ui.label(RichText::new(text)
                            .color(color)
                            .size(self.font_size));
                        last_pos = pos + 1;
                    }
                    // 渲染剩余文本
                    if last_pos < line_text.len() {
                        let text = &line_text[last_pos..];
                        ui.label(RichText::new(text)
                            .color(Color32::from_rgb(220, 220, 230))
                            .size(self.font_size));
                    }
                });

                // 渲染光标
                if line_idx == cursor_pos.0 {
                    self.render_cursor(ui, cursor_pos.1, &line_text);
                }
            }
        });
    }

    /// 渲染光标
    fn render_cursor(&self, ui: &mut Ui, col: usize, line_text: &str) {
        let char_count = line_text.chars().count();
        let col_pos = col.min(char_count);

        // 计算光标位置
        let mut x_pos = 0.0;
        for (i, ch) in line_text.chars().enumerate() {
            if i >= col_pos {
                break;
            }
            // 简单估算字符宽度
            if ch.is_ascii() {
                x_pos += self.font_size * 0.6;
            } else {
                x_pos += self.font_size * 1.0;
            }
        }

        // 绘制光标
        let cursor_rect = ui.available_rect_before_wrap()
            .with_x(ui.cursor().min.x + x_pos)
            .with_width(2.0)
            .with_height(self.line_height);

        ui.painter().rect_filled(
            cursor_rect,
            0.0,
            Color32::from_rgb(100, 200, 255),
        );

        // 光标闪烁效果 (使用当前时间)
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        if (now / 500) % 2 == 0 {
            // 光标可见
        }
    }

    /// 设置字体大小
    pub fn set_font_size(&mut self, size: f32) {
        self.font_size = size;
        self.line_height = size * 1.4;
    }
}

### 23. src/editor/text_editor/input.rs

```rust
// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use parking_lot::RwLock;
use std::sync::Arc;
use egui::{Key, KeyboardShortcut, Modifiers};
use tracing::debug;

use super::buffer::TextBuffer;

/// 文本输入处理器
pub struct TextInputHandler {
    buffer: Arc<RwLock<TextBuffer>>,
    cursor_line: usize,
    cursor_col: usize,
    selection_start: Option<(usize, usize)>,
    selection_end: Option<(usize, usize)>,
    dirty: bool,
    insert_mode: bool,
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
            insert_mode: true,
        }
    }

    /// 更新输入处理
    pub fn update(&mut self, _dt: f32) {
        // 实际输入处理由egui的事件系统完成
        // 这里只是更新状态
        self.dirty = false;
    }

    /// 处理键盘输入
    pub fn handle_key(&mut self, key: Key, modifiers: Modifiers) -> bool {
        let mut buffer = self.buffer.write();
        let char_count = buffer.char_count();
        let line_count = buffer.line_count();

        match key {
            Key::Enter => {
                self.insert_newline(&mut buffer);
                return true;
            }
            Key::Backspace => {
                self.delete_backward(&mut buffer);
                return true;
            }
            Key::Delete => {
                self.delete_forward(&mut buffer);
                return true;
            }
            Key::ArrowLeft => {
                self.move_cursor_left(&mut buffer);
                return true;
            }
            Key::ArrowRight => {
                self.move_cursor_right(&mut buffer);
                return true;
            }
            Key::ArrowUp => {
                self.move_cursor_up(&mut buffer);
                return true;
            }
            Key::ArrowDown => {
                self.move_cursor_down(&mut buffer);
                return true;
            }
            Key::Home => {
                self.move_cursor_home(&mut buffer);
                return true;
            }
            Key::End => {
                self.move_cursor_end(&mut buffer);
                return true;
            }
            Key::A if modifiers.ctrl => {
                self.select_all(&mut buffer);
                return true;
            }
            Key::C if modifiers.ctrl => {
                self.copy_selection(&mut buffer);
                return true;
            }
            Key::V if modifiers.ctrl => {
                self.paste_text(&mut buffer);
                return true;
            }
            Key::X if modifiers.ctrl => {
                self.cut_selection(&mut buffer);
                return true;
            }
            Key::Z if modifiers.ctrl => {
                self.undo(&mut buffer);
                return true;
            }
            Key::Y if modifiers.ctrl => {
                self.redo(&mut buffer);
                return true;
            }
            _ => false,
        }
    }

    /// 处理字符输入
    pub fn handle_char(&mut self, ch: char) {
        if ch.is_control() {
            return;
        }

        let mut buffer = self.buffer.write();
        self.insert_char(&mut buffer, ch);
    }

    /// 插入字符
    fn insert_char(&mut self, buffer: &mut TextBuffer, ch: char) {
        let pos = self.get_cursor_position(buffer);
        let text = ch.to_string();
        buffer.insert_text(pos, &text);
        self.dirty = true;
        self.move_cursor_right(buffer);
    }

    /// 插入新行
    fn insert_newline(&mut self, buffer: &mut TextBuffer) {
        let pos = self.get_cursor_position(buffer);
        buffer.insert_text(pos, "\n");
        self.dirty = true;
        self.cursor_line += 1;
        self.cursor_col = 0;
    }

    /// 删除后退
    fn delete_backward(&mut self, buffer: &mut TextBuffer) {
        if let Some(selection) = self.get_selection_range(buffer) {
            buffer.delete_text(selection);
            self.clear_selection();
            return;
        }

        let pos = self.get_cursor_position(buffer);
        if pos > 0 {
            let start = if self.cursor_col > 0 {
                self.cursor_col -= 1;
                pos - 1
            } else if self.cursor_line > 0 {
                let prev_line_end = buffer.line_end(self.cursor_line - 1);
                self.cursor_line -= 1;
                self.cursor_col = buffer.line_end(self.cursor_line) - buffer.line_start(self.cursor_line);
                prev_line_end
            } else {
                return;
            };
            buffer.delete_text(start..pos);
            self.dirty = true;
        }
    }

    /// 删除向前
    fn delete_forward(&mut self, buffer: &mut TextBuffer) {
        if let Some(selection) = self.get_selection_range(buffer) {
            buffer.delete_text(selection);
            self.clear_selection();
            return;
        }

        let pos = self.get_cursor_position(buffer);
        if pos < buffer.char_count() {
            let end = if self.cursor_col < buffer.line_end(self.cursor_line) - buffer.line_start(self.cursor_line) {
                pos + 1
            } else if self.cursor_line + 1 < buffer.line_count() {
                pos + 1
            } else {
                return;
            };
            buffer.delete_text(pos..end);
            self.dirty = true;
        }
    }

    /// 移动光标左
    fn move_cursor_left(&mut self, buffer: &TextBuffer) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_line > 0 {
            self.cursor_line -= 1;
            self.cursor_col = buffer.line_end(self.cursor_line) - buffer.line_start(self.cursor_line);
        }
        self.clear_selection();
    }

    /// 移动光标右
    fn move_cursor_right(&mut self, buffer: &TextBuffer) {
        let line_end = buffer.line_end(self.cursor_line);
        let line_start = buffer.line_start(self.cursor_line);
        if self.cursor_col < line_end - line_start {
            self.cursor_col += 1;
        } else if self.cursor_line + 1 < buffer.line_count() {
            self.cursor_line += 1;
            self.cursor_col = 0;
        }
        self.clear_selection();
    }

    /// 移动光标上
    fn move_cursor_up(&mut self, buffer: &TextBuffer) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            let line_end = buffer.line_end(self.cursor_line) - buffer.line_start(self.cursor_line);
            self.cursor_col = self.cursor_col.min(line_end);
        }
        self.clear_selection();
    }

    /// 移动光标下
    fn move_cursor_down(&mut self, buffer: &TextBuffer) {
        if self.cursor_line + 1 < buffer.line_count() {
            self.cursor_line += 1;
            let line_end = buffer.line_end(self.cursor_line) - buffer.line_start(self.cursor_line);
            self.cursor_col = self.cursor_col.min(line_end);
        }
        self.clear_selection();
    }

    /// 移动光标到行首
    fn move_cursor_home(&mut self, _buffer: &TextBuffer) {
        self.cursor_col = 0;
        self.clear_selection();
    }

    /// 移动光标到行尾
    fn move_cursor_end(&mut self, buffer: &TextBuffer) {
        self.cursor_col = buffer.line_end(self.cursor_line) - buffer.line_start(self.cursor_line);
        self.clear_selection();
    }

    /// 全选
    fn select_all(&mut self, buffer: &TextBuffer) {
        self.selection_start = Some((0, 0));
        self.selection_end = Some((buffer.line_count() - 1,
            buffer.line_end(buffer.line_count() - 1) - buffer.line_start(buffer.line_count() - 1)));
    }

    /// 复制选中
    fn copy_selection(&mut self, buffer: &TextBuffer) {
        if let Some(range) = self.get_selection_range(buffer) {
            let text = buffer.get_text();
            let selected = &text[range.start..range.end];
            // 复制到剪贴板
            if let Err(e) = arboard::Clipboard::new().and_then(|mut clip| clip.set_text(selected)) {
                debug!("Failed to copy: {}", e);
            }
        }
    }

    /// 粘贴文本
    fn paste_text(&mut self, buffer: &mut TextBuffer) {
        if let Ok(mut clip) = arboard::Clipboard::new() {
            if let Ok(text) = clip.get_text() {
                let pos = self.get_cursor_position(buffer);
                buffer.insert_text(pos, &text);
                self.dirty = true;
            }
        }
    }

    /// 剪切选中
    fn cut_selection(&mut self, buffer: &mut TextBuffer) {
        if let Some(range) = self.get_selection_range(buffer) {
            let text = buffer.get_text();
            let selected = &text[range.start..range.end];
            // 复制到剪贴板
            if let Err(e) = arboard::Clipboard::new().and_then(|mut clip| clip.set_text(selected)) {
                debug!("Failed to cut: {}", e);
            }
            buffer.delete_text(range);
            self.clear_selection();
            self.dirty = true;
        }
    }

    /// 撤销
    fn undo(&mut self, buffer: &mut TextBuffer) {
        if buffer.undo() {
            self.dirty = true;
            self.cursor_line = 0;
            self.cursor_col = 0;
            self.clear_selection();
        }
    }

    /// 重做
    fn redo(&mut self, buffer: &mut TextBuffer) {
        if buffer.redo() {
            self.dirty = true;
        }
    }

    /// 获取光标位置
    fn get_cursor_position(&self, buffer: &TextBuffer) -> usize {
        let line_start = buffer.line_start(self.cursor_line);
        line_start + self.cursor_col
    }

    /// 获取选择范围
    fn get_selection_range(&self, buffer: &TextBuffer) -> Option<std::ops::Range<usize>> {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let start_pos = buffer.line_start(start.0) + start.1;
            let end_pos = buffer.line_start(end.0) + end.1;
            if start_pos != end_pos {
                return Some(start_pos.min(end_pos)..start_pos.max(end_pos));
            }
        }
        None
    }

    /// 清除选择
    fn clear_selection(&mut self) {
        self.selection_start = None;
        self.selection_end = None;
    }

    /// 获取光标位置
    pub fn cursor_position(&self) -> (usize, usize) {
        (self.cursor_line, self.cursor_col)
    }

    /// 获取选择
    pub fn selection(&self) -> Option<((usize, usize), (usize, usize))> {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            Some((start, end))
        } else {
            None
        }
    }

    /// 是否有修改
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}
