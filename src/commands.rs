use std::fs;

use crate::png::Png;


pub fn print(file_path: &String) -> () {
    let data = fs::read(file_path);
    if data.is_err() {
        println!("could not open file");
        return;
    }
    let data = data.unwrap();
    let png = Png::try_from(&data[..]);
    match png {
        Ok(file) => println!("{}", file.to_string()),
        Err(e) => eprintln!("{}", e)
    }
}