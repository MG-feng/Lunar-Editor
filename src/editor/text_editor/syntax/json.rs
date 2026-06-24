// Copyright 2026 Lunar Editor Team
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
