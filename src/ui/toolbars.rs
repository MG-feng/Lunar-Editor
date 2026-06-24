// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use egui::{Color32, RichText, Rounding, Stroke, Ui};

/// 工具栏管理器
pub struct ToolbarManager {
    tools: Vec<ToolbarItem>,
    active_tool: Option<String>,
}

/// 工具栏项
pub struct ToolbarItem {
    pub id: String,
    pub label: String,
    pub icon: String,
    pub shortcut: Option<String>,
    pub enabled: bool,
}

impl ToolbarManager {
    pub fn new() -> Self {
        let mut manager = Self {
            tools: Vec::new(),
            active_tool: None,
        };

        // 添加默认工具
        manager.add_tool(ToolbarItem {
            id: "select".to_string(),
            label: "Select".to_string(),
            icon: "⬆".to_string(),
            shortcut: Some("V".to_string()),
            enabled: true,
        });

        manager.add_tool(ToolbarItem {
            id: "move".to_string(),
            label: "Move".to_string(),
            icon: "✚".to_string(),
            shortcut: Some("G".to_string()),
            enabled: true,
        });

        manager.add_tool(ToolbarItem {
            id: "rotate".to_string(),
            label: "Rotate".to_string(),
            icon: "⟳".to_string(),
            shortcut: Some("R".to_string()),
            enabled: true,
        });

        manager.add_tool(ToolbarItem {
            id: "scale".to_string(),
            label: "Scale".to_string(),
            icon: "⤡".to_string(),
            shortcut: Some("S".to_string()),
            enabled: true,
        });

        manager
    }

    pub fn add_tool(&mut self, tool: ToolbarItem) {
        self.tools.push(tool);
    }

    pub fn render(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            for tool in &mut self.tools {
                let is_active = Some(&tool.id) == self.active_tool.as_ref();

                let button = egui::Button::new(
                    RichText::new(format!("{} {}", tool.icon, tool.label))
                        .size(12.0)
                        .color(if is_active {
                            Color32::from_rgb(0, 240, 255)
                        } else if tool.enabled {
                            Color32::WHITE
                        } else {
                            Color32::from_rgb(80, 80, 80)
                        }),
                )
                .fill(if is_active {
                    Color32::from_rgb(20, 40, 60)
                } else {
                    Color32::from_rgb(13, 21, 32)
                })
                .stroke(if is_active {
                    Stroke::new(2.0, Color32::from_rgb(0, 240, 255))
                } else if ui.is_hovered() && tool.enabled {
                    Stroke::new(1.0, Color32::from_rgb(30, 60, 90))
                } else {
                    Stroke::new(1.0, Color32::from_rgb(20, 30, 50))
                })
                .rounding(Rounding::same(4.0));

                if ui.add(button).clicked() && tool.enabled {
                    self.active_tool = Some(tool.id.clone());
                }

                if ui.is_hovered() && tool.enabled {
                    if let Some(shortcut) = &tool.shortcut {
                        ui.tooltip_text(format!("{} ({})", tool.label, shortcut));
                    } else {
                        ui.tooltip_text(&tool.label);
                    }
                }
            }
        });
    }
}
