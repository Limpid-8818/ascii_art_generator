use std::error::Error;
use clap::{Arg, Command};
use clap::builder::TypedValueParser;
use crate::ascii_mapping::AsciiConfig;

pub struct CliArgs {
    pub input_path: String,
    pub output_path: Option<String>,
    pub config: AsciiConfig,
}

pub fn parse_args() -> Result<CliArgs, Box<dyn Error>> {
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
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output path")
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
                .short('t')
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
        .arg(
            Arg::new("color")
                .short('c')
                .long("color")
                .help("Enable color output")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let input_path = matches
        .get_one::<String>("input")
        .ok_or_else(|| "Input file is required.")?
        .clone();

    let output_path_ref = matches
        .get_one::<String>("output");

    let output_path= match output_path_ref {
        Some(s) => Some(s.clone()),
        None => None
    };

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

    let color = matches.get_flag("color");

    let config = AsciiConfig {
        width,
        height,
        gamma,
        color,
        ..AsciiConfig::default()
    };

    Ok(CliArgs {
        input_path,
        output_path,
        config
    })
}