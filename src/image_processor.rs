use std::io::Cursor;
use image::{ RgbaImage};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageError {
    #[error("image decoding failed")]
    DecodingError,
    #[error("image processing error: {0}")]
    ProcessingError(String),
}

#[derive(Debug, Clone)]
pub struct ImageProcessor {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl ImageProcessor {
    pub fn new(data: &[u8]) -> Result<ImageProcessor, String> {
        console_error_panic_hook::set_once();

        let img = image::load_from_memory(data)
            .map_err(|_e| ImageError::DecodingError.to_string())?;

        let rgba_img = img.to_rgba8();
        let (width, height) = rgba_img.dimensions();

        Ok(Self {
            width,
            height,
            data: rgba_img.into_raw(),
        })
    }

    /// 转换为灰度图
    pub fn grayscale(&mut self) -> Result<(), String> {
        self.process_pixels(|r, g, b, _| {
            // 灰度公式：Y = 0.299R + 0.587G + 0.114B
            let gray = (f32::from(r) * 0.299 +
                f32::from(g) * 0.587 +
                f32::from(b) * 0.114) as u8;
            (gray, gray, gray)
        })
    }

    /// 转换为复古（棕褐色）效果
    pub fn sepia(&mut self) -> Result<(), String> {
        self.process_pixels(|r, g, b, _| {
            let tr = (0.393 * r as f32 + 0.769 * g as f32 + 0.189 * b as f32).min(255.0) as u8;
            let tg = (0.349 * r as f32 + 0.686 * g as f32 + 0.168 * b as f32).min(255.0) as u8;
            let tb = (0.272 * r as f32 + 0.534 * g as f32 + 0.131 * b as f32).min(255.0) as u8;
            (tr, tg, tb)
        })
    }

    pub fn invert(&mut self) -> Result<(), String> {
        self.process_pixels(|r, g, b, _| {
            let tr = 255 - r;
            let tg = 255 - g;
            let tb = 255 - b;
            (tr, tg, tb)
        })
    }

    /// 获取处理后的PNG图像数据
    pub fn get_png_data(&self) -> Result<Vec<u8>, String> {
        let img_buffer = RgbaImage::from_raw(self.width, self.height, self.data.clone())
            .ok_or_else(|| ImageError::ProcessingError("Invalid image dimensions".to_string())).map_err(|e| e.to_string())?;

        let mut output = Cursor::new(Vec::<u8>::new());
        img_buffer.write_to(&mut output, image::ImageFormat::Png)
            .map_err(|e| ImageError::ProcessingError(e.to_string()).to_string())?;

        Ok(output.into_inner())
    }

    /// 通用像素处理函数
    fn process_pixels<F>(&mut self, mut processor: F) -> Result<(), String>
    where
        F: FnMut(u8, u8, u8, u8) -> (u8, u8, u8),
    {
        let pixels = &mut self.data;
        if pixels.len() % 4 != 0 {
            return Err(ImageError::ProcessingError(
                "Invalid pixel data format".to_string()
            ).to_string());
        }

        // 移除并行处理，WASM 环境不支持 rayon
        for chunk in pixels.chunks_exact_mut(4) {
            let (r, g, b, a) = (chunk[0], chunk[1], chunk[2], chunk[3]);
            let (new_r, new_g, new_b) = processor(r, g, b, a);
            chunk[0] = new_r;
            chunk[1] = new_g;
            chunk[2] = new_b;
            // Alpha 通道保持不变
        }

        Ok(())
    }
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grayscale_conversion() {
        // 创建测试图像（RGBA）
        let test_data = vec![
            255, 0, 0, 255,   // 红色像素
            0, 255, 0, 255,   // 绿色像素
            0, 0, 255, 255,   // 蓝色像素
        ];

        let processor = ImageProcessor::new(&test_data).unwrap();


        // 验证灰度值计算正确
        let expected_red = (255.0 * 0.299) as u8;
        let expected_green = (255.0 * 0.587) as u8;
        let expected_blue = (255.0 * 0.114) as u8;

        assert_eq!(&processor.data[0..4], &[expected_red, expected_red, expected_red, 255]);
        assert_eq!(&processor.data[4..8], &[expected_green, expected_green, expected_green, 255]);
        assert_eq!(&processor.data[8..12], &[expected_blue, expected_blue, expected_blue, 255]);
    }
}
