// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use egui::{Color32, Frame, Rounding, Shadow, Stroke, Ui};

/// 霓虹风格面板
pub struct NeonPanel {
    title: String,
    collapsible: bool,
    open: bool,
}

impl NeonPanel {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            collapsible: true,
            open: true,
        }
    }

    pub fn render<F>(&mut self, ui: &mut Ui, content: F)
    where
        F: FnOnce(&mut Ui),
    {
        let frame = Frame::new()
            .fill(Color32::from_rgb(13, 21, 32))
            .stroke(Stroke::new(1.0, Color32::from_rgb(30, 50, 80)))
            .rounding(Rounding::same(6.0))
            .shadow(Shadow {
                extrusion: 4.0,
                color: Color32::from_rgba_premultiplied(0, 0, 0, 100),
            });

        egui::CollapsingHeader::new(
            RichText::new(&self.title)
                .color(Color32::from_rgb(0, 240, 255))
                .size(14.0),
        )
        .default_open(self.open)
        .open(&mut self.open)
        .show(ui, |ui| {
            frame.show(ui, |ui| {
                ui.add_space(4.0);
                content(ui);
                ui.add_space(4.0);
            });
        });
    }
}
