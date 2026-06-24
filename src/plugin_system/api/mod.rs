// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

mod editor_api;
mod scene_api;
mod render_api;
mod fs_api;
mod lunar_api;

pub use editor_api::EditorAPI;
pub use scene_api::SceneAPI;
pub use render_api::RenderAPI;
pub use fs_api::FileSystemAPI;
pub use lunar_api::LunarAPI;

use std::sync::Arc;
use crate::app::EditorApp;
use crate::editor::Editor;
use crate::project::ProjectManager;

/// 插件API - 提供给插件的完整接口
pub struct PluginAPI {
    pub editor: Arc<EditorAPI>,
    pub scene: Arc<SceneAPI>,
    pub render: Arc<RenderAPI>,
    pub fs: Arc<FileSystemAPI>,
    pub lunar: Arc<LunarAPI>,
}

impl PluginAPI {
    pub fn new(
        app: Arc<EditorApp>,
        project_manager: Arc<ProjectManager>,
    ) -> Self {
        Self {
            editor: Arc::new(EditorAPI::new(app.clone())),
            scene: Arc::new(SceneAPI::new(app.clone())),
            render: Arc::new(RenderAPI::new(app.clone())),
            fs: Arc::new(FileSystemAPI::new(project_manager.clone())),
            lunar: Arc::new(LunarAPI::new()),
        }
    }
}
