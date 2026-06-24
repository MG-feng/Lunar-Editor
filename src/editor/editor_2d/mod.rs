// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

mod scene;
mod tools;
mod render;

pub use scene::Scene2D;
pub use tools::ToolManager2D;
pub use render::Renderer2D;

use egui::Ui;
use super::Editor;

/// 2D可视化编辑器
pub struct Editor2D {
    scene: Scene2D,
    tools: ToolManager2D,
    renderer: Renderer2D,
    selected_objects: Vec<usize>,
}

impl Editor2D {
    pub fn new() -> Self {
        Self {
            scene: Scene2D::new(),
            tools: ToolManager2D::new(),
            renderer: Renderer2D::new(),
            selected_objects: Vec::new(),
        }
    }
}

impl Editor for Editor2D {
    fn update(&mut self, dt: f32) {
        self.scene.update(dt);
        self.tools.update(dt);
    }

    fn render(&mut self, ui: &mut Ui) {
        // 工具栏
        ui.horizontal(|ui| {
            ui.label("2D Editor");
            ui.separator();
            self.tools.render_toolbar(ui);
        });

        ui.separator();

        // 场景渲染区域
        egui::Frame::canvas(ui.style())
            .fill(ui.style().visuals.panel_fill)
            .show(ui, |ui| {
                let rect = ui.available_rect_before_wrap();
                self.renderer.render(ui, &self.scene, rect);
            });

        // 属性面板 - 显示选中的对象
        if !self.selected_objects.is_empty() {
            ui.separator();
            ui.collapsing("Properties", |ui| {
                for &obj_id in &self.selected_objects {
                    if let Some(obj) = self.scene.get_object(obj_id) {
                        ui.label(format!("Object: {:?}", obj));
                    }
                }
            });
        }
    }

    fn memory_usage(&self) -> usize {
        self.scene.memory_usage() + self.renderer.memory_usage()
    }

    fn name(&self) -> &'static str {
        "2D Editor"
    }
}

impl Default for Editor2D {
    fn default() -> Self {
        Self::new()
    }
}
