// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use anyhow::Result;
use std::collections::HashMap;

/// Lunar语言API接口 - 提供给插件的Lunar语言操作接口
/// 注意：这只是接口定义，实际实现将在后续添加
pub struct LunarAPI {
    // 这里只保留接口占位
    scripts: HashMap<String, String>,
}

impl LunarAPI {
    pub fn new() -> Self {
        Self {
            scripts: HashMap::new(),
        }
    }

    /// 加载Lunar脚本
    pub fn load_script(&self, _path: &str) -> Result<()> {
        // TODO: 实现Lunar脚本加载
        // 此接口保留，等待Lunar引擎完整实现
        Ok(())
    }

    /// 执行Lunar脚本
    pub fn execute_script(&self, _code: &str) -> Result<String> {
        // TODO: 实现Lunar脚本执行
        // 此接口保留，等待Lunar引擎完整实现
        Ok("Lunar execution not yet implemented".to_string())
    }

    /// 注册Lunar函数
    pub fn register_function(
        &self,
        _name: &str,
        _func: Box<dyn Fn(&[String]) -> String + Send + Sync>,
    ) -> Result<()> {
        // TODO: 实现Lunar函数注册
        // 此接口保留，等待Lunar引擎完整实现
        Ok(())
    }

    /// 获取Lunar变量
    pub fn get_variable(&self, _name: &str) -> Option<String> {
        // TODO: 实现Lunar变量获取
        // 此接口保留，等待Lunar引擎完整实现
        None
    }

    /// 设置Lunar变量
    pub fn set_variable(&self, _name: &str, _value: &str) -> Result<()> {
        // TODO: 实现Lunar变量设置
        // 此接口保留，等待Lunar引擎完整实现
        Ok(())
    }
}
