// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use ropey::Rope;
use std::ops::Range;
use parking_lot::RwLock;
use std::sync::Arc;
use tracing::debug;

/// 文本缓冲区 - 基于Rope数据结构
pub struct TextBuffer {
    rope: Rope,
    version: u64,
    undo_stack: Vec<UndoAction>,
    redo_stack: Vec<UndoAction>,
    max_undo: usize,
}

#[derive(Clone)]
struct UndoAction {
    kind: UndoKind,
    position: usize,
    text: String,
    version: u64,
}

#[derive(Clone, Copy)]
enum UndoKind {
    Insert,
    Delete,
    Replace,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            version: 0,
            undo_stack: Vec::with_capacity(100),
            redo_stack: Vec::with_capacity(100),
            max_undo: 100,
        }
    }

    /// 获取文本
    pub fn get_text(&self) -> String {
        self.rope.to_string()
    }

    /// 设置文本
    pub fn set_text(&mut self, text: &str) {
        self.rope = Rope::from_str(text);
        self.version += 1;
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// 插入文本
    pub fn insert_text(&mut self, position: usize, text: &str) {
        let old_text = self.get_text();
        let inserted = text.to_string();

        // 执行插入
        let byte_index = self.rope.char_to_byte(position);
        self.rope.insert(byte_index, text);
        self.version += 1;

        // 记录撤销操作
        if !text.is_empty() {
            let action = UndoAction {
                kind: UndoKind::Insert,
                position,
                text: inserted,
                version: self.version,
            };
            self.push_undo(action);
        }
    }

    /// 删除文本
    pub fn delete_text(&mut self, range: Range<usize>) -> String {
        if range.start >= range.end || range.end > self.rope.len_chars() {
            return String::new();
        }

        let start_byte = self.rope.char_to_byte(range.start);
        let end_byte = self.rope.char_to_byte(range.end);

        // 获取要删除的文本
        let deleted = self.rope.slice(start_byte..end_byte).to_string();

        // 执行删除
        self.rope.remove(start_byte..end_byte);
        self.version += 1;

        // 记录撤销操作
        if !deleted.is_empty() {
            let action = UndoAction {
                kind: UndoKind::Delete,
                position: range.start,
                text: deleted,
                version: self.version,
            };
            self.push_undo(action);
        }

        deleted
    }

    /// 撤销
    pub fn undo(&mut self) -> bool {
        if let Some(action) = self.undo_stack.pop() {
            self.redo_stack.push(action.clone());

            match action.kind {
                UndoKind::Insert => {
                    // 删除插入的文本
                    let start_byte = self.rope.char_to_byte(action.position);
                    let end_byte = self.rope.char_to_byte(action.position + action.text.len());
                    self.rope.remove(start_byte..end_byte);
                }
                UndoKind::Delete => {
                    // 重新插入删除的文本
                    let byte_index = self.rope.char_to_byte(action.position);
                    self.rope.insert(byte_index, &action.text);
                }
                UndoKind::Replace => {
                    // 替换
                    let start_byte = self.rope.char_to_byte(action.position);
                    let end_byte = self.rope.char_to_byte(action.position + action.text.len());
                    self.rope.remove(start_byte..end_byte);
                    self.rope.insert(start_byte, &action.text);
                }
            }

            self.version += 1;
            debug!("Undo: {}", action.text.len());
            true
        } else {
            false
        }
    }

    /// 重做
    pub fn redo(&mut self) -> bool {
        if let Some(action) = self.redo_stack.pop() {
            self.undo_stack.push(action.clone());

            match action.kind {
                UndoKind::Insert => {
                    let byte_index = self.rope.char_to_byte(action.position);
                    self.rope.insert(byte_index, &action.text);
                }
                UndoKind::Delete => {
                    let start_byte = self.rope.char_to_byte(action.position);
                    let end_byte = self.rope.char_to_byte(action.position + action.text.len());
                    self.rope.remove(start_byte..end_byte);
                }
                UndoKind::Replace => {
                    let start_byte = self.rope.char_to_byte(action.position);
                    let end_byte = self.rope.char_to_byte(action.position + action.text.len());
                    self.rope.remove(start_byte..end_byte);
                    self.rope.insert(start_byte, &action.text);
                }
            }

            self.version += 1;
            debug!("Redo: {}", action.text.len());
            true
        } else {
            false
        }
    }

    /// 获取字符
    pub fn char_at(&self, index: usize) -> Option<char> {
        if index < self.rope.len_chars() {
            Some(self.rope.char(index))
        } else {
            None
        }
    }

    /// 获取行数
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    /// 获取指定行的文本
    pub fn get_line(&self, line: usize) -> String {
        if line < self.rope.len_lines() {
            let line_start = self.rope.line_to_char(line);
            let line_end = if line + 1 < self.rope.len_lines() {
                self.rope.line_to_char(line + 1)
            } else {
                self.rope.len_chars()
            };
            self.rope.slice(line_start..line_end).to_string()
        } else {
            String::new()
        }
    }

    /// 获取行的起始字符索引
    pub fn line_start(&self, line: usize) -> usize {
        if line < self.rope.len_lines() {
            self.rope.line_to_char(line)
        } else {
            self.rope.len_chars()
        }
    }

    /// 获取行的结束字符索引
    pub fn line_end(&self, line: usize) -> usize {
        if line < self.rope.len_lines() {
            let end = if line + 1 < self.rope.len_lines() {
                self.rope.line_to_char(line + 1)
            } else {
                self.rope.len_chars()
            };
            // 移除换行符
            let line_text = self.rope.slice(self.rope.line_to_char(line)..end).to_string();
            let trimmed = line_text.trim_end_matches(&['\n', '\r'] as &[_]);
            let chars = trimmed.chars().count();
            self.rope.line_to_char(line) + chars
        } else {
            self.rope.len_chars()
        }
    }

    /// 获取总字符数
    pub fn char_count(&self) -> usize {
        self.rope.len_chars()
    }

    /// 获取字节数
    pub fn byte_count(&self) -> usize {
        self.rope.len_bytes()
    }

    /// 获取内存使用
    pub fn memory_usage(&self) -> usize {
        self.rope.len_bytes() +
        self.undo_stack.capacity() * std::mem::size_of::<UndoAction>() +
        self.redo_stack.capacity() * std::mem::size_of::<UndoAction>()
    }

    /// 清空
    pub fn clear(&mut self) {
        self.rope = Rope::new();
        self.version += 1;
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.rope.is_empty()
    }

    fn push_undo(&mut self, action: UndoAction) {
        if self.undo_stack.len() >= self.max_undo {
            self.undo_stack.remove(0);
        }
        self.undo_stack.push(action);
        self.redo_stack.clear();
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}
