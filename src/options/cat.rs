use std::fs::{read_to_string, write};
use std::io::Result;

pub fn concatenate_and_write_files(path1: &str, path2: &str, new_path: &str) -> Result<String> {
    let content1 = read_to_string(path1)?;
    let content2 = read_to_string(path2)?;
    let combined_content = content1 + &content2;

    write(new_path, combined_content)?;

    Ok(new_path.to_string())
}