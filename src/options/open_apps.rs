use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;
use std::{thread, time};

use crate::menu::OptionError;

const FILE_NAME: &str = "apps.txt";

pub fn run_apps(_: &[String]) -> Result<(), Box<dyn Error>> {
    if !Path::new(FILE_NAME).exists() {
        File::create(FILE_NAME)?;
        println!("Created file {}", FILE_NAME);
        return Err(Box::new(OptionError::FileDoesntExist(FILE_NAME.to_string())));
    }

    let file = File::open(FILE_NAME)?;
    println!("Running script...");
    let reader = io::BufReader::new(file);
    let three_seconds = time::Duration::from_secs(3);

    for line in reader.lines() {
        let line = line?;
        let line_splitted: Vec<&str> = line.split(' ').collect();

        if let Some(program) = line_splitted.first() {
            match Command::new(program).args(&line_splitted[1..]).spawn() {
                Ok(_) => println!("{} (Success)", program),
                Err(e) => eprintln!("{} (Error: {})", program, e),
            }
        } else {
            println!(" (No command found in line)");
        }

        thread::sleep(three_seconds);
    }

    Ok(())
}
