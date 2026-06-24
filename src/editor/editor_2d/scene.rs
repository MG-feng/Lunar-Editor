// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use glam::Vec2;
use std::collections::HashMap;
use uuid::Uuid;

/// 2D场景
pub struct Scene2D {
    objects: HashMap<usize, SceneObject2D>,
    next_id: usize,
}

/// 2D场景对象
#[derive(Debug, Clone)]
pub struct SceneObject2D {
    pub id: usize,
    pub uuid: Uuid,
    pub name: String,
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
    pub visible: bool,
    pub object_type: ObjectType2D,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum ObjectType2D {
    Rectangle { width: f32, height: f32, color: [f32; 4] },
    Circle { radius: f32, color: [f32; 4] },
    Triangle { points: [Vec2; 3], color: [f32; 4] },
    Image { path: String, size: Vec2 },
    Text { content: String, size: f32, color: [f32; 4] },
    Group { children: Vec<usize> },
}

impl Scene2D {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            next_id: 0,
        }
    }

    /// 创建对象
    pub fn create_object(&mut self, object_type: ObjectType2D, name: String) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let obj = SceneObject2D {
            id,
            uuid: Uuid::new_v4(),
            name: if name.is_empty() { format!("Object_{}", id) } else { name },
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
            visible: true,
            object_type,
            properties: HashMap::new(),
        };

        self.objects.insert(id, obj);
        id
    }

    /// 删除对象
    pub fn delete_object(&mut self, id: usize) -> Option<SceneObject2D> {
        self.objects.remove(&id)
    }

    /// 获取对象
    pub fn get_object(&self, id: usize) -> Option<&SceneObject2D> {
        self.objects.get(&id)
    }

    /// 获取对象（可变）
    pub fn get_object_mut(&mut self, id: usize) -> Option<&mut SceneObject2D> {
        self.objects.get_mut(&id)
    }

    /// 获取所有对象
    pub fn get_all_objects(&self) -> Vec<&SceneObject2D> {
        self.objects.values().collect()
    }

    /// 获取所有对象（可变）
    pub fn get_all_objects_mut(&mut self) -> Vec<&mut SceneObject2D> {
        self.objects.values_mut().collect()
    }

    /// 更新场景
    pub fn update(&mut self, dt: f32) {
        // 更新所有对象
        for obj in self.objects.values_mut() {
            // 在这里可以添加动画等更新逻辑
        }
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
        }
        total
    }
}

impl Default for Scene2D {
    fn default() -> Self {
        Self::new()
    }
}
