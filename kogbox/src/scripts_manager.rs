// Dependencies
use std::fs;

// Create Scripts Directory
pub fn create_dir() -> std::io::Result<()> {
    fs::create_dir("./scripts")?;
    Ok(())
}