// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

mod lunar;
mod rust;
mod json;

pub use lunar::LunarHighlighter;
pub use rust::RustHighlighter;
pub use json::JsonHighlighter;

use egui::Color32;
use std::collections::HashMap;

/// 语法高亮类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxType {
    Lunar,
    Rust,
    Json,
    Plain,
    Text,
}

impl SyntaxType {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "lunar" | "l" => SyntaxType::Lunar,
            "rs" => SyntaxType::Rust,
            "json" => SyntaxType::Json,
            "txt" => SyntaxType::Text,
            _ => SyntaxType::Plain,
        }
    }
}

/// 语法高亮器特征
pub trait SyntaxHighlighter {
    /// 高亮指定文本
    fn highlight(&self, text: &str) -> Vec<(usize, Color32)>;

    /// 获取高亮名称
    fn name(&self) -> &'static str;
}

/// 语法高亮管理器
pub struct SyntaxManager {
    highlighters: HashMap<SyntaxType, Box<dyn SyntaxHighlighter>>,
}

impl SyntaxManager {
    pub fn new() -> Self {
        let mut highlighters: HashMap<SyntaxType, Box<dyn SyntaxHighlighter>> = HashMap::new();
        highlighters.insert(SyntaxType::Lunar, Box::new(LunarHighlighter::new()));
        highlighters.insert(SyntaxType::Rust, Box::new(RustHighlighter::new()));
        highlighters.insert(SyntaxType::Json, Box::new(JsonHighlighter::new()));

        Self { highlighters }
    }

    pub fn get(&self, syntax: SyntaxType) -> Option<&Box<dyn SyntaxHighlighter>> {
        self.highlighters.get(&syntax)
    }
}
