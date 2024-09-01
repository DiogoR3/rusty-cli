use crate::menu::OptionError;
use colored::Colorize;
use std::{error::Error, fs, path::Path};

pub fn list(directories: &[String]) -> Result<(), Box<dyn Error>> {

    for directory in directories {
        let directory_with_slash = ensure_trailing_slash(directory);
        println!("Listing {} and {} at {}", "directories".red(), "files".bright_red(), directory_with_slash);
        let path = Path::new(&directory_with_slash);

        if !path.is_dir() {
            return Err(Box::new(OptionError::DirectoryDoesntExist(directory_with_slash.to_string())));
        }

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_name_str = entry.file_name().to_string_lossy().to_string();

            println!("{}", if entry.metadata()?.is_dir() { file_name_str.red() } else { file_name_str.bright_red() } );
        }
    }

    println!(); // Jump line
    Ok(())
}

fn ensure_trailing_slash(directory: &str) -> String {
    if directory.ends_with('/') {
        directory.to_string()
    } else {
        format!("{}{}", directory, '/')
    }
}

pub fn locate_files(_parameters: &[String]) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn find_text(_parameters: &[String]) -> Result<(), Box<dyn Error>> {
    Ok(())
}
