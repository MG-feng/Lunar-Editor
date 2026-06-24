// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use egui::{Color32, RichText, Stroke, Ui};
use notify::{RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use tracing::{debug, info};
use walkdir::WalkDir;

/// 文件树
pub struct FileTree {
    root: Option<PathBuf>,
    expanded: Vec<PathBuf>,
    selected: Option<PathBuf>,
    watcher: Option<RecommendedWatcher>,
}

impl FileTree {
    pub fn new() -> Self {
        Self {
            root: None,
            expanded: Vec::new(),
            selected: None,
            watcher: None,
        }
    }

    /// 设置根目录
    pub fn set_root(&mut self, path: &Path) {
        self.root = Some(path.to_path_buf());
        info!("File tree root set to: {}", path.display());

        // 启动文件监视器
        self.start_watcher(path);
    }

    fn start_watcher(&mut self, path: &Path) {
        let (tx, rx) = channel();

        let mut watcher = notify::recommended_watcher(tx).unwrap();
        watcher.watch(path, RecursiveMode::Recursive).unwrap();

        self.watcher = Some(watcher);

        // 处理文件变化通知
        std::thread::spawn(move || {
            for res in rx {
                match res {
                    Ok(event) => {
                        debug!("File event: {:?}", event);
                        // TODO: 刷新文件树
                    }
                    Err(e) => {
                        debug!("File watcher error: {}", e);
                    }
                }
            }
        });
    }

    pub fn render(&mut self, ui: &mut Ui) {
        if let Some(root) = &self.root {
            egui::Frame::new()
                .fill(ui.style().visuals.panel_fill)
                .show(ui, |ui| {
                    ui.heading(
                        RichText::new("📁 Project Files").color(Color32::from_rgb(0, 240, 255)),
                    );
                    ui.separator();
                    self.render_dir(ui, root, 0);
                });
        }
    }

    fn render_dir(&mut self, ui: &mut Ui, path: &Path, depth: usize) {
        let indent = depth * 20;

        // 检查是否是目录
        if path.is_dir() {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "?".to_string());

            let is_expanded = self.expanded.contains(&path.to_path_buf());
            let is_selected = Some(path) == self.selected.as_ref().map(|p| p.as_path());

            ui.horizontal(|ui| {
                ui.add_space(indent as f32);

                // 展开/折叠按钮
                if ui.button(if is_expanded { "▼" } else { "▶" }).clicked() {
                    if is_expanded {
                        self.expanded.retain(|p| p != path);
                    } else {
                        self.expanded.push(path.to_path_buf());
                    }
                }

                // 目录名
                let label = if is_selected {
                    RichText::new(format!("📁 {}", name)).color(Color32::from_rgb(0, 240, 255))
                } else {
                    RichText::new(format!("📁 {}", name)).color(Color32::from_rgb(200, 210, 220))
                };

                if ui.selectable_label(false, label).clicked() {
                    self.selected = Some(path.to_path_buf());
                }
            });

            // 渲染子项
            if is_expanded {
                if let Ok(entries) = std::fs::read_dir(path) {
                    let mut dirs = Vec::new();
                    let mut files = Vec::new();

                    for entry in entries.flatten() {
                        let entry_path = entry.path();
                        if entry_path.is_dir() {
                            dirs.push(entry_path);
                        } else {
                            files.push(entry_path);
                        }
                    }

                    // 先渲染目录
                    dirs.sort();
                    for dir in dirs {
                        self.render_dir(ui, &dir, depth + 1);
                    }

                    // 再渲染文件
                    files.sort();
                    for file in files {
                        self.render_file(ui, &file, depth + 1);
                    }
                }
            }
        }
    }

    fn render_file(&mut self, ui: &mut Ui, path: &Path, depth: usize) {
        let indent = depth * 20;
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "?".to_string());

        let is_selected = Some(path) == self.selected.as_ref().map(|p| p.as_path());

        // 根据扩展名选择图标
        let icon = match path.extension().and_then(|ext| ext.to_str()) {
            Some("rs") => "🦀",
            Some("lunar") | Some("l") => "🌙",
            Some("json") => "📋",
            Some("toml") => "⚙️",
            Some("md") => "📝",
            Some("png") | Some("jpg") | Some("jpeg") | Some("gif") => "🖼️",
            Some("mp3") | Some("wav") => "🎵",
            Some("mp4") | Some("avi") => "🎬",
            _ => "📄",
        };

        ui.horizontal(|ui| {
            ui.add_space(indent as f32);

            let label = if is_selected {
                RichText::new(format!("{} {}", icon, name)).color(Color32::from_rgb(0, 240, 255))
            } else {
                RichText::new(format!("{} {}", icon, name)).color(Color32::from_rgb(180, 190, 200))
            };

            if ui.selectable_label(false, label).clicked() {
                self.selected = Some(path.to_path_buf());
                // TODO: 打开文件
            }
        });
    }

    pub fn update(&mut self, _dt: f32) {
        // 更新文件树
    }
}
