// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use std::collections::HashSet;
use super::Permission;
use tracing::warn;

/// 权限管理器
pub struct PermissionManager {
    allowed_permissions: HashSet<Permission>,
    denied_permissions: HashSet<Permission>,
    require_confirmation: HashSet<Permission>,
    strict_mode: bool,
}

impl PermissionManager {
    pub fn new() -> Self {
        Self {
            allowed_permissions: HashSet::new(),
            denied_permissions: HashSet::new(),
            require_confirmation: HashSet::new(),
            strict_mode: true,
        }
    }

    /// 检查权限是否允许
    pub fn is_permission_allowed(&self, permission: &Permission) -> bool {
        // 首先检查是否被明确拒绝
        if self.denied_permissions.contains(permission) {
            return false;
        }

        // 检查是否被明确允许
        if self.allowed_permissions.contains(permission) {
            return true;
        }

        // 检查是否需要确认
        if self.require_confirmation.contains(permission) {
            warn!("Permission requires confirmation: {:?}", permission);
            return false;
        }

        // 严格模式下默认拒绝
        if self.strict_mode {
            return false;
        }

        // 非严格模式下默认允许
        true
    }

    /// 允许权限
    pub fn allow_permission(&mut self, permission: Permission) {
        self.allowed_permissions.insert(permission);
    }

    /// 拒绝权限
    pub fn deny_permission(&mut self, permission: Permission) {
        self.denied_permissions.insert(permission);
    }

    /// 设置权限需要确认
    pub fn require_confirmation(&mut self, permission: Permission) {
        self.require_confirmation.insert(permission);
    }

    /// 设置严格模式
    pub fn set_strict_mode(&mut self, strict: bool) {
        self.strict_mode = strict;
    }

    /// 获取所有允许的权限
    pub fn get_allowed_permissions(&self) -> &HashSet<Permission> {
        &self.allowed_permissions
    }

    /// 获取所有拒绝的权限
    pub fn get_denied_permissions(&self) -> &HashSet<Permission> {
        &self.denied_permissions
    }
}
