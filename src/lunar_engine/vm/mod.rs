// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

mod bytecode;
mod interpreter;

pub use bytecode::Bytecode;
pub use interpreter::Interpreter;

/// Lunar虚拟机 - 接口占位
pub struct VirtualMachine {
    interpreter: Interpreter,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
        }
    }

    pub fn execute(&mut self, _bytecode: Vec<Bytecode>) {
        // TODO: 实际实现将在后续添加
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}
