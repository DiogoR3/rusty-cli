use std::error::Error;

use crate::menu::OptionError;

pub fn print(parameters: &[String]) -> Result<(), Box<dyn Error>> {
    for parameter in parameters {
        println!("{}", parameter);
    }

    Err(Box::new(OptionError::InvalidOption))
    //Ok(())
}