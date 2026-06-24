// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use egui::{Color32, Style, Visuals, FontId, FontFamily, TextStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 颜色方案 - 霓虹科幻风格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    // 背景色
    pub bg_primary: Color32,
    pub bg_secondary: Color32,
    pub bg_panel: Color32,
    pub bg_hover: Color32,
    pub bg_window: Color32,

    // 霓虹色
    pub neon_cyan: Color32,
    pub neon_purple: Color32,
    pub neon_pink: Color32,
    pub neon_green: Color32,
    pub neon_orange: Color32,

    // 文本色
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_dim: Color32,
    pub text_selected: Color32,

    // 边框色
    pub border: Color32,
    pub border_active: Color32,
    pub border_hover: Color32,

    // 状态色
    pub selection: Color32,
    pub error: Color32,
    pub warning: Color32,
    pub success: Color32,

    // 滚动条
    pub scrollbar: Color32,
    pub scrollbar_hover: Color32,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::dark()
    }
}

impl ColorScheme {
    /// 暗色主题 (霓虹风格)
    pub fn dark() -> Self {
        Self {
            bg_primary: Color32::from_rgb(10, 14, 23),
            bg_secondary: Color32::from_rgb(17, 25, 39),
            bg_panel: Color32::from_rgb(13, 21, 32),
            bg_hover: Color32::from_rgb(26, 42, 58),
            bg_window: Color32::from_rgb(8, 10, 18),

            neon_cyan: Color32::from_rgb(0, 240, 255),
            neon_purple: Color32::from_rgb(168, 85, 247),
            neon_pink: Color32::from_rgb(255, 45, 149),
            neon_green: Color32::from_rgb(57, 255, 20),
            neon_orange: Color32::from_rgb(255, 107, 53),

            text_primary: Color32::from_rgb(232, 237, 245),
            text_secondary: Color32::from_rgb(138, 155, 181),
            text_dim: Color32::from_rgb(74, 90, 112),
            text_selected: Color32::WHITE,

            border: Color32::from_rgb(30, 50, 80),
            border_active: Color32::from_rgb(0, 240, 255),
            border_hover: Color32::from_rgb(60, 90, 130),

            selection: Color32::from_rgba_premultiplied(26, 79, 120, 180),
            error: Color32::from_rgb(255, 23, 68),
            warning: Color32::from_rgb(255, 171, 0),
            success: Color32::from_rgb(0, 230, 118),

            scrollbar: Color32::from_rgb(40, 60, 90),
            scrollbar_hover: Color32::from_rgb(60, 90, 130),
        }
    }

    /// 转换为 egui 样式
    pub fn to_egui_style(&self) -> Style {
        let mut style = Style::default();

        // 设置视觉样式
        style.visuals = Visuals {
            dark_mode: true,
            panel_fill: self.bg_panel,
            window_fill: self.bg_window,
            window_stroke: egui::Stroke::new(1.0, self.border),
            widgets: egui::style::Widgets {
                noninteractive: egui::style::WidgetVisuals {
                    bg_fill: self.bg_secondary,
                    bg_stroke: egui::Stroke::new(1.0, self.border),
                    fg_stroke: egui::Stroke::new(1.0, self.text_secondary),
                    corner_radius: 4.0,
                },
                inactive: egui::style::WidgetVisuals {
                    bg_fill: self.bg_secondary,
                    bg_stroke: egui::Stroke::new(1.0, self.border),
                    fg_stroke: egui::Stroke::new(1.0, self.text_primary),
                    corner_radius: 4.0,
                },
                hovered: egui::style::WidgetVisuals {
                    bg_fill: self.bg_hover,
                    bg_stroke: egui::Stroke::new(1.0, self.border_hover),
                    fg_stroke: egui::Stroke::new(1.0, self.text_primary),
                    corner_radius: 4.0,
                },
                active: egui::style::WidgetVisuals {
                    bg_fill: self.bg_hover,
                    bg_stroke: egui::Stroke::new(1.0, self.border_active),
                    fg_stroke: egui::Stroke::new(1.0, self.neon_cyan),
                    corner_radius: 4.0,
                },
                open: egui::style::WidgetVisuals {
                    bg_fill: self.bg_panel,
                    bg_stroke: egui::Stroke::new(1.0, self.border),
                    fg_stroke: egui::Stroke::new(1.0, self.text_primary),
                    corner_radius: 4.0,
                },
            },
            selection: egui::style::Selection {
                bg_fill: self.selection,
                fg_stroke: egui::Stroke::new(1.0, self.text_selected),
                corner_radius: 2.0,
            },
            ..Default::default()
        };

        // 设置文本样式
        style.text_styles = [
            (TextStyle::Heading, FontId::new(20.0, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(14.0, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(14.0, FontFamily::Monospace)),
            (TextStyle::Button, FontId::new(14.0, FontFamily::Proportional)),
            (TextStyle::Small, FontId::new(10.0, FontFamily::Proportional)),
        ].into();

        style
    }
}

/// 主题管理器
pub struct Theme {
    pub color_scheme: ColorScheme,
    pub dark_mode: bool,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            color_scheme: ColorScheme::dark(),
            dark_mode: true,
        }
    }

    pub fn load_default() -> Self {
        Self::new()
    }

    pub fn to_egui_style(&self) -> Style {
        self.color_scheme.to_egui_style()
    }

    pub fn toggle_dark_mode(&mut self) {
        self.dark_mode = !self.dark_mode;
        if self.dark_mode {
            self.color_scheme = ColorScheme::dark();
        } else {
            // TODO: 实现亮色主题
            self.color_scheme = ColorScheme::dark();
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}
