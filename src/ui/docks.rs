// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use egui::{Color32, Rect, Ui};
use std::collections::HashMap;

/// 停靠系统
pub struct DockSystem {
    docks: HashMap<String, DockPanel>,
    active_dock: Option<String>,
}

/// 停靠面板
pub struct DockPanel {
    pub title: String,
    pub position: DockPosition,
    pub visible: bool,
    pub width: f32,
    pub height: f32,
    pub content: Option<Box<dyn DockContent>>,
}

/// 停靠位置
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DockPosition {
    Left,
    Right,
    Top,
    Bottom,
    Center,
    Floating,
}

/// 停靠内容特征
pub trait DockContent: Send + Sync {
    fn render(&mut self, ui: &mut Ui);
    fn name(&self) -> &'static str;
}

impl DockSystem {
    pub fn new() -> Self {
        Self {
            docks: HashMap::new(),
            active_dock: None,
        }
    }

    pub fn add_dock(&mut self, id: &str, panel: DockPanel) {
        self.docks.insert(id.to_string(), panel);
    }

    pub fn render(&mut self, ui: &mut Ui) {
        // 左侧面板
        if let Some(dock) = self.docks.get_mut("left") {
            if dock.visible {
                egui::SidePanel::left("left_dock")
                    .resizable(true)
                    .default_width(250.0)
                    .show(ui, |ui| {
                        self.render_dock_content(ui, dock);
                    });
            }
        }

        // 右侧面板
        if let Some(dock) = self.docks.get_mut("right") {
            if dock.visible {
                egui::SidePanel::right("right_dock")
                    .resizable(true)
                    .default_width(250.0)
                    .show(ui, |ui| {
                        self.render_dock_content(ui, dock);
                    });
            }
        }

        // 顶部面板
        if let Some(dock) = self.docks.get_mut("top") {
            if dock.visible {
                egui::TopBottomPanel::top("top_dock")
                    .resizable(true)
                    .default_height(40.0)
                    .show(ui, |ui| {
                        self.render_dock_content(ui, dock);
                    });
            }
        }

        // 底部面板
        if let Some(dock) = self.docks.get_mut("bottom") {
            if dock.visible {
                egui::TopBottomPanel::bottom("bottom_dock")
                    .resizable(true)
                    .default_height(200.0)
                    .show(ui, |ui| {
                        self.render_dock_content(ui, dock);
                    });
            }
        }
    }

    fn render_dock_content(&mut self, ui: &mut Ui, dock: &mut DockPanel) {
        ui.heading(RichText::new(&dock.title).color(Color32::from_rgb(0, 240, 255)));
        ui.separator();

        if let Some(content) = &mut dock.content {
            content.render(ui);
        }
    }

    pub fn update(&mut self, _dt: f32) {
        // 更新所有停靠面板
    }
}
