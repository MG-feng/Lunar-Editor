// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use super::SyntaxHighlighter;
use egui::Color32;
use std::collections::HashSet;

/// Lunar语言语法高亮器
pub struct LunarHighlighter {
    keywords: HashSet<String>,
    builtins: HashSet<String>,
    types: HashSet<String>,
    decorators: HashSet<String>,
}

impl LunarHighlighter {
    pub fn new() -> Self {
        let mut keywords = HashSet::new();
        keywords.extend(vec![
            "if".to_string(), "elif".to_string(), "else".to_string(),
            "for".to_string(), "while".to_string(), "loop".to_string(),
            "func".to_string(), "local".to_string(), "global".to_string(),
            "class".to_string(), "enum".to_string(), "property".to_string(),
            "switch".to_string(), "match".to_string(), "super".to_string(),
            "const".to_string(), "return".to_string(), "break".to_string(),
            "continue".to_string(), "async".to_string(), "await".to_string(),
            "in".to_string(), "where".to_string(), "as".to_string(),
            "true".to_string(), "false".to_string(), "nil".to_string(),
        ]);

        let mut builtins = HashSet::new();
        builtins.extend(vec![
            "load".to_string(), "import".to_string(), "print".to_string(),
            "assert".to_string(), "panic".to_string(), "sleep".to_string(),
            "type".to_string(), "sizeof".to_string(), "alignof".to_string(),
        ]);

        let mut types = HashSet::new();
        types.extend(vec![
            "int".to_string(), "float".to_string(), "string".to_string(),
            "bool".to_string(), "array".to_string(), "map".to_string(),
            "any".to_string(), "void".to_string(), "auto".to_string(),
        ]);

        let mut decorators = HashSet::new();
        decorators.extend(vec![
            "gc".to_string(), "compile_mode".to_string(), "cpu_core".to_string(),
            "max_threads".to_string(), "render_backend".to_string(),
            "gpu_device".to_string(), "max_memory".to_string(),
            "log".to_string(), "measure".to_string(), "profile".to_string(),
        ]);

        Self {
            keywords,
            builtins,
            types,
            decorators,
        }
    }

    fn color_for_token(&self, token: &str, next_char: char) -> Option<Color32> {
        if token.starts_with('@') && self.decorators.contains(&token[1..]) {
            return Some(Color32::from_rgb(255, 200, 50)); // 金色
        }

        if self.keywords.contains(token) {
            return Some(Color32::from_rgb(200, 120, 255)); // 紫色
        }

        if self.builtins.contains(token) {
            return Some(Color32::from_rgb(100, 200, 255)); // 浅蓝
        }

        if self.types.contains(token) {
            return Some(Color32::from_rgb(100, 255, 200)); // 青绿
        }

        // 数字
        if token.chars().all(|c| c.is_ascii_digit() || c == '.') {
            return Some(Color32::from_rgb(255, 200, 100)); // 橙色
        }

        // 字符串
        if token.starts_with('"') && token.ends_with('"') ||
           token.starts_with('\'') && token.ends_with('\'') {
            return Some(Color32::from_rgb(100, 255, 100)); // 绿色
        }

        // 注释
        if token.starts_with("//") || token.starts_with("/*") {
            return Some(Color32::from_rgb(100, 120, 100)); // 暗绿
        }

        None
    }
}

impl SyntaxHighlighter for LunarHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, Color32)> {
        let mut result = Vec::new();
        let mut pos = 0;
        let mut current_token = String::new();

        for (i, ch) in text.chars().enumerate() {
            if ch.is_alphanumeric() || ch == '_' || ch == '@' {
                current_token.push(ch);
                continue;
            }

            // 处理当前token
            if !current_token.is_empty() {
                let next_char = text.chars().nth(i).unwrap_or(' ');
                if let Some(color) = self.color_for_token(&current_token, next_char) {
                    result.push((pos, color));
                }
                current_token.clear();
            }

            // 处理特殊字符
            if ch == '"' || ch == '\'' {
                let mut string_token = String::from(ch);
                let mut escaped = false;
                for c in text.chars().skip(i + 1) {
                    if c == '\\' {
                        escaped = !escaped;
                        string_token.push(c);
                        continue;
                    }
                    string_token.push(c);
                    if c == ch && !escaped {
                        break;
                    }
                    escaped = false;
                }
                result.push((i, Color32::from_rgb(100, 255, 100)));
                pos += string_token.len();
                continue;
            }

            // 单行注释
            if ch == '/' && text.chars().nth(i + 1) == Some('/') {
                let comment: String = text.chars().skip(i).collect();
                result.push((i, Color32::from_rgb(100, 120, 100)));
                break;
            }

            pos += 1;
        }

        // 处理最后一个token
        if !current_token.is_empty() {
            if let Some(color) = self.color_for_token(&current_token, ' ') {
                result.push((pos, color));
            }
        }

        result
    }

    fn name(&self) -> &'static str {
        "Lunar"
    }
}
