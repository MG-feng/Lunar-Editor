// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use anyhow::{Result, anyhow};
use std::path::Path;
use wasmtime::{
    Engine, Module, Store, Linker, Func, Val,
    Config, OptLevel, Strategy, Instance
};
use tracing::{info, debug, error};

/// WASM运行时主机
pub struct WasmHost {
    engine: Engine,
    linker: Linker<()>,
}

impl WasmHost {
    pub fn new() -> Result<Self> {
        info!("Initializing WASM host...");

        // 配置引擎
        let mut config = Config::new();
        config.cranelift_opt_level(OptLevel::SpeedAndSize);
        config.strategy(Strategy::Cranelift);
        config.wasm_memory64(false);
        config.wasm_threads(false);
        config.wasm_simd(false);
        config.wasm_bulk_memory(true);
        config.wasm_reference_types(false);
        config.wasm_multi_value(true);
        config.wasm_multi_memory(false);
        config.wasm_tail_call(false);
        config.wasm_relaxed_simd(false);

        // 启用缓存
        #[cfg(feature = "wasm_cache")]
        config.cache_config_load(Default::default())?;

        let engine = Engine::new(&config)?;

        // 创建链接器
        let mut linker = Linker::new(&engine);

        // 注册导入函数
        linker.func_wrap("env", "log", |message: i32| {
            debug!("Plugin log: {}", message);
            Ok(())
        })?;

        linker.func_wrap("env", "warn", |message: i32| {
            // 警告日志
            Ok(())
        })?;

        linker.func_wrap("env", "error", |message: i32| {
            // 错误日志
            Ok(())
        })?;

        Ok(Self { engine, linker })
    }

    /// 加载WASM模块
    pub fn load_module(&self, path: &Path) -> Result<Module> {
        debug!("Loading WASM module: {}", path.display());

        let wasm_bytes = std::fs::read(path)?;

        // 验证WASM文件
        if wasm_bytes.len() < 8 {
            return Err(anyhow!("Invalid WASM file: too small"));
        }

        // 检查WASM魔数
        if &wasm_bytes[0..4] != b"\0asm" {
            return Err(anyhow!("Invalid WASM magic number"));
        }

        let module = Module::new(&self.engine, &wasm_bytes)?;
        debug!("✅ WASM module loaded: {}", path.display());

        Ok(module)
    }

    /// 执行插件初始化
    pub fn execute_init(&self, module: &Module) -> Result<()> {
        let mut store = Store::new(&self.engine, ());
        let instance = self.linker.instantiate(&mut store, module)?;

        // 查找并调用初始化函数
        if let Some(init_func) = instance.get_func(&mut store, "init") {
            let result = init_func.call(&mut store, &[], &mut [])?;
            debug!("Plugin init function executed successfully");
            Ok(())
        } else {
            // 没有init函数，插件可能不需要初始化
            debug!("No init function found, skipping");
            Ok(())
        }
    }

    /// 执行插件更新
    pub fn execute_update(&self, module: &Module, dt: f32) -> Result<()> {
        let mut store = Store::new(&self.engine, ());
        let instance = self.linker.instantiate(&mut store, module)?;

        // 查找并调用更新函数
        if let Some(update_func) = instance.get_func(&mut store, "update") {
            let dt_val = Val::F32(dt);
            let result = update_func.call(&mut store, &[dt_val], &mut [])?;
            Ok(())
        } else {
            // 没有update函数，插件可能不需要每帧更新
            Ok(())
        }
    }

    /// 执行插件函数
    pub fn execute_function(
        &self,
        module: &Module,
        func_name: &str,
        params: &[Val]
    ) -> Result<Vec<Val>> {
        let mut store = Store::new(&self.engine, ());
        let instance = self.linker.instantiate(&mut store, module)?;

        if let Some(func) = instance.get_func(&mut store, func_name) {
            let mut results = vec![Val::I32(0); func.ty(&store).results().len()];
            func.call(&mut store, params, &mut results)?;
            Ok(results)
        } else {
            Err(anyhow!("Function not found: {}", func_name))
        }
    }
}
