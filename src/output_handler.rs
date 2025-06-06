use crate::ascii_mapping::{AsciiConfig, Charset};
use crate::ascii_to_image::AsciiToImageRenderer;
use ansi_to_html::Converter;
use html_escape::encode_text;
use image::Rgb;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

pub trait AsciiArtOutputFormat {
    fn write_to(&self, writer: &mut File, ascii_art: &str, config: &AsciiConfig) -> Result<(), Box<dyn Error>>;
    fn file_extension(&self) -> &str;
}

pub struct TxtFormat;
pub struct JsonFormat;
pub struct HtmlFormat;

pub struct ImageFormat {
    extension: String,
}

impl AsciiArtOutputFormat for TxtFormat {
    fn write_to(&self, writer: &mut File, ascii_art: &str, ascii_config: &AsciiConfig) -> Result<(), Box<dyn Error>> {
        writer.write_all(ascii_art.as_bytes())?;

        let charset_str = if ascii_config.charset == Charset::CUSTOM {
            &ascii_config.custom_charset
        } else {
            ascii_config.charset.as_str()
        };

        writeln!(writer, "\n{}", "-".repeat(50))?;
        writeln!(writer, "Generated by ASCII Art Generator v{}", env!("CARGO_PKG_VERSION"))?;
        writeln!(writer, "Charset: {}, Enable Color: {}, Invert Output: {}", charset_str, ascii_config.color, ascii_config.invert)?;

        Ok(())
    }

    fn file_extension(&self) -> &str {
        "txt"
    }
}

#[derive(Serialize, Deserialize)]
struct AsciiArtJson {
    info: String,
    version: String,
    config: AsciiConfigJson,
    ascii_art: String,
}

#[derive(Serialize, Deserialize)]
struct AsciiConfigJson {
    width: u32,
    height: u32,
    gamma: f32,
    charset: String,
    color_enable: bool,
    invert_output: bool,
}

impl AsciiArtOutputFormat for JsonFormat {
    fn write_to(&self, writer: &mut File, ascii_art: &str, config: &AsciiConfig) -> Result<(), Box<dyn Error>> {
        let charset_str = if config.charset == Charset::CUSTOM {
            config.custom_charset.clone()
        } else {
            config.charset.as_str().to_string()
        };

        let actual_height = if config.height == 0 {
            count_lines(ascii_art)
        } else {
            config.height
        };

        let json_data = AsciiArtJson {
            info: "Generated by ASCII Art Generator".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config: AsciiConfigJson {
                width: config.width,
                height: actual_height,
                gamma: config.gamma,
                charset: charset_str,
                color_enable: config.color,
                invert_output: config.invert,
            },
            ascii_art: ascii_art.to_string()
        };

        let json_string = serde_json::to_string_pretty(&json_data)?;

        writer.write_all(json_string.as_bytes())?;
        writeln!(writer)?;

        Ok(())
    }

    fn file_extension(&self) -> &str {
        "json"
    }
}

impl AsciiArtOutputFormat for HtmlFormat {
    fn write_to(&self, writer: &mut File, ascii_art: &str, config: &AsciiConfig) -> Result<(), Box<dyn Error>> {
        let charset_str = if config.charset == Charset::CUSTOM {
            &config.custom_charset
        } else {
            config.charset.as_str()
        };

        let actual_height = if config.height == 0 {
            count_lines(ascii_art)
        } else {
            config.height
        };

        let converter = Converter::new()
            .skip_escape(!config.color)  // 仅在无颜色时跳过 HTML 转义
            .skip_optimize(false);

        let html_content = if config.color {
            converter.convert(ascii_art).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        } else {
            // 无颜色时直接转义特殊 HTML 字符
            encode_text(ascii_art).to_string()
        };

        write!(
            writer,
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>ASCII Art - {}x{}</title>
    <style>
        body {{
            background-color: #000;
            color: #fff;
            font-family: monospace;
            margin: 0;
            padding: 0;
            display: flex;
            flex-direction: column;
            min-height: 100vh;
        }}
        .config {{
            background-color: #1a1a1a;
            padding: 15px 30px;
            margin: 20px;
            border-radius: 5px;
            font-family: sans-serif;
            white-space: normal;
            text-align: left; /* 文字左对齐 */
        }}
        .ascii-container {{
            flex: 1;
            display: flex;
            justify-content: center;
            align-items: center;
            padding: 20px;
            overflow-x: auto;
        }}
        .ascii-art {{
            line-height: 1.2;
            letter-spacing: 0.8px;
            text-align: left; /* 保持 ASCII 艺术内部的左对齐 */
        }}
        /* 定义 ANSI 颜色变量 */
        :root {{
            --black: #000000;
            --red: #cd0000;
            --green: #00cd00;
            --yellow: #cdcd00;
            --blue: #0000ee;
            --magenta: #cd00cd;
            --cyan: #00cdcd;
            --white: #e5e5e5;
            --bright-black: #7f7f7f;
            --bright-red: #ff0000;
            --bright-green: #00ff00;
            --bright-yellow: #ffff00;
            --bright-blue: #5c5cff;
            --bright-magenta: #ff00ff;
            --bright-cyan: #00ffff;
            --bright-white: #ffffff;
        }}
    </style>
</head>
<body>
    <div class="config">
        <h3>ASCII Art Configuration</h3>
        <p><strong>Generator:</strong> ASCII Art Generator v{}</p>
        <p><strong>Charset:</strong> {}</p>
        <p><strong>Dimensions:</strong> {}x{}</p>
        <p><strong>Gamma Correction:</strong> {}</p>
        <p><strong>Enable Color:</strong> {}</p>
        <p><strong>Invert Output:</strong> {}</p>
    </div>
    <div class="ascii-container">
        <pre class="ascii-art">
{}
        </pre>
    </div>
</body>
</html>
"#,
            config.width,
            actual_height,
            env!("CARGO_PKG_VERSION"),
            charset_str,
            config.width,
            actual_height,
            config.gamma,
            config.color,
            config.invert,
            html_content
        )?;

        Ok(())
    }

    fn file_extension(&self) -> &str {
        "html"
    }
}

fn count_lines(s: &str) -> u32 {
    if s.is_empty() {
        0
    } else {
        s.chars().filter(|&c| c == '\n').count() as u32
    }
}

impl AsciiArtOutputFormat for ImageFormat {
    fn write_to(&self, writer: &mut File, ascii_art: &str, config: &AsciiConfig) -> Result<(), Box<dyn Error>> {
        let mut renderer = AsciiToImageRenderer::new(config.clone(), 32)?
            .with_colors(
                Rgb([0x0C, 0x0C, 0x0C]),
                Rgb([0xCC, 0xCC, 0xCC])
            );

        let img = renderer.render_ascii_to_image(ascii_art);

        let format = match self.file_extension() {
            "png" => image::ImageFormat::Png,
            "jpg" | "jpeg" => image::ImageFormat::Jpeg,
            _ => image::ImageFormat::Png,
        };

        img?.write_to(writer, format)?;

        Ok(())
    }

    fn file_extension(&self) -> &str {
        match self.extension.as_str() {
            "jpg" => "jpg",
            "jpeg" => "jpeg",
            "png" => "png",
            _ => "png",
        }
    }
}

pub struct OutputHandler {
    format: Box<dyn AsciiArtOutputFormat>,
}

impl OutputHandler {
    pub fn new(format: Box<dyn AsciiArtOutputFormat>) -> Self {
        Self {format}
    }

    pub fn from_path(mut output_path: String) -> Result<(Self, String), Box<dyn Error>> {
        let mut path = PathBuf::from(&output_path);

        // 检查是否有扩展名
        let has_extension = path.extension().is_some();

        // 如果没有扩展名，添加默认扩展名".txt"
        let format = if !has_extension {
            let default_format = TxtFormat;
            path.set_extension(default_format.file_extension());
            output_path = path.to_string_lossy().into();
            Box::new(default_format) as Box<dyn AsciiArtOutputFormat>
        } else {
            // 根据扩展名选择格式
            match path.extension().and_then(|s| s.to_str()) {
                Some("txt") => Box::new(TxtFormat) as Box<dyn AsciiArtOutputFormat>,
                Some("json") => Box::new(JsonFormat) as Box<dyn AsciiArtOutputFormat>,
                Some("html") => Box::new(HtmlFormat) as Box<dyn AsciiArtOutputFormat>,
                Some("png" | "jpg" | "jpeg") => Box::new(ImageFormat {extension: path.extension().and_then(|ext| ext.to_str()).unwrap().to_string() }),
                Some(ext) => return Err(format!("Unsupported file extension: .{}", ext).into()),
                None => return Err("Failed to parse file extension".into()),
            }
        };

        Ok((Self::new(format), output_path))
    }

    pub fn save_ascii_art_to_file(&self, ascii_art: &str, output_path: &str, ascii_config: &AsciiConfig) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(output_path)?;
        self.format.write_to(&mut file, ascii_art, ascii_config)?;

        println!("ASCII Art saved to {}", output_path);
        Ok(())
    }
}
