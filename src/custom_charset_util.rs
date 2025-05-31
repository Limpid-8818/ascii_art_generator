use image::{ImageBuffer, Luma};
use rusttype::{Font, Point, Scale};

pub fn sort_charset_by_density(charset: String) -> String {
    let font_data = include_bytes!("../res/DejaVuSansMono.ttf") as &[u8];
    let font = Font::try_from_bytes(font_data).expect("Fail to load font");

    let scale = Scale::uniform(24.0);

    let mut char_densities: Vec<(char, u32)> = charset
        .chars()
        .map(|c| {
            let density = calculate_char_density(&font, scale, c);
            (c, density)
        })
        .collect();

    char_densities.sort_by_key(|&(_, d)| d);
    char_densities.into_iter().map(|(c, _)| c).collect()
}

fn calculate_char_density(font: &Font, scale: Scale, c: char) -> u32 {
    let size = 32;
    let mut img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(size, size);

    for pixel in img.pixels_mut() {
        *pixel = Luma([255])
    }

    let point = Point { x: 0.0, y: scale.y };

    let glyph = font.glyph(c)
        .scaled(scale)
        .positioned(point);

    let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);
    let mut buf = vec![0.0; size as usize * size as usize];

    glyph.draw(|x, y, coverage| {
        let pixel_x = x;
        let pixel_y = y;

        // 更新边界
        if pixel_x < min_x { min_x = pixel_x; }
        if pixel_y < min_y { min_y = pixel_y; }
        if pixel_x > max_x { max_x = pixel_x; }
        if pixel_y > max_y { max_y = pixel_y; }

        // 将覆盖值存储到缓冲区中
        let index = (pixel_y * size + pixel_x) as usize;
        buf[index] = coverage;
    });

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pixel_x = x;
            let pixel_y = y;

            if pixel_x >= size || pixel_y >= size {
                continue;
            }

            let index = (pixel_y * size + pixel_x) as usize;
            let coverage = buf[index];

            let value = (255.0 * (1.0 - coverage)) as u8;
            img[(pixel_x, pixel_y)] = Luma([value]);
        }
    }

    img.pixels()
        .filter(|p| p[0] < 200)
        .count() as u32
}