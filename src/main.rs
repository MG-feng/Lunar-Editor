// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::all, rust_2018_idioms)]

use std::sync::Arc;
use tracing::{info, error};
use tracing_subscriber::prelude::*;

mod app;
mod editor;
mod ui;
mod plugin_system;
mod project;
mod render_backend;
mod lunar_engine;
mod utils;

use app::EditorApp;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging();

    info!("🚀 Lunar Editor v{} starting...", env!("CARGO_PKG_VERSION"));

    if let Err(e) = check_system_requirements() {
        error!("❌ System requirements check failed: {}", e);
        return Err(e);
    }

    let app = EditorApp::new().await?;

    info!("✅ Application initialized successfully");
    if let Err(e) = app.run().await {
        error!("❌ Application error: {}", e);
        return Err(e);
    }

    info!("👋 Lunar Editor shut down successfully");
    Ok(())
}

fn init_logging() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .pretty();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();
}

fn check_system_requirements() -> anyhow::Result<()> {
    let cpu_count = num_cpus::get();
    info!("💻 System: {} cores", cpu_count);
    Ok(())
}
