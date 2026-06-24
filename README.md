```markdown
# 🌙 Lunar Editor

**A high-performance code editor with 2D/3D visualization capabilities**

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)]()

## ✨ Features

- 🚀 **Performance First**: Memory usage under 500MB
- 📝 **Text Editor**: Advanced code editing with syntax highlighting
- 🎨 **2D Visualization**: Visual editor for 2D scenes
- 🌍 **3D Visualization**: Full 3D scene editing with real-time preview
- 🔌 **Plugin System**: WASM-based plugins with extensive permissions
- 🌙 **Lunar Language**: Native support for the Lunar programming language
- 🎯 **Multi-Mode**: Switch between text, 2D, and 3D modes seamlessly
- 🎨 **Modern UI**: Cyberpunk neon theme inspired by lunar-nexus.web1337.net

## 🏗️ Architecture

```
Lunar Editor
├── Core Engine (Rust + wgpu)
├── UI Framework (egui)
├── Text Editor (ropey + tree-sitter)
├── 2D Editor (custom scene graph)
├── 3D Editor (wgpu + glTF support)
├── Plugin System (wasmtime)
└── Lunar Engine (self-hosted VM)
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+
- Cargo
- Git

### Build from Source

```bash
# Clone the repository
git clone https://github.com/your-org/lunar-editor.git
cd lunar-editor

# Build in release mode
cargo build --release

# Run
./target/release/lunar-editor
```

### Development

```bash
# Run in development mode
cargo run

# Run with logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Run benchmarks
cargo bench

# Check memory usage
cargo build --release && ./target/release/lunar-editor
```

## 📦 Project Structure

```
Lunar-Editor/
├── src/
│   ├── app/              # Application core
│   │   ├── editor_app.rs # Main application
│   │   ├── modes.rs      # Mode management
│   │   └── theme.rs      # UI theme system
│   ├── editor/           # Editor implementations
│   │   ├── text_editor/  # Text editor with syntax highlighting
│   │   ├── editor_2d/    # 2D visualization editor
│   │   └── editor_3d/    # 3D scene editor
│   ├── ui/               # UI components
│   │   ├── widgets/      # Custom widgets
│   │   ├── docks.rs      # Docking system
│   │   └── toolbars.rs   # Toolbars
│   ├── plugin_system/    # WASM plugin system
│   │   ├── api/          # Plugin API
│   │   ├── permissions.rs # Permission system
│   │   └── wasm_host.rs  # WASM runtime
│   ├── project/          # Project management
│   ├── render_backend/   # Graphics backend (wgpu)
│   ├── lunar_engine/     # Lunar language support
│   └── utils/            # Utilities
├── assets/               # Embedded resources
│   ├── fonts/            # Font files
│   ├── icons/            # Icons
│   └── themes/           # Theme configurations
├── project/              # User projects
├── plugins/              # Plugin installations
├── scripts/              # Build and utility scripts
├── Cargo.toml            # Dependencies
├── build.rs              # Build script
└── LICENSE               # Apache-2.0
```

## 🔌 Plugin System

Lunar Editor supports WASM-based plugins with extensive permissions, similar to Minecraft mods.

### Plugin Example

```rust
use lunar_editor_plugin::prelude::*;

#[plugin]
pub struct MyPlugin {
    name: String,
}

#[plugin_init]
fn init(api: &mut PluginAPI) -> Result<(), PluginError> {
    // Register a custom tool
    api.editor.register_tool("my_tool", MyTool::new());
    
    // Add menu item
    api.editor.add_menu_item("Tools/My Tool", || {
        println!("Hello from plugin!");
    });
    
    // Access scene
    let scene = api.scene.current()?;
    scene.add_node(Box::new(MyCustomNode::new()));
    
    // Register hotkey
    api.editor.register_hotkey("Ctrl+Shift+M", || {
        // Handle hotkey
    });
    
    Ok(())
}

#[plugin_shutdown]
fn shutdown() {
    // Cleanup resources
}
```

### Permissions System

Plugins request permissions in their manifest:

```toml
[plugin]
id = "my-plugin"
name = "My Plugin"
version = "1.0.0"

[permissions]
fs_read = ["/project/**", "/assets/**"]
fs_write = ["/project/output/**"]
editor_modify = true
scene_modify = true
network_http = ["api.github.com"]
```

## 🌙 Lunar Language

Lunar is a hybrid static/dynamic language designed for high-performance applications and game development.

### Key Features

- **Controlled Memory Management**: `@gc auto`, `@gc manual`, `@gc hybrid`
- **Dual Mode**: Interpreted (development) and Compiled (production)
- **Rich Type System**: Optional static typing with inference
- **Modern Syntax**: Similar to Python + C# + Rust
- **Hardware Control**: CPU/GPU binding, memory limits
- **Cross-language**: Import JSON, CSV, TOML natively

### Code Example

```lunar
@gc hybrid
@compile_mode compiled
@cpu_core 4
@max_memory 2gb

local config = load("config.json")
global shared_data = {}

class GameObject {
    property position: Vector3
    property rotation: Quaternion
    
    func new(pos: Vector3) {
        self.position = pos
        self.rotation = Quaternion.identity()
    }
    
    func update(dt: f32) {
        self.position += velocity * dt
    }
}

enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver
}

func main() {
    // Range loop with condition
    for i in 1..100 where i % 2 == 0 {
        data |> map(transform) |> filter(is_valid)
    }
    
    // Pattern matching
    match state {
        GameState.Menu => show_menu()
        GameState.Playing => update_game()
        GameState.Paused => show_pause_menu()
        GameState.GameOver => show_game_over()
    }
    
    // Null safety
    let value = obj?.field ?? default_value
    
    // String interpolation
    let message = $"Hello {name}!"
}
```

## 🎨 UI Theme

Inspired by lunar-nexus.web1337.net, featuring a cyberpunk neon aesthetic:

- **Background**: Deep space black (#0A0E17)
- **Neon Cyan**: #00F0FF (primary accent)
- **Neon Purple**: #A855F7 (secondary accent)
- **Neon Pink**: #FF2D95 (warning/active)
- **Text Primary**: #E8EDF5 (bright white)
- **Text Secondary**: #8A9BB5 (dimmed)

The theme is fully customizable through JSON configuration.

## 📊 Performance Goals

| Metric | Target | Status |
|--------|--------|--------|
| Startup Time | < 1.5s | 🚧 In Progress |
| Memory (Idle) | < 150MB | 🚧 In Progress |
| Memory (3D Scene) | < 450MB | 🚧 In Progress |
| Frame Rate (2D) | 144+ FPS | 🚧 In Progress |
| Frame Rate (3D) | 60+ FPS | 🚧 In Progress |
| Plugin Load Time | < 100ms | 🚧 In Progress |
| Save/Load (100KB) | < 500ms | 🚧 In Progress |

## 🛠️ Development Status

| Component | Status | Progress |
|-----------|--------|----------|
| Core Framework | 🚧 In Progress | 10% |
| Text Editor | ⏳ Planned | 0% |
| 2D Editor | ⏳ Planned | 0% |
| 3D Editor | ⏳ Planned | 0% |
| Plugin System | ⏳ Planned | 0% |
| Lunar Engine | ⏳ Planned | 0% |
| UI System | 🚧 In Progress | 15% |

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Fork the repository
# Clone your fork
git clone https://github.com/your-username/lunar-editor.git
cd lunar-editor

# Create a branch
git checkout -b feature/your-feature

# Make changes and test
cargo test
cargo run

# Submit a pull request
```

## 📚 Documentation

- [User Guide](docs/USER_GUIDE.md)
- [API Reference](docs/API.md)
- [Plugin Development Guide](docs/PLUGINS.md)
- [Lunar Language Reference](docs/LUNAR.md)
- [Architecture Overview](docs/ARCHITECTURE.md)

## 📄 License

Copyright 2026 Lunar Editor Team

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

## 🙏 Acknowledgments

- [egui](https://github.com/emilk/egui) - Immediate mode GUI framework
- [wgpu](https://github.com/gfx-rs/wgpu) - Cross-platform graphics API
- [wasmtime](https://github.com/bytecodealliance/wasmtime) - WASM runtime
- [tree-sitter](https://github.com/tree-sitter/tree-sitter) - Parser generator
- [ropey](https://github.com/cessen/ropey) - Rope data structure
- [glam](https://github.com/bitshifter/glam-rs) - SIMD math library

## 🌟 Show Your Support

If you like this project, please consider:

- ⭐ Starring the repository on GitHub
- 🐛 Reporting issues and suggesting features
- 🔧 Contributing code or documentation
- 📢 Sharing with others

---

**Made with ❤️ by the Lunar Editor Team**
```
