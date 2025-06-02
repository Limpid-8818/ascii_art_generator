use crate::ascii_mapping::{AsciiConfig, AsciiMapper};
use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;
use std::error::Error;
use std::{
    fs::File,
    io::{self, BufReader, Write},
    thread::sleep,
    time::Duration,
};

pub struct GifAsciiPlayer {
    config: AsciiConfig,
}

impl GifAsciiPlayer {
    pub fn new(config: AsciiConfig) -> Self {
        GifAsciiPlayer { config }
    }

    pub fn play_gif(&self, path: &str, loops: Option<u32>) -> Result<(), Box<dyn Error>> {
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

    fn config_to_ascii(&self, img: &image::DynamicImage) -> Result<String, Box<dyn Error>> {
        let mapper = AsciiMapper::new(self.config.clone());
        mapper.image_to_ascii(img)
    }
}