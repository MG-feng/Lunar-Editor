// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

mod plugin_manager;
mod wasm_host;
mod permissions;
pub mod api;

pub use plugin_manager::PluginManager;
pub use wasm_host::WasmHost;
pub use permissions::PermissionManager;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 插件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub entry_point: String,
    pub permissions: Vec<Permission>,
    pub dependencies: Vec<String>,
}

/// 插件实例
pub struct PluginInstance {
    pub id: Uuid,
    pub metadata: PluginMetadata,
    pub wasm_module: Option<wasmtime::Module>,
    pub state: PluginState,
    pub memory_usage: usize,
}

/// 插件状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginState {
    Loaded,
    Running,
    Paused,
    Unloaded,
    Error,
}

/// 插件权限
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Permission {
    // 文件系统
    #[serde(rename = "fs.read")]
    FsRead { paths: Vec<String> },
    #[serde(rename = "fs.write")]
    FsWrite { paths: Vec<String> },
    #[serde(rename = "fs.delete")]
    FsDelete { paths: Vec<String> },

    // 编辑器
    #[serde(rename = "editor.modify")]
    EditorModify,
    #[serde(rename = "editor.register_tool")]
    RegisterTool,
    #[serde(rename = "editor.register_menu")]
    RegisterMenu,
    #[serde(rename = "editor.hotkey")]
    RegisterHotkey,
    #[serde(rename = "editor.read")]
    EditorRead,

    // 场景
    #[serde(rename = "scene.read")]
    SceneRead,
    #[serde(rename = "scene.modify")]
    SceneModify,
    #[serde(rename = "scene.create")]
    SceneCreate,
    #[serde(rename = "scene.delete")]
    SceneDelete,

    // 渲染
    #[serde(rename = "render.custom")]
    RenderCustom,
    #[serde(rename = "render.shader")]
    LoadShader,
    #[serde(rename = "render.framebuffer")]
    FrameBufferAccess,

    // 网络
    #[serde(rename = "network.http")]
    NetworkHttp { hosts: Vec<String> },
    #[serde(rename = "network.websocket")]
    NetworkWebSocket { hosts: Vec<String> },
    #[serde(rename = "network.tcp")]
    NetworkTcp { ports: Vec<u16> },

    // 系统
    #[serde(rename = "system.command")]
    SystemCommand { commands: Vec<String> },
    #[serde(rename = "system.process")]
    SystemProcess { max: usize },
    #[serde(rename = "system.environment")]
    SystemEnvironment,

    // 插件管理
    #[serde(rename = "plugin.load")]
    LoadPlugin,
    #[serde(rename = "plugin.unload")]
    UnloadPlugin,
    #[serde(rename = "plugin.communicate")]
    InterPluginCommunication,

    // 危险操作
    #[serde(rename = "dangerous")]
    DangerousOperations,
}
