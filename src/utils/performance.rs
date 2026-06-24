// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use std::collections::VecDeque;
use std::time::{Instant, Duration};
use tracing::debug;

/// 性能监控器
pub struct PerformanceMonitor {
    frame_times: VecDeque<Duration>,
    max_samples: usize,
    current_frame: Instant,
    fps: f32,
    min_fps: f32,
    max_fps: f32,
    avg_frame_time: Duration,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            frame_times: VecDeque::with_capacity(120),
            max_samples: 120,
            current_frame: Instant::now(),
            fps: 0.0,
            min_fps: f32::MAX,
            max_fps: 0.0,
            avg_frame_time: Duration::from_secs(0),
        }
    }

    /// 开始新帧
    pub fn begin_frame(&mut self) {
        self.current_frame = Instant::now();
    }

    /// 结束帧并记录帧时间
    pub fn end_frame(&mut self) {
        let elapsed = self.current_frame.elapsed();
        self.frame_times.push_back(elapsed);

        if self.frame_times.len() > self.max_samples {
            self.frame_times.pop_front();
        }

        // 计算FPS
        self.fps = 1.0 / elapsed.as_secs_f32();
        if self.fps < self.min_fps {
            self.min_fps = self.fps;
        }
        if self.fps > self.max_fps {
            self.max_fps = self.fps;
        }

        // 计算平均帧时间
        let total: Duration = self.frame_times.iter().sum();
        self.avg_frame_time = total / self.frame_times.len() as u32;

        // 调试输出
        if self.fps < 30.0 {
            debug!("⚠️  Low FPS: {:.1}", self.fps);
        }
    }

    /// 获取当前FPS
    pub fn fps(&self) -> f32 {
        self.fps
    }

    /// 获取平均FPS
    pub fn avg_fps(&self) -> f32 {
        if self.avg_frame_time.as_secs_f32() > 0.0 {
            1.0 / self.avg_frame_time.as_secs_f32()
        } else {
            0.0
        }
    }

    /// 获取最小FPS
    pub fn min_fps(&self) -> f32 {
        if self.min_fps == f32::MAX {
            0.0
        } else {
            self.min_fps
        }
    }

    /// 获取最大FPS
    pub fn max_fps(&self) -> f32 {
        self.max_fps
    }

    /// 获取帧时间 (毫秒)
    pub fn frame_time_ms(&self) -> f32 {
        self.avg_frame_time.as_secs_f32() * 1000.0
    }

    /// 重置统计
    pub fn reset(&mut self) {
        self.frame_times.clear();
        self.min_fps = f32::MAX;
        self.max_fps = 0.0;
        self.fps = 0.0;
    }

    /// 获取性能报告
    pub fn report(&self) -> String {
        format!(
            "FPS: {:.1} (Min: {:.1}, Max: {:.1}) | Frame: {:.2}ms | Samples: {}",
            self.avg_fps(),
            self.min_fps(),
            self.max_fps(),
            self.frame_time_ms(),
            self.frame_times.len()
        )
    }
}

/// 性能分析器 - 用于测量代码块执行时间
pub struct Profiler {
    name: String,
    start: Instant,
    min: Duration,
    max: Duration,
    total: Duration,
    count: u64,
}

impl Profiler {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            start: Instant::now(),
            min: Duration::MAX,
            max: Duration::ZERO,
            total: Duration::ZERO,
            count: 0,
        }
    }

    pub fn begin(&mut self) {
        self.start = Instant::now();
    }

    pub fn end(&mut self) {
        let elapsed = self.start.elapsed();
        self.count += 1;
        self.total += elapsed;

        if elapsed < self.min {
            self.min = elapsed;
        }
        if elapsed > self.max {
            self.max = elapsed;
        }
    }

    pub fn avg(&self) -> Duration {
        if self.count > 0 {
            self.total / self.count as u32
        } else {
            Duration::ZERO
        }
    }

    pub fn min(&self) -> Duration {
        if self.min == Duration::MAX {
            Duration::ZERO
        } else {
            self.min
        }
    }

    pub fn max(&self) -> Duration {
        self.max
    }

    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn report(&self) -> String {
        format!(
            "{}: Count={}, Avg={:?}, Min={:?}, Max={:?}, Total={:?}",
            self.name,
            self.count,
            self.avg(),
            self.min(),
            self.max(),
            self.total
        )
    }
}

/// 性能计数器
pub struct PerformanceCounter {
    counters: Vec<Profiler>,
}

impl PerformanceCounter {
    pub fn new() -> Self {
        Self {
            counters: Vec::new(),
        }
    }

    pub fn add_profiler(&mut self, profiler: Profiler) {
        self.counters.push(profiler);
    }

    pub fn get_profiler(&mut self, name: &str) -> Option<&mut Profiler> {
        self.counters.iter_mut().find(|p| p.name == name)
    }

    pub fn report_all(&self) -> String {
        let mut report = String::new();
        for counter in &self.counters {
            report.push_str(&counter.report());
            report.push('\n');
        }
        report
    }
}
