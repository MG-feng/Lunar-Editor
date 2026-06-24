#!/bin/bash

echo "🔨 Building Lunar Editor..."

# 清理旧构建
cargo clean

# 构建 Release 版本
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📦 Binary location:"
    if [ -f "target/release/lunar-editor.exe" ]; then
        echo "   Windows: target/release/lunar-editor.exe"
    else
        echo "   Unix: target/release/lunar-editor"
    fi
else
    echo "❌ Build failed!"
    exit 1
fi
