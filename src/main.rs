mod ascii_mapping;
mod cli;
mod custom_charset_util;
mod output_handler;
mod gif_to_ascii;
mod ascii_to_image;

use crate::ascii_mapping::AsciiMapper;
use crate::cli::parse_args;
use crate::gif_to_ascii::GifAsciiHandler;
use crate::output_handler::OutputHandler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;

    let config = args.config.clone();

    let img = image::open(&args.input_path)?;
    
    let img_extension = args.input_path.split(".").last().unwrap();

    let mapper = AsciiMapper::new(args.config);

    let ascii_art = mapper.image_to_ascii(&img)?;
    
    if img_extension == "gif" {
        if let Some(output_path) = args.output_path {
            if output_path.ends_with(".gif") {
                // gif输出
                println!("Exporting to gif...");
                let handler = GifAsciiHandler::new(config);
                handler.export_to_gif(&args.input_path, &output_path)?;
                println!("ASCII Art saved to {}", output_path);
            } else {
                // 常规输出
                let (handler, final_path)= OutputHandler::from_path(output_path)?;
                handler.save_ascii_art_to_file(&ascii_art, &final_path, &config)?;
            }
        } else {
            // gif播放
            let player = GifAsciiHandler::new(config);
            player.play_gif(&args.input_path, None)?;
        }
    } else { 
        if let Some(output_path) = args.output_path {
            let (handler, final_path)= OutputHandler::from_path(output_path)?;
            handler.save_ascii_art_to_file(&ascii_art, &final_path, &config)?;
        } else { 
            println!("{}", ascii_art)
        }
    }
    
    Ok(())
}