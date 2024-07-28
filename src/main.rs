mod menu;
mod options;

fn main() {
    let all_args: Vec<String> = std::env::args().collect();
    let all_args_except_first: &[String] = &all_args[1..];

    if all_args_except_first.len() >= 1 {

    } else {
        let mut selected_option: i8 = 0;
        menu::show_menu(true, &mut selected_option);
        menu::execute_option(selected_option);
    }
}
