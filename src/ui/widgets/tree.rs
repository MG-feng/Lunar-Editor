// Copyright 2026 Lunar Editor Team
// Licensed under the Apache License, Version 2.0

use egui::{Color32, RichText, Stroke, Ui};
use std::collections::HashMap;

/// 树节点
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub label: String,
    pub icon: String,
    pub children: Vec<TreeNode>,
    pub expanded: bool,
    pub selected: bool,
}

impl TreeNode {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            icon: "📁".to_string(),
            children: Vec::new(),
            expanded: false,
            selected: false,
        }
    }

    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = icon.to_string();
        self
    }

    pub fn add_child(mut self, child: TreeNode) -> Self {
        self.children.push(child);
        self
    }
}

/// 霓虹风格树形控件
pub struct NeonTree {
    root: TreeNode,
    selected_id: Option<usize>,
    node_counter: usize,
    nodes: HashMap<usize, TreeNode>,
}

impl NeonTree {
    pub fn new(root: TreeNode) -> Self {
        let mut nodes = HashMap::new();
        let mut counter = 0;
        Self::flatten_tree(&root, &mut nodes, &mut counter);

        Self {
            root,
            selected_id: None,
            node_counter: counter,
            nodes,
        }
    }

    fn flatten_tree(node: &TreeNode, nodes: &mut HashMap<usize, TreeNode>, counter: &mut usize) {
        nodes.insert(*counter, node.clone());
        *counter += 1;
        for child in &node.children {
            Self::flatten_tree(child, nodes, counter);
        }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        self.render_node(ui, &self.root, 0);
    }

    fn render_node(&mut self, ui: &mut Ui, node: &TreeNode, depth: usize) {
        let indent = depth * 20;

        ui.horizontal(|ui| {
            ui.add_space(indent as f32);

            // 展开/折叠按钮
            if !node.children.is_empty() {
                let icon = if node.expanded { "▼" } else { "▶" };
                if ui.button(icon).clicked() {
                    // 切换展开状态
                    // TODO: 实现展开/折叠
                }
            } else {
                ui.add_space(20.0);
            }

            // 节点标签
            let label = format!("{} {}", node.icon, node.label);
            let is_selected = self.selected_id == Some(0); // TODO: 实际ID

            let text = if is_selected {
                RichText::new(label).color(Color32::from_rgb(0, 240, 255))
            } else {
                RichText::new(label).color(Color32::from_rgb(200, 210, 220))
            };

            if ui.selectable_label(false, text).clicked() {
                self.selected_id = Some(0);
            }
        });

        // 渲染子节点
        if node.expanded {
            for child in &node.children {
                self.render_node(ui, child, depth + 1);
            }
        }
    }
}
