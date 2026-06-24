// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use anyhow::{Result, anyhow};
use std::path::Path;
use glam::{Vec3, Vec2};
use tracing::info;

use super::scene::{Scene3D, SceneObject3D, ObjectType3D, Vertex3D, Material, MeshData};

/// 模型加载器
pub struct ModelLoader {
    supported_formats: Vec<String>,
}

impl ModelLoader {
    pub fn new() -> Self {
        Self {
            supported_formats: vec!["gltf".to_string(), "glb".to_string(), "obj".to_string()],
        }
    }

    /// 加载模型
    pub fn load_model(&self, path: &str, scene: &mut Scene3D) -> Result<usize> {
        let path = Path::new(path);

        if !path.exists() {
            return Err(anyhow!("Model file not found: {}", path.display()));
        }

        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        match ext.to_lowercase().as_str() {
            "gltf" | "glb" => self.load_gltf(path, scene),
            "obj" => self.load_obj(path, scene),
            _ => Err(anyhow!("Unsupported format: {}", ext)),
        }
    }

    fn load_gltf(&self, path: &Path, scene: &mut Scene3D) -> Result<usize> {
        info!("Loading GLTF model: {}", path.display());

        // TODO: 实现完整的GLTF加载
        // 这里创建一个占位模型
        let vertices = vec![
            Vertex3D { position: [-0.5, -0.5, 0.0], normal: [0.0, 0.0, 1.0], uv: [0.0, 0.0], tangent: [1.0, 0.0, 0.0, 0.0] },
            Vertex3D { position: [0.5, -0.5, 0.0], normal: [0.0, 0.0, 1.0], uv: [1.0, 0.0], tangent: [1.0, 0.0, 0.0, 0.0] },
            Vertex3D { position: [0.0, 0.5, 0.0], normal: [0.0, 0.0, 1.0], uv: [0.5, 1.0], tangent: [1.0, 0.0, 0.0, 0.0] },
        ];
        let indices = vec![0, 1, 2];

        let material = Material {
            name: "Default".to_string(),
            albedo: Vec3::new(0.5, 0.5, 0.8),
            metallic: 0.0,
            roughness: 0.5,
            albedo_texture: None,
            normal_texture: None,
            metallic_roughness_texture: None,
        };

        let obj_type = ObjectType3D::Mesh {
            vertices,
            indices,
            material,
        };

        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Model".to_string());

        Ok(scene.create_object(obj_type, name))
    }

    fn load_obj(&self, path: &Path, scene: &mut Scene3D) -> Result<usize> {
        info!("Loading OBJ model: {}", path.display());
        // TODO: 实现OBJ加载
        // 暂时返回一个默认网格
        self.load_gltf(path, scene)
    }

    pub fn memory_usage(&self) -> usize {
        std::mem::size_of_val(self)
    }
}

impl Default for ModelLoader {
    fn default() -> Self {
        Self::new()
    }
}
