// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing::info;
use tracing_subscriber::prelude::*;

fn main() -> anyhow::Result<()> {
    init_logging();
    info!("🚀 Lunar Editor v{} starting...", env!("CARGO_PKG_VERSION"));
    info!("✅ Application initialized");
    info!("👋 Lunar Editor shut down");
    Ok(())
}

fn init_logging() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .pretty();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(fmt_layer)
        .init();
}
