use crate::ascii_mapping::AsciiConfig;
use image::{ImageBuffer, Rgb};
use rusttype::{Font, Scale};

pub struct AsciiToImageRenderer {
    config: AsciiConfig,
    font_size: u32,
    font_data: Vec<u8>,
    background_color: Rgb<u8>,
    foreground_color: Rgb<u8>,
}

impl AsciiToImageRenderer {
    pub fn new(config: AsciiConfig, font_size: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let font_data = include_bytes!("../res/DejaVuSansMono.ttf").to_vec();

        Ok(Self {
            config,
            font_size,
            font_data,
            background_color: Rgb([0x0C, 0x0C, 0x0C]),
            foreground_color: Rgb([0xCC, 0xCC, 0xCC]),
        })
    }
    
    pub fn with_colors(mut self, background: Rgb<u8>, foreground: Rgb<u8>) -> Self {
        self.background_color = background;
        self.foreground_color = foreground;
        self
    }

    // 辅助函数：从ANSI转义序列中提取RGB颜色
    fn parse_ansi_color(ansi_sequence: &str) -> Option<Rgb<u8>> {
        let parts: Vec<&str> = ansi_sequence.split(';').collect();
        if parts.len() == 5 && parts[0] == "\x1B[38" && parts[1] == "2" {
            if let (Some(r), Some(g), Some(b)) = (
                parts[2].parse::<u8>().ok(),
                parts[3].parse::<u8>().ok(),
                parts[4].strip_suffix('m').and_then(|s| s.parse::<u8>().ok()),
            ) {
                return Some(Rgb([r, g, b]));
            }
        }
        None
    }

    // 检查是否是重置颜色的ANSI序列([0m)
    fn is_reset_sequence(seq: &str) -> bool {
        seq == "\x1B[0m"
    }

    pub fn render_ascii_to_image(&mut self, ascii_art: &str) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
        let lines: Vec<&str> = ascii_art.lines().collect();
        let char_width = (self.font_size as f32 * 0.6) as u32;
        let width = self.config.width * char_width;
        let height = lines.len() as u32 * self.font_size;

        // 创建带有指定背景色的图像
        let mut img = ImageBuffer::from_pixel(width, height, self.background_color);

        // 加载字体
        let font = Font::try_from_vec(self.font_data.clone())
            .ok_or("Failed to load font")?;
        
        let scale = Scale {
            x: self.font_size as f32,
            y: self.font_size as f32,
        };

        // 获取字体的垂直度量信息
        let v_metrics = font.v_metrics(scale);
        let ascent = v_metrics.ascent;
        let descent = v_metrics.descent;
        let font_height = (ascent - descent).ceil() as u32;

        // 计算字符单元格的中心偏移量
        let cell_center_offset_y = self.font_size / 2;
        // 基于字体基线的偏移量
        let baseline_offset = (ascent - (font_height as f32 / 2.0)).ceil() as u32;

        // 渲染每个字符
        for (y, line) in lines.iter().enumerate() {
            let mut current_x = 0;
            let mut chars = line.chars().peekable();

            while let Some(c) = chars.next() {
                if c == '\x1B' { // 检查ANSI转义序列
                    let mut seq = String::from('\x1B');
                    while let Some(&next_c) = chars.peek() {
                        seq.push(next_c);
                        chars.next();
                        if next_c == 'm' {
                            break;
                        }
                    }

                    // 检查是否是重置序列，如果是则忽略它
                    if Self::is_reset_sequence(&seq) {
                        continue;
                    }

                    // 处理颜色序列
                    if let Some(color) = Self::parse_ansi_color(&seq) {
                        self.foreground_color = color;
                        continue;
                    }
                }

                if c == ' ' {
                    current_x += 1;
                    continue;
                }

                // 计算字符在图像中的位置
                let base_x = current_x as u32 * char_width;
                let base_y = y as u32 * self.font_size + cell_center_offset_y;

                // 获取字符的字形
                let glyph = font.glyph(c).scaled(scale);
                if glyph.id() != rusttype::GlyphId(0) {
                    // 计算字形的像素位置，基于基线对齐
                    let glyph_pos = glyph.positioned(rusttype::point(0.0, ascent));

                    // 获取字形的像素覆盖范围
                    if let Some(bb) = glyph_pos.pixel_bounding_box() {
                        // 计算字符的宽度和水平偏移
                        let glyph_width = bb.width() as u32;

                        // 安全计算水平偏移量
                        let h_offset = if glyph_width > char_width {
                            0 // 如果字形宽度大于单元格宽度，不应用额外偏移
                        } else {
                            ((char_width - glyph_width) / 2).saturating_sub(bb.min.x as u32)
                        };

                        // 渲染字形到图像
                        glyph_pos.draw(|gx, gy, v| {
                            // 修正后的坐标计算，添加水平偏移
                            let img_x = base_x + h_offset + gx;
                            let img_y = base_y - baseline_offset + gy;

                            // 确保在图像边界内
                            if img_x < width && img_y < height {
                                // 根据字符的透明度混合前景色和背景色
                                let alpha = v;
                                let r = ((self.foreground_color[0] as f32 * alpha) +
                                    (self.background_color[0] as f32 * (1.0 - alpha))) as u8;
                                let g = ((self.foreground_color[1] as f32 * alpha) +
                                    (self.background_color[1] as f32 * (1.0 - alpha))) as u8;
                                let b = ((self.foreground_color[2] as f32 * alpha) +
                                    (self.background_color[2] as f32 * (1.0 - alpha))) as u8;

                                img.put_pixel(img_x, img_y, Rgb([r, g, b]));
                            }
                        });
                    }
                }

                current_x += 1;
            }
        }

        Ok(img)
    }
}