// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

// Lunar语言支持 - 仅接口定义，具体实现后续添加
// 此模块保留接口，等待Lunar引擎完整实现

mod ast;
mod lexer;
mod parser;
mod vm;

pub use ast::ASTNode;
pub use lexer::Lexer;
pub use parser::Parser;
pub use vm::VirtualMachine;

use anyhow::Result;
use std::collections::HashMap;

/// Lunar引擎 - 接口占位
pub struct LunarEngine {
    vm: VirtualMachine,
    loaded_scripts: HashMap<String, String>,
}

impl LunarEngine {
    pub fn new() -> Self {
        Self {
            vm: VirtualMachine::new(),
            loaded_scripts: HashMap::new(),
        }
    }

    /// 加载Lunar脚本
    pub fn load_script(&mut self, name: &str, code: &str) -> Result<()> {
        self.loaded_scripts
            .insert(name.to_string(), code.to_string());
        // TODO: 实际实现将在后续添加
        Ok(())
    }

    /// 执行Lunar脚本
    pub fn execute(&mut self, name: &str) -> Result<()> {
        if let Some(code) = self.loaded_scripts.get(name) {
            // TODO: 实际实现将在后续添加
            Ok(())
        } else {
            Err(anyhow::anyhow!("Script not found: {}", name))
        }
    }

    /// 获取变量
    pub fn get_variable(&self, _name: &str) -> Option<String> {
        // TODO: 实际实现将在后续添加
        None
    }

    /// 设置变量
    pub fn set_variable(&mut self, _name: &str, _value: &str) {
        // TODO: 实际实现将在后续添加
    }
}

impl Default for LunarEngine {
    fn default() -> Self {
        Self::new()
    }
}
