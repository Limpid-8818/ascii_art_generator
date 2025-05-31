mod ascii_mapping;

use image::io::Reader as ImageReader;
use image::DynamicImage;
use crate::ascii_mapping::{AsciiConfig, AsciiMapper};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载图片
    let img: DynamicImage = ImageReader::open("images/b746283d2fbb9a42eb02250c0f39f1d2.jpeg")?.decode()?;

    // 创建默认配置
    let config = AsciiConfig::default();

    // 创建ASCII映射器
    let mapper = AsciiMapper::new(config);

    // 将图片转换为ASCII艺术
    let ascii_art = mapper.image_to_ascii(&img)?;

    // 打印ASCII艺术
    println!("{}", ascii_art);

    Ok(())
}