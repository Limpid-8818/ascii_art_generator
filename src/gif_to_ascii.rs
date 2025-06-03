use crate::ascii_mapping::{AsciiConfig, AsciiMapper};
use crate::ascii_to_image::AsciiToImageRenderer;
use image::codecs::gif::Repeat::Infinite;
use image::codecs::gif::{GifDecoder, GifEncoder};
use image::{AnimationDecoder, Delay, Frame, ImageBuffer, Rgba};
use std::error::Error;
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

        let file = File::create(output_path)?;
        let mut encoder = GifEncoder::new(file);
        encoder.set_repeat(Infinite)?;

        println!("Total Frames: {}", ascii_frames.len());
        let mut count = 0;
        for (frame, delay) in ascii_frames.into_iter().zip(delays) {
            let img = self.ascii_frame_to_img(&frame)?;
            let new_frame = Frame::from_parts(img, 0, 0, Delay::from_saturating_duration(Duration::from_millis(delay)));
            encoder.encode_frame(new_frame)?;
            println!("Render Frame {count} Succeed");
            count += 1;
        }
        
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