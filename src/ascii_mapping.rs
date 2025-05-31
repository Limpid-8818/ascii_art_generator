use std::string::String;
use std::error::Error;
use image::{GenericImageView, Pixel};

const DEFAULT_CHARSET: &'static str = " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

pub struct AsciiConfig {
    pub width: u32,
    pub height: u32,
    pub gamma: f32,
    pub charset: String,
}

impl Default for AsciiConfig {
    fn default() -> Self {
        AsciiConfig {
            width: 50,
            height: 0,
            gamma: 1.0,
            charset: DEFAULT_CHARSET.to_string()
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
        let height = self.config.height;
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
                
                ascii_art.push(self.luminance_to_ascii(luminance))
            }
            ascii_art.push('\n')
        }

        Ok(ascii_art)
    }

    fn rgb_to_luminance(r: u32, g:u32, b: u32) -> u32 {
        (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u32
    }

    fn apply_gamma_correction(luminance: u32, gamma: f32) -> u32 {
        ((luminance as f32 / 255.0).powf(gamma) * 255.0) as u32
    }

    fn luminance_to_ascii(&self, luminance: u32) -> char {
        let charset = self.config.charset.chars().collect::<Vec<char>>();
        let index = (luminance as f32 * charset.len() as f32 / 255.0) as usize;
        charset[index.min(charset.len() - 1)]
    }
}