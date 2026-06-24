// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

mod wgpu_backend;
mod pipeline;

pub use wgpu_backend::RenderBackend;
pub use pipeline::PipelineManager;

use anyhow::Result;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, Adapter, Instance};
use winit::window::Window;
use std::sync::Arc;

/// 渲染后端配置
#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub vsync: bool,
    pub msaa_samples: u32,
    pub power_preference: wgpu::PowerPreference,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            vsync: true,
            msaa_samples: 4,
            power_preference: wgpu::PowerPreference::HighPerformance,
        }
    }
}

/// 渲染统计信息
#[derive(Debug, Clone, Default)]
pub struct RenderStats {
    pub frame_count: u64,
    pub draw_calls: u32,
    pub triangles: u32,
    pub vertices: u32,
    pub gpu_time: std::time::Duration,
    pub cpu_time: std::time::Duration,
}
