use std::{fs, str::FromStr};

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};


pub fn print(file_path: &String) -> () {
    let data = read_file(file_path);
    let png = Png::try_from(&data[..]);
    match png {
        Ok(file) => println!("{}", file.to_string()),
        Err(e) => eprintln!("{}", e)
    }
}

pub fn encode(file_path: &String, chunk_type: &String, message: &String, output: Option<&String>) -> () {
    let data = read_file(file_path);
    let mut png = Png::try_from(&data[..]).expect("could not convert to png");
    let chunk_type = ChunkType::from_str(&chunk_type).expect("could not create chunk type");
    let chunk = Chunk::new(chunk_type, message.clone().into_bytes());
    png.append_chunk(chunk);
    match output {
       Some(path) => fs::write(path, png.as_bytes()),
       None => fs::write(file_path, png.as_bytes()),
    }.expect("could not write file");
}

fn read_file(file_path: &String) -> Vec<u8> {
    let data = fs::read(file_path);
    data.expect("could not open file")
}