// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use anyhow::{Result, anyhow};
use wgpu::{
    Adapter, Backend, Device, DeviceDescriptor, Features, Instance,
    InstanceDescriptor, Limits, Queue, RequestAdapterOptions,
    Surface, SurfaceConfiguration, SurfaceTarget, TextureFormat,
    PowerPreference
};
use winit::window::Window;
use std::sync::Arc;
use tracing::{info, debug, warn, error};

use super::{RenderConfig, RenderStats};

/// 渲染后端 - 基于 wgpu
pub struct RenderBackend {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub config: SurfaceConfiguration,
    pub render_config: RenderConfig,
    pub stats: RenderStats,
    pub surface_format: TextureFormat,
}

impl RenderBackend {
    /// 创建新的渲染后端
    pub async fn new(window: &Arc<Window>) -> Result<Self> {
        info!("Initializing render backend...");

        // 创建实例
        let instance = Instance::new(InstanceDescriptor {
            backends: Backend::PRIMARY,
            ..Default::default()
        });

        // 创建表面
        let surface = unsafe { instance.create_surface(window)? };

        // 获取适配器
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow!("Failed to find suitable adapter"))?;

        info!("Adapter: {:?}", adapter.get_info());

        // 获取设备
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("Lunar Editor Device"),
                    features: Features::empty(),
                    limits: Limits::default(),
                },
                None,
            )
            .await?;

        // 配置表面
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let size = window.inner_size();
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        info!("✅ Render backend initialized successfully");
        info!("   - Format: {:?}", surface_format);
        info!("   - Size: {}x{}", config.width, config.height);
        info!("   - Adapter: {:?}", adapter.get_info().name);

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
            surface,
            config,
            render_config: RenderConfig::default(),
            stats: RenderStats::default(),
            surface_format,
        })
    }

    /// 调整窗口大小
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);

        debug!("Resized to {}x{}", width, height);
    }

    /// 获取当前帧
    pub fn get_frame(&self) -> Result<wgpu::SurfaceTexture> {
        Ok(self.surface.get_current_texture()?)
    }

    /// 结束帧
    pub fn present(&self, texture: wgpu::SurfaceTexture) {
        texture.present();
    }

    /// 创建命令编码器
    pub fn create_command_encoder(&self) -> wgpu::CommandEncoder {
        self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("main_encoder"),
        })
    }

    /// 提交命令缓冲区
    pub fn submit(&self, command_buffers: Vec<wgpu::CommandBuffer>) {
        self.queue.submit(command_buffers);
    }

    /// 创建缓冲区
    pub fn create_buffer(&self, label: &str, size: u64, usage: wgpu::BufferUsages) -> wgpu::Buffer {
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size,
            usage,
            mapped_at_creation: false,
        })
    }

    /// 创建缓冲区并填充数据
    pub fn create_buffer_with_data<T: bytemuck::Pod>(
        &self,
        label: &str,
        data: &[T],
        usage: wgpu::BufferUsages,
    ) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;
        self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: bytemuck::cast_slice(data),
            usage,
        })
    }

    /// 创建纹理
    pub fn create_texture(
        &self,
        label: &str,
        size: wgpu::Extent3d,
        format: wgpu::TextureFormat,
        usage: wgpu::TextureUsages,
    ) -> wgpu::Texture {
        self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage,
            view_formats: &[],
        })
    }

    /// 等待GPU完成所有工作
    pub fn poll(&self) {
        self.device.poll(wgpu::Maintain::Wait);
    }

    /// 更新渲染统计
    pub fn update_stats(&mut self, draw_calls: u32, triangles: u32, vertices: u32) {
        self.stats.frame_count += 1;
        self.stats.draw_calls = draw_calls;
        self.stats.triangles = triangles;
        self.stats.vertices = vertices;
    }
}

/// 渲染上下文 - 用于渲染时传递
pub struct RenderContext<'a> {
    pub device: &'a Device,
    pub queue: &'a Queue,
    pub surface: &'a Surface<'static>,
    pub config: &'a SurfaceConfiguration,
}
