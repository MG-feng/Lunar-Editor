// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use super::ast::ASTNode;

/// Lunar语法分析器 - 接口占位
pub struct Parser {
    // 接口保留
}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, _tokens: &[super::lexer::Token]) -> Vec<ASTNode> {
        // TODO: 实际实现将在后续添加
        Vec::new()
    }
}
