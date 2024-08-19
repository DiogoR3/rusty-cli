use std::fs::{read_to_string, write};

pub fn concatenate_and_write_files(paths: &[String]) {
    let content1 = read_to_string(&paths[0]).unwrap();
    let content2 = read_to_string(&paths[1]).unwrap();
    let combined_content = content1 + &content2;

    write(&paths[2], combined_content);
}