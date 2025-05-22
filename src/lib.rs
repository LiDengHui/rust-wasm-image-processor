// src/lib.rs
mod image_processor;

use wasm_bindgen::prelude::*;
use js_sys::{Uint8Array, Promise, Function, Reflect};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response, console};
use image_processor::{ImageProcessor, ImageError};
// 异步初始化 wasm_bindgen（适用于现代浏览器）
#[wasm_bindgen(start)]
pub fn init() {
    // 设置 panic 钩子
    console_error_panic_hook::set_once();

}

/// 核心图像处理函数（同步版）
#[wasm_bindgen]
pub fn process_image(data: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut processor = ImageProcessor::new(data)
        .map_err(|e| JsValue::from_str(&e))?;

    processor.sepia()
       .map_err(|e| JsValue::from_str(&e)).expect("TODO: panic message");

    processor.get_png_data()
        .map_err(|e| JsValue::from_str(&e))
}

/// 高级图像处理（支持滤镜链）
#[wasm_bindgen(js_name = "processWithOptions")]
pub fn process_with_options(
    data: &[u8],
    options: ProcessOptions
) -> Result<Vec<u8>, JsValue> {
    let mut processor = ImageProcessor::new(data)
        .map_err(|e| JsValue::from_str(&e))?;

    if options.grayscale {
        processor.grayscale()
            .map_err(|e| JsValue::from_str(&e))?;
    }

    if options.sepia {
        processor.sepia()
            .map_err(|e| JsValue::from_str(&e))?;
    }

    if options.invert {
        processor.invert()
            .map_err(|e| JsValue::from_str(&e))?;
    }

    processor.get_png_data()
        .map_err(|e| JsValue::from_str(&e))
}

/// 异步获取并处理图片（返回 Promise）
#[wasm_bindgen(js_name = "fetchAndProcess")]
pub async fn fetch_and_process(url: &str, options: ProcessOptions ) -> Result<JsValue, JsValue> {
    // 创建 Fetch 请求
    let  opts = RequestInit::new();
    opts.set_method("GET");
    let request = Request::new_with_str_and_init(url, &opts)?;

    // 发送请求
    let global = js_sys::global();

    let fetch: Function = Reflect::get(&global, &JsValue::from_str("fetch"))
       ?.dyn_into()?;
    // 调用 fetch
    let resp_promise: Promise = fetch.call1(&global, &request.into())?.dyn_into()?;
    let resp_value = JsFuture::from(resp_promise).await?;

    // 处理响应
    let resp: Response = resp_value.dyn_into()?;

    if !resp.ok() {
        return Err(JsValue::from_str(&format!(
            "HTTP Error: {}",
            resp.status()
        )));
    }

    // 获取 ArrayBuffer
    let array_buffer = wasm_bindgen_futures::JsFuture::from(
        resp.array_buffer()?
    ).await?;

    // 转换为 Uint8Array
    let bytes = Uint8Array::new(&array_buffer).to_vec();

    // 处理图像
    let processed = process_with_options(&bytes, options)?;

    // 返回处理结果
    Ok(Uint8Array::from(processed.as_slice()).into())
}

/// 面向对象接口
#[wasm_bindgen]
pub struct ImageProcessorWrapper {
    inner: ImageProcessor,
}

#[wasm_bindgen]
impl ImageProcessorWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(data: &[u8]) -> Result<ImageProcessorWrapper, JsValue> {
        Ok(Self {
            inner: ImageProcessor::new(data)
                .map_err(|e| JsValue::from_str(&e))?
        })
    }

    #[wasm_bindgen(js_name = "applyGrayscale")]
    pub fn apply_grayscale(&mut self) -> Result<(), JsValue> {
        self.inner.grayscale()
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = "getImageData")]
    pub fn get_image_data(&self) -> Result<Vec<u8>, JsValue> {
        self.inner.get_png_data()
            .map_err(|e| JsValue::from_str(&e))
    }
}

// 错误转换
impl From<ImageError> for JsValue {
    fn from(err: ImageError) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

#[wasm_bindgen]
pub struct ProcessOptions {
    grayscale: bool,
    sepia: bool,
    invert: bool
}


#[wasm_bindgen]
impl ProcessOptions {

    #[wasm_bindgen(constructor)]
    pub fn new () -> Self {
        Self {
            grayscale: false,
            sepia: false,
            invert: false
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_grayscale(&mut self, value: JsValue) {
        self.grayscale = value.is_truthy();
    }
    #[wasm_bindgen(setter)]
    pub fn set_sepia(&mut self, value: JsValue) {

        self.sepia = value.is_truthy();
    }
    #[wasm_bindgen(setter)]
    pub fn set_invert(&mut self, value: JsValue) {

        self.invert = value.is_truthy();
    }

    #[wasm_bindgen(getter)]
    pub fn grayscale(&self) -> JsValue {
        JsValue::from_bool(self.grayscale)
    }
    #[wasm_bindgen(getter)]
    pub fn sepia(&self) -> JsValue {
        JsValue::from_bool(self.sepia)
    }
    #[wasm_bindgen(getter)]
    pub fn invert(&self) -> JsValue  {
        JsValue::from_bool(self.invert)
    }
}
// 为 JS Promise 实现异步处理（高级用法）
#[wasm_bindgen]
pub async fn async_process(url: &str,  options: ProcessOptions) -> Promise {
    let url_clone = url.to_string();
    let future = async move {
        match fetch_and_process(&url_clone, options).await {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    };
    wasm_bindgen_futures::future_to_promise(future)
}
