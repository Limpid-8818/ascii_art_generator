mod ascii_mapping;
mod cli;
mod custom_charset_util;
mod output_handler;
mod gif_to_ascii;
mod ascii_to_image;

use crate::ascii_mapping::AsciiMapper;
use crate::cli::parse_args;
use crate::gif_to_ascii::GifAsciiPlayer;
use crate::output_handler::OutputHandler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;

    let config = args.config.clone();

    let img = image::open(&args.input_path)?;
    
    let img_extension = args.input_path.split(".").last().unwrap();

    let mapper = AsciiMapper::new(args.config);

    let ascii_art = mapper.image_to_ascii(&img)?;

    if let Some(output_path) = args.output_path {
        let (handler, final_path)= OutputHandler::from_path(output_path)?;
        handler.save_ascii_art_to_file(&ascii_art, &final_path, &config)?;
    } else { 
        if img_extension == "gif" { 
            let player = GifAsciiPlayer::new(config);
            player.play_gif(&args.input_path, None)?;
        } else {
            println!("{}", ascii_art);
        }
    }
    
    Ok(())
}