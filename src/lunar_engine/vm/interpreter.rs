// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

/// Lunar解释器 - 接口占位
pub struct Interpreter {
    // 接口保留
}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, _bytecode: &[super::bytecode::Bytecode]) {
        // TODO: 实际实现将在后续添加
    }
}
