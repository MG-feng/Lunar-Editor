// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

mod docks;
mod file_tree;
mod toolbars;
mod widgets;

pub use docks::DockSystem;
pub use file_tree::FileTree;
pub use toolbars::ToolbarManager;
pub use widgets::*;

use egui::Ui;

/// UI组件特征
pub trait UIComponent {
    fn render(&mut self, ui: &mut Ui);
    fn update(&mut self, dt: f32);
    fn name(&self) -> &'static str;
}

/// UI管理器
pub struct UIManager {
    components: Vec<Box<dyn UIComponent>>,
    dock_system: DockSystem,
    toolbar_manager: ToolbarManager,
    file_tree: FileTree,
}

impl UIManager {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            dock_system: DockSystem::new(),
            toolbar_manager: ToolbarManager::new(),
            file_tree: FileTree::new(),
        }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        // 渲染文件树
        self.file_tree.render(ui);

        // 渲染组件
        for component in &mut self.components {
            component.render(ui);
        }
    }

    pub fn update(&mut self, dt: f32) {
        for component in &mut self.components {
            component.update(dt);
        }
        self.dock_system.update(dt);
        self.file_tree.update(dt);
    }
}
