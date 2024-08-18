use std::io::{self, Write};
use crate::options::{cat, directories, echo};

pub fn show_menu(show_creator_name: bool, selected_option: &mut i8) { 

    if show_creator_name {
        let separator = "-".repeat(5);
        println!("{} Rusty CLI by Diogo Carreira {}", separator, separator)
    };

    println!("Choose one of the options below:");
    println!("1 - Echo the same written text"); // echo
    println!("2 - Concatenate files"); // cat
    println!("3 - List directories"); // ls
    println!("4 - Locate files or directories"); // find
    println!("5 - Find text in files"); // grep
    println!("0 - Exit\n");

    let mut input;
    loop {
        input = get_option();
        match input.parse::<i8>() {
            Ok(value) => { 
                *selected_option = value; 
                break;
            }
            Err(_) => println!("Invalid input, please enter a valid number."),
        }
    }
}

fn get_option() -> String {
    let mut input = String::new();
    print!("Enter a option: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string() // Remove any trailing newline characters
}

pub fn execute_option(option: i8) {
    match option {
        1 => echo::print(option.to_string()),
        2 => {
            match cat::concatenate_and_write_files("D:/FileA.txt", "D:/FileB.txt", "D:/TextC.txt") {
                Ok(path) => println!("Files concatenated and written to: {}", path),
                Err(e) => eprintln!("Error concatenating and writing files: {}", e),
            }
        }
        3 => directories::list(),
        4 => directories::locate_files(),
        5 => directories::find_text(),
        0 => {},
        _ => println!("No option identified as {} was found!", option)
    }

}