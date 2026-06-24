// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use glam::{Vec3, Quat, Mat4};
use std::collections::HashMap;
use uuid::Uuid;

/// 3D场景
pub struct Scene3D {
    objects: HashMap<usize, SceneObject3D>,
    next_id: usize,
    ambient_color: Vec3,
    ambient_intensity: f32,
}

/// 3D场景对象
#[derive(Debug, Clone)]
pub struct SceneObject3D {
    pub id: usize,
    pub uuid: Uuid,
    pub name: String,
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub visible: bool,
    pub object_type: ObjectType3D,
    pub properties: HashMap<String, String>,
    pub children: Vec<usize>,
    pub parent: Option<usize>,
}

/// 3D对象类型
#[derive(Debug, Clone)]
pub enum ObjectType3D {
    Mesh {
        vertices: Vec<Vertex3D>,
        indices: Vec<u32>,
        material: Material,
    },
    Model {
        path: String,
        meshes: Vec<MeshData>,
    },
    Light {
        light_type: LightType,
        intensity: f32,
        color: Vec3,
        range: f32,
    },
    Camera {
        fov: f32,
        near: f32,
        far: f32,
        orthographic: bool,
        ortho_size: f32,
    },
    Empty,
    Group,
}

/// 3D顶点
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex3D {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub tangent: [f32; 4],
}

/// 网格数据
#[derive(Debug, Clone)]
pub struct MeshData {
    pub vertices: Vec<Vertex3D>,
    pub indices: Vec<u32>,
    pub material: Material,
}

/// 材质
#[derive(Debug, Clone)]
pub struct Material {
    pub name: String,
    pub albedo: Vec3,
    pub metallic: f32,
    pub roughness: f32,
    pub albedo_texture: Option<String>,
    pub normal_texture: Option<String>,
    pub metallic_roughness_texture: Option<String>,
}

/// 光源类型
#[derive(Debug, Clone, Copy)]
pub enum LightType {
    Directional,
    Point,
    Spot,
}

impl Scene3D {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            next_id: 0,
            ambient_color: Vec3::new(0.1, 0.1, 0.15),
            ambient_intensity: 0.3,
        }
    }

    /// 创建对象
    pub fn create_object(&mut self, object_type: ObjectType3D, name: String) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let obj = SceneObject3D {
            id,
            uuid: Uuid::new_v4(),
            name: if name.is_empty() { format!("Object_{}", id) } else { name },
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            visible: true,
            object_type,
            properties: HashMap::new(),
            children: Vec::new(),
            parent: None,
        };

        self.objects.insert(id, obj);
        id
    }

    /// 删除对象
    pub fn delete_object(&mut self, id: usize) -> Option<SceneObject3D> {
        if let Some(obj) = self.objects.remove(&id) {
            // 移除所有子对象
            for child_id in &obj.children {
                self.objects.remove(child_id);
            }
            // 从父对象中移除
            if let Some(parent_id) = obj.parent {
                if let Some(parent) = self.objects.get_mut(&parent_id) {
                    parent.children.retain(|&x| x != id);
                }
            }
            Some(obj)
        } else {
            None
        }
    }

    /// 获取对象
    pub fn get_object(&self, id: usize) -> Option<&SceneObject3D> {
        self.objects.get(&id)
    }

    /// 获取对象（可变）
    pub fn get_object_mut(&mut self, id: usize) -> Option<&mut SceneObject3D> {
        self.objects.get_mut(&id)
    }

    /// 获取所有对象
    pub fn get_all_objects(&self) -> Vec<&SceneObject3D> {
        self.objects.values().collect()
    }

    /// 获取所有对象（可变）
    pub fn get_all_objects_mut(&mut self) -> Vec<&mut SceneObject3D> {
        self.objects.values_mut().collect()
    }

    /// 计算世界矩阵
    pub fn get_world_matrix(&self, id: usize) -> Mat4 {
        if let Some(obj) = self.objects.get(&id) {
            let local = Mat4::from_rotation_translation_scale(
                obj.rotation,
                obj.position,
                obj.scale,
            );

            if let Some(parent_id) = obj.parent {
                return self.get_world_matrix(parent_id) * local;
            }

            local
        } else {
            Mat4::IDENTITY
        }
    }

    /// 更新场景
    pub fn update(&mut self, _dt: f32) {
        // 更新所有对象
        // 这里可以添加动画等更新逻辑
    }

    /// 获取内存使用
    pub fn memory_usage(&self) -> usize {
        let mut total = 0;
        for obj in self.objects.values() {
            total += std::mem::size_of_val(obj);
            total += obj.name.capacity();
            for (k, v) in &obj.properties {
                total += k.capacity() + v.capacity();
            }
            // 估算网格数据大小
            if let ObjectType3D::Mesh { vertices, indices, .. } = &obj.object_type {
                total += vertices.capacity() * std::mem::size_of::<Vertex3D>();
                total += indices.capacity() * std::mem::size_of::<u32>();
            }
            if let ObjectType3D::Model { meshes, .. } = &obj.object_type {
                for mesh in meshes {
                    total += mesh.vertices.capacity() * std::mem::size_of::<Vertex3D>();
                    total += mesh.indices.capacity() * std::mem::size_of::<u32>();
                }
            }
        }
        total
    }

    /// 设置环境光
    pub fn set_ambient(&mut self, color: Vec3, intensity: f32) {
        self.ambient_color = color;
        self.ambient_intensity = intensity;
    }
}

impl Default for Scene3D {
    fn default() -> Self {
        Self::new()
    }
}
