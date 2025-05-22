#!/bin/bash

# 清理旧构建
rm -rf pkg
# 复制静态文件
# 编译 WASM
echo "Building WASM..."
wasm-pack build --target web --release



echo "Build complete! Run:"
echo "npx http-server"
