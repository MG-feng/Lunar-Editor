// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use glam::{Vec3, Mat4, Quat};
use egui::Rect;

/// 3D摄像机
pub struct Camera3D {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
    pub orbit_speed: f32,
    pub zoom_speed: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera3D {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(5.0, 5.0, 5.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            fov: 60.0_f32.to_radians(),
            aspect: 1.0,
            near: 0.1,
            far: 1000.0,
            orbit_speed: 0.01,
            zoom_speed: 0.1,
            min_distance: 0.5,
            max_distance: 100.0,
            distance: 10.0,
            yaw: 45.0_f32.to_radians(),
            pitch: 30.0_f32.to_radians(),
        }
    }

    /// 获取视图矩阵
    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    /// 获取投影矩阵
    pub fn get_projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect, self.near, self.far)
    }

    /// 更新摄像机位置
    pub fn update(&mut self, _dt: f32) {
        // 基于球坐标计算位置
        let sin_yaw = self.yaw.sin();
        let cos_yaw = self.yaw.cos();
        let sin_pitch = self.pitch.sin();
        let cos_pitch = self.pitch.cos();

        let x = self.distance * cos_pitch * sin_yaw;
        let y = self.distance * sin_pitch;
        let z = self.distance * cos_pitch * cos_yaw;

        self.position = self.target + Vec3::new(x, y, z);
    }

    /// 轨道旋转
    pub fn orbit(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw += delta_x * self.orbit_speed;
        self.pitch += delta_y * self.orbit_speed;
        self.pitch = self.pitch.clamp(-1.5, 1.5);
    }

    /// 缩放
    pub fn zoom(&mut self, delta: f32) {
        self.distance -= delta * self.zoom_speed;
        self.distance = self.distance.clamp(self.min_distance, self.max_distance);
    }

    /// 重置摄像机
    pub fn reset(&mut self) {
        self.position = Vec3::new(5.0, 5.0, 5.0);
        self.target = Vec3::ZERO;
        self.distance = 10.0;
        self.yaw = 45.0_f32.to_radians();
        self.pitch = 30.0_f32.to_radians();
    }

    /// 设置宽高比
    pub fn set_aspect(&mut self, width: f32, height: f32) {
        if height > 0.0 {
            self.aspect = width / height;
        }
    }

    /// 获取前方向
    pub fn forward(&self) -> Vec3 {
        (self.target - self.position).normalize()
    }

    /// 获取右方向
    pub fn right(&self) -> Vec3 {
        self.forward().cross(self.up).normalize()
    }

    /// 获取上方向
    pub fn up_direction(&self) -> Vec3 {
        self.up
    }

    /// 判断点是否在视锥内
    pub fn is_point_visible(&self, _point: Vec3) -> bool {
        // 简化的视锥体检测
        // TODO: 实现完整的视锥体检测
        true
    }
}

impl Default for Camera3D {
    fn default() -> Self {
        Self::new()
    }
}
