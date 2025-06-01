use std::error::Error;
use clap::{Arg, Command};
use crate::ascii_mapping::{AsciiConfig, Charset};
use crate::custom_charset_util::sort_charset_by_density;

pub struct CliArgs {
    pub input_path: String,
    pub output_path: Option<String>,
    pub config: AsciiConfig,
}

pub fn parse_args() -> Result<CliArgs, Box<dyn Error>> {
    let matches = Command::new("ASCII Art Generator")
        .version(env!("CARGO_PKG_VERSION"))
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
                .help("Output path (supports .txt(default), .json, .html extensions)")
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
        .arg(
            Arg::new("invert")
                .short('v')
                .long("invert")
                .help("Invert the character set")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("charset")
                .long("charset")
                .help("Character set to use (default, simple, block or pixel)")
                .value_name("CHARSET")
                .default_value("default")
        )
        .arg(
            Arg::new("custom-charset")
                .long("custom-charset")
                .help("Custom Character set to use ([option: --charset] will be ignored)")
                .value_name("CHARSET")
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

    let invert = matches.get_flag("invert");

    let mut custom_charset = matches.get_one::<String>("custom-charset")
        .unwrap_or(&String::new())
        .clone();

    let charset = if custom_charset.is_empty() {
        matches.get_one::<String>("charset")
            .and_then(|s| s.parse::<Charset>().ok())
            .ok_or_else(|| "Invalid charset value.")?
    } else {
        Charset::CUSTOM
    };

    // 自定义字符集处理
    if !custom_charset.is_empty() {
        custom_charset = sort_charset_by_density(custom_charset);
    }

    let config = AsciiConfig {
        width,
        height,
        gamma,
        color,
        charset,
        custom_charset,
        invert,
        ..AsciiConfig::default()
    };

    Ok(CliArgs {
        input_path,
        output_path,
        config
    })
}