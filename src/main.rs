mod menu;
mod options;
use colored::Colorize;
use menu::{get_option, show_menu, OptionError};
use std::{fmt::Display, process, thread, time};

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
            Ok(selected_option) => match selected_option.get_parameters() {
                Ok(parameters) => {
                    if let Some(func) = selected_option.function {
                        match func(&parameters) {
                            Ok(_) => continue,
                            Err(e) => print_err(e),
                        }
                    } else {
                        print_err("The selected option does not have a function");
                    }
                }
                Err(e) => print_err(e),
            },
            Err(e) => print_error_and_wait(e),
        }
    }
}

fn print_error_and_wait(e: OptionError) {
    print_err(e);
    let one_second = time::Duration::from_secs(1);
    thread::sleep(one_second);
}

fn print_err<E: Display>(e: E) {
    eprintln!("{}", e.to_string().red());
}
