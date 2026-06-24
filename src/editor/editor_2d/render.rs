// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use egui::{Ui, Rect, Painter, Color32, Stroke, Shape};
use glam::Vec2;

use super::scene::{Scene2D, SceneObject2D, ObjectType2D};

/// 2D渲染器
pub struct Renderer2D {
    grid_size: f32,
    show_grid: bool,
    zoom: f32,
}

impl Renderer2D {
    pub fn new() -> Self {
        Self {
            grid_size: 20.0,
            show_grid: true,
            zoom: 1.0,
        }
    }

    /// 渲染场景
    pub fn render(&mut self, ui: &mut Ui, scene: &Scene2D, rect: Rect) {
        let painter = ui.painter();

        // 渲染背景
        painter.rect_filled(rect, 0.0, Color32::from_rgb(20, 24, 36));

        // 渲染网格
        if self.show_grid {
            self.render_grid(painter, rect);
        }

        // 渲染所有对象
        let objects = scene.get_all_objects();
        for obj in objects {
            if obj.visible {
                self.render_object(painter, obj, rect);
            }
        }

        // 渲染边框（选中对象）
        // 这里可以添加选中对象的边框渲染
    }

    /// 渲染网格
    fn render_grid(&self, painter: &Painter, rect: Rect) {
        let spacing = self.grid_size * self.zoom;
        let start_x = (rect.min.x / spacing).floor() * spacing;
        let start_y = (rect.min.y / spacing).floor() * spacing;
        let end_x = rect.max.x;
        let end_y = rect.max.y;

        let color = Color32::from_rgba_premultiplied(50, 60, 80, 100);
        let stroke = Stroke::new(1.0, color);

        let mut x = start_x;
        while x <= end_x {
            painter.line_segment([(x, rect.min.y), (x, rect.max.y)], stroke);
            x += spacing;
        }

        let mut y = start_y;
        while y <= end_y {
            painter.line_segment([(rect.min.x, y), (rect.max.x, y)], stroke);
            y += spacing;
        }
    }

    /// 渲染对象
    fn render_object(&self, painter: &Painter, obj: &SceneObject2D, rect: Rect) {
        let center = rect.center() + Vec2::new(obj.position.x, obj.position.y) * self.zoom;

        match &obj.object_type {
            ObjectType2D::Rectangle { width, height, color } => {
                let half_w = width * self.zoom / 2.0;
                let half_h = height * self.zoom / 2.0;
                let rect_shape = Rect::from_center_size(
                    egui::pos2(center.x, center.y),
                    egui::vec2(half_w * 2.0, half_h * 2.0),
                );
                painter.rect_filled(
                    rect_shape,
                    0.0,
                    Color32::from_rgba_premultiplied(
                        (color[0] * 255.0) as u8,
                        (color[1] * 255.0) as u8,
                        (color[2] * 255.0) as u8,
                        (color[3] * 255.0) as u8,
                    ),
                );
                // 边框
                painter.rect_stroke(
                    rect_shape,
                    0.0,
                    Stroke::new(1.0, Color32::from_rgb(100, 150, 200)),
                );
            }
            ObjectType2D::Circle { radius, color } => {
                let radius = radius * self.zoom;
                let center_pos = egui::pos2(center.x, center.y);
                let circle = Shape::circle_filled(center_pos, radius, Color32::from_rgba_premultiplied(
                    (color[0] * 255.0) as u8,
                    (color[1] * 255.0) as u8,
                    (color[2] * 255.0) as u8,
                    (color[3] * 255.0) as u8,
                ));
                painter.add(circle);
            }
            ObjectType2D::Triangle { points, color } => {
                let pts: [egui::Pos2; 3] = [
                    egui::pos2(center.x + points[0].x * self.zoom, center.y + points[0].y * self.zoom),
                    egui::pos2(center.x + points[1].x * self.zoom, center.y + points[1].y * self.zoom),
                    egui::pos2(center.x + points[2].x * self.zoom, center.y + points[2].y * self.zoom),
                ];
                let triangle = Shape::triangle_filled(
                    pts[0],
                    pts[1],
                    pts[2],
                    Color32::from_rgba_premultiplied(
                        (color[0] * 255.0) as u8,
                        (color[1] * 255.0) as u8,
                        (color[2] * 255.0) as u8,
                        (color[3] * 255.0) as u8,
                    ),
                );
                painter.add(triangle);
            }
            ObjectType2D::Image { path, size } => {
                // 简化处理，显示占位符
                let half_w = size.x * self.zoom / 2.0;
                let half_h = size.y * self.zoom / 2.0;
                let rect_shape = Rect::from_center_size(
                    egui::pos2(center.x, center.y),
                    egui::vec2(half_w * 2.0, half_h * 2.0),
                );
                painter.rect_filled(rect_shape, 0.0, Color32::from_rgb(40, 40, 60));
                painter.rect_stroke(rect_shape, 0.0, Stroke::new(1.0, Color32::from_rgb(80, 80, 120)));
                painter.text(
                    rect_shape.center(),
                    egui::Align2::CENTER_CENTER,
                    format!("📷 {}", path),
                    egui::FontId::proportional(12.0),
                    Color32::from_rgb(150, 150, 180),
                );
            }
            ObjectType2D::Text { content, size, color } => {
                let font_id = egui::FontId::proportional(*size * self.zoom);
                painter.text(
                    egui::pos2(center.x, center.y),
                    egui::Align2::CENTER_CENTER,
                    content,
                    font_id,
                    Color32::from_rgba_premultiplied(
                        (color[0] * 255.0) as u8,
                        (color[1] * 255.0) as u8,
                        (color[2] * 255.0) as u8,
                        (color[3] * 255.0) as u8,
                    ),
                );
            }
            ObjectType2D::Group { children } => {
                // 递归渲染子对象
                // 这里简化处理
                if children.is_empty() {
                    painter.text(
                        egui::pos2(center.x, center.y),
                        egui::Align2::CENTER_CENTER,
                        "📁 Group",
                        egui::FontId::proportional(12.0),
                        Color32::from_rgb(100, 150, 200),
                    );
                }
            }
        }

        // 渲染对象名称
        if obj.name.len() > 1 {
            painter.text(
                egui::pos2(center.x, center.y - 20.0 * self.zoom),
                egui::Align2::CENTER_BOTTOM,
                &obj.name,
                egui::FontId::proportional(10.0 * self.zoom),
                Color32::from_rgb(150, 180, 210),
            );
        }
    }

    /// 获取内存使用
    pub fn memory_usage(&self) -> usize {
        std::mem::size_of_val(self)
    }
}
