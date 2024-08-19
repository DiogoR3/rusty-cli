mod options;
mod menu;
use std::process;
use menu::get_option;

fn main() {
    let all_args: Vec<String> = std::env::args().collect();
    let all_args_except_first: &[String] = &all_args[1..];

    if all_args_except_first.len() >= 1 {
        for option in all_args_except_first {
            println!("{}", option);
        }

        process::exit(1);
    } 
    
    menu::show_menu(true);

    if let Ok(option) = get_option() {
        if let Ok(parameters) = option.get_parameters() {
            if let Some(func) = option.function {
                func(&parameters);
            } else {
                process::exit(0);
            }
        } else {
            eprintln!("Error: {}", option.get_parameters().err().unwrap());
        }
    } else {
        eprintln!("Error: {}", get_option().err().unwrap());
    }
}
