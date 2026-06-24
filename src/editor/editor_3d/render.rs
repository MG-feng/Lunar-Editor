// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;

use egui::{Ui, Rect, Painter, Color32, Stroke, Shape, Pos2};
use glam::{Vec3, Mat4};

use super::scene::{Scene3D, SceneObject3D, ObjectType3D, Vertex3D};
use super::camera::Camera3D;

/// 3D渲染器（简化版）
pub struct Renderer3D {
    // 这里应该包含wgpu相关的渲染资源
    // 简化版本使用egui绘制
}

impl Renderer3D {
    pub fn new() -> Self {
        Self {}
    }

    /// 渲染3D场景
    pub fn render(&mut self, ui: &mut Ui, scene: &Scene3D, camera: &Camera3D, rect: Rect) {
        let painter = ui.painter();

        // 渲染背景（星空效果）
        self.render_background(painter, rect);

        // 渲染网格地面
        self.render_grid(painter, camera, rect);

        // 渲染所有对象
        let objects = scene.get_all_objects();
        for obj in objects {
            if obj.visible {
                self.render_object(painter, obj, camera, rect);
            }
        }

        // 渲染坐标轴辅助
        self.render_axis(painter, camera, rect);
    }

    /// 渲染背景
    fn render_background(&self, painter: &Painter, rect: Rect) {
        // 渐变背景（从深蓝到深紫）
        let gradient = egui::epaint::LinearGradient::from_linear(
            rect,
            egui::pos2(rect.min.x, rect.min.y),
            egui::pos2(rect.min.x, rect.max.y),
            Color32::from_rgb(10, 10, 30),
            Color32::from_rgb(30, 10, 50),
        );
        painter.rect_filled(rect, 0.0, gradient);

        // 随机星星
        let mut rng = SmallRng::seed_from_u64(12345);
        for _ in 0..100 {
            let x = rect.min.x + rng.gen_range(0.0..rect.width());
            let y = rect.min.y + rng.gen_range(0.0..rect.height());
            let size = rng.gen_range(1.0..3.0);
            let brightness = rng.gen_range(100..255);
            painter.circle_filled(
                egui::pos2(x, y),
                size,
                Color32::from_rgb(brightness, brightness, brightness + 50),
            );
        }
    }

    /// 渲染网格地面
    fn render_grid(&self, painter: &Painter, camera: &Camera3D, rect: Rect) {
        let grid_size = 20;
        let spacing = 1.0;

        // 计算视图矩阵和投影矩阵
        let view = camera.get_view_matrix();
        let proj = camera.get_projection_matrix();
        let view_proj = proj * view;

        let center = rect.center();
        let scale = rect.width().min(rect.height()) * 0.4;

        // 使用简化的2D投影来绘制网格
        let color = Color32::from_rgba_premultiplied(60, 70, 100, 80);
        let stroke = Stroke::new(1.0, color);

        for i in -grid_size..=grid_size {
            let x = i as f32 * spacing;

            // 计算屏幕位置
            let world_pos = Vec3::new(x, 0.0, -grid_size as f32);
            let screen_pos = self.world_to_screen(world_pos, view_proj, rect, center, scale);
            let world_pos2 = Vec3::new(x, 0.0, grid_size as f32);
            let screen_pos2 = self.world_to_screen(world_pos2, view_proj, rect, center, scale);

            if let (Some(p1), Some(p2)) = (screen_pos, screen_pos2) {
                painter.line_segment([p1, p2], stroke);
            }

            let world_pos = Vec3::new(-grid_size as f32, 0.0, i as f32);
            let screen_pos = self.world_to_screen(world_pos, view_proj, rect, center, scale);
            let world_pos2 = Vec3::new(grid_size as f32, 0.0, i as f32);
            let screen_pos2 = self.world_to_screen(world_pos2, view_proj, rect, center, scale);

            if let (Some(p1), Some(p2)) = (screen_pos, screen_pos2) {
                painter.line_segment([p1, p2], stroke);
            }
        }
    }

    /// 渲染对象
    fn render_object(&self, painter: &Painter, obj: &SceneObject3D, camera: &Camera3D, rect: Rect) {
        let view = camera.get_view_matrix();
        let proj = camera.get_projection_matrix();
        let view_proj = proj * view;

        let center = rect.center();
        let scale = rect.width().min(rect.height()) * 0.4;

        match &obj.object_type {
            ObjectType3D::Mesh { vertices, indices, material } => {
                // 渲染网格（简化版 - 使用线框）
                for i in (0..indices.len()).step_by(3) {
                    if i + 2 >= indices.len() { break; }

                    let v0 = vertices[indices[i] as usize];
                    let v1 = vertices[indices[i + 1] as usize];
                    let v2 = vertices[indices[i + 2] as usize];

                    let world_pos0 = obj.position + Vec3::from(v0.position);
                    let world_pos1 = obj.position + Vec3::from(v1.position);
                    let world_pos2 = obj.position + Vec3::from(v2.position);

                    let screen0 = self.world_to_screen(world_pos0, view_proj, rect, center, scale);
                    let screen1 = self.world_to_screen(world_pos1, view_proj, rect, center, scale);
                    let screen2 = self.world_to_screen(world_pos2, view_proj, rect, center, scale);

                    if let (Some(p0), Some(p1), Some(p2)) = (screen0, screen1, screen2) {
                        let color = Color32::from_rgb(
                            (material.albedo.x * 200.0 + 55.0) as u8,
                            (material.albedo.y * 200.0 + 55.0) as u8,
                            (material.albedo.z * 200.0 + 55.0) as u8,
                        );
                        let stroke = Stroke::new(1.0, color);
                        painter.line_segment([p0, p1], stroke);
                        painter.line_segment([p1, p2], stroke);
                        painter.line_segment([p2, p0], stroke);
                    }
                }
            }
            ObjectType3D::Model { path, meshes } => {
                // 渲染模型的所有网格
                for mesh in meshes {
                    for i in (0..mesh.indices.len()).step_by(3) {
                        if i + 2 >= mesh.indices.len() { break; }

                        let v0 = mesh.vertices[mesh.indices[i] as usize];
                        let v1 = mesh.vertices[mesh.indices[i + 1] as usize];
                        let v2 = mesh.vertices[mesh.indices[i + 2] as usize];

                        let world_pos0 = obj.position + Vec3::from(v0.position);
                        let world_pos1 = obj.position + Vec3::from(v1.position);
                        let world_pos2 = obj.position + Vec3::from(v2.position);

                        let screen0 = self.world_to_screen(world_pos0, view_proj, rect, center, scale);
                        let screen1 = self.world_to_screen(world_pos1, view_proj, rect, center, scale);
                        let screen2 = self.world_to_screen(world_pos2, view_proj, rect, center, scale);

                        if let (Some(p0), Some(p1), Some(p2)) = (screen0, screen1, screen2) {
                            let color = Color32::from_rgb(150, 180, 220);
                            let stroke = Stroke::new(1.0, color);
                            painter.line_segment([p0, p1], stroke);
                            painter.line_segment([p1, p2], stroke);
                            painter.line_segment([p2, p0], stroke);
                        }
                    }
                }
            }
            ObjectType3D::Light { light_type, intensity, color, range } => {
                // 渲染光源指示器
                let pos = obj.position;
                let screen_pos = self.world_to_screen(pos, view_proj, rect, center, scale);

                if let Some(p) = screen_pos {
                    let size = 10.0 + intensity * 10.0;
                    let color = Color32::from_rgb(
                        (color.x * 255.0) as u8,
                        (color.y * 255.0) as u8,
                        (color.z * 255.0) as u8,
                    );
                    painter.circle_filled(p, size, color);
                    // 发光效果
                    painter.circle_filled(p, size * 2.0, color.gamma_multiply(0.2));

                    let label = match light_type {
                        LightType::Directional => "☀️ Directional",
                        LightType::Point => "💡 Point",
                        LightType::Spot => "🔦 Spot",
                    };
                    painter.text(
                        egui::pos2(p.x, p.y - 20.0),
                        egui::Align2::CENTER_BOTTOM,
                        label,
                        egui::FontId::proportional(10.0),
                        Color32::WHITE,
                    );
                }
            }
            ObjectType3D::Camera { fov, near, far, orthographic, ortho_size } => {
                // 渲染摄像机指示器
                let pos = obj.position;
                let screen_pos = self.world_to_screen(pos, view_proj, rect, center, scale);

                if let Some(p) = screen_pos {
                    painter.circle_filled(p, 8.0, Color32::from_rgb(100, 200, 255));
                    painter.circle_stroke(p, 8.0, Stroke::new(2.0, Color32::from_rgb(100, 200, 255)));

                    let label = if *orthographic { "📷 Orthographic" } else { "📷 Perspective" };
                    painter.text(
                        egui::pos2(p.x, p.y - 20.0),
                        egui::Align2::CENTER_BOTTOM,
                        label,
                        egui::FontId::proportional(10.0),
                        Color32::from_rgb(100, 200, 255),
                    );
                }
            }
            ObjectType3D::Empty => {
                // 空对象 - 显示一个小点
                let pos = obj.position;
                let screen_pos = self.world_to_screen(pos, view_proj, rect, center, scale);

                if let Some(p) = screen_pos {
                    painter.circle_filled(p, 3.0, Color32::from_rgb(200, 200, 200));
                }
            }
            ObjectType3D::Group => {
                // 组 - 显示组指示器
                let pos = obj.position;
                let screen_pos = self.world_to_screen(pos, view_proj, rect, center, scale);

                if let Some(p) = screen_pos {
                    painter.circle_filled(p, 5.0, Color32::from_rgb(200, 200, 100));
                    painter.circle_stroke(p, 5.0, Stroke::new(1.0, Color32::from_rgb(200, 200, 100)));

                    painter.text(
                        egui::pos2(p.x, p.y - 20.0),
                        egui::Align2::CENTER_BOTTOM,
                        "📁 Group",
                        egui::FontId::proportional(10.0),
                        Color32::from_rgb(200, 200, 100),
                    );
                }
            }
        }
    }

    /// 渲染坐标轴辅助
    fn render_axis(&self, painter: &Painter, camera: &Camera3D, rect: Rect) {
        let view = camera.get_view_matrix();
        let proj = camera.get_projection_matrix();
        let view_proj = proj * view;

        let center = rect.center();
        let scale = rect.width().min(rect.height()) * 0.4;

        // X轴 (红)
        let origin = Vec3::ZERO;
        let x_axis = Vec3::new(1.0, 0.0, 0.0);
        let y_axis = Vec3::new(0.0, 1.0, 0.0);
        let z_axis = Vec3::new(0.0, 0.0, 1.0);

        let origin_screen = self.world_to_screen(origin, view_proj, rect, center, scale);
        let x_screen = self.world_to_screen(origin + x_axis * 0.5, view_proj, rect, center, scale);
        let y_screen = self.world_to_screen(origin + y_axis * 0.5, view_proj, rect, center, scale);
        let z_screen = self.world_to_screen(origin + z_axis * 0.5, view_proj, rect, center, scale);

        if let (Some(o), Some(x), Some(y), Some(z)) = (origin_screen, x_screen, y_screen, z_screen) {
            // X轴
            painter.line_segment([o, x], Stroke::new(2.0, Color32::RED));
            painter.text(x, egui::Align2::CENTER_TOP, "X", egui::FontId::proportional(12.0), Color32::RED);

            // Y轴
            painter.line_segment([o, y], Stroke::new(2.0, Color32::GREEN));
            painter.text(y, egui::Align2::CENTER_TOP, "Y", egui::FontId::proportional(12.0), Color32::GREEN);

            // Z轴
            painter.line_segment([o, z], Stroke::new(2.0, Color32::BLUE));
            painter.text(z, egui::Align2::CENTER_TOP, "Z", egui::FontId::proportional(12.0), Color32::BLUE);
        }
    }

    /// 世界坐标转屏幕坐标
    fn world_to_screen(&self, world: Vec3, view_proj: Mat4, rect: Rect, center: Pos2, scale: f32) -> Option<Pos2> {
        let clip = view_proj * Vec3::new(world.x, world.y, world.z).extend(1.0);

        if clip.w > 0.0 {
            let ndc = Vec3::new(clip.x / clip.w, clip.y / clip.w, clip.z / clip.w);
            if ndc.x.abs() > 1.0 || ndc.y.abs() > 1.0 {
                // 在视锥外
                return None;
            }

            let x = center.x + ndc.x * scale;
            let y = center.y - ndc.y * scale;
            Some(egui::pos2(x, y))
        } else {
            None
        }
    }

    /// 获取内存使用
    pub fn memory_usage(&self) -> usize {
        std::mem::size_of_val(self)
    }
}
