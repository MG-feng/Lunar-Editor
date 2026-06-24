// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use anyhow::Result;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::debug;

/// 资源管理器
pub struct AssetManager {
    assets: Arc<RwLock<HashMap<String, Asset>>>,
    asset_path: PathBuf,
}

/// 资源
#[derive(Debug, Clone)]
pub struct Asset {
    pub id: String,
    pub path: PathBuf,
    pub name: String,
    pub asset_type: AssetType,
    pub size: u64,
}

/// 资源类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetType {
    Texture,
    Model,
    Audio,
    Font,
    Script,
    Data,
    Other,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            assets: Arc::new(RwLock::new(HashMap::new())),
            asset_path: PathBuf::from("assets"),
        }
    }

    /// 设置资源路径
    pub fn set_asset_path(&mut self, path: &Path) {
        self.asset_path = path.to_path_buf();
    }

    /// 扫描资源目录
    pub fn scan_assets(&self) -> Result<()> {
        let mut assets = self.assets.write();
        assets.clear();

        if !self.asset_path.exists() {
            return Ok(());
        }

        self.scan_directory(&self.asset_path, &mut assets)?;

        debug!("Scanned {} assets", assets.len());
        Ok(())
    }

    fn scan_directory(&self, path: &Path, assets: &mut HashMap<String, Asset>) -> Result<()> {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                self.scan_directory(&path, assets)?;
            } else if path.is_file() {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();

                let asset_type = Self::get_asset_type(&path);
                let size = path.metadata().map(|m| m.len()).unwrap_or(0);

                let asset = Asset {
                    id: format!("asset_{}", assets.len()),
                    path: path.clone(),
                    name,
                    asset_type,
                    size,
                };

                assets.insert(asset.id.clone(), asset);
            }
        }

        Ok(())
    }

    fn get_asset_type(path: &Path) -> AssetType {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("png") | Some("jpg") | Some("jpeg") | Some("bmp") | Some("tiff")
            | Some("webp") => AssetType::Texture,
            Some("gltf") | Some("glb") | Some("obj") | Some("fbx") => AssetType::Model,
            Some("mp3") | Some("wav") | Some("ogg") | Some("flac") => AssetType::Audio,
            Some("ttf") | Some("otf") | Some("woff") | Some("woff2") => AssetType::Font,
            Some("lunar") | Some("l") | Some("rs") => AssetType::Script,
            Some("json") | Some("toml") | Some("yaml") | Some("xml") => AssetType::Data,
            _ => AssetType::Other,
        }
    }

    /// 获取资源
    pub fn get_asset(&self, id: &str) -> Option<Asset> {
        self.assets.read().get(id).cloned()
    }

    /// 获取所有资源
    pub fn get_all_assets(&self) -> Vec<Asset> {
        self.assets.read().values().cloned().collect()
    }

    /// 按类型获取资源
    pub fn get_assets_by_type(&self, asset_type: AssetType) -> Vec<Asset> {
        self.assets
            .read()
            .values()
            .filter(|a| a.asset_type == asset_type)
            .cloned()
            .collect()
    }

    /// 获取资源路径
    pub fn get_asset_path(&self, id: &str) -> Option<PathBuf> {
        self.assets.read().get(id).map(|a| a.path.clone())
    }

    /// 加载资源内容
    pub fn load_asset_content(&self, id: &str) -> Result<Vec<u8>> {
        if let Some(path) = self.get_asset_path(id) {
            Ok(std::fs::read(&path)?)
        } else {
            Err(anyhow::anyhow!("Asset not found: {}", id))
        }
    }
}

impl Default for AssetManager {
    fn default() -> Self {
        Self::new()
    }
}
