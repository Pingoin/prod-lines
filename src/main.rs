use std::fs::File;
use std::io::prelude::*;
mod recipie;
use crate::recipie::{Recipie, Ressource, Target};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct InputFile {
    #[serde(default)]
    resources: Vec<Ressource>,
    #[serde(default)]
    recepies: Vec<Recipie>,
    #[serde(default)]
    targets: Vec<Target>,
}

fn main() -> std::io::Result<()> {
    let path = String::from("production.json");
    let mut input_file = match File::open(&path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut string_data = String::from("");
    let bla = input_file.read_to_string(&mut string_data);
    match bla {
        Ok(bldup) => println!("size:{}", bldup),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    

    let data = {
        match serde_json::from_str::<InputFile>(&string_data) {
            Ok(result) => result,
            Err(error) => panic!("Problem : {:?}", error),
        }
    };

    let json_data = match serde_json::to_string_pretty(&data) {
        Ok(res) => res,
        Err(_) => String::from("Müll"),
    };
    let mut input_file = match File::create(&path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file for writing: {:?}", error),
    };
    match input_file.write_all(&json_data.as_bytes()){
        Ok(_)=>{},
        Err(e)=>panic!("Problem writing file: {}",e),
    };
    Ok(())
}
