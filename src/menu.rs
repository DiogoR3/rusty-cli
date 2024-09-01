use std::{error::Error, io::{self, Write}, process};
use colored::Colorize;

use crate::options::{cat, files_directories, open_apps};
use std::fmt;

#[derive(Debug)]
pub enum OptionError {
    InvalidOption,
    FileDoesntExist(String),
    DirectoryDoesntExist(String)
    //MissingParameter,
}

impl Error for OptionError {}

pub struct Option {
    id: i8,
    text: &'static str,
    parameters_count: i8,
    pub function: std::option::Option<fn(&[String]) -> Result<(), Box<dyn std::error::Error>>>
} 

impl Option {
    pub fn get_parameters(&self) -> Result<Vec<String>, OptionError> {
        let mut parameters = Vec::new();
        let mut input = String::new();
        let mut parameters_remaining = self.parameters_count;

        while parameters_remaining > 0 {
            print!("Parameter #{}: ", (self.parameters_count - parameters_remaining + 1).to_string().cyan());
            io::stdout().flush().expect("Failed to flush stdout");
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let trimmed_input = input.trim();

            parameters.push(trimmed_input.to_string());
            parameters_remaining -= 1;
        }
        
        Ok(parameters)
    }
}

const OPTIONS: [Option; 6] = [
    Option { id: 1, text: "Open Apps", parameters_count: 0, function: Some(open_apps::run_apps) },
    Option { id: 2, text: "Concatenate files", parameters_count: 3, function: Some(cat::concatenate_and_write_files) },
    Option { id: 3, text: "List files and directories", parameters_count: 1, function: Some(files_directories::list) },
    Option { id: 4, text: "Locate files", parameters_count: 1, function: Some(files_directories::locate_files) },
    Option { id: 5, text: "Find text", parameters_count: 1, function: Some(files_directories::find_text) },
    Option { id: 0, text: "Exit", parameters_count: 0, function: Some(|_| -> Result<(), Box<dyn Error>> { process::exit(0) } as fn(&[String]) -> Result<(), Box<dyn Error>>) },
];

impl fmt::Display for OptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionError::InvalidOption => write!(f, "The option provided is invalid."),
            OptionError::FileDoesntExist(file_name) => write!(f, "File {} doesn't exist.", file_name),
            OptionError::DirectoryDoesntExist(directory) => write!(f, "Directory {} doesn't exist.", directory)
        }
    }
}

pub fn show_menu(show_creator_name: bool) { 
    if show_creator_name {
        let separator: String = "-".repeat(5);
        println!("{} {} {}", separator, "Rusty CLI by dcarreira".underline(), separator)
    };
    
    println!("Choose one of the options below:");
    for option in OPTIONS.iter() {
        println!("{} - {}", option.id, option.text);
    }
}

pub fn get_option() -> Result<&'static Option, OptionError> {
    let mut input = String::new();
    loop {
        print!("Enter an option: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<i8>() {
            Ok(value) => {
                if let Some(selected_option) = OPTIONS.iter().find(|&opt| opt.id == value) {
                    return Ok(selected_option);
                }

                return Err(OptionError::InvalidOption);
            }
            Err(_) => return Err(OptionError::InvalidOption)
        }
    }
}