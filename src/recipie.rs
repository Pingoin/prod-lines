use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, Error};

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct Recipie {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub inputs: Vec<RessourceUsed>,
    #[serde(default)]
    pub outputs: Vec<RessourceUsed>,
    #[serde(default)]
    pub duration: f32,
}

#[derive(Deserialize, Serialize, Debug,Clone)]

pub struct Production {
    #[serde(default)]
    pub name: String,
    #[serde(default = "float1")]
    pub factor: f32,
    #[serde(default)]
    pub recipies: Vec<String>,
    #[serde(default = "default_true")]
    #[serde(rename = "devideRecipies")]
    pub devide_recipies: bool,
}

fn float1() -> f32 {
    1.0
}

fn default_true() -> bool {
    true
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct Ressource {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: f32,
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct RessourceUsed {
    #[serde(default)]
    pub ressource: String,
    #[serde(default)]
    pub amount: f32,
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct Target {
    #[serde(default)]
    pub ressource: String,
    #[serde(default)]
    pub input: bool,
    #[serde(default)]
    pub amount: f32,
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct InputFile {
    #[serde(default)]
    pub resources: HashMap<String,Ressource>,
    #[serde(default)]
    pub recepies: HashMap<String,Recipie>,
    #[serde(default)]
    pub targets: HashMap<String,Target>,
    #[serde(default)]
    pub productions: HashMap<String,Production>,
}

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

impl InputFile {
    pub fn write_to_file(&self, path: &String) -> Result<(), Error> {
        let json_data = match serde_json::to_string_pretty(self) {
            Ok(res) => res,
            Err(error) => return Err(Error::from(error)),
        };
        let mut input_file = match File::create(&path) {
            Ok(file) => file,
            Err(error) => return Err(error),
        };
        match input_file.write_all(&json_data.as_bytes()) {
            Ok(_) => {}
            Err(error) => return Err(error),
        };
        Ok(())
    }
}
