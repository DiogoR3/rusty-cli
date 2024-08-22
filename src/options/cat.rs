use std::{fs::{read_to_string, write}, io};
use std::{fmt, error::Error};
use colored::Colorize;
#[derive(Debug)]
pub struct FileError {
    filename: String,
    source: io::Error,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error with file '{}': {}", self.filename.bright_red(), self.source)
    }
}

impl Error for FileError {}

pub fn concatenate_and_write_files(paths: &[String]) -> Result<(), Box<dyn Error>>
{
    let content1 = read_to_string(&paths[0]).map_err(|e| Box::new(FileError { filename: paths[0].clone(), source: e }))?;
    let content2 = read_to_string(&paths[1]).map_err(|e| Box::new(FileError { filename: paths[1].clone(), source: e }))?;
    let combined_content = content1 + &content2;

    write(&paths[2], &combined_content).map_err(|e| Box::new(FileError { filename: paths[2].clone(), source: e }))?;

    Ok(())
}