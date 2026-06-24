// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, warn, error, debug};
use uuid::Uuid;

use super::{PluginInstance, PluginMetadata, PluginState, Permission};
use super::wasm_host::WasmHost;
use super::permissions::PermissionManager;

/// 插件管理器
pub struct PluginManager {
    plugins: Arc<RwLock<HashMap<Uuid, PluginInstance>>>,
    plugin_paths: Vec<PathBuf>,
    wasm_host: WasmHost,
    permission_manager: PermissionManager,
    enabled: bool,
}

impl PluginManager {
    pub fn new() -> Result<Self> {
        info!("Initializing Plugin Manager...");

        let wasm_host = WasmHost::new()?;
        let permission_manager = PermissionManager::new();

        // 创建插件目录
        let plugins_dir = Path::new("plugins");
        if !plugins_dir.exists() {
            std::fs::create_dir_all(plugins_dir)?;
        }

        Ok(Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            plugin_paths: vec![plugins_dir.to_path_buf()],
            wasm_host,
            permission_manager,
            enabled: true,
        })
    }

    /// 加载插件
    pub fn load_plugin(&mut self, path: &Path) -> Result<Uuid> {
        if !self.enabled {
            return Err(anyhow!("Plugin system is disabled"));
        }

        info!("Loading plugin from: {}", path.display());

        // 读取插件清单
        let manifest_path = path.join("plugin.toml");
        if !manifest_path.exists() {
            return Err(anyhow!("Plugin manifest not found: {}", manifest_path.display()));
        }

        let manifest_content = std::fs::read_to_string(&manifest_path)?;
        let metadata: PluginMetadata = toml::from_str(&manifest_content)?;

        // 检查权限
        for perm in &metadata.permissions {
            if !self.permission_manager.is_permission_allowed(perm) {
                return Err(anyhow!(
                    "Permission denied for plugin '{}': {:?}",
                    metadata.name, perm
                ));
            }
        }

        // 检查依赖
        for dep in &metadata.dependencies {
            let found = self.plugins.read().iter().any(|(_, p)| p.metadata.id == *dep);
            if !found {
                return Err(anyhow!("Missing dependency: {}", dep));
            }
        }

        // 加载WASM模块
        let wasm_path = path.join(&metadata.entry_point);
        if !wasm_path.exists() {
            return Err(anyhow!("WASM entry point not found: {}", wasm_path.display()));
        }

        let wasm_module = self.wasm_host.load_module(&wasm_path)?;

        // 创建插件实例
        let id = Uuid::new_v4();
        let instance = PluginInstance {
            id,
            metadata: metadata.clone(),
            wasm_module: Some(wasm_module),
            state: PluginState::Loaded,
            memory_usage: 0,
        };

        self.plugins.write().insert(id, instance);

        info!("✅ Plugin '{}' loaded successfully (ID: {})", metadata.name, id);
        Ok(id)
    }

    /// 卸载插件
    pub fn unload_plugin(&mut self, id: Uuid) -> Result<()> {
        let mut plugins = self.plugins.write();

        if let Some(plugin) = plugins.get_mut(&id) {
            plugin.state = PluginState::Unloaded;
            plugin.wasm_module = None;

            info!("✅ Plugin '{}' unloaded successfully", plugin.metadata.name);
            plugins.remove(&id);
            Ok(())
        } else {
            Err(anyhow!("Plugin not found: {}", id))
        }
    }

    /// 启动插件
    pub fn start_plugin(&mut self, id: Uuid) -> Result<()> {
        let mut plugins = self.plugins.write();

        if let Some(plugin) = plugins.get_mut(&id) {
            if plugin.state == PluginState::Loaded || plugin.state == PluginState::Paused {
                // 执行插件的入口函数
                if let Some(module) = &plugin.wasm_module {
                    // 调用插件初始化函数
                    self.wasm_host.execute_init(module)?;
                    plugin.state = PluginState::Running;
                    info!("✅ Plugin '{}' started", plugin.metadata.name);
                    return Ok(());
                }
            }
            Err(anyhow!("Plugin cannot be started in state: {:?}", plugin.state))
        } else {
            Err(anyhow!("Plugin not found: {}", id))
        }
    }

    /// 暂停插件
    pub fn pause_plugin(&mut self, id: Uuid) -> Result<()> {
        let mut plugins = self.plugins.write();

        if let Some(plugin) = plugins.get_mut(&id) {
            if plugin.state == PluginState::Running {
                plugin.state = PluginState::Paused;
                info!("⏸️ Plugin '{}' paused", plugin.metadata.name);
                return Ok(());
            }
            Err(anyhow!("Plugin cannot be paused in state: {:?}", plugin.state))
        } else {
            Err(anyhow!("Plugin not found: {}", id))
        }
    }

    /// 更新插件
    pub fn update(&mut self, dt: f32) {
        let plugins = self.plugins.read();

        for (_, plugin) in plugins.iter() {
            if plugin.state == PluginState::Running {
                // 调用插件的更新函数
                if let Some(module) = &plugin.wasm_module {
                    if let Err(e) = self.wasm_host.execute_update(module, dt) {
                        error!("Plugin '{}' update error: {}", plugin.metadata.name, e);
                    }
                }
            }
        }
    }

    /// 获取插件列表
    pub fn get_plugins(&self) -> Vec<&PluginInstance> {
        self.plugins.read().values().collect()
    }

    /// 获取插件
    pub fn get_plugin(&self, id: Uuid) -> Option<&PluginInstance> {
        self.plugins.read().get(&id)
    }

    /// 获取插件（可变）
    pub fn get_plugin_mut(&mut self, id: Uuid) -> Option<&mut PluginInstance> {
        self.plugins.write().get_mut(&id)
    }

    /// 启用/禁用插件系统
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// 添加插件搜索路径
    pub fn add_plugin_path(&mut self, path: PathBuf) {
        self.plugin_paths.push(path);
    }

    /// 扫描并加载所有插件
    pub fn scan_and_load(&mut self) -> Result<Vec<Uuid>> {
        let mut loaded = Vec::new();

        for path in &self.plugin_paths {
            if !path.exists() {
                continue;
            }

            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    // 检查是否包含plugin.toml
                    let manifest = path.join("plugin.toml");
                    if manifest.exists() {
                        match self.load_plugin(&path) {
                            Ok(id) => loaded.push(id),
                            Err(e) => {
                                warn!("Failed to load plugin from {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
        }

        info!("Scanned and loaded {} plugins", loaded.len());
        Ok(loaded)
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
