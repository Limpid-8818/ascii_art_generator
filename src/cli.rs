use std::error::Error;
use clap::{Arg, Command};
use clap::builder::TypedValueParser;
use crate::ascii_mapping::AsciiConfig;

pub fn parse_args() -> Result<AsciiConfig, Box<dyn Error>> {
    let matches = Command::new("ASCII Art Generator")
        .version("0.1.0")
        .author("Limpid")
        .about("A Tool for Converting Images to ASCII Art")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("Input image file")
                .required(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::new("width")
                .short('w')
                .long("width")
                .help("Width of the output ASCII art")
                .value_name("WIDTH")
                .default_value("80"),
        )
        .arg(
            Arg::new("height")
                .short('h')
                .long("height")
                .help("Height of the output ASCII art")
                .value_name("HEIGHT"),
        )
        .arg(
            Arg::new("gamma")
                .short('g')
                .long("gamma")
                .help("Gamma correction factor")
                .value_name("GAMMA")
                .default_value("1.0"),
        )
        .get_matches();

    let width = matches
        .get_one::<String>("width")
        .and_then(|w| w.parse::<u32>().ok())
        .ok_or_else(|| "Invalid width value.")?;

    let height = matches.get_one::<String>("height")
        .and_then(|h| h.parse::<u32>().ok())
        .unwrap_or(0);  // 0表示需要根据图像比例计算

    let gamma = matches.get_one::<String>("gamma")
        .and_then(|g| g.parse::<f32>().ok())
        .ok_or_else(|| "Invalid gamma value")?;
    
    let config = AsciiConfig {
        width,
        height,
        gamma,
        ..AsciiConfig::default()
    };
    
    Ok(config)
}