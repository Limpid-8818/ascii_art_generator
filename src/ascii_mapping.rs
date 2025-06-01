use std::string::String;
use std::error::Error;
use image::{GenericImageView, Pixel};

const ANSI_RESET: &str = "\x1b[0m";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Charset {
    SIMPLE,
    DEFAULT,
}

impl Charset {
    pub fn as_str(&self) -> &'static str {
        match self {
            Charset::SIMPLE => " .:-=+*#%@",
            Charset::DEFAULT => " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$",
        }
    }
}

impl std::str::FromStr for Charset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DEFAULT" => Ok(Charset::DEFAULT),
            "SIMPLE" => Ok(Charset::SIMPLE),
            _ => Err(format!("不支持或未定义的字符集: {s}"))
        }
    }
}

pub struct AsciiConfig {
    pub width: u32,
    pub height: u32,
    pub gamma: f32,
    pub charset: Charset,
    pub color: bool,
    pub invert: bool,
}

impl Default for AsciiConfig {
    fn default() -> Self {
        AsciiConfig {
            width: 80,
            height: 0,
            gamma: 1.0,
            charset: Charset::DEFAULT,
            color: false,
            invert: false,
        }
    }
}

pub struct AsciiMapper {
    config: AsciiConfig,
}

impl AsciiMapper {
    pub fn new(config: AsciiConfig) -> Self {
        AsciiMapper { config }
    }

    pub fn image_to_ascii(&self, img: &image::DynamicImage) -> Result<String, Box<dyn Error>> {
        let mut ascii_art = String::new();

        let width = self.config.width;
        let height = if self.config.height == 0 {
            self.dynamic_height(img)
        } else {
            self.config.height
        };
        let gamma = self.config.gamma;

        let width_ratio = img.width() as f32 / width as f32;
        let height_ratio = img.height() as f32 / height as f32;

        for y in 0..height {
            for x in 0..width {
                let base_x = (x as f32 * width_ratio) as u32;
                let base_y = (y as f32 * height_ratio) as u32;

                let mut total_r = 0;
                let mut total_g = 0;
                let mut total_b = 0;

                for yy in 0..height_ratio as u32 {
                    for xx in 0..width_ratio as u32 {
                        let pixel = img.get_pixel(base_x + xx, base_y +yy);
                        let rgb_channel = pixel.channels();
                        total_r += rgb_channel[0] as u32;
                        total_g += rgb_channel[1] as u32;
                        total_b += rgb_channel[2] as u32;
                    }
                }

                let pixel_count = width_ratio as u32 * height_ratio as u32;
                let avg_r = total_r / pixel_count;
                let avg_g = total_g / pixel_count;
                let avg_b = total_b / pixel_count;

                let mut luminance = Self::rgb_to_luminance(avg_r, avg_g, avg_b);
                luminance = Self::apply_gamma_correction(luminance, gamma);

                let ascii_char = self.luminance_to_ascii(luminance);

                if self.config.color {
                    // 添加 ANSI 转义序列实现彩色输出
                    let color_code = format!("\x1B[38;2;{};{};{}m", avg_r, avg_g, avg_b);
                    ascii_art.push_str(&color_code);
                    ascii_art.push(ascii_char);
                    ascii_art.push_str(ANSI_RESET);
                } else {
                    ascii_art.push(ascii_char);
                }
            }
            ascii_art.push('\n')
        }

        Ok(ascii_art)
    }

    fn rgb_to_luminance(r: u32, g: u32, b: u32) -> u32 {
        (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u32
    }

    fn apply_gamma_correction(luminance: u32, gamma: f32) -> u32 {
        ((luminance as f32 / 255.0).powf(gamma) * 255.0) as u32
    }

    fn luminance_to_ascii(&self, luminance: u32) -> char {
        let charset = if self.config.invert {
            self.config.charset.as_str().chars().rev().collect::<Vec<char>>()
        } else {
            self.config.charset.as_str().chars().collect::<Vec<char>>()
        };
        let index = (luminance as f32 * charset.len() as f32 / 255.0) as usize;
        charset[index.min(charset.len() - 1)]
    }

    fn dynamic_height(&self, image: &image::DynamicImage) -> u32 {
        let aspect_ratio = image.height() as f32 / image.width() as f32;
        let calculated_height = (self.config.width as f32 * aspect_ratio) as u32;
        calculated_height
    }
}