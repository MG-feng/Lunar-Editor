// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use std::sync::atomic::{AtomicUsize, Ordering};
use std::alloc::{GlobalAlloc, Layout, System};
use tracing::{info, warn, error};

/// 内存管理器 - 限制总内存使用
pub struct MemoryManager {
    budget: usize,
    used: AtomicUsize,
    peak: AtomicUsize,
    enabled: bool,
}

impl MemoryManager {
    pub fn new(budget_mb: usize) -> Self {
        let budget = budget_mb * 1024 * 1024;
        info!("Memory manager initialized with {}MB budget", budget_mb);

        Self {
            budget,
            used: AtomicUsize::new(0),
            peak: AtomicUsize::new(0),
            enabled: true,
        }
    }

    /// 分配内存
    pub fn allocate(&self, size: usize) -> Result<(), &'static str> {
        if !self.enabled {
            return Ok(());
        }

        let new_used = self.used.fetch_add(size, Ordering::SeqCst) + size;

        if new_used > self.budget {
            self.used.fetch_sub(size, Ordering::SeqCst);
            error!("Memory limit exceeded! Used: {}MB / {}MB",
                   new_used / (1024 * 1024),
                   self.budget / (1024 * 1024));
            return Err("Memory limit exceeded");
        }

        // 更新峰值
        let current_peak = self.peak.load(Ordering::SeqCst);
        if new_used > current_peak {
            self.peak.store(new_used, Ordering::SeqCst);
        }

        // 检查是否接近限制
        let usage_ratio = new_used as f32 / self.budget as f32;
        if usage_ratio > 0.85 {
            warn!("Memory usage at {:.1}%", usage_ratio * 100.0);
        }

        Ok(())
    }

    /// 释放内存
    pub fn deallocate(&self, size: usize) {
        if self.enabled {
            self.used.fetch_sub(size, Ordering::SeqCst);
        }
    }

    /// 获取当前使用量 (MB)
    pub fn used_mb(&self) -> usize {
        self.used.load(Ordering::SeqCst) / (1024 * 1024)
    }

    /// 获取峰值使用量 (MB)
    pub fn peak_mb(&self) -> usize {
        self.peak.load(Ordering::SeqCst) / (1024 * 1024)
    }

    /// 获取预算 (MB)
    pub fn budget_mb(&self) -> usize {
        self.budget / (1024 * 1024)
    }

    /// 启用/禁用内存管理
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// 获取使用率 (0.0 - 1.0)
    pub fn usage_ratio(&self) -> f32 {
        let used = self.used.load(Ordering::SeqCst);
        used as f32 / self.budget as f32
    }
}

/// 内存跟踪分配器 - 用于调试
#[global_allocator]
static TRACKING_ALLOCATOR: TrackingAllocator = TrackingAllocator;

pub struct TrackingAllocator;

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        if !ptr.is_null() {
            // 跟踪分配
            let size = layout.size();
            // TODO: 更新全局内存统计
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();
        // TODO: 更新全局内存统计
        System.dealloc(ptr, layout);
    }
}

/// 内存统计信息
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub used: usize,
    pub peak: usize,
    pub budget: usize,
    pub allocations: u64,
    pub deallocations: u64,
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self {
            used: 0,
            peak: 0,
            budget: 0,
            allocations: 0,
            deallocations: 0,
        }
    }
}
