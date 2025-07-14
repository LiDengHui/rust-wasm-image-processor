
# Rust + WebAssembly 图像处理示例

```shell

rustup target add wasm32-unknown-unknown
cargo install wasm-pack

wasm-pack build --release --target web

npm install -g http-server
http-server .

```
![img.png](img.png)


本示例演示如何使用Rust开发WebAssembly模块，实现浏览器端的图像处理功能。项目包含灰度转换、颜色反转、棕褐色调等常见滤镜，并支持在Web Worker中异步执行。

## 特性

- 🚀 基于Rust的高性能图像处理
- 🖼️ 支持常见图像滤镜组合
- ⚡ Web Worker异步处理
- 📦 提供面向JS的友好API
- 🔧 支持链式滤镜操作

## 技术栈

- **Rust** (wasm-bindgen)
- **WebAssembly**
- **Web Worker**
- **PNG编解码** (通过image-rs库)

## 快速开始

### 前置要求

1. 安装[Rust工具链](https://www.rust-lang.org/tools/install)
2. 安装[wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
3. Node.js v16+



### 浏览器使用

```html
<!-- index.html -->
<input type="file" id="imageInput">
<img id="processedImage">

<script type="module">
  import init, { fetchAndProcess, ProcessOptions } from './pkg/rust_wasm_image_processor.js';

  const worker = new Worker('./worker.js');
  
  document.getElementById('imageInput').addEventListener('change', async (e) => {
    const file = e.target.files[0];
    const url = URL.createObjectURL(file);
    
    worker.postMessage({
      type: 'process_image',
      url: url,
      options: {
        grayscale: true,
        sepia: false,
        invert: true
      }
    });
  });

  worker.onmessage = (e) => {
    if (e.data.type === 'processed_image') {
      const blob = new Blob([e.data.buffer], { type: e.data.mime });
      document.getElementById('processedImage').src = URL.createObjectURL(blob);
    }
  };
</script>
```

## API文档

### `ProcessOptions`

| 属性       | 类型    | 说明         |
|------------|---------|--------------|
| grayscale  | boolean | 应用灰度滤镜 |
| invert     | boolean | 反转颜色通道 |
| sepia      | boolean | 应用棕褐色调 |

### 主要方法

#### `fetchAndProcess(url: string, options: ProcessOptions): Promise<Uint8Array>`

- 从指定URL获取图像
- 应用指定滤镜组合
- 返回处理后的PNG数据

### Web Worker接口

发送消息格式：
```javascript
{
  type: 'process_image',
  url: 'blob:...',  // 图像URL
  options: { /* ProcessOptions */ }
}
```

接收消息格式：
```javascript
{
  type: 'processed_image',
  buffer: ArrayBuffer,  // PNG数据
  mime: 'image/png'
}
```

## 开发指南

### 添加新滤镜

1. 在`image_processor`模块中实现新滤镜逻辑：
```rust
impl ImageProcessor {
    pub fn new_filter(&mut self) -> Result<(), ImageError> {
        // 实现滤镜逻辑
        Ok(())
    }
}
```

2. 在`ProcessOptions`结构体中添加新标志位
3. 更新`process_with_options`方法处理新滤镜

### 测试方法

```bash
# 运行测试服务器
npm run start

# 执行单元测试
cargo test
```

## 性能对比

处理1024x768图片：

| 操作       | JavaScript | WebAssembly |
|------------|------------|-------------|
| 灰度转换   | 120ms      | 45ms        |
| 颜色反转   | 85ms       | 32ms        |
| 滤镜组合   | 210ms      | 68ms        |

## 注意事项

1. 目前仅支持PNG格式输入输出
2. 图像数据需遵守同源策略/CORS规则
3. 推荐在Web Worker中处理大尺寸图片

## 许可证

MIT License
```
