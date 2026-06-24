// Copyright 2026 Lunar Editor Team
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
