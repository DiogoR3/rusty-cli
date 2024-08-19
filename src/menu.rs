use std::io::{self, Write};
use crate::options::{cat, directories, echo};
use std::fmt;

pub enum OptionError {
    InvalidOption,
    MissingParameter,
}

pub struct Option {
    id: i8,
    text: &'static str,
    parameters_count: i8,
    pub function: std::option::Option<fn(&[String])>
} 

impl Option {
    pub fn get_parameters(&self) -> Result<Vec<String>, OptionError> {
        let mut parameters = Vec::new();
        let mut input = String::new();
        let mut parameters_remaining = self.parameters_count;

        while parameters_remaining > 0 {
            print!("Parameter #{}: ", self.parameters_count - parameters_remaining + 1);
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
    Option { id: 1, text: "Echo text", parameters_count: 1, function: Some(echo::print) },
    Option { id: 2, text: "Concatenate files", parameters_count: 3, function: Some(cat::concatenate_and_write_files) },
    Option { id: 3, text: "List directories", parameters_count: 1, function: Some(directories::list) },
    Option { id: 4, text: "Locate files", parameters_count: 1, function: Some(directories::locate_files) },
    Option { id: 5, text: "Find text", parameters_count: 1, function: Some(directories::find_text) },
    Option { id: 0, text: "Exit", parameters_count: 0, function: None },
];

impl fmt::Display for OptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionError::InvalidOption => write!(f, "The option provided is invalid."),
            OptionError::MissingParameter => write!(f, "A required parameter is missing."),
        }
    }
}

pub fn show_menu(show_creator_name: bool) { 
    if show_creator_name {
        let separator: String = "-".repeat(5);
        println!("{} Rusty CLI by Diogo Carreira {}", separator, separator)
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