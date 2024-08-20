mod options;
mod menu;
use std::{process, thread, time};
use menu::{get_option, show_menu};
use colored::Colorize;

fn main() {
    let all_args: Vec<String> = std::env::args().collect();
    let all_args_except_first: &[String] = &all_args[1..];

    if !all_args_except_first.is_empty() {
        for option in all_args_except_first {
            println!("{}", option);
        }
        process::exit(1);
    } 

    loop {
        show_menu(true);
        match get_option() {
            Ok(selected_option) => {
                match selected_option.get_parameters() {
                    Ok(parameters) => {
                        if let Some(func) = selected_option.function {
                            func(&parameters);
                        } else {
                            eprintln!("{}", "The selected option does not have a function".red());
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e.to_string().red());
                    }
                }
            },
            Err(e) => {
                eprintln!("{}", e.to_string().red());
                let one_second = time::Duration::from_secs(1);
                thread::sleep(one_second);
            }
        }
    };
}
