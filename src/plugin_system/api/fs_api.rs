// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use std::sync::Arc;
use std::path::{Path, PathBuf};
use crate::project::ProjectManager;
use anyhow::{Result, anyhow};

/// 文件系统API - 提供给插件的文件操作接口
pub struct FileSystemAPI {
    project_manager: Arc<ProjectManager>,
}

impl FileSystemAPI {
    pub fn new(project_manager: Arc<ProjectManager>) -> Self {
        Self { project_manager }
    }

    /// 读取文件
    pub fn read_file(&self, path: &str) -> Result<String> {
        let full_path = self.resolve_path(path)?;
        std::fs::read_to_string(&full_path)
            .map_err(|e| anyhow!("Failed to read file {}: {}", path, e))
    }

    /// 写入文件
    pub fn write_file(&self, path: &str, content: &str) -> Result<()> {
        let full_path = self.resolve_path(path)?;

        // 确保目录存在
        if let Some(parent) = full_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        std::fs::write(&full_path, content)
            .map_err(|e| anyhow!("Failed to write file {}: {}", path, e))
    }

    /// 删除文件
    pub fn delete_file(&self, path: &str) -> Result<()> {
        let full_path = self.resolve_path(path)?;
        if full_path.exists() {
            std::fs::remove_file(&full_path)
                .map_err(|e| anyhow!("Failed to delete file {}: {}", path, e))
        } else {
            Err(anyhow!("File not found: {}", path))
        }
    }

    /// 列出目录
    pub fn list_directory(&self, path: &str) -> Result<Vec<PathBuf>> {
        let full_path = self.resolve_path(path)?;
        let mut entries = Vec::new();

        if full_path.exists() && full_path.is_dir() {
            for entry in std::fs::read_dir(&full_path)? {
                let entry = entry?;
                entries.push(entry.path());
            }
        }

        Ok(entries)
    }

    /// 创建目录
    pub fn create_directory(&self, path: &str) -> Result<()> {
        let full_path = self.resolve_path(path)?;
        std::fs::create_dir_all(&full_path)
            .map_err(|e| anyhow!("Failed to create directory {}: {}", path, e))
    }

    /// 检查文件是否存在
    pub fn file_exists(&self, path: &str) -> bool {
        self.resolve_path(path)
            .map(|p| p.exists())
            .unwrap_or(false)
    }

    /// 解析路径（相对于项目根目录）
    fn resolve_path(&self, path: &str) -> Result<PathBuf> {
        let project_path = self.project_manager.project_path();
        let full_path = project_path.join(path);

        // 检查路径是否在项目目录内（安全检查）
        if !full_path.starts_with(&project_path) {
            return Err(anyhow!("Path outside project directory: {}", path));
        }

        Ok(full_path)
    }
}
