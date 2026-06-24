// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use egui::{Ui, Button, RichText};

/// 2D工具管理器
pub struct ToolManager2D {
    current_tool: ToolType2D,
    tools: Vec<ToolType2D>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolType2D {
    Select,
    Move,
    Rotate,
    Scale,
    Rectangle,
    Circle,
    Triangle,
    Text,
    Eyedropper,
    Zoom,
}

impl ToolType2D {
    pub fn name(&self) -> &'static str {
        match self {
            ToolType2D::Select => "Select",
            ToolType2D::Move => "Move",
            ToolType2D::Rotate => "Rotate",
            ToolType2D::Scale => "Scale",
            ToolType2D::Rectangle => "Rectangle",
            ToolType2D::Circle => "Circle",
            ToolType2D::Triangle => "Triangle",
            ToolType2D::Text => "Text",
            ToolType2D::Eyedropper => "Eyedropper",
            ToolType2D::Zoom => "Zoom",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ToolType2D::Select => "⬆",
            ToolType2D::Move => "✚",
            ToolType2D::Rotate => "⟳",
            ToolType2D::Scale => "⤡",
            ToolType2D::Rectangle => "▭",
            ToolType2D::Circle => "◯",
            ToolType2D::Triangle => "△",
            ToolType2D::Text => "A",
            ToolType2D::Eyedropper => "⊡",
            ToolType2D::Zoom => "⊕",
        }
    }
}

impl ToolManager2D {
    pub fn new() -> Self {
        let tools = vec![
            ToolType2D::Select,
            ToolType2D::Move,
            ToolType2D::Rotate,
            ToolType2D::Scale,
            ToolType2D::Rectangle,
            ToolType2D::Circle,
            ToolType2D::Triangle,
            ToolType2D::Text,
            ToolType2D::Eyedropper,
            ToolType2D::Zoom,
        ];

        Self {
            current_tool: ToolType2D::Select,
            tools,
        }
    }

    /// 渲染工具栏
    pub fn render_toolbar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            for &tool in &self.tools {
                let is_selected = self.current_tool == tool;
                let button = Button::new(
                    RichText::new(tool.icon())
                        .size(20.0)
                )
                .fill(if is_selected {
                    ui.style().visuals.widgets.active.bg_fill
                } else {
                    ui.style().visuals.widgets.inactive.bg_fill
                });

                if ui.add(button).clicked() {
                    self.current_tool = tool;
                }

                if ui.is_hovered() {
                    ui.tooltip_text(tool.name());
                }
            }
        });
    }

    /// 更新工具
    pub fn update(&mut self, _dt: f32) {
        // 更新当前工具的状态
    }

    /// 获取当前工具
    pub fn current_tool(&self) -> ToolType2D {
        self.current_tool
    }
}
