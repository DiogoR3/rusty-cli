use std::io::{self, Write};
use crate::options::{cat, echo};

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
        input = get_input();
        match input.parse::<i8>() {
            Ok(value) => { 
                *selected_option = value; 
                break;
            }
            Err(_) => println!("Invalid input, please enter a valid number."),
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string() // Remove any trailing newline characters
}

pub fn execute_option(option: i8) {
    match option {
        1 => echo::print(option.to_string()),
        2 => cat::concatenate(),
        0 => {},
        _ => println!("No option identified as {} was found!", option)
    }

}