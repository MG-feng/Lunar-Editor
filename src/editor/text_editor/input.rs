// Copyright 2026 Lunar Editor Team
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
