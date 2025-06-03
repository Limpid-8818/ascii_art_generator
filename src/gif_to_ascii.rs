use crate::ascii_mapping::{AsciiConfig, AsciiMapper};
use crate::ascii_to_image::AsciiToImageRenderer;
use image::codecs::gif::Repeat::Infinite;
use image::codecs::gif::{GifDecoder, GifEncoder};
use image::{AnimationDecoder, Delay, Frame, ImageBuffer, Rgba};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::error::Error;
use std::time::Instant;
use std::{
    fs::File,
    io::{self, BufReader, Write},
    thread::sleep,
    time::Duration,
};

pub struct GifAsciiHandler {
    config: AsciiConfig,
}

impl GifAsciiHandler {
    pub fn new(config: AsciiConfig) -> Self {
        GifAsciiHandler { config }
    }

    fn gif_to_ascii(&self, path: &str) -> Result<(Vec<String>, Vec<u64>), Box<dyn Error>> {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        let decoder = GifDecoder::new(buf_reader)?;
        let frames = decoder.into_frames().collect_frames()?;

        // 预转换所有帧并缓存
        let mut ascii_frames = Vec::with_capacity(frames.len());
        let mut delays = Vec::with_capacity(frames.len());

        for frame in frames {
            let ascii = self.config_to_ascii(&image::DynamicImage::ImageRgba8(frame.clone().into_buffer()))?;
            ascii_frames.push(ascii);

            // 提取帧延迟时间
            let delay = frame.delay().numer_denom_ms().0 as u64;
            delays.push(delay);
        }

        Ok((ascii_frames, delays))
    }

    pub fn play_gif(&self, path: &str, loops: Option<u32>) -> Result<(), Box<dyn Error>> {
        let (ascii_frames, delays) = self.gif_to_ascii(path)?;

        let stdout = io::stdout();
        let mut handle = stdout.lock();

        let loop_count = match loops {
            Some(n) => 0..n,
            None => 0u32..u32::MAX,
        };
        
        for _ in loop_count {
            for (ascii, delay) in ascii_frames.clone().into_iter().zip(delays.clone()) {
                write!(handle, "\x1B[2J\x1B[H")?; // 清屏
                handle.write_all(ascii.as_bytes())?;
                handle.flush()?;
                sleep(Duration::from_millis(delay));
            }
        }
        Ok(())
    }

    pub fn export_to_gif(&self, input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
        let (ascii_frames, delays) = self.gif_to_ascii(input_path)?;
        println!("Total Frames: {}", ascii_frames.len());

        let file = File::create(output_path)?;
        let mut encoder = GifEncoder::new(file);
        encoder.set_repeat(Infinite)?;

        let timer = Instant::now();

        // 并行处理图像
        let frames_with_index: Vec<(usize, ImageBuffer<Rgba<u8>, Vec<u8>>)> = (0..ascii_frames.len())
            .into_par_iter() // 转换为并行迭代器
            .map(|i| {
                let img = self.ascii_frame_to_img(&ascii_frames[i]).expect("Failed to convert ASCII frame to image");
                (i, img)
            })
            .collect();

        // 根据原始索引对图像进行排序，确保顺序正确（Rayon库提供的并行迭代器大多数情况下不会改变顺序，但还是排序一下以确保不出错）
        let mut frames: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> = vec![ImageBuffer::new(0, 0); frames_with_index.len()];
        for (index, frame) in frames_with_index {
            frames[index] = frame;
        }

        let mut count = 1;
        // 编码GIF
        for (frame, delay) in frames.into_iter().zip(delays) {
            let sub_timer = Instant::now();
            let new_frame = Frame::from_parts(frame, 0, 0, Delay::from_saturating_duration(Duration::from_millis(delay)));
            encoder.encode_frame(new_frame)?;
            println!("Render Frame {}/{} succeed in {}", count, ascii_frames.len() , format_duration(sub_timer.elapsed()));
            count += 1;
        }

        println!("Rendering finished in {}", format_duration(timer.elapsed()));

        Ok(())
    }

    fn config_to_ascii(&self, img: &image::DynamicImage) -> Result<String, Box<dyn Error>> {
        let mapper = AsciiMapper::new(self.config.clone());
        mapper.image_to_ascii(img)
    }

    fn ascii_frame_to_img(&self, ascii: &str) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, Box<dyn Error>> {
        let mut renderer = AsciiToImageRenderer::new(self.config.clone(), 16)?;
        let img = renderer.render_ascii_to_image(ascii)?;
        let mut rgba_img = ImageBuffer::new(img.width(), img.height());
        for (x, y, pixel) in img.enumerate_pixels() {
            let rgba_pixel = Rgba([pixel[0], pixel[1], pixel[2], 255]);
            rgba_img.put_pixel(x, y, rgba_pixel);
        }
        let img = rgba_img;

        Ok(img)
    }
}

fn format_duration(d: Duration) -> String {
    let secs = d.as_secs();
    let millis = d.subsec_millis();
    let micros = d.subsec_micros() % 1000;
    let nanos = d.subsec_nanos() % 1000;

    if secs >= 60 {
        let minutes = secs / 60;
        let remaining_secs = secs % 60;
        if remaining_secs > 0 || millis > 0 {
            format!(
                "{}m{:.2}s",
                minutes,
                remaining_secs as f64 + millis as f64 * 1e-3
            )
        } else {
            format!("{}m", minutes)
        }
    } else if secs > 0 {
        format!("{:.2}s", secs as f64 + millis as f64 * 1e-3)
    } else if millis > 0 {
        format!("{:.2}ms", millis as f64 + micros as f64 * 1e-3)
    } else if micros > 0 {
        format!("{}μs", micros)
    } else {
        format!("{}ns", nanos)
    }
}