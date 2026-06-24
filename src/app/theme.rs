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
            bg_primary: Color32::from_rgb(10, 14, 23),     // #0A0E17
            bg_secondary: Color32::from_rgb(17, 25, 39),   // #111927
            bg_panel: Color32::from_rgb(13, 21, 32),       // #0D1520
            bg_hover: Color32::from_rgb(26, 42, 58),       // #1A2A3A
            bg_window: Color32::from_rgb(8, 10, 18),       // #080A12

            neon_cyan: Color32::from_rgb(0, 240, 255),     // #00F0FF
            neon_purple: Color32::from_rgb(168, 85, 247),  // #A855F7
            neon_pink: Color32::from_rgb(255, 45, 149),    // #FF2D95
            neon_green: Color32::from_rgb(57, 255, 20),    // #39FF14
            neon_orange: Color32::from_rgb(255, 107, 53),  // #FF6B35

            text_primary: Color32::from_rgb(232, 237, 245), // #E8EDF5
            text_secondary: Color32::from_rgb(138, 155, 181), // #8A9BB5
            text_dim: Color32::from_rgb(74, 90, 112),      // #4A5A70
            text_selected: Color32::WHITE
