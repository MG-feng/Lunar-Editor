// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use std::sync::Arc;
use crate::app::EditorApp;
use crate::editor::Editor;
use anyhow::Result;

/// 编辑器API - 提供给插件的编辑器操作接口
pub struct EditorAPI {
    app: Arc<EditorApp>,
}

impl EditorAPI {
    pub fn new(app: Arc<EditorApp>) -> Self {
        Self { app }
    }

    /// 注册工具
    pub fn register_tool(&self, name: &str, tool: Box<dyn Editor>) -> Result<()> {
        // TODO: 实现工具注册
        Ok(())
    }

    /// 添加菜单项
    pub fn add_menu_item(&self, path: &str, callback: impl Fn() + Send + Sync + 'static) -> Result<()> {
        // TODO: 实现菜单项添加
        Ok(())
    }

    /// 注册快捷键
    pub fn register_hotkey(&self, key: &str, callback: impl Fn() + Send + Sync + 'static) -> Result<()> {
        // TODO: 实现快捷键注册
        Ok(())
    }

    /// 打开文件
    pub fn open_file(&self, path: &str) -> Result<()> {
        // TODO: 实现文件打开
        Ok(())
    }

    /// 获取当前模式
    pub fn current_mode(&self) -> String {
        // TODO: 返回当前模式
        "text".to_string()
    }

    /// 切换模式
    pub fn switch_mode(&self, mode: &str) -> Result<()> {
        // TODO: 实现模式切换
        Ok(())
    }
}
