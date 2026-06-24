// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use std::collections::HashSet;
use tracing::{info, warn, error};

/// 许可证检查器 - 确保所有依赖使用兼容许可证
pub struct LicenseChecker {
    allowed_licenses: HashSet<String>,
    blocked_licenses: HashSet<String>,
}

impl LicenseChecker {
    pub fn new() -> Self {
        let mut allowed = HashSet::new();
        allowed.insert("MIT".to_string());
        allowed.insert("Apache-2.0".to_string());
        allowed.insert("BSD-2-Clause".to_string());
        allowed.insert("BSD-3-Clause".to_string());
        allowed.insert("ISC".to_string());
        allowed.insert("CC0-1.0".to_string());
        allowed.insert("Unlicense".to_string());
        allowed.insert("MPL-2.0".to_string());
        allowed.insert("Zlib".to_string());

        let mut blocked = HashSet::new();
        blocked.insert("GPL-1.0".to_string());
        blocked.insert("GPL-2.0".to_string());
        blocked.insert("GPL-3.0".to_string());
        blocked.insert("AGPL-1.0".to_string());
        blocked.insert("AGPL-3.0".to_string());
        blocked.insert("LGPL-2.0".to_string());
        blocked.insert("LGPL-2.1".to_string());
        blocked.insert("LGPL-3.0".to_string());

        Self {
            allowed_licenses: allowed,
            blocked_licenses: blocked,
        }
    }

    /// 检查许可证
    pub fn check_license(&self, license: &str) -> LicenseStatus {
        let license_lower = license.to_lowercase();

        // 检查是否被阻止
        for blocked in &self.blocked_licenses {
            if license_lower.contains(&blocked.to_lowercase()) {
                return LicenseStatus::Blocked(blocked.clone());
            }
        }

        // 检查是否允许
        for allowed in &self.allowed_licenses {
            if license_lower.contains(&allowed.to_lowercase()) {
                return LicenseStatus::Allowed;
            }
        }

        // 未知许可证 - 警告但允许
        LicenseStatus::Unknown(license.to_string())
    }

    /// 检查依赖列表
    pub fn check_dependencies(&self, dependencies: &[(&str, &str)]) -> LicenseCheckResult {
        let mut result = LicenseCheckResult::new();

        for (name, license) in dependencies {
            let status = self.check_license(license);

            match status {
                LicenseStatus::Allowed => {
                    result.allowed.push((name.to_string(), license.to_string()));
                }
                LicenseStatus::Blocked(blocked_by) => {
                    result.blocked.push((
                        name.to_string(),
                        license.to_string(),
                        blocked_by,
                    ));
                    error!("❌ Blocked license: {} ({}) - blocked by {}", name, license, blocked_by);
                }
                LicenseStatus::Unknown(lic) => {
                    result.unknown.push((name.to_string(), lic));
                    warn!("⚠️  Unknown license: {} ({})", name, license);
                }
            }
        }

        result
    }

    /// 生成许可证报告
    pub fn generate_report(&self, result: &LicenseCheckResult) -> String {
        let mut report = String::new();
        report.push_str("========================================\n");
        report.push_str("📋 License Check Report\n");
        report.push_str("========================================\n\n");

        report.push_str(&format!("✅ Allowed: {}\n", result.allowed.len()));
        for (name, license) in &result.allowed {
            report.push_str(&format!("   - {}: {}\n", name, license));
        }

        report.push('\n');
        report.push_str(&format!("⚠️  Unknown: {}\n", result.unknown.len()));
        for (name, license) in &result.unknown {
            report.push_str(&format!("   - {}: {}\n", name, license));
        }

        if !result.blocked.is_empty() {
            report.push('\n');
            report.push_str(&format!("❌ Blocked: {}\n", result.blocked.len()));
            for (name, license, blocked_by) in &result.blocked {
                report.push_str(&format!("   - {}: {} (blocked by {})\n", name, license, blocked_by));
            }
        }

        report.push_str("\n========================================\n");
        report
    }
}

/// 许可证状态
#[derive(Debug, Clone)]
pub enum LicenseStatus {
    Allowed,
    Blocked(String),
    Unknown(String),
}

/// 许可证检查结果
#[derive(Debug, Clone)]
pub struct LicenseCheckResult {
    pub allowed: Vec<(String, String)>,
    pub blocked: Vec<(String, String, String)>,
    pub unknown: Vec<(String, String)>,
}

impl LicenseCheckResult {
    pub fn new() -> Self {
        Self {
            allowed: Vec::new(),
            blocked: Vec::new(),
            unknown: Vec::new(),
        }
    }

    pub fn has_blocked(&self) -> bool {
        !self.blocked.is_empty()
    }

    pub fn has_unknown(&self) -> bool {
        !self.unknown.is_empty()
    }

    pub fn total(&self) -> usize {
        self.allowed.len() + self.blocked.len() + self.unknown.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_checker() {
        let checker = LicenseChecker::new();

        // 测试允许的许可证
        assert!(matches!(
            checker.check_license("MIT"),
            LicenseStatus::Allowed
        ));
        assert!(matches!(
            checker.check_license("Apache-2.0"),
            LicenseStatus::Allowed
        ));

        // 测试阻止的许可证
        if let LicenseStatus::Blocked(blocked) = checker.check_license("GPL-3.0") {
            assert_eq!(blocked, "GPL-3.0");
        } else {
            panic!("GPL-3.0 should be blocked");
        }

        // 测试未知许可证
        assert!(matches!(
            checker.check_license("Custom-License"),
            LicenseStatus::Unknown(_)
        ));
    }
}
