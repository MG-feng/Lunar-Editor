// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

mod memory;
mod performance;
mod license_check;

pub use memory::MemoryManager;
pub use performance::PerformanceMonitor;
pub use license_check::LicenseChecker;

use std::time::{Duration, Instant};

/// 帧定时器
pub struct FrameTimer {
    last_frame: Instant,
    frame_time: Duration,
    fps: f32,
}

impl FrameTimer {
    pub fn new() -> Self {
        Self {
            last_frame: Instant::now(),
            frame_time: Duration::from_secs(0),
            fps: 0.0,
        }
    }

    pub fn tick(&mut self) -> f32 {
        let now = Instant::now();
        self.frame_time = now - self.last_frame;
        self.last_frame = now;
        self.fps = 1.0 / self.frame_time.as_secs_f32();
        self.fps
    }

    pub fn delta_time(&self) -> f32 {
        self.frame_time.as_secs_f32()
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }
}
