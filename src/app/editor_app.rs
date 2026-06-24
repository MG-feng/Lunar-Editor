// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use std::sync::Arc;
use std::time::Instant;
use anyhow::Result;
use egui::{Context, FontDefinitions, FontFamily, FontData};
use egui_wgpu::renderer::Renderer;
use egui_winit::winit::event::{Event, WindowEvent};
use egui_winit::winit::event_loop::{EventLoop, ControlFlow};
use egui_winit::winit::window::{Window, WindowBuilder};
use parking_lot::Mutex;
use tracing::{info, debug, error};
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, TextureFormat};
use winit::dpi::LogicalSize;

use super::{EditorMode, ModeManager, Theme};
use crate::render_backend::RenderBackend;
use crate::plugin_system::PluginManager;
use crate::project::ProjectManager;

/// 主应用程序
pub struct EditorApp {
    // 核心组件
    window: Arc<Window>,
    event_loop: EventLoop<()>,
    render_backend: Arc<RenderBackend>,

    // UI
    egui_context: Context,
    egui_renderer: Renderer,
    theme: Theme,

    // 编辑器
    mode_manager: ModeManager,

    // 插件
    plugin_manager: PluginManager,

    // 项目
    project_manager: ProjectManager,

    // 性能
    frame_time: Instant,
    fps_counter: FpsCounter,

    // 状态
    running: bool,
    needs_repaint: bool,
}

impl EditorApp {
    /// 创建新的应用实例
    pub async fn new() -> Result<Self> {
        info!("Initializing Lunar Editor...");

        // 创建事件循环
        let event_loop = EventLoop::new();

        // 创建窗口
        let window = Arc::new(WindowBuilder::new()
            .with_title("Lunar Editor")
            .with_inner_size(LogicalSize::new(1280.0, 720.0))
            .with_min_inner_size(LogicalSize::new(800.0, 600.0))
            .with_resizable(true)
            .with_decorations(true)
            .with_visible(true)
            .build(&event_loop)?);

        // 初始化渲染后端
        let render_backend = Arc::new(RenderBackend::new(&window).await?);

        // 初始化egui
        let egui_context = Context::default();
        let egui_renderer = Renderer::new(
            &render_backend.device,
            render_backend.surface.get_capabilities(&render_backend.adapter).formats[0],
            None,
            1,
        );

        // 加载主题
        let theme = Theme::load_default();

        // 设置字体
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert("JetBrainsMono".to_owned(), FontData::from_static(include_bytes!("../../assets/fonts/JetBrainsMono.ttf")));
        fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "JetBrainsMono".to_owned());
        fonts.families.get_mut(&FontFamily::Monospace).unwrap().insert(0, "JetBrainsMono".to_owned());
        egui_context.set_fonts(fonts);

        // 应用主题
        egui_context.set_style(theme.to_egui_style());

        // 初始化管理器
        let mode_manager = ModeManager::new();
        let plugin_manager = PluginManager::new()?;
        let project_manager = ProjectManager::new();

        Ok(Self {
            window,
            event_loop,
            render_backend,
            egui_context,
            egui_renderer,
            theme,
            mode_manager,
            plugin_manager,
            project_manager,
            frame_time: Instant::now(),
            fps_counter: FpsCounter::new(),
            running: true,
            needs_repaint: true,
        })
    }

    /// 运行主循环
    pub async fn run(mut self) -> Result<()> {
        info!("Starting main loop...");

        let window = self.window.clone();
        let render_backend = self.render_backend.clone();

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = if self.running {
                ControlFlow::Poll
            } else {
                ControlFlow::Exit
            };

            match event {
                Event::WindowEvent { window_id, event } if window_id == window.id() => {
                    self.handle_window_event(&event);
                }
                Event::MainEventsCleared => {
                    self.update();
                    self.render();
                }
                Event::LoopDestroyed => {
                    info!("Event loop destroyed");
                }
                _ => {}
            }
        });
    }

    /// 处理窗口事件
    fn handle_window_event(&mut self, event: &WindowEvent) {
        // 将事件传递给egui
        let response = self.egui_context.on_window_event(&self.window, event);

        // 处理窗口事件
        match event {
            WindowEvent::CloseRequested => {
                self.shutdown();
            }
            WindowEvent::Resized(size) => {
                self.render_backend.resize(*size);
                self.needs_repaint = true;
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.egui_context.set_pixels_per_point(*scale_factor);
                self.needs_repaint = true;
            }
            _ => {}
        }
    }

    /// 更新逻辑
    fn update(&mut self) {
        let now = Instant::now();
        let dt = (now - self.frame_time).as_secs_f32();
        self.frame_time = now;

        // 更新FPS计数器
        self.fps_counter.update();

        // 更新各个子系统
        self.mode_manager.update(dt);
        self.plugin_manager.update(dt);
        self.project_manager.update(dt);

        // 请求重绘
        if self.needs_repaint {
            self.window.request_redraw();
            self.needs_repaint = false;
        }
    }

    /// 渲染
    fn render(&mut self) {
        let start = Instant::now();

        // 开始帧
        let raw_input = self.egui_context.take_input();
        let full_output = self.egui_context.run(raw_input, |ctx| {
            self.render_ui(ctx);
        });

        // 处理输出
        if full_output.needs_repaint() {
            self.needs_repaint = true;
        }

        // 渲染
        let (device, queue, surface) = (
            &self.render_backend.device,
            &self.render_backend.queue,
            &self.render_backend.surface,
        );

        let output = surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("render_encoder"),
        });

        let screen_descriptor = self.egui_context.screen_descriptor();
        let pixels_per_point = self.egui_context.pixels_per_point();

        // 构建渲染pass
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("main_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.04,
                        g: 0.04,
                        b: 0.06,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // 使用egui渲染器
        let renderer = &mut self.egui_renderer;
        renderer
            .update(&self.egui_context, screen_descriptor, &device, &queue)
            .unwrap();

        renderer
            .render(&mut encoder, &view, pass, &self.egui_context, pixels_per_point)
            .unwrap();

        queue.submit(std::iter::once(encoder.finish()));
        output.present();

        let elapsed = start.elapsed();
        debug!("Render time: {:?}", elapsed);
    }

    /// 渲染UI
    fn render_ui(&mut self, ctx: &egui::Context) {
        // 主菜单栏
        self.render_menu_bar(ctx);

        // 主区域
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_main_panel(ui);
        });
    }

    /// 渲染菜单栏
    fn render_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Project").clicked() {
                        // TODO: 创建新项目
                    }
                    if ui.button("Open Project").clicked() {
                        // TODO: 打开项目
                    }
                    ui.separator();
                    if ui.button("Save").clicked() {
                        // TODO: 保存
                    }
                    if ui.button("Save As").clicked() {
                        // TODO: 另存为
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        self.shutdown();
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() { /* TODO */ }
                    if ui.button("Redo").clicked() { /* TODO */ }
                    ui.separator();
                    if ui.button("Cut").clicked() { /* TODO */ }
                    if ui.button("Copy").clicked() { /* TODO */ }
                    if ui.button("Paste").clicked() { /* TODO */ }
                });

                ui.menu_button("View", |ui| {
                    if ui.button("Toggle Theme").clicked() {
                        self.theme.toggle_dark_mode();
                        ctx.set_style(self.theme.to_egui_style());
                    }
                    ui.separator();
                    if ui.button("Reset Layout").clicked() { /* TODO */ }
                });

                ui.menu_button("Mode", |ui| {
                    if ui.button("Text Editor").clicked() {
                        self.mode_manager.switch(EditorMode::Text2D);
                    }
                    if ui.button("2D Editor").clicked() {
                        self.mode_manager.switch(EditorMode::Visual2D);
                    }
                    if ui.button("3D Editor").clicked() {
                        self.mode_manager.switch(EditorMode::Visual3D);
                    }
                });

                ui.menu_button("Plugins", |ui| {
                    if ui.button("Manage Plugins").clicked() { /* TODO */ }
                    ui.separator();
                    if ui.button("Reload All").clicked() { /* TODO */ }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() { /* TODO */ }
                    if ui.button("Documentation").clicked() { /* TODO */ }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("FPS: {}", self.fps_counter.fps()));
                    ui.separator();
                    ui.label(format!("Mode: {}", self.mode_manager.current_mode()));
                    ui.separator();
                    ui.label(format!("Memory: {:.1}MB", self.get_memory_usage()));
                });
            });
        });
    }

    /// 渲染主面板
    fn render_main_panel(&mut self, ui: &mut egui::Ui) {
        // 获取当前编辑器并渲染
        self.mode_manager.render(ui);
    }

    /// 获取内存使用
    fn get_memory_usage(&self) -> f32 {
        // TODO: 实现内存监控
        0.0
    }

    /// 关闭应用
    fn shutdown(&mut self) {
        info!("Shutting down...");
        self.running = false;
        self.event_loop.exit();
    }
}

/// FPS计数器
struct FpsCounter {
    frames: u32,
    last_update: Instant,
    current_fps: u32,
}

impl FpsCounter {
    fn new() -> Self {
        Self {
            frames: 0,
            last_update: Instant::now(),
            current_fps: 0,
        }
    }

    fn update(&mut self) {
        self.frames += 1;
        let now = Instant::now();
        if now - self.last_update >= std::time::Duration::from_secs(1) {
            self.current_fps = self.frames;
            self.frames = 0;
            self.last_update = now;
        }
    }

    fn fps(&self) -> u32 {
        self.current_fps
    }
}
