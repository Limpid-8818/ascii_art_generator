mod ascii_mapping;
mod cli;

use crate::ascii_mapping::{AsciiConfig, AsciiMapper};
use crate::cli::parse_args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;

    let img = image::open(&args.input_path)?;

    let mapper = AsciiMapper::new(args.config);

    let ascii_art = mapper.image_to_ascii(&img)?;

    if let Some(output_path) = args.output_path {
        let path = output_path.clone();
        std::fs::write(output_path, ascii_art)?;
        println!("ASCII Art saved to {}", path);
    } else { 
        println!("{}", ascii_art);
    }
    
    Ok(())
}