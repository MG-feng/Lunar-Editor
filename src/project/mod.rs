// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

mod asset_manager;
mod project_manager;

pub use asset_manager::AssetManager;
pub use project_manager::ProjectManager;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 项目配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub default_mode: String,
    pub plugins: Vec<String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "Untitled Project".to_string(),
            version: "0.1.0".to_string(),
            author: "Unknown".to_string(),
            description: "".to_string(),
            default_mode: "text".to_string(),
            plugins: Vec::new(),
        }
    }
}
