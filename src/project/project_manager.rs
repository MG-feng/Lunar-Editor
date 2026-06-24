// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use anyhow::{anyhow, Result};
use serde_json;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

use super::ProjectConfig;

/// 项目管理器
pub struct ProjectManager {
    project_path: Option<PathBuf>,
    config: ProjectConfig,
    dirty: bool,
}

impl ProjectManager {
    pub fn new() -> Self {
        Self {
            project_path: None,
            config: ProjectConfig::default(),
            dirty: false,
        }
    }

    /// 创建新项目
    pub fn create_project(&mut self, path: &Path, name: &str) -> Result<()> {
        if path.exists() {
            return Err(anyhow!("Project path already exists: {}", path.display()));
        }

        std::fs::create_dir_all(path)?;

        self.project_path = Some(path.to_path_buf());
        self.config.name = name.to_string();
        self.dirty = true;

        // 创建项目子目录
        let src_dir = path.join("src");
        let assets_dir = path.join("assets");
        std::fs::create_dir_all(&src_dir)?;
        std::fs::create_dir_all(&assets_dir)?;

        // 保存项目配置
        self.save()?;

        info!("✅ Project created: {}", path.display());
        Ok(())
    }

    /// 打开项目
    pub fn open_project(&mut self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(anyhow!("Project path does not exist: {}", path.display()));
        }

        let config_path = path.join("project.json");
        if !config_path.exists() {
            return Err(anyhow!(
                "Project config not found: {}",
                config_path.display()
            ));
        }

        let config_content = std::fs::read_to_string(&config_path)?;
        self.config = serde_json::from_str(&config_content)?;
        self.project_path = Some(path.to_path_buf());
        self.dirty = false;

        info!("✅ Project opened: {}", path.display());
        Ok(())
    }

    /// 保存项目
    pub fn save(&mut self) -> Result<()> {
        if let Some(path) = &self.project_path {
            let config_path = path.join("project.json");
            let config_content = serde_json::to_string_pretty(&self.config)?;
            std::fs::write(&config_path, config_content)?;
            self.dirty = false;
            debug!("Project saved: {}", config_path.display());
            Ok(())
        } else {
            Err(anyhow!("No project loaded"))
        }
    }

    /// 获取项目路径
    pub fn project_path(&self) -> PathBuf {
        self.project_path
            .clone()
            .unwrap_or_else(|| PathBuf::from("."))
    }

    /// 获取配置
    pub fn config(&self) -> &ProjectConfig {
        &self.config
    }

    /// 获取配置（可变）
    pub fn config_mut(&mut self) -> &mut ProjectConfig {
        self.dirty = true;
        &mut self.config
    }

    /// 是否有未保存的更改
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// 更新
    pub fn update(&mut self, _dt: f32) {
        // 检查项目文件变化
        if self.dirty {
            let _ = self.save();
        }
    }
}
