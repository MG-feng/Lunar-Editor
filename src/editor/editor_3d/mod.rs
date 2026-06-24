// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

mod scene;
mod camera;
mod render;
mod gizmo;
mod loader;

pub use scene::Scene3D;
pub use camera::Camera3D;
pub use render::Renderer3D;
pub use gizmo::Gizmo3D;
pub use loader::ModelLoader;

use egui::Ui;
use super::Editor;

/// 3D可视化编辑器
pub struct Editor3D {
    scene: Scene3D,
    camera: Camera3D,
    renderer: Renderer3D,
    gizmo: Gizmo3D,
    loader: ModelLoader,
    selected_objects: Vec<usize>,
}

impl Editor3D {
    pub fn new() -> Self {
        Self {
            scene: Scene3D::new(),
            camera: Camera3D::new(),
            renderer: Renderer3D::new(),
            gizmo: Gizmo3D::new(),
            loader: ModelLoader::new(),
            selected_objects: Vec::new(),
        }
    }

    /// 加载模型
    pub fn load_model(&mut self, path: &str) -> anyhow::Result<usize> {
        self.loader.load_model(path, &mut self.scene)
    }
}

impl Editor for Editor3D {
    fn update(&mut self, dt: f32) {
        self.scene.update(dt);
        self.camera.update(dt);
        self.gizmo.update(dt);
    }

    fn render(&mut self, ui: &mut Ui) {
        // 工具栏
        ui.horizontal(|ui| {
            ui.label("3D Editor");
            ui.separator();
            if ui.button("🔄 Reset Camera").clicked() {
                self.camera.reset();
            }
            ui.separator();
            if ui.button("📦 Import Model").clicked() {
                // TODO: 文件选择器
            }
        });

        ui.separator();

        // 3D场景渲染区域
        egui::Frame::canvas(ui.style())
            .fill(ui.style().visuals.panel_fill)
            .show(ui, |ui| {
                let rect = ui.available_rect_before_wrap();

                // 处理鼠标交互
                let response = ui.interact(rect, egui::Id::new("3d_viewport"), egui::Sense::click_and_drag());
                if response.hovered() {
                    // 鼠标悬停处理
                }
                if response.dragged() {
                    let delta = response.drag_delta();
                    self.camera.orbit(delta.x, delta.y);
                }

                // 滚动缩放
                if let Some(scroll) = ui.input().scroll_delta.y {
                    if response.hovered() {
                        self.camera.zoom(scroll);
                    }
                }

                self.renderer.render(ui, &self.scene, &self.camera, rect);
            });

        // 属性面板 - 显示选中的对象
        if !self.selected_objects.is_empty() {
            ui.separator();
            ui.collapsing("Properties", |ui| {
                for &obj_id in &self.selected_objects {
                    if let Some(obj) = self.scene.get_object(obj_id) {
                        ui.label(format!("Object: {}", obj.name));
                        ui.label(format!("  Position: ({:.2}, {:.2}, {:.2})",
                            obj.position.x, obj.position.y, obj.position.z));
                        ui.label(format!("  Rotation: ({:.2}, {:.2}, {:.2})",
                            obj.rotation.x, obj.rotation.y, obj.rotation.z));
                        ui.label(format!("  Scale: ({:.2}, {:.2}, {:.2})",
                            obj.scale.x, obj.scale.y, obj.scale.z));
                    }
                }
            });
        }
    }

    fn memory_usage(&self) -> usize {
        self.scene.memory_usage() +
        self.renderer.memory_usage() +
        self.loader.memory_usage()
    }

    fn name(&self) -> &'static str {
        "3D Editor"
    }
}

impl Default for Editor3D {
    fn default() -> Self {
        Self::new()
    }
}
