pub mod recipie;
pub mod production;
pub mod input_file;
pub mod ressource;
pub mod target;

use std::fs::File;
use std::io::{prelude::*};

use input_file::InputFile;

pub fn read_from_file(path: &String) -> InputFile {
    let mut input_file = match File::open(path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut string_data = String::from("");
    let bla = input_file.read_to_string(&mut string_data);
    match bla {
        Ok(_) => {},
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let data = {
        match serde_json::from_str::<InputFile>(&string_data) {
            Ok(result) => result,
            Err(error) => panic!("Problem : {:?}", error),
        }
    };
    data
}