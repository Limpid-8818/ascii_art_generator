use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn save_ascii_art_to_file(ascii_art: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(output_path)?;
    file.write_all(ascii_art.as_bytes())?;
    println!("ASCII Art saved to {}", output_path);
    Ok(())
}