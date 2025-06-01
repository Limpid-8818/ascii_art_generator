mod ascii_mapping;
mod cli;
mod custom_charset_util;
mod output_handler;

use crate::ascii_mapping::AsciiMapper;
use crate::cli::parse_args;
use crate::output_handler::OutputHandler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;

    let config = args.config.clone();

    let img = image::open(&args.input_path)?;

    let mapper = AsciiMapper::new(args.config);

    let ascii_art = mapper.image_to_ascii(&img)?;

    if let Some(output_path) = args.output_path {
        let (handler, final_path)= OutputHandler::from_path(output_path)?;
        handler.save_ascii_art_to_file(&ascii_art, &final_path, &config)?;
    } else { 
        println!("{}", ascii_art);
    }
    
    Ok(())
}