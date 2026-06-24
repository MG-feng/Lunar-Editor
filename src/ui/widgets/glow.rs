// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use egui::{Color32, Rect, Stroke, Ui};

/// 发光效果
pub struct GlowEffect {
    intensity: f32,
    color: Color32,
    radius: f32,
}

impl GlowEffect {
    pub fn new() -> Self {
        Self {
            intensity: 0.0,
            color: Color32::from_rgb(0, 240, 255),
            radius: 20.0,
        }
    }

    pub fn update(&mut self, ui: &Ui) {
        // 增加发光强度
        self.intensity = (self.intensity + 0.05).min(1.0);

        // 获取鼠标位置
        let mouse_pos = ui.input().pointer_interact_pos();
        if let Some(pos) = mouse_pos {
            // 在鼠标位置绘制光晕
            let rect =
                Rect::from_center_size(pos, egui::vec2(self.radius * 2.0, self.radius * 2.0));
            let color = self.color.gamma_multiply(self.intensity * 0.3);
            ui.painter().rect_filled(rect, self.radius, color);
        }
    }

    pub fn reset(&mut self) {
        self.intensity = 0.0;
    }
}
