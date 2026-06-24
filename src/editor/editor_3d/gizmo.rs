// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use glam::{Vec3, Mat4};
use egui::{Ui, Painter, Color32, Stroke};

/// 3D Gizmo - 变换操作器
pub struct Gizmo3D {
    pub enabled: bool,
    pub mode: GizmoMode,
    pub snap: bool,
    pub snap_value: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GizmoMode {
    Translate,
    Rotate,
    Scale,
}

impl Gizmo3D {
    pub fn new() -> Self {
        Self {
            enabled: true,
            mode: GizmoMode::Translate,
            snap: false,
            snap_value: 0.1,
        }
    }

    pub fn update(&mut self, _dt: f32) {
        // 更新gizmo状态
    }

    pub fn render(&self, _ui: &mut Ui, _painter: &Painter, _camera_view: Mat4, _camera_proj: Mat4) {
        if !self.enabled {
            return;
        }

        // TODO: 实现Gizmo渲染
        // 这里使用简化版本，仅绘制坐标轴
    }

    pub fn set_mode(&mut self, mode: GizmoMode) {
        self.mode = mode;
    }

    pub fn toggle_snap(&mut self) {
        self.snap = !self.snap;
    }
}

impl Default for Gizmo3D {
    fn default() -> Self {
        Self::new()
    }
}
